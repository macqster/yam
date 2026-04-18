#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import tempfile
import sys
from pathlib import Path


LAYOUT_ALIASES = {
    "support_mask_path": "trunk_mask_path",
    "support_mask_threshold": "trunk_mask_threshold",
    "support_mask_scale_x": "trunk_mask_scale_x",
    "support_mask_scale_y": "trunk_mask_scale_y",
    "support_mask_offset_x": "trunk_mask_offset_x",
    "support_mask_offset_y": "trunk_mask_offset_y",
}


def load_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        raise SystemExit(f"ERROR: missing config file: {path}")
    except json.JSONDecodeError as exc:
        raise SystemExit(f"ERROR: invalid JSON in {path}: {exc}")


def merge_dicts(base: dict, overlay: dict) -> dict:
    merged = dict(base)
    for key, value in overlay.items():
        if isinstance(value, dict) and isinstance(merged.get(key), dict):
            merged[key] = merge_dicts(merged[key], value)
        else:
            merged[key] = value
    return merged


def is_numeric(value: object) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def add_issue(issues: list[tuple[str, str]], level: str, message: str) -> None:
    issues.append((level, message))


def validate_known_keys(
    data: dict,
    template: dict,
    path: str,
    issues: list[tuple[str, str]],
) -> None:
    for key, value in data.items():
        if key.startswith("_"):
            continue
        if key not in template and not (path.endswith(".layout") and key in LAYOUT_ALIASES):
            add_issue(issues, "error", f"{path}.{key}: unknown key")
            continue
        template_value = template.get(key)
        if template_value is None and path.endswith(".layout") and key in LAYOUT_ALIASES:
            template_value = template[LAYOUT_ALIASES[key]]
        next_path = f"{path}.{key}"
        if isinstance(value, dict) and isinstance(template_value, dict):
            validate_known_keys(value, template_value, next_path, issues)


def validate_types(
    data: dict,
    template: dict,
    path: str,
    issues: list[tuple[str, str]],
) -> None:
    for key, value in data.items():
        if key.startswith("_") or (key not in template and not (path.endswith(".layout") and key in LAYOUT_ALIASES)):
            continue
        template_value = template.get(key)
        if template_value is None and path.endswith(".layout") and key in LAYOUT_ALIASES:
            template_value = template[LAYOUT_ALIASES[key]]
        next_path = f"{path}.{key}"
        if isinstance(template_value, dict):
            if not isinstance(value, dict):
                add_issue(issues, "error", f"{next_path}: expected object")
            else:
                validate_types(value, template_value, next_path, issues)
            continue

        if isinstance(template_value, bool):
            if not isinstance(value, bool):
                add_issue(issues, "error", f"{next_path}: expected boolean")
        elif isinstance(template_value, int) and not isinstance(template_value, bool):
            if not isinstance(value, int) or isinstance(value, bool):
                add_issue(issues, "error", f"{next_path}: expected integer")
        elif isinstance(template_value, float):
            if not is_numeric(value):
                add_issue(issues, "error", f"{next_path}: expected number")
        elif isinstance(template_value, str):
            if not isinstance(value, str):
                add_issue(issues, "error", f"{next_path}: expected string")
        elif isinstance(template_value, list):
            if not isinstance(value, list):
                add_issue(issues, "error", f"{next_path}: expected array")


