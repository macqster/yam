"""Verification path for the v2 clock snapshot helper."""

from __future__ import annotations

from dataclasses import replace
from datetime import datetime
import subprocess
from pathlib import Path

from engine.ecosystem import Ecosystem, Organism
from config import SceneConfig
from render.composer import compose_frame
from render.clock_font import render_clock
from render.text_overlay import TextOverlay
from runtime.messages import KeyMsg, ResizeMsg, TickMsg
from runtime.model import RuntimeModel
from ui.model import UIModel
from ui.router import UIRouter
from theme.model import theme_by_name
from wcwidth import wcswidth


def _render_go_frame(width: int, height: int, clock_value: str, day_value: str) -> str:
    repo_root = Path(__file__).resolve().parents[1]
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
        cwd=repo_root,
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
    )
    theme_by_name(scene.theme_name)
    clock_value = clock_text or datetime.now().strftime(scene.clock_format)
    day_value = _polish_day_label(datetime.now())
    try:
        return _render_go_frame(model.width, model.height, clock_value, day_value)
    except Exception:
        clock_text_block = render_clock(clock_value)
        clock_width = max((len(line) for line in clock_text_block.splitlines()), default=0)
        clock_height = len(clock_text_block.splitlines())
        clock_y = max(0, model.height // 2 - clock_height // 2 - 1)
        clock_x = max(0, model.width // 2 - clock_width // 2)
        clock = []
        for idx, line in enumerate(clock_text_block.splitlines()):
            clock.extend(TextOverlay(x=clock_x, y=clock_y + idx, text=line).shapes())
        day = TextOverlay(x=max(0, clock_x + (clock_width - max(0, wcswidth(day_value))) // 2), y=clock_y + clock_height, text=day_value).shapes()
        return compose_frame(
            model.width,
            model.height,
            [*clock, *day],
        )
