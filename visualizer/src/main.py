from __future__ import annotations

import json
import signal
import sys
import time
from pathlib import Path

import info_panel
import renderer
import terminal
from chafa_pipeline import ChafaPipeline
from ivy_engine import IvyEngine
from layout import build_layout


RUNNING = True


def _handle_signal(signum, frame) -> None:
    del signum, frame
    global RUNNING
    RUNNING = False


def load_config(repo_root: Path) -> dict:
    config_path = repo_root / "config/visualizer.json"
    return json.loads(config_path.read_text(encoding="utf-8"))


def _positive_float(value: object, fallback: float) -> float:
    try:
        parsed = float(value)
    except (TypeError, ValueError):
        return fallback
    return parsed if parsed > 0 else fallback


def load_timing(config: dict) -> dict[str, float]:
    timing = config.get("timing", {})
    return {
        "render_fps": _positive_float(timing.get("render_fps"), 12.0),
        "hero_fps": _positive_float(timing.get("hero_fps"), 0.5),
        "ivy_tick_seconds": _positive_float(timing.get("ivy_tick_seconds"), 3.0),
        "info_refresh_seconds": _positive_float(timing.get("info_refresh_seconds"), 1.0),
    }


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    config = load_config(repo_root)
    timing = load_timing(config)

    pipeline = ChafaPipeline(repo_root, config)
    hero_frames = pipeline.load_frames()
    hero_width = config["chafa"]["width"]
    hero_height = config["chafa"]["height"]
    if not hero_frames:
        hero_frames = [[" " * hero_width for _ in range(hero_height)]]

    ivy = IvyEngine(config)
    last_ivy_tick = 0.0
    last_info_tick = 0.0
    last_hero_tick = 0.0
    panel_lines = info_panel.build_panel_lines(config)
    frame_index = 0
    previous_size = None
    scene_layout = None

    signal.signal(signal.SIGINT, _handle_signal)
    signal.signal(signal.SIGTERM, _handle_signal)

    render_fps = timing["render_fps"]
    hero_fps = timing["hero_fps"]
    frame_delay = 1.0 / max(1, render_fps)
    hero_frame_delay = 1.0 / hero_fps if hero_fps > 0 else None

    with terminal.terminal_session():
        while RUNNING:
            size = terminal.get_size()
            if previous_size != size:
                scene_layout = build_layout(size, config, hero_width, hero_height)
                ivy.reset(size, scene_layout)
                previous_size = size

            assert scene_layout is not None

            now = time.time()
            if now - last_ivy_tick >= timing["ivy_tick_seconds"]:
                ivy.tick(scene_layout)
                last_ivy_tick = now

            if now - last_info_tick >= timing["info_refresh_seconds"]:
                panel_lines = info_panel.build_panel_lines(config)
                last_info_tick = now

            if hero_frame_delay is not None and now - last_hero_tick >= hero_frame_delay:
                frame_index = (frame_index + 1) % len(hero_frames)
                last_hero_tick = now

            scene = renderer.compose_scene(
                size=size,
                layout=scene_layout,
                hero_lines=hero_frames[frame_index],
                vine_segments=ivy.get_segments(),
                panel_lines=panel_lines,
                debug_enabled=bool(config.get("ivy", {}).get("debug", {}).get("enabled")),
            )
            terminal.move_home()
            sys.stdout.write(scene)
            terminal.flush()

            time.sleep(frame_delay)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
