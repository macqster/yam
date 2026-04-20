"""Scene configuration for the v2 scaffold."""

from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path


@dataclass(slots=True)
class SceneConfig:
    """Small explicit scene config surface."""

    gif_path: Path
    clock_format: str = "%H:%M"
    theme_name: str = "btas_dark_deco"


def default_scene_config_path(repo_root: Path) -> Path:
    """Return the repo-tracked default scene config path."""
    return repo_root / "v2" / "scene_config.json"


def load_scene_config(path: Path) -> SceneConfig:
    """Load scene config from JSON, falling back to defaults if absent."""
    if not path.exists():
        return SceneConfig(gif_path=Path("visualizer/assets/source.gif"))

    data = json.loads(path.read_text(encoding="utf-8"))
    gif_path = Path(data.get("gif_path", "visualizer/assets/source.gif"))
    clock_format = str(data.get("clock_format", "%H:%M"))
    theme_name = str(data.get("theme_name", "btas_dark_deco"))
    return SceneConfig(gif_path=gif_path, clock_format=clock_format, theme_name=theme_name)


def dump_scene_config(config: SceneConfig, path: Path) -> None:
    """Write scene config to JSON."""
    payload = {
        "gif_path": str(config.gif_path),
        "clock_format": config.clock_format,
        "theme_name": config.theme_name,
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def update_scene_config_value(path: Path, key: str, value: str) -> SceneConfig:
    """Update one supported scene config key and persist the file."""
    config = load_scene_config(path)
    if key == "gif_path":
        config.gif_path = Path(value)
    elif key == "clock_format":
        config.clock_format = value
    elif key == "theme_name":
        config.theme_name = value
    else:
        raise ValueError(f"unsupported scene config key: {key}")
    dump_scene_config(config, path)
    return config