def validate_base_config(config: dict, repo_root: Path, issues: list[tuple[str, str]]) -> None:
    for section in ("chafa", "timing", "layout", "vines", "scaffold", "panel"):
        if section not in config or not isinstance(config.get(section), dict):
            add_issue(issues, "error", f"{section}: missing or not an object")

    chafa = config.get("chafa", {})
    timing = config.get("timing", {})
    layout = config.get("layout", {})
    vines = config.get("vines", {})
    scaffold = config.get("scaffold", {})
    panel = config.get("panel", {})

    for key in ("width", "height", "frame_count"):
        value = chafa.get(key)
        if not isinstance(value, int) or isinstance(value, bool) or value <= 0:
            add_issue(issues, "error", f"chafa.{key}: must be a positive integer")

    if not is_numeric(chafa.get("threshold")) or not (0.0 <= float(chafa["threshold"]) <= 1.0):
        add_issue(issues, "error", "chafa.threshold: must be between 0.0 and 1.0")

    for key in ("render_fps", "hero_fps", "vines_tick_seconds", "info_refresh_seconds"):
        value = timing.get(key)
        if not is_numeric(value) or float(value) <= 0.0:
            add_issue(issues, "error", f"timing.{key}: must be a positive number")

    if not isinstance(layout.get("hero_anchor"), str) or layout["hero_anchor"] not in {"left", "center", "right"}:
        add_issue(issues, "error", "layout.hero_anchor: must be one of left, center, right")

    for key in ("min_terminal_columns", "min_terminal_rows", "outer_margin_x", "outer_margin_y", "info_width", "info_height", "info_gap"):
        value = layout.get(key)
        if not isinstance(value, int) or isinstance(value, bool) or value <= 0:
            add_issue(issues, "error", f"layout.{key}: must be a positive integer")

    for key in (
        "hero_offset_x",
        "hero_offset_y",
        "hero_mask_offset_x",
        "hero_mask_offset_y",
        "trunk_mask_offset_x",
        "trunk_mask_offset_y",
        "info_offset_x",
        "info_offset_y",
        "hero_collision_trim_left",
        "hero_collision_trim_top",
        "hero_collision_trim_right",
        "hero_collision_trim_bottom",
        "hero_collision_bleed_right",
        "hero_corner_trim_top_left",
        "hero_corner_trim_top_left_y",
        "hero_corner_trim_top_right",
        "hero_corner_trim_top_right_y",
        "hero_corner_trim_bottom_left",
        "hero_corner_trim_bottom_right",
        "hero_safe_pad_x",
        "hero_safe_pad_y",
        "info_collision_trim_left",
        "info_collision_trim_top",
        "info_collision_trim_right",
        "info_collision_trim_bottom",
        "info_safe_pad_x",
        "info_safe_pad_y",
    ):
        value = layout.get(key)
        if not isinstance(value, int) or isinstance(value, bool):
            add_issue(issues, "error", f"layout.{key}: must be an integer")
        elif key.endswith("pad_x") or key.endswith("pad_y") or "trim" in key or key == "info_gap":
            if value < 0:
                add_issue(issues, "error", f"layout.{key}: must be non-negative")

    for key in (
        "hero_mask_threshold",
        "trunk_mask_threshold",
    ):
        value = layout.get(key)
        if not isinstance(value, int) or isinstance(value, bool) or not (0 <= value <= 255):
            add_issue(issues, "error", f"layout.{key}: must be an integer between 0 and 255")

    for key in (
        "hero_mask_scale_x",
        "hero_mask_scale_y",
        "trunk_mask_scale_x",
        "trunk_mask_scale_y",
    ):
        value = layout.get(key)
        if not is_numeric(value) or float(value) <= 0.0:
            add_issue(issues, "error", f"layout.{key}: must be a positive number")

    for key in ("max_tips", "max_structural_cells", "max_ornament_cells", "branch_life_min", "branch_life_max", "trunk_seed_offset_x", "trunk_seed_bottom_margin"):
        value = vines.get(key)
        if not isinstance(value, int) or isinstance(value, bool) or value < 0:
            add_issue(issues, "error", f"vines.{key}: must be a non-negative integer")

    for key in (
        "branch_decay",
        "branch_chance",
        "forward_bonus",
        "turn_penalty",
        "organic_variation",
        "trunk_life",
        "trunk_decay",
        "trunk_climb_bonus",
        "trunk_diagonal_bonus",
        "trunk_reverse_penalty",
        "support_traverse_bonus",
        "support_wrap_bonus",
        "settle_down_bonus",
        "branch_gravity_bonus",
        "hero_contour_attraction",
        "hero_boundary_attraction",
        "info_boundary_attraction",
        "trunk_attraction",
        "trunk_contact_bonus",
        "trunk_core_bonus",
        "collision_proximity_penalty",
        "hero_collision_proximity_penalty",
        "hero_ascent_bonus",
        "hero_approach_left_bonus",
        "hero_approach_diagonal_bonus",
        "pre_ascent_diagonal_bonus",
        "hero_top_traverse_bonus",
        "hero_top_commit_bonus",
        "hero_top_drop_penalty",
        "hero_top_state_bonus",
        "hero_top_state_drop_penalty",
        "hero_exit_left_bonus",
        "hero_exit_downleft_bonus",
        "hero_exit_right_penalty",
        "hero_exit_up_penalty",
        "hero_pileup_penalty",
        "right_staging_left_penalty",
        "panel_corridor_penalty",
        "pre_contact_branch_factor",
        "hero_top_branch_factor",
        "hero_exit_branch_factor",
        "info_hanger_life",
        "hero_lateral_entry_penalty",
        "below_hero_left_traverse_penalty",
        "below_hero_recovery_bonus",
        "info_collision_proximity_penalty",
        "top_edge_penalty",
        "floor_horizontal_penalty",
        "floor_escape_bonus",
        "thickening_min_age",
        "thickening_full_age",
        "thickening_spread_chance",
        "trunk_thickening_min_age",
        "trunk_thickening_bonus",
        "trunk_thickening_core_bias",
    ):
        value = vines.get(key)
        if not is_numeric(value):
            add_issue(issues, "error", f"vines.{key}: must be numeric")
        elif key.endswith("chance") or key.endswith("factor") or "variation" in key or "spread" in key:
            if float(value) < 0.0:
                add_issue(issues, "error", f"vines.{key}: must be non-negative")
        elif "penalty" in key:
            if float(value) < 0.0:
                add_issue(issues, "error", f"vines.{key}: must be non-negative")

    if not isinstance(vines.get("branch_life_min"), int) or not isinstance(vines.get("branch_life_max"), int):
        pass
    elif vines["branch_life_min"] > vines["branch_life_max"]:
        add_issue(issues, "error", "vines.branch_life_min: must be <= branch_life_max")

    for key in ("debug",):
        value = vines.get(key)
        if not isinstance(value, dict):
            add_issue(issues, "error", "vines.debug: must be an object")
        else:
            if not isinstance(value.get("enabled"), bool):
                add_issue(issues, "error", "vines.debug.enabled: must be boolean")
            if not isinstance(value.get("stem_only_view"), bool):
                add_issue(issues, "error", "vines.debug.stem_only_view: must be boolean")

    for key in ("enabled",):
        if not isinstance(scaffold.get(key), bool):
            add_issue(issues, "error", "scaffold.enabled: must be boolean")
    for key in ("base_x", "base_y", "trunk_height", "fork_y", "left_reach", "right_reach", "upper_lift"):
        value = scaffold.get(key)
        if not isinstance(value, int) or isinstance(value, bool):
            add_issue(issues, "error", f"scaffold.{key}: must be an integer")
        elif key != "upper_lift" and value < 0:
            add_issue(issues, "error", f"scaffold.{key}: must be non-negative")

    for key in ("enabled", "show_weather_placeholder"):
        if not isinstance(panel.get(key), bool):
            add_issue(issues, "error", f"panel.{key}: must be boolean")
    if not isinstance(panel.get("title"), str):
        add_issue(issues, "error", "panel.title: must be a string")
    if not isinstance(panel.get("timezone"), str):
        add_issue(issues, "error", "panel.timezone: must be a string")

    for path_key, rel_path in (
        ("chafa.source_gif", chafa.get("source_gif")),
        ("chafa.fallback_image", chafa.get("fallback_image")),
        ("layout.hero_mask_path", layout.get("hero_mask_path")),
        ("layout.trunk_mask_path", layout.get("trunk_mask_path")),
    ):
        if not isinstance(rel_path, str) or not rel_path:
            add_issue(issues, "error", f"{path_key}: must be a non-empty string")
            continue
        path = Path(rel_path)
        if not path.is_absolute():
            path = repo_root / path
        if not path.exists():
            add_issue(issues, "error", f"{path_key}: file does not exist at {path}")

    for cache_key, rel_path in (
        ("chafa.cache_dir_raw", chafa.get("cache_dir_raw")),
        ("chafa.cache_dir_chafa", chafa.get("cache_dir_chafa")),
    ):
        if isinstance(rel_path, str) and rel_path:
            path = Path(rel_path)
            if not path.is_absolute():
                path = repo_root / path
            if not path.exists():
                add_issue(issues, "warning", f"{cache_key}: directory does not exist yet at {path}")


