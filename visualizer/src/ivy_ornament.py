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


def rebuild_flower_stamps(state: IvyState, config: dict, layout: SceneLayout, rng: random.Random) -> None:
    lifespan = int(config.get("flower_lifespan_frames", 32))
    maturity_threshold = int(config.get("flower_maturity_frames", 12))
    max_clusters = int(config.get("flower_max_clusters", 5))
    max_cluster_cells = int(config.get("flower_max_cluster_cells", 5))
    spawn_threshold = int(config.get("flower_spawn_percent", 8))

    new_stamps: dict[Point, str] = {}
    new_birth: dict[Point, int] = {}
    new_parent: dict[Point, Point] = {}
    cluster_sizes: dict[Point, int] = {}

    # Keep existing flower cells alive while their lifecycle lasts and the parent stem survives.
    for point, birth in state.flower_birth.items():
        parent = state.flower_parent.get(point)
        if parent is None or parent not in state.stems:
            continue
        age = state.frame - birth
        if age >= lifespan:
            continue
        if point in state.stems or point in state.leaf_stamps or point in state.dead_leaf_stamps:
            continue
        color = flower_color_for_age(age)
        new_stamps[point] = f"{color}✿{RESET}"
        new_birth[point] = birth
        new_parent[point] = parent
        cluster_sizes[parent] = cluster_sizes.get(parent, 0) + 1

    active_clusters = set(cluster_sizes.keys())

    def is_cluster_open(fx: int, fy: int) -> bool:
        if (fx, fy) in state.stems:
            return False
        if (fx, fy) in state.leaf_stamps:
            return False
        if (fx, fy) in state.dead_leaf_stamps:
            return False
        if (fx, fy) in state.thickened_wood:
            return False
        if (fx, fy) in state.active_leaf_positions:
            return False
        if (fx, fy) in new_stamps:
            return False
        if (fx, fy) not in layout.allowed_cells:
            return False
        return True

    # Spawn and/or grow flower clusters from leaf groups only.
    for (x, y) in state.leaf_stamps:
        nearby_stems = [
            (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1),
            (x - 1, y - 1), (x + 1, y - 1), (x - 1, y + 1), (x + 1, y + 1),
        ]
        parent = next((pt for pt in nearby_stems if pt in state.stems), None)
        if parent is None:
            continue

        parent_age = state.frame - state.stem_birth.get(parent, state.frame)
        if parent_age < maturity_threshold:
            continue

        if stable_index(x, y, 97, 100) >= spawn_threshold:
            continue

        # Respect global cluster cap unless this parent already owns a cluster.
        if parent not in active_clusters and len(active_clusters) >= max_clusters:
            continue

        current_size = cluster_sizes.get(parent, 0)
        if current_size >= max_cluster_cells:
            continue

        dx = x - parent[0]
        dy = y - parent[1]
        if dx == 0 and dy == 0:
            continue

        # Prefer the outer side of the leaf relative to the parent stem.
        anchor_candidates = [
            (x + dx, y + dy),
            (x + dx + dy, y + dy + dx),
            (x + dx - dy, y + dy - dx),
            (x, y - 1), (x + 1, y), (x - 1, y), (x, y + 1),
        ]

        anchor = next(((fx, fy) for fx, fy in anchor_candidates if is_cluster_open(fx, fy)), None)
        if anchor is None:
            continue

        # Deterministic target size per parent, clamped to 1..max_cluster_cells.
        target_size = 1 + stable_index(parent[0], parent[1], 131, max_cluster_cells)
        target_size = min(target_size, max_cluster_cells)
        cells_to_add = max(0, target_size - current_size)
        if cells_to_add <= 0:
            continue

        cluster_candidates = [
            anchor,
            (anchor[0] + 1, anchor[1]),
            (anchor[0] - 1, anchor[1]),
            (anchor[0], anchor[1] + 1),
            (anchor[0], anchor[1] - 1),
            (anchor[0] + 1, anchor[1] + 1),
            (anchor[0] - 1, anchor[1] + 1),
            (anchor[0] + 1, anchor[1] - 1),
            (anchor[0] - 1, anchor[1] - 1),
        ]

        added = 0
        for fx, fy in cluster_candidates:
            if added >= cells_to_add or cluster_sizes.get(parent, 0) >= max_cluster_cells:
                break
            if not is_cluster_open(fx, fy):
                continue
            new_stamps[(fx, fy)] = f"{flower_color_for_age(0)}✿{RESET}"
            new_birth[(fx, fy)] = state.frame
            new_parent[(fx, fy)] = parent
            cluster_sizes[parent] = cluster_sizes.get(parent, 0) + 1
            active_clusters.add(parent)
            added += 1

    state.flower_stamps = new_stamps
    state.flower_birth = new_birth
    state.flower_parent = new_parent


