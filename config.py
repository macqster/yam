"""Scene configuration for the current YAM runtime."""

from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path


@dataclass(slots=True)
class SceneConfig:
    """Small explicit scene config surface."""

    clock_font_name: str
    day_format: str = "%A, %d %B"
    clock_format: str = "%H:%M"
    theme_name: str = "btas_dark_deco"


def default_scene_config_path(repo_root: Path) -> Path:
    """Return the repo-tracked default scene config path."""
    return repo_root / "scene_config.json"


def load_scene_config(path: Path) -> SceneConfig:
    """Load scene config from JSON, falling back to defaults if absent."""
    if not path.exists():
        return SceneConfig(
            clock_font_name="Fender",
            day_format="%A, %d %B",
        )

    data = json.loads(path.read_text(encoding="utf-8"))
    clock_font_name = str(data.get("clock_font_name", "Fender"))
    day_format = str(data.get("day_format", "%A, %d %B"))
    clock_format = str(data.get("clock_format", "%H:%M"))
    theme_name = str(data.get("theme_name", "btas_dark_deco"))
    return SceneConfig(
        clock_font_name=clock_font_name,
        day_format=day_format,
        clock_format=clock_format,
        theme_name=theme_name,
    )


def dump_scene_config(config: SceneConfig, path: Path) -> None:
    """Write scene config to JSON."""
    payload = {
        "clock_font_name": config.clock_font_name,
        "day_format": config.day_format,
        "clock_format": config.clock_format,
        "theme_name": config.theme_name,
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def update_scene_config_value(path: Path, key: str, value: str) -> SceneConfig:
    """Update one supported scene config key and persist the file."""
    config = load_scene_config(path)
    if key == "clock_font_name":
        config.clock_font_name = value
    elif key == "day_format":
        config.day_format = value
    elif key == "clock_format":
        config.clock_format = value
    elif key == "theme_name":
        config.theme_name = value
    else:
        raise ValueError(f"unsupported scene config key: {key}")
    dump_scene_config(config, path)
    return config
