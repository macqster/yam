from __future__ import annotations

import random

from ivy_state import IvyState
from ivy_types import BROWN, DARK_BROWN, GREEN, LIGHT_GREEN, OLIVE, Point
from layout import SceneLayout
from terminal import RESET


LEAF_PATTERNS: list[list[tuple[int, int, str]]] = [
    [(-2, 0, "-"), (-1, 0, "~"), (0, 0, "~"), (1, 0, "|"), (2, 0, "\\"), (3, 0, "*")],
    [(-2, 0, "-"), (-1, 0, "/"), (0, 0, "~"), (1, 0, "~"), (2, 0, "|"), (3, 0, ">"), (4, 0, ">")],
    [(-1, -1, "."), (0, -1, "/"), (1, -1, "~"), (-1, 0, "-"), (0, 0, "|"), (1, 0, "~"), (2, 0, "*")],
    [(-1, 0, "<"), (0, 0, "<"), (1, 0, "~"), (2, 0, "|"), (3, 0, "/"), (4, 0, ".")],
    [(-2, 0, "-"), (-1, 0, "~"), (0, 0, "~"), (1, 0, "|"), (2, 0, "/"), (3, 0, "*"), (0, 1, "'"), (1, 1, ".")],
    [(0, -1, "."), (1, -1, "+"), (0, 0, "|"), (1, 0, "~"), (2, 0, "~"), (3, 0, "~"), (4, 0, "|"), (5, 0, ">"), (6, 0, ">")],
    [(-2, 0, "-"), (-1, 0, "-"), (0, 0, "|"), (1, 0, "~"), (2, 0, "~"), (3, 0, "/"), (4, 0, "*"), (5, 0, ".")],
    [(-1, -1, "."), (0, -1, "+"), (1, -1, "."), (-1, 0, "-"), (0, 0, "~"), (1, 0, "|"), (2, 0, "\\"), (3, 0, "*")],
]


def rebuild_leaf_stamps(state: IvyState, config: dict, layout: SceneLayout, rng: random.Random) -> None:
    state.leaf_stamps = {}
    if not state.active_leaf_positions:
        return

    chance = float(config["leaf_stamp_chance"])
    for x, y in state.active_leaf_positions:
        if rng.random() >= chance:
            continue
        direction = state.active_leaf_dirs.get((x, y))
        if direction is not None:
            stamp_oriented_leaf(state, x, y, direction[0], direction[1], layout, rng)
        else:
            stamp_leaf(state, x, y, rng.choice(LEAF_PATTERNS), layout)


def rebuild_thickened_wood(state: IvyState, config: dict, layout: SceneLayout, rng: random.Random) -> None:
    state.thickened_wood = {}
    min_age = int(config["thickening_min_age"])
    full_age = int(config["thickening_full_age"])
    spread_chance = float(config["thickening_spread_chance"])

    for x, y in state.stems:
        age = state.frame - state.stem_birth.get((x, y), state.frame)
        if age < min_age:
            continue

        left = (x - 1, y) in state.stems
        right = (x + 1, y) in state.stems
        up = (x, y - 1) in state.stems
        down = (x, y + 1) in state.stems

        side_cells: list[tuple[int, int, str]] = []
        if up or down:
            side_cells.extend([(x - 1, y, "|"), (x + 1, y, "|")])
        if left or right:
            side_cells.extend([(x, y - 1, "-"), (x, y + 1, "-")])

        for sx, sy, wood_char in side_cells:
            if not is_ornament_open(state, sx, sy, layout):
                continue
            if age >= full_age or rng.random() < spread_chance:
                if stable_index(sx, sy, 41, 100) < 65:
                    leaf = stable_choice(["·", "'", "+", "•", "~"], sx, sy, 43)
                    state.thickened_wood[(sx, sy)] = f"{OLIVE}{leaf}{RESET}"
                else:
                    color = stable_choice([BROWN, DARK_BROWN], sx, sy, 47)
                    state.thickened_wood[(sx, sy)] = f"{color}{wood_char}{RESET}"


def stamp_death_cluster(
    state: IvyState,
    center_x: int,
    center_y: int,
    dx: int,
    dy: int,
    layout: SceneLayout,
    rng: random.Random,
) -> None:
    if dx != 0:
        side = rng.choice([-1, 1])
        pattern = [
            (0, side, "·"),
            (1, side, "~"),
            (2, side, "~"),
            (3, side, "*"),
            (1, side * 2, "."),
            (2, side * 2, "+"),
            (3, side * 2, "•"),
            (0, 0, "'"),
        ]
    else:
        side = rng.choice([-1, 1])
        pattern = [
            (side, 0, "·"),
            (side, 1, "~"),
            (side, 2, "~"),
            (side, 3, "*"),
            (side * 2, 1, "."),
            (side * 2, 2, "+"),
            (side * 2, 3, "•"),
            (0, 0, "'"),
        ]

    for ox, oy, char in pattern:
        sx = center_x + ox
        sy = center_y + oy
        if not is_ornament_open(state, sx, sy, layout):
            continue
        if (sx, sy) in state.dead_leaf_stamps:
            continue
        color = stable_choice([GREEN, OLIVE, LIGHT_GREEN], sx, sy, 37)
        state.dead_leaf_stamps[(sx, sy)] = f"{color}{char}{RESET}"