def rebuild_thickened_wood(state: IvyState, config: dict, layout: SceneLayout, rng: random.Random) -> None:
    state.thickened_wood = {}
    min_age = int(config["thickening_min_age"])
    full_age = int(config["thickening_full_age"])
    spread_chance = float(config["thickening_spread_chance"])

    for x, y in state.stems:
        age = state.frame - state.stem_birth.get((x, y), state.frame)
        if age < min_age:
            continue

        age_span = max(1, full_age - min_age)
        maturity = max(0.0, min(1.0, (age - min_age) / age_span))

        left = (x - 1, y) in state.stems
        right = (x + 1, y) in state.stems
        up = (x, y - 1) in state.stems
        down = (x, y + 1) in state.stems
        diag_bias = stable_index(x, y, 173, 2)

        side_cells: list[tuple[int, int, str, float]] = []

        # Prefer angled outer growth; keep perpendicular thickening much rarer,
        # especially on newer parts of the trunk. Use only one diagonal bias per
        # stem cell so we avoid closed diamond/box motifs.
        if up or down:
            if diag_bias == 0:
                side_cells.extend([
                    (x - 1, y - 1, "╱", 0.75 + 0.65 * maturity),
                    (x + 1, y + 1, "╱", 0.75 + 0.65 * maturity),
                ])
            else:
                side_cells.extend([
                    (x - 1, y + 1, "╲", 0.75 + 0.65 * maturity),
                    (x + 1, y - 1, "╲", 0.75 + 0.65 * maturity),
                ])
            side_cells.extend([
                (x - 1, y, "|", 0.08 + 0.35 * maturity),
                (x + 1, y, "|", 0.08 + 0.35 * maturity),
            ])
        if left or right:
            if diag_bias == 0:
                side_cells.extend([
                    (x - 1, y - 1, "╲", 0.75 + 0.65 * maturity),
                    (x + 1, y + 1, "╲", 0.75 + 0.65 * maturity),
                ])
            else:
                side_cells.extend([
                    (x + 1, y - 1, "╱", 0.75 + 0.65 * maturity),
                    (x - 1, y + 1, "╱", 0.75 + 0.65 * maturity),
                ])
            side_cells.extend([
                (x, y - 1, "-", 0.08 + 0.35 * maturity),
                (x, y + 1, "-", 0.08 + 0.35 * maturity),
            ])

        for sx, sy, wood_char, chance_scale in side_cells:
            if not is_ornament_open(state, sx, sy, layout):
                continue
            # Leave occasional breathing gaps in the sprawl.
            if stable_index(sx, sy, 211, 100) < 20:
                continue
            local_spread = spread_chance * chance_scale * (0.32 + 0.68 * maturity) * 0.72
            if maturity >= 0.78 or rng.random() < local_spread:
                # Mix occasional moss/bark specks with wood strokes to break up
                # the blocky grid look, but bias mature wood toward more solid marks.
                if stable_index(sx, sy, 41, 100) < 28:
                    leaf = stable_choice(["·", "'", ":", "•", "~"], sx, sy, 43)
                    state.thickened_wood[(sx, sy)] = f"{OLIVE}{leaf}{RESET}"
                else:
                    color = stable_choice([BROWN, DARK_BROWN], sx, sy, 47)
                    if maturity >= 0.78:
                        glyph = stable_choice([wood_char, wood_char, wood_char, "┆", "╌", "·"], sx, sy, 59)
                    else:
                        glyph = stable_choice([wood_char, wood_char, wood_char, wood_char, "┆", "╌", "·"], sx, sy, 59)
                    state.thickened_wood[(sx, sy)] = f"{color}{glyph}{RESET}"


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
    combined = list(state.leaf_stamps.items()) + list(state.dead_leaf_stamps.items()) + list(state.thickened_wood.items()) + list(state.flower_stamps.items())
    if len(combined) <= max_ornaments:
        return

    overflow = len(combined) - max_ornaments
    for store in (state.leaf_stamps, state.dead_leaf_stamps, state.thickened_wood, state.flower_stamps):
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
    for point, glyph in state.flower_stamps.items():
        merged[point] = glyph

    for point in state.active_leaf_positions:
        if point not in occupied_points(state, include_active=False):
            # Sparse, very subtle preview markers (not every position)
            if stable_index(point[0], point[1], 71, 100) < 30:
                merged[point] = f"{OLIVE}·{RESET}"

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
        len(state.active_leaf_positions) + len(state.leaf_stamps) + len(state.dead_leaf_stamps) + len(state.thickened_wood) + len(state.flower_stamps)
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
    occupied.update(state.flower_stamps)
    if include_active:
        occupied.update(state.active_leaf_positions)
    return occupied