def lint_config(path: Path, repo_root: Path, label: str) -> int:
    issues: list[tuple[str, str]] = []
    config = load_json(path)
    template = load_json(repo_root / "config/visualizer.json")

    validate_known_keys(config, template, label, issues)
    validate_types(config, template, label, issues)
    validate_base_config(config, repo_root, issues)

    errors = [message for level, message in issues if level == "error"]
    warnings = [message for level, message in issues if level == "warning"]

    print(f"{label}: {'OK' if not errors else 'FAILED'}")
    for message in warnings:
        print(f"  WARN  {message}")
    for message in errors:
        print(f"  ERROR {message}")

    return 1 if errors else 0


def list_recipes(recipe_dir: Path) -> list[Path]:
    return sorted(p for p in recipe_dir.glob("*.json") if p.is_file())


def main() -> int:
    parser = argparse.ArgumentParser(description="Lint the visualizer config and recipe overlays.")
    parser.add_argument("--config", type=Path, help="Lint a specific JSON config file.")
    parser.add_argument("--recipe", type=str, help="Lint a named recipe overlay against the base config.")
    parser.add_argument("--all-recipes", action="store_true", help="Lint the base config and every recipe overlay.")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent
    base_config_path = repo_root / "config/visualizer.json"
    recipe_dir = repo_root / "recipes"

    if args.config and (args.recipe or args.all_recipes):
        parser.error("--config cannot be combined with --recipe or --all-recipes")

    if args.recipe and args.all_recipes:
        parser.error("--recipe cannot be combined with --all-recipes")

    if args.config:
        return lint_config(args.config, repo_root, str(args.config.relative_to(repo_root)) if args.config.is_relative_to(repo_root) else str(args.config))

    if args.recipe:
        recipe_path = recipe_dir / f"{args.recipe}.json"
        if not recipe_path.exists():
            raise SystemExit(
                f"ERROR: unknown recipe {args.recipe!r}. Available: "
                + ", ".join(p.stem for p in list_recipes(recipe_dir))
            )
        merged = merge_dicts(load_json(base_config_path), load_json(recipe_path))
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as tmp:
            tmp_path = Path(tmp.name)
            tmp.write(json.dumps(merged, indent=2) + "\n")
        try:
            return lint_config(tmp_path, repo_root, f"recipe:{args.recipe}")
        finally:
            try:
                tmp_path.unlink()
            except FileNotFoundError:
                pass

    if args.all_recipes:
        exit_code = lint_config(base_config_path, repo_root, "base")
        for recipe_path in list_recipes(recipe_dir):
            merged = merge_dicts(load_json(base_config_path), load_json(recipe_path))
            with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as tmp:
                tmp_path = Path(tmp.name)
                tmp.write(json.dumps(merged, indent=2) + "\n")
            try:
                exit_code |= lint_config(tmp_path, repo_root, f"recipe:{recipe_path.stem}")
            finally:
                try:
                    tmp_path.unlink()
                except FileNotFoundError:
                    pass
        return exit_code

    return lint_config(base_config_path, repo_root, "base")


if __name__ == "__main__":
    raise SystemExit(main())
