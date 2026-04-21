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


def _polish_day_label(now: datetime) -> str:
    weekdays = {
        0: "poniedziałek",
        1: "wtorek",
        2: "środa",
        3: "czwartek",
        4: "piątek",
        5: "sobota",
        6: "niedziela",
    }
    months = {
        1: "stycznia",
        2: "lutego",
        3: "marca",
        4: "kwietnia",
        5: "maja",
        6: "czerwca",
        7: "lipca",
        8: "sierpnia",
        9: "września",
        10: "października",
        11: "listopada",
        12: "grudnia",
    }
    return f"{weekdays[now.weekday()]}, {now.day} {months[now.month]}"


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
    day_value = _polish_day_label(datetime.now())
    try:
        return _render_go_frame(model.width, model.height, clock_value, day_value)
    except Exception:
        clock_text_block = render_clock(clock_value)
        clock_width = max((len(line) for line in clock_text_block.splitlines()), default=0)
        clock_y = max(0, model.height // 4)
        clock_x = max(0, (model.width * 3) // 4 - clock_width // 2)
        clock = []
        for idx, line in enumerate(clock_text_block.splitlines()):
            clock.extend(TextOverlay(x=clock_x, y=clock_y + idx, text=line).shapes())
        day = TextOverlay(x=max(0, clock_x + (clock_width - len(day_value)) // 2), y=clock_y + 6, text=day_value).shapes()
        footer_text = "q quit •   space pause"
        footer_x = max(0, (model.width - len(footer_text)) // 2)
        footer = TextOverlay(x=footer_x, y=max(0, model.height - 2), text=footer_text).shapes()
        ui_shapes = ui_overlay_shapes(ui) if ui is not None else []
        return compose_frame(
            model.width,
            model.height,
            [*build_shapes(ecosystem.organisms), *ui_shapes, *clock, *day, *footer],
        )
