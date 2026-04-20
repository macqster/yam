"""Runtime message handling for the v2 scaffold."""

from __future__ import annotations

from dataclasses import replace
from datetime import datetime
from pathlib import Path

from v2.engine.ecosystem import Ecosystem, Organism
from v2.config import SceneConfig
from v2.render.gif_renderer import GifRenderer
from v2.render.composer import compose_frame
from v2.render.text_overlay import TextOverlay
from v2.runtime.messages import KeyMsg, ResizeMsg, TickMsg
from v2.runtime.model import RuntimeModel
from v2.shape.model import build_shapes
from v2.ui.model import UIModel
from v2.ui.overlay import ui_overlay_shapes
from v2.ui.router import UIRouter
from v2.theme.model import theme_by_name


def handle_message(
    model: RuntimeModel,
    ecosystem: Ecosystem,
    ui: UIModel,
    msg: TickMsg | ResizeMsg | KeyMsg,
) -> tuple[RuntimeModel, Ecosystem, UIModel]:
    """Return the next runtime, ecosystem, and UI state."""
    router = UIRouter()

    if isinstance(msg, TickMsg):
        next_model = replace(model, tick=model.tick + 1)
        next_ecosystem = ecosystem.step(next_model.width, next_model.height)
        return next_model, next_ecosystem, ui

    if isinstance(msg, ResizeMsg):
        return replace(model, width=msg.width, height=msg.height), ecosystem, ui

    if isinstance(msg, KeyMsg) and msg.key == "spawn":
        spawned = Organism(
            name=f"seed-{model.tick}",
            x=min(model.width - 1, max(0, model.width // 2 + 1)),
            y=min(model.height - 1, max(0, model.height // 2 + 2)),
            glyph="·",
        )
        return model, Ecosystem(
            organisms=[*ecosystem.organisms, spawned],
            environment=ecosystem.environment,
            balance=ecosystem.balance,
        ), ui

    if isinstance(msg, KeyMsg):
        return model, ecosystem, router.handle(ui, msg)

    return model, ecosystem, ui


def render_frame(model: RuntimeModel, ecosystem: Ecosystem, ui: UIModel | None = None) -> str:
    """Render the current ecosystem state into a text frame."""
    default_scene = SceneConfig(
        gif_path=Path(__file__).resolve().parents[2] / "visualizer" / "assets" / "source.gif"
    )
    return render_frame_with_clock(model, ecosystem, ui, default_scene)


def render_frame_with_clock(
    model: RuntimeModel,
    ecosystem: Ecosystem,
    ui: UIModel | None = None,
    scene: SceneConfig | None = None,
    clock_text: str | None = None,
) -> str:
    """Render the current ecosystem state into a text frame with an optional clock override."""
    scene = scene or SceneConfig(
        gif_path=Path(__file__).resolve().parents[2] / "visualizer" / "assets" / "source.gif"
    )
    theme_by_name(scene.theme_name)
    gif_shapes = GifRenderer(scene.gif_path, model.width, model.height).render(model.tick)
    clock_value = clock_text or datetime.now().strftime(scene.clock_format)
    clock = TextOverlay(x=max(0, model.width - 8), y=0, text=clock_value).shapes()
    ui_shapes = ui_overlay_shapes(ui) if ui is not None else []
    return compose_frame(
        model.width,
        model.height,
        [*gif_shapes, *build_shapes(ecosystem.organisms), *ui_shapes, *clock],
    )
