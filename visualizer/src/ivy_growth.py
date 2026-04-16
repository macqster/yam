from __future__ import annotations

import random

from ivy_ornament import count_debug
from ivy_state import IvyState
from ivy_types import Direction, GrowthTip, NEIGHBORS_4, Point
from layout import SceneLayout



def initial_trunk_seed(config: dict, layout: SceneLayout, rng: random.Random, state: IvyState) -> Point | None:
    hero_zone = layout.hero_guide
    hero_right = _effective_hero_right(layout)
    target_x = min(layout.ivy_bounds.width - 2, hero_right + int(config["trunk_seed_offset_x"]))
    start_y = layout.ivy_bounds.height - max(3, int(config["trunk_seed_bottom_margin"]))
    end_y = 1

    best: Point | None = None
    best_distance = 10**9
    for y in range(start_y, end_y - 1, -1):
        for dx in range(0, max(2, layout.ivy_bounds.width // 3)):
            for candidate_x in (target_x - dx, target_x + dx):
                point = (candidate_x, y)
                if point not in layout.allowed_cells:
                    continue
                distance = abs(candidate_x - target_x)
                if distance < best_distance:
                    best = point
                    best_distance = distance
        if best is not None:
            break

    if best is not None:
        count_debug(state, "spawn_origin_counts", "trunk_seed")
    return best


# Helper function for effective hero right boundary
def _effective_hero_right(layout: SceneLayout) -> int:
    hero_zone = layout.hero_guide
    right_crop = 3
    if hero_zone.width > right_crop:
        return hero_zone.right - right_crop
    return hero_zone.right


def select_move(
    tip: GrowthTip,
    state: IvyState,
    config: dict,
    layout: SceneLayout,
    rng: random.Random,
    debug_config: dict,
) -> tuple[int, int, int, int] | None:
    scored_moves: list[tuple[float, int, int]] = []
    for dx, dy in candidate_moves(tip):
        new_x = tip.x + dx
        new_y = tip.y + dy
        if not is_open(new_x, new_y, layout):
            record_failed_move(state, new_x, new_y, layout, debug_config)
            continue
        score = move_score(tip, state, config, new_x, new_y, dx, dy, layout, rng)
        scored_moves.append((score, dx, dy))

    if not scored_moves:
        return None

    scored_moves.sort(reverse=True)
    limit = min(3, len(scored_moves))
    weights = [max(0.05, scored_moves[index][0]) for index in range(limit)]
    choice_index = rng.choices(range(limit), weights=weights, k=1)[0]
    _, dx, dy = scored_moves[choice_index]
    return (tip.x + dx, tip.y + dy, dx, dy)


def candidate_moves(tip: GrowthTip) -> list[Direction]:
    if tip.is_trunk:
        raw_moves = [(tip.dx, tip.dy), (0, -1), (-1, -1), (-1, 0), (1, 0), (1, -1), (0, 1)]
    else:
        raw_moves = [(tip.dx, tip.dy), (0, 1), (-1, 1), (-1, 0), (1, 0), (1, 1), (0, -1)]

    moves: list[Direction] = []
    for move in raw_moves:
        if move not in moves:
            moves.append(move)
    return moves


def move_score(
    tip: GrowthTip,
    state: IvyState,
    config: dict,
    x: int,
    y: int,
    dx: int,
    dy: int,
    layout: SceneLayout,
    rng: random.Random,
) -> float:
    score = 0.0
    if (dx, dy) == (tip.dx, tip.dy):
        score += float(config["forward_bonus"])
    else:
        score -= float(config["turn_penalty"])

    if tip.is_trunk:
        score += trunk_guidance_score(tip, state, config, x, y, dx, dy, layout)
    else:
        score += branch_guidance_score(config, x, y, dx, dy, layout)

    distance = distance_to_structure(state, x, y)
    score += min(distance, 3) * 0.25 if tip.is_trunk else min(distance, 2) * 0.1
    score += sum((x + ox, y + oy) in state.stems for ox, oy in NEIGHBORS_4) * 0.25
    score += rng.random() * float(config["organic_variation"])
    return score


def trunk_guidance_score(
    tip: GrowthTip,
    state: IvyState,
    config: dict,
    x: int,
    y: int,
    dx: int,
    dy: int,
    layout: SceneLayout,
) -> float:
    hero_zone = layout.hero_guide
    hero_right = _effective_hero_right(layout)
    info_zone = layout.info_guide
    support_top, support_bottom, support_left, support_right = support_geometry(config, layout)
    top_soft_limit = int(config["top_edge_soft_limit"])
    ascent_margin = int(config.get("hero_ascent_margin", 0))
    top_traverse_margin = int(config.get("hero_top_traverse_margin", 0))
    top_commit_margin = int(config.get("hero_top_commit_margin", 1))
    top_commit_bonus = float(config.get("hero_top_commit_bonus", 0.0))
    top_drop_penalty = float(config.get("hero_top_drop_penalty", 0.0))
    hero_contact_margin = int(config.get("hero_contact_margin", 2))
    right_staging_margin = int(config.get("right_staging_margin", 12))
    right_staging_left_penalty = float(config.get("right_staging_left_penalty", 0.0))
    panel_corridor_penalty = float(config.get("panel_corridor_penalty", 0.0))
    approach_left_bonus = float(config.get("hero_approach_left_bonus", 0.0))
    approach_diagonal_bonus = float(config.get("hero_approach_diagonal_bonus", 0.0))
    top_commit_state_bonus = float(config.get("hero_top_state_bonus", 0.0))
    top_commit_state_drop_penalty = float(config.get("hero_top_state_drop_penalty", 0.0))

    score = 0.0
    hero_contacted = x <= hero_right + hero_contact_margin

    if not hero_contacted:
        if x > hero_right + right_staging_margin:
            if dx == -1:
                score += approach_left_bonus
            if dx == -1 and dy == -1:
                score += approach_diagonal_bonus
            if dx == 0 and dy == -1:
                score += 0.5 * approach_left_bonus
            if dx >= 0 and dy == 0:
                score -= right_staging_left_penalty
        if (
            info_zone.x - 1 <= x <= info_zone.right + 2
            and hero_zone.y - 2 <= y <= info_zone.bottom + 3
        ):
            if dx >= 0:
                score -= panel_corridor_penalty
            if dx == -1:
                score += 0.75 * panel_corridor_penalty

    if hero_right < tip.x <= hero_right + ascent_margin and tip.y >= hero_zone.y:
        if dy == -1:
            score += float(config.get("hero_ascent_bonus", 0.0))
        if dx == -1 and dy == -1:
            score += 0.6 * float(config.get("hero_ascent_bonus", 0.0))
        if dx == 0 and dy == -1:
            score += 0.35 * float(config.get("hero_ascent_bonus", 0.0))
        if dy == 0:
            score -= 0.75 * float(config.get("hero_pileup_penalty", 0.0))
        if dy == 1:
            score -= float(config.get("hero_pileup_penalty", 0.0))

    if tip.y > support_bottom:
        if dy == -1:
            score += float(config["trunk_climb_bonus"])
        if dx == -1 and dy == -1:
            score += float(config["trunk_diagonal_bonus"])
        if abs(dx) == 1:
            score += 0.5
        if dy == 1:
            score -= float(config["trunk_reverse_penalty"])
        if y < tip.y:
            score += 3.0
    elif support_top <= tip.y <= support_bottom:
        if dx == -1:
            score += float(config["support_traverse_bonus"])
        if dx == -1 and dy == 1:
            score += float(config["support_wrap_bonus"])
        if dy == -1:
            score += 0.25
        if dy == 1:
            score += 1.25
        if x < support_left or x > support_right:
            score -= 3.5
        if y < support_top or y > support_bottom + 1:
            score -= 2.0
        if x < tip.x:
            score += 2.5
    else:
        if dy == 1:
            score += float(config["settle_down_bonus"])
        if dx == -1:
            score += 2.0

    if y <= top_soft_limit:
        score -= float(config["top_edge_penalty"])
        if dy == 1:
            score += 1.5
        if dy == -1:
            score -= 1.0

    if x > hero_right and dx == -1:
        score += 1.25
    if x < hero_zone.x and dx == 1:
        score -= 0.75

    if x > hero_right + ascent_margin and dx == -1 and dy == -1:
        score += float(config.get("pre_ascent_diagonal_bonus", 0.0))
    if x > hero_right + ascent_margin and dy == 0:
        score -= 0.5 * float(config.get("hero_pileup_penalty", 0.0))

    if (
        hero_zone.y - top_traverse_margin <= y <= hero_zone.y + top_traverse_margin
        and hero_zone.x <= x <= hero_right + ascent_margin
    ):
        if dx == -1:
            score += float(config.get("hero_top_traverse_bonus", 0.0))
        if dx == -1 and dy == 0:
            score += 0.5 * float(config.get("hero_top_traverse_bonus", 0.0))
        if dy == 1:
            score -= 0.75 * float(config.get("hero_top_traverse_bonus", 0.0))

    hero_mid_x = hero_zone.x + hero_zone.width // 2
    if (
        hero_zone.y - top_commit_margin <= y <= hero_zone.y + top_commit_margin
        and x >= hero_mid_x
    ):
        if dx == -1:
            score += top_commit_bonus
        if dx == -1 and dy == 0:
            score += 0.5 * top_commit_bonus
        if dy == 1:
            score -= top_drop_penalty
        if dx >= 0 and dy >= 0:
            score -= 0.75 * top_drop_penalty

    if state.trunk_route_phase == "hero_top":
        if dx == -1:
            score += top_commit_state_bonus
        if dx == -1 and dy == 0:
            score += 0.5 * top_commit_state_bonus
        if dy == 1:
            score -= top_commit_state_drop_penalty
        if dx >= 0 and dy >= 0:
            score -= 0.75 * top_commit_state_drop_penalty

    score += below_hero_recovery_score(config, x, y, dx, dy, layout, branch=False)
    score += floor_avoidance_score(config, y, dx, dy, layout, branch=False)

    overlap_margin = int(config.get("hero_lateral_overlap_margin", 0))
    lateral_penalty = float(config.get("hero_lateral_entry_penalty", 0.0))
    if lateral_penalty > 0 and (hero_zone.y - overlap_margin) <= y <= (hero_zone.bottom + overlap_margin):
        if hero_right < x <= hero_right + overlap_margin and dx < 0:
            score -= lateral_penalty
            if dy != 0:
                score += 0.75
        if hero_zone.x - overlap_margin <= x < hero_zone.x and dx > 0:
            score -= lateral_penalty
            if dy != 0:
                score += 0.75

    score += contour_follow_score(config, x, y, dx, dy, layout)
    score -= proximity_pressure_score(config, x, y, dx, dy, layout)
    return score


def branch_guidance_score(config: dict, x: int, y: int, dx: int, dy: int, layout: SceneLayout) -> float:
    hero_zone = layout.hero_guide
    hero_right = _effective_hero_right(layout)
    ascent_margin = int(config.get("hero_ascent_margin", 0))
    score = 0.0
    if dy == 1:
        score += float(config["branch_gravity_bonus"])
    if dy == -1:
        score -= 0.75
    if y >= hero_zone.bottom and dx == 0:
        score += 0.5

    nearest_zone_x = hero_zone.x if x < hero_zone.x else hero_right
    if abs(x - nearest_zone_x) <= int(config.get("hero_boundary_attraction", config["hero_contour_attraction"])):
        score += 0.75
    if (x, y) in layout.region_cells.get("below_hero", frozenset()):
        score += 0.5
    if y <= int(config["top_edge_soft_limit"]):
        score -= 0.5 * float(config["top_edge_penalty"])
        if dy == 1:
            score += 1.0

    if hero_right < x <= hero_right + ascent_margin and hero_zone.y <= y <= hero_zone.bottom:
        if dy == -1:
            score += 0.5 * float(config.get("hero_ascent_bonus", 0.0))
        if dx != 0 and dy == 0:
            score -= 0.5 * float(config.get("hero_pileup_penalty", 0.0))

    overlap_margin = int(config.get("hero_lateral_overlap_margin", 0))
    lateral_penalty = 0.6 * float(config.get("hero_lateral_entry_penalty", 0.0))
    if lateral_penalty > 0 and (hero_zone.y - overlap_margin) <= y <= (hero_zone.bottom + overlap_margin):
        if hero_right < x <= hero_right + overlap_margin and dx < 0:
            score -= lateral_penalty
            if dy != 0:
                score += 0.5
        if hero_zone.x - overlap_margin <= x < hero_zone.x and dx > 0:
            score -= lateral_penalty
            if dy != 0:
                score += 0.5

    score += 0.75 * contour_follow_score(config, x, y, dx, dy, layout)
    score -= 0.75 * proximity_pressure_score(config, x, y, dx, dy, layout)
    score += below_hero_recovery_score(config, x, y, dx, dy, layout, branch=True)
    score += floor_avoidance_score(config, y, dx, dy, layout, branch=True)
    return score


def branch_direction(config: dict, x: int, y: int, dy: int, layout: SceneLayout) -> Direction | None:
    if dy != 0:
        midpoint = layout.ivy_bounds.width // 2
        branch_dx, branch_dy = ((1, 0) if x > midpoint else (-1, 0))
    else:
        branch_dx, branch_dy = (0, 1)

    if is_open(x + branch_dx, y + branch_dy, layout):
        return (branch_dx, branch_dy)
    for candidate in ((-1, 0), (1, 0), (0, 1), (0, -1)):
        if is_open(x + candidate[0], y + candidate[1], layout):
            return candidate
    return None


def support_geometry(config: dict, layout: SceneLayout) -> tuple[int, int, int, int]:
    hero_zone = layout.hero_guide
    hero_right = _effective_hero_right(layout)
    band_height = int(config["support_band_height"])
    above_gap = max(0, hero_zone.y - 1)
    below_gap = max(0, layout.ivy_bounds.height - hero_zone.bottom - 2)
    min_headroom = int(config["support_min_headroom"])

    if above_gap >= band_height + min_headroom:
        support_top = max(1, hero_zone.y - int(config["support_band_above"]))
        support_bottom = max(support_top, support_top + band_height - 1)
    elif below_gap >= band_height:
        support_top = min(layout.ivy_bounds.height - 2, hero_zone.bottom + 1)
        support_bottom = min(layout.ivy_bounds.height - 2, support_top + band_height - 1)
    else:
        support_top = max(1, min(layout.ivy_bounds.height - 2, hero_zone.y + band_height))
        support_bottom = min(layout.ivy_bounds.height - 2, max(support_top, hero_zone.bottom - 2))

    support_left = max(1, hero_zone.x - int(config["support_span_left"]))
    support_right = min(layout.ivy_bounds.width - 2, hero_right + int(config["support_span_right"]))
    return support_top, support_bottom, support_left, support_right


def contour_follow_score(config: dict, x: int, y: int, dx: int, dy: int, layout: SceneLayout) -> float:
    score = 0.0
    hero_attraction = float(config.get("hero_boundary_attraction", config["hero_contour_attraction"]))
    score += _mask_contour_follow_score(
        x,
        y,
        dx,
        dy,
        layout.hero_mask_boundary_cells,
        layout.hero_mask_cells,
        hero_attraction,
    )

    info_zone = layout.info_guide
    info_attraction = float(config.get("info_boundary_attraction", config["hero_contour_attraction"]))
    horizontal_gap = min(abs(x - info_zone.x), abs(x - info_zone.right))
    vertical_gap = min(abs(y - info_zone.y), abs(y - info_zone.bottom))
    near_vertical_face = info_zone.y <= y <= info_zone.bottom and horizontal_gap <= info_attraction
    near_horizontal_face = info_zone.x <= x <= info_zone.right and vertical_gap <= info_attraction
    if near_vertical_face and dy != 0:
        score += 2.0
    if near_horizontal_face and dx != 0:
        score += 2.0
    if info_zone.contains(x + dx, y + dy):
        score -= 6.0
    return score


def proximity_pressure_score(config: dict, x: int, y: int, dx: int, dy: int, layout: SceneLayout) -> float:
    score = 0.0
    score += _mask_proximity_pressure_score(
        x,
        y,
        dx,
        dy,
        layout.hero_mask_boundary_cells,
        int(config.get("hero_collision_soft_margin", config.get("collision_soft_margin", 0))),
        float(config.get("hero_collision_proximity_penalty", config.get("collision_proximity_penalty", 0.0))),
    )

    info_zone = layout.info_guide
    soft_margin = int(config.get("info_collision_soft_margin", config.get("collision_soft_margin", 0)))
    proximity_penalty = float(config.get("info_collision_proximity_penalty", config.get("collision_proximity_penalty", 0.0)))
    if soft_margin <= 0 or proximity_penalty <= 0:
        return score
    horizontal_gap = info_zone.x - x if x < info_zone.x else x - info_zone.right if x > info_zone.right else 0
    vertical_gap = info_zone.y - y if y < info_zone.y else y - info_zone.bottom if y > info_zone.bottom else 0
    gap = min(horizontal_gap, vertical_gap) if horizontal_gap and vertical_gap else max(horizontal_gap, vertical_gap)
    if gap <= 0 or gap > soft_margin:
        return score

    pressure = (soft_margin - gap + 1) / soft_margin
    score += proximity_penalty * pressure
    if x < info_zone.x and dx > 0:
        score += 0.5 * proximity_penalty * pressure
    elif x > info_zone.right and dx < 0:
        score += 0.5 * proximity_penalty * pressure
    if y < info_zone.y and dy > 0:
        score += 0.5 * proximity_penalty * pressure
    elif y > info_zone.bottom and dy < 0:
        score += 0.5 * proximity_penalty * pressure

    return score


def _mask_contour_follow_score(
    x: int,
    y: int,
    dx: int,
    dy: int,
    boundary_cells: frozenset[Point],
    blocked_cells: frozenset[Point],
    attraction: float,
) -> float:
    if not boundary_cells or attraction <= 0:
        return 0.0
    nearest = _nearest_point(boundary_cells, x, y)
    if nearest is None:
        return 0.0
    bx, by = nearest
    horizontal_gap = abs(x - bx)
    vertical_gap = abs(y - by)
    score = 0.0
    if horizontal_gap <= attraction and dy != 0:
        score += 2.0
    if vertical_gap <= attraction and dx != 0:
        score += 2.0
    if (x + dx, y + dy) in blocked_cells:
        score -= 6.0
    return score


def _mask_proximity_pressure_score(
    x: int,
    y: int,
    dx: int,
    dy: int,
    boundary_cells: frozenset[Point],
    soft_margin: int,
    proximity_penalty: float,
) -> float:
    if not boundary_cells or soft_margin <= 0 or proximity_penalty <= 0:
        return 0.0
    nearest = _nearest_point(boundary_cells, x, y)
    if nearest is None:
        return 0.0
    bx, by = nearest
    horizontal_gap = abs(x - bx)
    vertical_gap = abs(y - by)
    gap = min(horizontal_gap, vertical_gap) if horizontal_gap and vertical_gap else max(horizontal_gap, vertical_gap)
    if gap <= 0 or gap > soft_margin:
        return 0.0

    pressure = (soft_margin - gap + 1) / soft_margin
    score = proximity_penalty * pressure
    if x < bx and dx > 0:
        score += 0.5 * proximity_penalty * pressure
    elif x > bx and dx < 0:
        score += 0.5 * proximity_penalty * pressure
    if y < by and dy > 0:
        score += 0.5 * proximity_penalty * pressure
    elif y > by and dy < 0:
        score += 0.5 * proximity_penalty * pressure
    return score


def _nearest_point(points: frozenset[Point], x: int, y: int) -> Point | None:
    if not points:
        return None
    return min(points, key=lambda point: abs(point[0] - x) + abs(point[1] - y))


def below_hero_recovery_score(
    config: dict,
    x: int,
    y: int,
    dx: int,
    dy: int,
    layout: SceneLayout,
    branch: bool,
) -> float:
    hero_zone = layout.hero_guide
    hero_right = _effective_hero_right(layout)
    if (x, y) not in layout.region_cells.get("below_hero", frozenset()):
        return 0.0

    lateral_penalty = float(config.get("below_hero_left_traverse_penalty", 0.0))
    recovery_bonus = float(config.get("below_hero_recovery_bonus", 0.0))
    margin = int(config.get("below_hero_escape_margin", 0))
    scale = 0.6 if branch else 1.0

    score = 0.0
    if x <= hero_right + margin:
        if dx < 0 and dy == 0:
            score -= lateral_penalty * scale
        if dx < 0 and dy > 0:
            score -= 0.75 * lateral_penalty * scale
        if dy < 0:
            score += recovery_bonus * scale
        if dx == 0 and dy < 0:
            score += 0.5 * recovery_bonus * scale
    return score


def floor_avoidance_score(
    config: dict,
    y: int,
    dx: int,
    dy: int,
    layout: SceneLayout,
    branch: bool,
) -> float:
    band_height = int(config.get("floor_band_height", 0))
    if band_height <= 0:
        return 0.0

    floor_start = layout.ivy_bounds.height - 1 - band_height
    if y < floor_start:
        return 0.0

    penalty = float(config.get("floor_horizontal_penalty", 0.0))
    upward_bonus = float(config.get("floor_escape_bonus", 0.0))
    scale = 0.7 if branch else 1.0

    score = 0.0
    if dx != 0 and dy == 0:
        score -= penalty * scale
    if dy < 0:
        score += upward_bonus * scale
    return score


def limit_tips(tips: list[GrowthTip], max_tips: int, rng: random.Random) -> list[GrowthTip]:
    if len(tips) <= max_tips:
        return tips
    trunk_tips = [tip for tip in tips if tip.is_trunk]
    branch_tips = [tip for tip in tips if not tip.is_trunk]
    kept: list[GrowthTip] = []
    if trunk_tips:
        kept.extend(trunk_tips[:1])
    remaining = max_tips - len(kept)
    if remaining > 0 and branch_tips:
        kept.extend(branch_tips if len(branch_tips) <= remaining else rng.sample(branch_tips, remaining))
    return kept


def is_open(x: int, y: int, layout: SceneLayout) -> bool:
    return (x, y) in layout.allowed_cells


def distance_to_structure(state: IvyState, x: int, y: int) -> int:
    nearest = 4
    for ox, oy in ((1, 0), (-1, 0), (0, 1), (0, -1), (2, 0), (-2, 0), (0, 2), (0, -2)):
        if (x + ox, y + oy) in state.stems:
            nearest = min(nearest, abs(ox) + abs(oy))
    return nearest


def record_failed_move(state: IvyState, x: int, y: int, layout: SceneLayout, debug_config: dict) -> None:
    if debug_config.get("enabled") is not True:
        return
    if x <= 0 or y <= 0 or x >= layout.ivy_bounds.width - 1 or y >= layout.ivy_bounds.height - 1:
        reason = "bounds"
    elif (x, y) in state.stems:
        reason = "stem"
    elif any(zone.contains(x, y) for zone in layout.no_go_zones):
        reason = "no_go"
    else:
        reason = "blocked"
    count_debug(state, "failed_move_counts", reason)
