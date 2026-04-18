from __future__ import annotations

import json
import time
from pathlib import Path

import info_panel
import renderer
import terminal
from ivy_particles import update_falling_leaves, render_falling_leaves
from chafa_pipeline import ChafaPipeline
from ivy_engine import IvyEngine
from layout import build_layout


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]

    config_path = repo_root / "config/visualizer.json"
    config = json.loads(config_path.read_text(encoding="utf-8"))
    config_mtime = config_path.stat().st_mtime

    def refresh_runtime_state(current_config: dict) -> tuple[
        ChafaPipeline,
        list[list[str]],
        int,
        int,
        IvyEngine,
        list[str],
        float,
        float,
        float,
        float,
    ]:
        timing = current_config.get("timing", {})
        render_fps = float(timing.get("render_fps", 12))
        hero_fps = float(timing.get("hero_fps", 0.5))
        ivy_tick_seconds = float(timing.get("ivy_tick_seconds", 3.0))
        info_refresh_seconds = float(timing.get("info_refresh_seconds", 1.0))

        pipeline = ChafaPipeline(repo_root, current_config)
        hero_frames = pipeline.load_frames()
        hero_width = current_config["chafa"]["width"]
        hero_height = current_config["chafa"]["height"]
        if not hero_frames:
            hero_frames = [[" " * hero_width for _ in range(hero_height)]]
        ivy = IvyEngine(current_config)
        panel_lines = info_panel.build_panel_lines(current_config)
        return (
            pipeline,
            hero_frames,
            hero_width,
            hero_height,
            ivy,
            panel_lines,
            render_fps,
            hero_fps,
            ivy_tick_seconds,
            info_refresh_seconds,
        )

    (
        pipeline,
        hero_frames,
        hero_width,
        hero_height,
        ivy,
        panel_lines,
        render_fps,
        hero_fps,
        ivy_tick_seconds,
        info_refresh_seconds,
    ) = refresh_runtime_state(config)

    last_ivy_tick = 0.0
    last_info_tick = 0.0
    last_hero_tick = 0.0

    frame_index = 0

    previous_size = None
    scene_layout = None
    config_changed = False

    with terminal.terminal_session():
        while True:
            current_mtime = config_path.stat().st_mtime
            if current_mtime != config_mtime:
                config = json.loads(config_path.read_text(encoding="utf-8"))
                config_mtime = current_mtime
                (
                    pipeline,
                    hero_frames,
                    hero_width,
                    hero_height,
                    ivy,
                    panel_lines,
                    render_fps,
                    hero_fps,
                    ivy_tick_seconds,
                    info_refresh_seconds,
                ) = refresh_runtime_state(config)
                frame_index = 0
                last_ivy_tick = 0.0
                last_info_tick = 0.0
                last_hero_tick = 0.0
                config_changed = True

            size = terminal.get_size()

            if previous_size != size or config_changed:
                scene_layout = build_layout(size, config, hero_width, hero_height)
                ivy.reset(size, scene_layout)
                previous_size = size
                config_changed = False

            assert scene_layout is not None

            now = time.time()

            if now - last_ivy_tick >= ivy_tick_seconds:
                ivy.tick(scene_layout)
                last_ivy_tick = now

            # update particles every frame
            update_falling_leaves(ivy.state, scene_layout, ivy.rng)

            if now - last_info_tick >= info_refresh_seconds:
                panel_lines = info_panel.build_panel_lines(config)
                last_info_tick = now

            hero_frame_delay = 1.0 / hero_fps if hero_fps > 0 else None
            if hero_frame_delay is not None and now - last_hero_tick >= hero_frame_delay:
                frame_index = (frame_index + 1) % len(hero_frames)
                last_hero_tick = now

            scene = renderer.compose_scene(
                size=size,
                layout=scene_layout,
                hero_lines=hero_frames[frame_index],
                vine_segments=ivy.get_segments(),
                panel_lines=panel_lines,
                config=config,
                falling_leaf_segments=render_falling_leaves(ivy.state, scene_layout),
                debug_enabled=bool(config.get("ivy", {}).get("debug", {}).get("enabled")),
            )

            terminal.move_home()
            print(scene, end="", flush=True)

            frame_delay = 1.0 / max(1, render_fps)
            time.sleep(frame_delay)


if __name__ == "__main__":
    raise SystemExit(main())