def trim_ornaments(state: IvyState, max_ornaments: int, rng: random.Random) -> None:
    combined = list(state.leaf_stamps.items()) + list(state.dead_leaf_stamps.items()) + list(state.thickened_wood.items())
    if len(combined) <= max_ornaments:
        return

    overflow = len(combined) - max_ornaments
    for store in (state.leaf_stamps, state.dead_leaf_stamps, state.thickened_wood):
        if overflow <= 0:
            break
        keys = list(store.keys())
        rng.shuffle(keys)
        for key in keys[: min(len(keys), overflow)]:
            store.pop(key, None)
            overflow -= 1
            if overflow <= 0:
                break


def merge_segments(state: IvyState, debug_config: dict) -> dict[Point, str]:
    merged: dict[Point, str] = {}

    for point in state.stems:
        merged[point] = f"{wood_color_for(point)}{wood_char_for_cell(state, *point)}{RESET}"
    for point, glyph in state.thickened_wood.items():
        merged[point] = glyph
    for point, glyph in state.dead_leaf_stamps.items():
        merged[point] = glyph
    for point, glyph in state.leaf_stamps.items():
        merged[point] = glyph
    for point in state.active_leaf_positions:
        if point not in occupied_points(state, include_active=False):
            merged[point] = f"{LIGHT_GREEN}{stable_choice(['*', 'o', '+', '·'], *point, 11)}{RESET}"

    if debug_config.get("stem_only_view"):
        return {point: glyph for point, glyph in merged.items() if point in state.stems or point in state.thickened_wood}
    return merged


def update_debug_stats(state: IvyState, layout: SceneLayout) -> None:
    region_coverage: dict[str, dict[str, int]] = {}
    for name, cells in layout.region_cells.items():
        stem_count = sum(1 for point in state.stems if point in cells)
        region_coverage[name] = {"allowed": len(cells), "stems": stem_count}
    state.debug_stats["region_coverage"] = region_coverage
    state.debug_stats["stem_count"] = len(state.stems)
    state.debug_stats["ornament_count"] = (
        len(state.active_leaf_positions) + len(state.leaf_stamps) + len(state.dead_leaf_stamps) + len(state.thickened_wood)
    )


def count_debug(state: IvyState, bucket: str, key: str) -> None:
    counts = state.debug_stats.setdefault(bucket, {})
    if isinstance(counts, dict):
        counts[key] = int(counts.get(key, 0)) + 1


def is_ornament_open(state: IvyState, x: int, y: int, layout: SceneLayout) -> bool:
    if (x, y) not in layout.allowed_cells:
        return False
    return (x, y) not in occupied_points(state, include_active=True)


def occupied_points(state: IvyState, include_active: bool) -> set[Point]:
    occupied = set(state.stems)
    occupied.update(state.leaf_stamps)
    occupied.update(state.dead_leaf_stamps)
    occupied.update(state.thickened_wood)
    if include_active:
        occupied.update(state.active_leaf_positions)
    return occupied


def stamp_leaf(state: IvyState, center_x: int, center_y: int, pattern: list[tuple[int, int, str]], layout: SceneLayout) -> None:
    for dx, dy, char in pattern:
        sx = center_x + dx
        sy = center_y + dy
        if not is_ornament_open(state, sx, sy, layout):
            continue
        if (sx, sy) in state.leaf_stamps:
            continue
        color = stable_choice([GREEN, LIGHT_GREEN, OLIVE], sx, sy, 23)
        state.leaf_stamps[(sx, sy)] = f"{color}{char}{RESET}"


def stamp_oriented_leaf(
    state: IvyState,
    center_x: int,
    center_y: int,
    dx: int,
    dy: int,
    layout: SceneLayout,
    rng: random.Random,
) -> None:
    if dx != 0:
        side = rng.choice([-1, 1])
        pattern = [(0, side, "·"), (1, side, "~"), (2, side, "~"), (3, side, "*"), (1, side * 2, "."), (2, side * 2, "+")]
    else:
        side = rng.choice([-1, 1])
        pattern = [(side, 0, "·"), (side, 1, "~"), (side, 2, "~"), (side, 3, "*"), (side * 2, 1, "."), (side * 2, 2, "+")]
    stamp_leaf(state, center_x, center_y, pattern, layout)


def wood_char_for_cell(state: IvyState, x: int, y: int) -> str:
    if (x - 1, y) in state.stems and (x + 1, y) in state.stems:
        return "-"
    if (x, y - 1) in state.stems and (x, y + 1) in state.stems:
        return "|"
    if (x - 1, y) in state.stems and (x, y - 1) in state.stems:
        return "/"
    if (x - 1, y) in state.stems and (x, y + 1) in state.stems:
        return "\\"
    return "+"


def wood_color_for(point: Point) -> str:
    return stable_choice([BROWN, DARK_BROWN], point[0], point[1], 53)


def stable_index(x: int, y: int, salt: int, count: int) -> int:
    return ((x * 92821) + (y * 68917) + salt) % count


def stable_choice(values: list[str], x: int, y: int, salt: int) -> str:
    return values[stable_index(x, y, salt, len(values))]
