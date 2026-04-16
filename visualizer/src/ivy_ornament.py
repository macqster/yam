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
    threshold = max(0, min(100, int(chance * 100)))
    for x, y in state.active_leaf_positions:
        phase = lifecycle_phase(x, y)
        if phase == "fresh":
            local_threshold = min(100, threshold + 32)
        elif phase == "mature":
            local_threshold = min(100, threshold + 24)
        elif phase == "aging":
            local_threshold = min(100, threshold + 10)
        else:  # decay
            local_threshold = max(0, threshold - 2)

        if stable_index(x, y, 11, 100) >= local_threshold:
            continue

        # Thin the left-side ornament field a bit so it breathes more and reads
        # less like stacked terminal construction.
        if x < layout.hero.x and stable_index(x, y, 19, 100) < 4:
            continue

        direction = state.active_leaf_dirs.get((x, y))
        if direction is not None:
            stamp_oriented_leaf(state, x, y, direction[0], direction[1], layout, rng)
        else:
            pattern = LEAF_PATTERNS[stable_index(x, y, 13, len(LEAF_PATTERNS))]
            stamp_leaf(state, x, y, pattern, layout)

    # --- subtle leaf accents along long top horizontal runs ---
    for x, y in state.stems:
        if y > 4:
            continue

        left = (x - 1, y) in state.stems
        right = (x + 1, y) in state.stems
        if not (left and right):
            continue

        if stable_index(x, y, 311, 100) >= 42:
            continue

        candidates = [(x, y - 1), (x, y + 1)]
        for sx, sy in candidates:
            if not is_ornament_open(state, sx, sy, layout):
                continue
            if (sx, sy) in state.leaf_stamps:
                continue

            color = stable_choice([GREEN, LIGHT_GREEN, OLIVE], sx, sy, 313)
            glyph = stable_choice(["*", "•"], sx, sy, 317)
            state.leaf_stamps[(sx, sy)] = f"{color}{glyph}{RESET}"
            break


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

        phase = lifecycle_phase(parent[0], parent[1])
        if phase == "fresh":
            local_spawn_threshold = max(0, spawn_threshold - 2)
        elif phase == "mature":
            local_spawn_threshold = min(100, spawn_threshold + 4)
        elif phase == "aging":
            local_spawn_threshold = max(0, spawn_threshold - 2)
        else:  # decay
            local_spawn_threshold = max(0, spawn_threshold - 6)

        if stable_index(x, y, 97, 100) >= local_spawn_threshold:
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
    trunk_min_age = int(config.get("trunk_thickening_min_age", max(1, min_age // 2)))
    trunk_bonus = float(config.get("trunk_thickening_bonus", 1.75))
    trunk_info_margin = int(config.get("trunk_thickening_info_margin", 2))
    trunk_core_bias = float(config.get("trunk_thickening_core_bias", 0.7))
    trunk_cutoff_x = layout.info_guide.x + trunk_info_margin
    base_trunk_count = int(config.get("trunk_base_segment_cells", 28))
    trunk_points_sorted = sorted(
        state.trunk_cells,
        key=lambda pt: (state.trunk_birth.get(pt, state.frame), pt[1], pt[0]),
    )
    base_trunk_cells = set(trunk_points_sorted[:base_trunk_count])

    for x, y in state.stems:
        point = (x, y)
        is_trunk = point in state.trunk_cells
        age_source = state.trunk_birth if is_trunk else state.stem_birth
        age = state.frame - age_source.get(point, state.frame)
        local_min_age = trunk_min_age if is_trunk else min_age
        if age < local_min_age:
            continue

        age_span = max(1, full_age - local_min_age)
        maturity = max(0.0, min(1.0, (age - local_min_age) / age_span))
        basal_trunk_segment = is_trunk and point in base_trunk_cells
        trunk_segment = is_trunk and x <= trunk_cutoff_x
        if basal_trunk_segment:
            maturity = min(1.0, maturity * (trunk_bonus + 0.9))
        elif trunk_segment:
            maturity = min(1.0, maturity * 0.75)

        left = (x - 1, y) in state.stems
        right = (x + 1, y) in state.stems
        up = (x, y - 1) in state.stems
        down = (x, y + 1) in state.stems
        up_left = (x - 1, y - 1) in state.stems
        up_right = (x + 1, y - 1) in state.stems
        down_left = (x - 1, y + 1) in state.stems
        down_right = (x + 1, y + 1) in state.stems
        diag_nw_se = up_left or down_right
        diag_ne_sw = up_right or down_left
        near_hero_top_edge = (
            layout.hero_guide.y - 1 <= y <= layout.hero_guide.y + 1
            and layout.hero_guide.x <= x <= layout.hero_guide.right
        )
        near_grid_top_edge = y <= 4
        long_top_run = near_grid_top_edge and left and right
        # Keep the top band broken and organic rather than turning it into a
        # continuous scaffold underline.
        long_top_cluster = long_top_run and stable_index(x, y, 241, 100) < 18
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
            if trunk_segment:
                trunk_side_bonus = 0.95 if basal_trunk_segment else 0.18
                side_cells.extend([
                    (x - 1, y, "│", trunk_core_bias + trunk_side_bonus * maturity),
                    (x + 1, y, "│", trunk_core_bias + trunk_side_bonus * maturity),
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
            if trunk_segment:
                trunk_side_bonus = 0.8 if basal_trunk_segment else 0.12
                side_cells.extend([
                    (x, y - 1, "─", trunk_core_bias + trunk_side_bonus * maturity),
                    (x, y + 1, "─", trunk_core_bias + trunk_side_bonus * maturity),
                ])
            if near_hero_top_edge:
                side_cells.append((x, y - 1, "─", max(0.9, trunk_core_bias + 0.5 * maturity)))
            if near_grid_top_edge:
                # Keep the top edge support light and broken so it reads like
                # irregular bark texture rather than a continuous rail.
                if stable_index(x, y, 219, 100) < 38:
                    side_cells.append(
                        (x, y + 1, stable_choice(["╌", "·", ":"], x, y + 1, 221), 0.32 + 0.16 * maturity)
                    )

        if basal_trunk_segment and (diag_nw_se or diag_ne_sw):
            if diag_nw_se:
                side_cells.extend([
                    (x - 1, y + 1, "╱", trunk_core_bias + 0.95 * maturity),
                    (x + 1, y - 1, "╱", trunk_core_bias + 0.95 * maturity),
                    (x - 1, y, "│", trunk_core_bias + 0.55 * maturity),
                    (x, y + 1, "─", trunk_core_bias + 0.55 * maturity),
                ])
            if diag_ne_sw:
                side_cells.extend([
                    (x - 1, y - 1, "╲", trunk_core_bias + 0.95 * maturity),
                    (x + 1, y + 1, "╲", trunk_core_bias + 0.95 * maturity),
                    (x - 1, y, "│", trunk_core_bias + 0.55 * maturity),
                    (x, y - 1, "─", trunk_core_bias + 0.55 * maturity),
                ])

        if near_hero_top_edge and left and right and maturity >= 0.08:
            side_cells.extend([
                (x, y - 1, "─", 1.05 + 0.45 * maturity),
            ])
        if long_top_cluster and maturity >= 0.0:
            side_cells.extend([
                (x, y + 1, stable_choice(["╌", "·", "·"], x, y + 1, 227), 0.54 + 0.18 * maturity),
                (x - 1, y + 1, stable_choice(["·", "·", "·"], x - 1, y + 1, 229), 0.16 + 0.08 * maturity),
                (x + 1, y + 1, stable_choice(["·", "·", "·"], x + 1, y + 1, 233), 0.16 + 0.08 * maturity),
            ])
        if basal_trunk_segment and maturity >= 0.18:
            core_cells = []
            if up or down:
                core_bonus = 0.95 if basal_trunk_segment else 0.75
                core_cells.extend([
                    (x - 1, y, "│", trunk_core_bias + core_bonus * maturity),
                    (x + 1, y, "│", trunk_core_bias + core_bonus * maturity),
                ])
            if left or right:
                core_bonus = 0.75 if basal_trunk_segment else 0.55
                core_cells.extend([
                    (x, y - 1, "─", trunk_core_bias + core_bonus * maturity),
                    (x, y + 1, "─", trunk_core_bias + core_bonus * maturity),
                ])
            if diag_nw_se:
                core_cells.extend([
                    (x - 1, y + 1, "╱", trunk_core_bias + 1.05 * maturity),
                    (x + 1, y - 1, "╱", trunk_core_bias + 1.05 * maturity),
                ])
            if diag_ne_sw:
                core_cells.extend([
                    (x - 1, y - 1, "╲", trunk_core_bias + 1.05 * maturity),
                    (x + 1, y + 1, "╲", trunk_core_bias + 1.05 * maturity),
                ])
            side_cells.extend(core_cells)

        if long_top_cluster:
            tx, ty = x, y + 1
            if is_ornament_open(state, tx, ty, layout) and stable_index(tx, ty, 223, 100) < 55:
                top_glyph = stable_choice(["╌", "·", ":", "·"], tx, ty, 223)
                state.thickened_wood[(tx, ty)] = f"{wood_color_for((tx, ty))}{top_glyph}{RESET}"
        for sx, sy, wood_char, chance_scale in side_cells:
            if not is_ornament_open(state, sx, sy, layout):
                continue
            # In non-trunk areas, avoid accreting wood inside already crowded
            # neighborhoods so branch clusters do not rebuild boxy scaffolds.
            if not trunk_segment:
                local_density = sum(
                    (sx + ox, sy + oy) in state.stems
                    for ox, oy in (
                        (-1, -1), (0, -1), (1, -1),
                        (-1, 0),           (1, 0),
                        (-1, 1),  (0, 1),  (1, 1),
                    )
                )
                if local_density >= 4 and stable_index(sx, sy, 271, 100) < 72:
                    continue
            # Leave occasional breathing gaps in the sprawl.
            if not trunk_segment and stable_index(sx, sy, 211, 100) < 20:
                continue
            local_spread = spread_chance * chance_scale * (0.32 + 0.68 * maturity) * 0.72
            if maturity >= 0.78 or rng.random() < local_spread:
                # Mix occasional moss/bark specks with wood strokes to break up
                # the blocky grid look, but bias mature wood toward more solid marks.
                if not trunk_segment and stable_index(sx, sy, 41, 100) < 28:
                    leaf = stable_choice(["·", "'", ":", "•", "~"], sx, sy, 43)
                    state.thickened_wood[(sx, sy)] = f"{OLIVE}{leaf}{RESET}"
                else:
                    color = stable_choice([BROWN, DARK_BROWN], sx, sy, 47)
                    if trunk_segment:
                        glyph = stable_choice([wood_char, wood_char, wood_char, wood_char, "┆", "╌"], sx, sy, 59)
                    elif maturity >= 0.78:
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
    phase = lifecycle_phase(center_x, center_y)
    if phase == "fresh":
        cluster_chance = 0.03
    elif phase == "mature":
        cluster_chance = 0.10
    elif phase == "aging":
        cluster_chance = 0.24
    else:  # decay
        cluster_chance = 0.42

    if rng.random() >= cluster_chance:
        return

    side = -1 if stable_index(center_x, center_y, 421, 2) == 0 else 1
    if dx != 0:
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
        color = stable_choice([OLIVE, BROWN, DARK_BROWN], sx, sy, 37)
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
    ornament_cells = getattr(layout, "ornament_cells", layout.allowed_cells)
    if (x, y) not in ornament_cells:
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


# --- Multi-cell live-leaf entity stamping helpers ---
def leaf_role_priority(role: str) -> int:
    if role in ("*",):
        return 6
    if role in ("~", "|", "/", "\\"):
        return 5
    if role in ("+",):
        return 4
    if role in ("-", "<", ">"):
        return 2
    if role in (".", "'"):
        return 1
    return 3


def leaf_glyph_for_role(role: str, x: int, y: int) -> str:
    # Mixed symbolic + dot vocabulary: keeps clusters readable and lively
    # without pushing them fully into floral/sparkle or full-dot territory.
    if role in ("~", "|", "/", "\\"):
        return stable_choice(["●", "•", "✿", "•", "●"], x, y, 521)
    if role in ("+", "*"):
        return stable_choice(["✽", "•", "●", "•"], x, y, 523)
    if role in ("-", "<", ">"):
        return stable_choice(["•", "·", "✧", "•"], x, y, 527)
    return stable_choice(["·", "•", "✧"], x, y, 529)


def leaf_color_for_role(role: str, x: int, y: int) -> str:
    if role in (".", "'"):
        return stable_choice([LIGHT_GREEN, GREEN], x, y, 531)
    return stable_choice([LIGHT_GREEN, GREEN, LIGHT_GREEN, OLIVE], x, y, 533)


def stamp_leaf(state: IvyState, center_x: int, center_y: int, pattern: list[tuple[int, int, str]], layout: SceneLayout) -> None:
    candidates: list[tuple[int, int, str]] = []
    for dx, dy, role in pattern:
        sx = center_x + dx
        sy = center_y + dy
        if not is_ornament_open(state, sx, sy, layout):
            continue
        if (sx, sy) in state.leaf_stamps:
            continue
        candidates.append((sx, sy, role))

    if not candidates:
        return

    min_cells = 3 if len(pattern) >= 5 else 2
    if len(candidates) < min_cells:
        return

    target_cells = min(len(candidates), 4)

    primary_roles = {"*", "~", "+", "|", "/", "\\"}
    accent_roles = {".", "'", "-", "<", ">"}

    primary_candidates = [item for item in candidates if item[2] in primary_roles]
    accent_candidates = [item for item in candidates if item[2] in accent_roles]
    other_candidates = [item for item in candidates if item[2] not in primary_roles and item[2] not in accent_roles]

    primary_ordered = sorted(
        primary_candidates,
        key=lambda item: (
            leaf_role_priority(item[2]),
            abs(item[0] - center_x) + abs(item[1] - center_y),
        ),
        reverse=True,
    )
    accent_ordered = sorted(
        accent_candidates,
        key=lambda item: (
            leaf_role_priority(item[2]),
            abs(item[0] - center_x) + abs(item[1] - center_y),
        ),
        reverse=True,
    )
    other_ordered = sorted(
        other_candidates,
        key=lambda item: (
            leaf_role_priority(item[2]),
            abs(item[0] - center_x) + abs(item[1] - center_y),
        ),
        reverse=True,
    )

    selected: list[tuple[int, int, str]] = []

    # Favor leaf-body cells first so clusters read as outward surfaces.
    for item in primary_ordered[:3]:
        selected.append(item)
        if len(selected) >= target_cells:
            break

    # Fill with any remaining structural cells before allowing accents.
    if len(selected) < target_cells:
        for item in other_ordered:
            if item in selected:
                continue
            selected.append(item)
            if len(selected) >= target_cells:
                break

    # Allow only one accent/backside cell at most.
    if len(selected) < target_cells and accent_ordered:
        for item in accent_ordered:
            if item in selected:
                continue
            selected.append(item)
            break

    if len(selected) < min_cells:
        return

    for sx, sy, role in selected:
        color = leaf_color_for_role(role, sx, sy)
        glyph = leaf_glyph_for_role(role, sx, sy)
        state.leaf_stamps[(sx, sy)] = f"{color}{glyph}{RESET}"


def stamp_oriented_leaf(
    state: IvyState,
    center_x: int,
    center_y: int,
    dx: int,
    dy: int,
    layout: SceneLayout,
    rng: random.Random,
) -> None:
    del rng

    # Build clusters primarily on one outer side of the stem so foliage reads
    # like a directional leaf mass rather than a symmetric blob.
    outward = -1 if stable_index(center_x, center_y, 17, 2) == 0 else 1
    accent = -outward if stable_index(center_x, center_y, 19, 100) < 18 else outward

    if dx != 0:
        pattern = [
            (0, outward, "~"),
            (1, outward, "~"),
            (2, outward, "*"),
            (1, outward * 2, "+"),
            (2, outward * 2, "*"),
            (3, outward, "."),
            (0, accent, "."),
        ]
    else:
        pattern = [
            (outward, 0, "~"),
            (outward, 1, "~"),
            (outward, 2, "*"),
            (outward * 2, 1, "+"),
            (outward * 2, 2, "*"),
            (outward, 3, "."),
            (accent, 0, "."),
        ]

    stamp_leaf(state, center_x, center_y, pattern, layout)


def wood_char_for_cell(state: IvyState, x: int, y: int) -> str:
    left = (x - 1, y) in state.stems
    right = (x + 1, y) in state.stems
    up = (x, y - 1) in state.stems
    down = (x, y + 1) in state.stems
    degree = sum((left, right, up, down))

    # Keep dense junctions readable, but avoid making them look like rigid
    # ASCII construction geometry.
    if degree >= 4:
        return stable_choice(["╌", "┆", ":", "·"], x, y, 149)
    if degree == 3:
        return stable_choice(["╌", "┆", ":", "·", "·"], x, y, 151)

    if left and right:
        return stable_choice(["╌", "·", ":", "-", "·"], x, y, 153)
    if up and down:
        return stable_choice(["┆", "|", ":", "·"], x, y, 157)
    if left and up:
        return "╱"
    if left and down:
        return "╲"
    if right and up:
        return "╲"
    if right and down:
        return "╱"
    if left or right:
        return stable_choice(["╌", "·", ":"], x, y, 161)
    if up or down:
        return stable_choice(["┆", "|", ":"], x, y, 163)
    return stable_choice(["┆", "·", ":"], x, y, 167)


def wood_color_for(point: Point) -> str:
    return stable_choice([BROWN, BROWN, DARK_BROWN], point[0], point[1], 53)


def lifecycle_phase(x: int, y: int) -> str:
    idx = stable_index(x, y, 401, 100)
    if idx < 25:
        return "fresh"
    if idx < 55:
        return "mature"
    if idx < 80:
        return "aging"
    return "decay"


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