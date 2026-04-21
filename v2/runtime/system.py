"""Legacy Python verification scaffolding for the v2 clock snapshot path."""

from __future__ import annotations

from dataclasses import replace
from datetime import datetime
import subprocess
from pathlib import Path

from v2.engine.ecosystem import Ecosystem, Organism
from v2.config import SceneConfig
from v2.render.composer import compose_frame
from v2.render.clock_font import render_clock
from v2.render.text_overlay import TextOverlay
from v2.runtime.messages import KeyMsg, ResizeMsg, TickMsg
from v2.runtime.model import RuntimeModel
from v2.shape.model import build_shapes
from v2.ui.model import UIModel
from v2.ui.overlay import ui_overlay_shapes
from v2.ui.router import UIRouter
from v2.theme.model import theme_by_name


def _render_go_frame(width: int, height: int, clock_value: str, day_value: str) -> str:
    repo_root = Path(__file__).resolve().parents[2]
    return subprocess.run(
        [
            "go",
            "run",
            "./cmd/yamv2",
            "--once",
            "--width",
            str(width),
            "--height",
            str(height),
            "--clock",
            clock_value,
            "--day",
            day_value,
        ],
        cwd=repo_root / "v2",
        check=True,
        capture_output=True,
        text=True,
    ).stdout.rstrip("\n")


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
        clock_font_name="Fender",
        gif_path=Path(__file__).resolve().parents[2] / "visualizer" / "assets" / "source.gif",
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
        clock_font_name="Fender",
        gif_path=Path(__file__).resolve().parents[2] / "visualizer" / "assets" / "source.gif",
    )
    theme_by_name(scene.theme_name)
    clock_value = clock_text or datetime.now().strftime(scene.clock_format)
    day_value = datetime.now().strftime(scene.day_format)
    try:
        return _render_go_frame(model.width, model.height, clock_value, day_value)
    except Exception:
        clock_text_block = render_clock(clock_value)
        clock_width = max((len(line) for line in clock_text_block.splitlines()), default=0)
        clock_y = max(2, model.height // 6)
        clock_x = max(0, (model.width - clock_width) // 2)
        clock = []
        for idx, line in enumerate(clock_text_block.splitlines()):
            clock.extend(TextOverlay(x=clock_x, y=clock_y + idx, text=line).shapes())
        day = TextOverlay(x=max(0, (model.width - len(day_value)) // 2), y=clock_y + 8, text=day_value).shapes()
        controls_value = "0123456789"
        controls_block = render_clock(controls_value)
        controls_width = max((len(line) for line in controls_block.splitlines()), default=0)
        controls_y = max(0, model.height - 8)
        controls_x = max(0, (model.width - controls_width) // 2)
        controls = []
        for idx, line in enumerate(controls_block.splitlines()):
            controls.extend(TextOverlay(x=controls_x, y=controls_y + idx, text=line).shapes())
        ui_shapes = ui_overlay_shapes(ui) if ui is not None else []
        return compose_frame(
            model.width,
            model.height,
            [*build_shapes(ecosystem.organisms), *ui_shapes, *clock, *day, *controls],
        )
