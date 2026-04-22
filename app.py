"""Top-level YAM application entrypoint."""

from __future__ import annotations

import argparse
import sys
import time
from pathlib import Path

from config import SceneConfig, default_scene_config_path, load_scene_config
from engine.ecosystem import Ecosystem, Organism
from runtime.messages import KeyMsg, TickMsg
from runtime.terminal import live_session
from runtime.system import handle_message, render_frame_with_clock
from runtime.model import RuntimeModel
from ui.model import UIModel


def build_demo_model(width: int = 40, height: int = 20) -> RuntimeModel:
    return RuntimeModel(width=width, height=height, tick=0, seed=7)


def build_demo_ecosystem() -> Ecosystem:
    return Ecosystem(organisms=[])


def main() -> int:
    """Run the current YAM application."""
    parser = argparse.ArgumentParser(prog="yam-v2")
    parser.add_argument("--steps", type=int, default=0, help="number of tick steps to run; 0 means live loop")
    parser.add_argument("--width", type=int, default=40, help="demo width")
    parser.add_argument("--height", type=int, default=12, help="demo height")
    parser.add_argument("--fps", type=float, default=6.0, help="live loop frame rate")
    parser.add_argument("--clock-format", default="%H:%M", help="clock format")
    parser.add_argument("--day-format", default="%A, %d %B", help="day format")
    parser.add_argument("--theme", default="btas_dark_deco", help="theme name")
    parser.add_argument("--config", default="", help="scene config JSON path")
    parser.add_argument("--spawn", action="store_true", help="inject a deterministic seed via input path")
    args = parser.parse_args()

    model = build_demo_model(args.width, args.height)
    ecosystem = build_demo_ecosystem()
    ui = UIModel()
    repo_root = Path(__file__).resolve().parent
    scene_path = Path(args.config) if args.config else default_scene_config_path(repo_root)
    scene = load_scene_config(scene_path)
    if args.clock_format != "%H:%M" or args.day_format != "%A, %d %B" or args.theme != "btas_dark_deco":
        scene = SceneConfig(
            clock_font_name="Fender",
            clock_format=args.clock_format,
            day_format=args.day_format,
            theme_name=args.theme,
        )
    if args.spawn:
        model, ecosystem, ui = handle_message(model, ecosystem, ui, KeyMsg(key="spawn"))

    scene_mtime = scene_path.stat().st_mtime if scene_path.exists() else None

    def emit_frame() -> None:
        frame = render_frame_with_clock(model, ecosystem, ui, scene)
        sys.stdout.write("\x1b[H\x1b[2J")
        sys.stdout.write(frame)
        sys.stdout.flush()

    def reload_scene_if_needed() -> None:
        nonlocal scene, scene_mtime
        if not scene_path.exists():
            return
        current_mtime = scene_path.stat().st_mtime
        if scene_mtime is None or current_mtime != scene_mtime:
            scene = load_scene_config(scene_path)
            scene_mtime = current_mtime

    if args.steps > 0:
        reload_scene_if_needed()
        for _ in range(args.steps):
            model, ecosystem, ui = handle_message(model, ecosystem, ui, TickMsg())
        print(render_frame_with_clock(model, ecosystem, ui, scene))
        return 0

    interval = 1.0 / max(1.0, args.fps)
    try:
        with live_session():
            while True:
                reload_scene_if_needed()
                emit_frame()
                time.sleep(interval)
                model, ecosystem, ui = handle_message(model, ecosystem, ui, TickMsg())
    except KeyboardInterrupt:
        return 0

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