def stamp_leaf(state: IvyState, center_x: int, center_y: int, pattern: list[tuple[int, int, str]], layout: SceneLayout) -> None:
    for dx, dy, _char in pattern:
        sx = center_x + dx
        sy = center_y + dy
        if not is_ornament_open(state, sx, sy, layout):
            continue
        if (sx, sy) in state.leaf_stamps:
            continue
        color = stable_choice([GREEN, LIGHT_GREEN, OLIVE], sx, sy, 23)
        # Use a single, clean leaf glyph (Nerd Font)
        state.leaf_stamps[(sx, sy)] = f"{color}{RESET}"
        break


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
    left = (x - 1, y) in state.stems
    right = (x + 1, y) in state.stems
    up = (x, y - 1) in state.stems
    down = (x, y + 1) in state.stems
    degree = sum((left, right, up, down))

    # Dense trunk core should read as heavier, but still slender.
    if degree >= 4:
        return stable_choice(["=", "║", "┆"], x, y, 149)
    if degree == 3:
        return stable_choice(["=", "┆", ":"], x, y, 151)

    if left and right:
        return stable_choice(["=", "-", "╌"], x, y, 153)
    if up and down:
        return stable_choice(["║", "|", "┆"], x, y, 157)
    if left and up:
        return "╱"
    if left and down:
        return "╲"
    if right and up:
        return "╲"
    if right and down:
        return "╱"
    if left or right:
        return stable_choice(["-", "╌"], x, y, 161)
    if up or down:
        return stable_choice(["|", "┆"], x, y, 163)
    return stable_choice(["┆", "·", ":"], x, y, 167)


def wood_color_for(point: Point) -> str:
    return stable_choice([BROWN, BROWN, DARK_BROWN], point[0], point[1], 53)


def stable_index(x: int, y: int, salt: int, count: int) -> int:
    return ((x * 92821) + (y * 68917) + salt) % count


def stable_choice(values: list[str], x: int, y: int, salt: int) -> str:
    return values[stable_index(x, y, salt, len(values))]


def flower_color_for_age(age: int) -> str:
    if age < 18:
        return "\x1b[38;5;39m"   # debug-bright blue, youthful
    elif age < 28:
        return "\x1b[38;5;111m"  # softer blue, mature
    else:
        return "\x1b[38;5;67m"   # duller blue, aging
