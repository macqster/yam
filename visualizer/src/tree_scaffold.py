from __future__ import annotations

from dataclasses import dataclass

from layout import SceneLayout
from render_field import glyph_for_density


DARK_BROWN = "\x1b[38;5;94m"
MID_BROWN = "\x1b[38;5;130m"
RESET = "\x1b[0m"


Point = tuple[int, int]


@dataclass(frozen=True)
class ScaffoldCell:
    x: int
    y: int
    glyph: str
    style: str
    density: float


@dataclass(frozen=True)
class TreeScaffold:
    cells: tuple[ScaffoldCell, ...]


def build_tree_scaffold(layout: SceneLayout, config: dict) -> TreeScaffold:
    """Build a static woody scaffold using the trunk distance field.

    The scaffold is intentionally mask-aware rather than mask-snapped. Config
    values steer the base, fork, reach, and vertical lift, while the trunk
    field provides a smooth preference surface so small changes visibly move
    the structure.
    """
    scaffold_cfg = config.get("scaffold", {})
    if not scaffold_cfg.get("enabled", True):
        return TreeScaffold(cells=())

    trunk_field = getattr(layout, "trunk_field", {})
    if not trunk_field:
        return TreeScaffold(cells=())

    hero = layout.hero
    hero_mask = layout.hero_raw_mask_cells
    bounds = layout.ivy_bounds
    scaffold_region = layout.region_cells.get("below_hero", layout.allowed_cells)
    if not scaffold_region:
        scaffold_region = layout.allowed_cells
    allowed_points = set(layout.allowed_cells)
    preferred_points = set(scaffold_region)

    base_x = int(scaffold_cfg.get("base_x", 0))
    base_y = int(scaffold_cfg.get("base_y", 0))
    trunk_height = max(0, int(scaffold_cfg.get("trunk_height", 4)))
    fork_y = int(scaffold_cfg.get("fork_y", 0))
    left_reach = max(1, int(scaffold_cfg.get("left_reach", 10)))
    right_reach = max(1, int(scaffold_cfg.get("right_reach", 12)))
    upper_lift = max(0, int(scaffold_cfg.get("upper_lift", 8)))
    thickness = max(1, int(scaffold_cfg.get("thickness", 2)))

    hero_center_x = hero.x + hero.width // 2
    hero_bottom = hero.bottom
    target_base_x = max(bounds.x + 1, min(hero_center_x + base_x // 2, bounds.right - 2))
    target_base_y = max(hero_bottom, min(hero_bottom + base_y // 2, bounds.bottom - 4))
    target_fork_y = max(
        target_base_y + 1,
        min(
            hero_bottom + max(1, trunk_height) + max(0, fork_y // 4),
            bounds.bottom - 2,
        ),
    )
    max_soft_distance = max(2, trunk_height + max(1, upper_lift // 2) + 1)
    corridor_x_window = max(4, left_reach + right_reach // 4 + 4)
    corridor_y_window = max(4, trunk_height + upper_lift + 6)

    base_point = _choose_soft_point(
        trunk_field=trunk_field,
        bounds=bounds,
        allowed_points=allowed_points,
        preferred_points=preferred_points,
        target_x=target_base_x,
        target_y=target_base_y,
        x_window=corridor_x_window,
        y_window=corridor_y_window,
        max_soft_distance=max_soft_distance,
    )

    trunk_points = _follow_soft_ridge(
        trunk_field=trunk_field,
        bounds=bounds,
        allowed_points=allowed_points,
        preferred_points=preferred_points,
        start=base_point,
        target_y=target_fork_y,
        preferred_x=target_base_x,
        max_soft_distance=max_soft_distance,
    )
    trunk_points = _smooth_polyline(trunk_points)
    if not trunk_points:
        trunk_points = [base_point]

    fork_point = min(trunk_points, key=lambda p: (abs(p[1] - target_fork_y), abs(p[0] - target_base_x)))

    branch_tip_y = fork_point[1] - max(1, upper_lift) - max(0, trunk_height // 4)
    left_tip = _choose_branch_tip(
        trunk_field=trunk_field,
        bounds=bounds,
        allowed_points=allowed_points,
        preferred_points=preferred_points,
        fork_point=fork_point,
        target_x=fork_point[0] - left_reach,
        target_y=branch_tip_y,
        side="left",
        max_soft_distance=max_soft_distance + 2,
    )
    right_tip = _choose_branch_tip(
        trunk_field=trunk_field,
        bounds=bounds,
        allowed_points=allowed_points,
        preferred_points=preferred_points,
        fork_point=fork_point,
        target_x=fork_point[0] + right_reach,
        target_y=branch_tip_y,
        side="right",
        max_soft_distance=max_soft_distance + 2,
    )

    cells: dict[tuple[int, int], ScaffoldCell] = {}
    _stamp_path(cells, trunk_points, DARK_BROWN, thickness=thickness, base_density=0.88)

    left_branch_points = _follow_soft_path(
        trunk_field=trunk_field,
        bounds=bounds,
        allowed_points=allowed_points,
        preferred_points=preferred_points,
        start=fork_point,
        end=left_tip,
        preferred_side="left",
        max_soft_distance=max_soft_distance + 2,
    )
    left_branch_points = _smooth_polyline(left_branch_points)
    _stamp_path(cells, left_branch_points, DARK_BROWN, thickness=max(1, thickness - 1), base_density=0.72)

    right_branch_points = _follow_soft_path(
        trunk_field=trunk_field,
        bounds=bounds,
        allowed_points=allowed_points,
        preferred_points=preferred_points,
        start=fork_point,
        end=right_tip,
        preferred_side="right",
        max_soft_distance=max_soft_distance + 2,
    )
    right_branch_points = _smooth_polyline(right_branch_points)
    _stamp_path(cells, right_branch_points, DARK_BROWN, thickness=max(1, thickness - 1), base_density=0.72)

    seat_candidates = [
        point
        for point in trunk_field
        if abs(point[1] - (hero.y + hero.height - 7)) <= 2
        and abs(point[0] - hero_center_x) <= 10
    ]
    if seat_candidates:
        seat_target = min(
            seat_candidates,
            key=lambda p: (
                abs(p[1] - (hero.y + hero.height - 7)),
                abs(p[0] - hero_center_x),
            ),
        )
        seat_branch = _follow_soft_path(
            trunk_field=trunk_field,
            bounds=bounds,
            allowed_points=allowed_points,
            preferred_points=preferred_points,
            start=fork_point,
            end=seat_target,
            preferred_side="right" if seat_target[0] >= fork_point[0] else "left",
            max_soft_distance=max_soft_distance + 2,
        )
        seat_branch = _smooth_polyline(seat_branch)
        _stamp_path(cells, seat_branch, MID_BROWN, thickness=1, base_density=0.58)

    visible_cells: dict[tuple[int, int], ScaffoldCell] = {}
    for cell in cells.values():
        if not _inside_bounds(cell.x, cell.y, bounds):
            continue
        if (cell.x, cell.y) in hero_mask:
            continue
        visible_cells[(cell.x, cell.y)] = cell

    anchor_point = next((point for point in trunk_points if point in visible_cells), None)
    if anchor_point is None and base_point in visible_cells:
        anchor_point = base_point
    elif anchor_point is None and fork_point in visible_cells:
        anchor_point = fork_point
    visible_cells = _keep_connected_component(visible_cells, anchor_point)
    trimmed = tuple(sorted(visible_cells.values(), key=lambda cell: (cell.y, cell.x)))
    return TreeScaffold(cells=trimmed)


def render_tree_scaffold(scaffold: TreeScaffold) -> list[str]:
    """Render scaffold cells into compact row strings for tests/debugging."""
    if not scaffold.cells:
        return []

    min_y = min(cell.y for cell in scaffold.cells)
    max_y = max(cell.y for cell in scaffold.cells)
    min_x = min(cell.x for cell in scaffold.cells)
    max_x = max(cell.x for cell in scaffold.cells)

    width = max_x - min_x + 1
    rows = [[" " for _ in range(width)] for _ in range(max_y - min_y + 1)]
    for cell in scaffold.cells:
        rows[cell.y - min_y][cell.x - min_x] = f"{cell.style}{glyph_for_density(cell.density, cell.glyph)}{RESET}"
    return ["".join(row) for row in rows]


def _stamp_path(
    cells: dict[tuple[int, int], ScaffoldCell],
    points: list[Point],
    style: str,
    *,
    thickness: int,
    base_density: float,
) -> None:
    if not points:
        return

    mid = max(1, len(points) - 1)
    for index, (x, y) in enumerate(points):
        glyph = _glyph_for_segment(points, index)
        taper = 0.12 * (1.0 - abs((index / mid) - 0.5) * 2.0)
        density = max(0.05, min(1.0, base_density + taper))
        for dy in range(-(thickness // 2), thickness // 2 + 1):
            for dx in range(-(thickness // 2), thickness // 2 + 1):
                px = x + dx
                py = y + dy
                cells[(px, py)] = ScaffoldCell(px, py, glyph, style, density=density)


def _glyph_for_segment(points: list[Point], index: int) -> str:
    prev_point = points[max(0, index - 1)]
    next_point = points[min(len(points) - 1, index + 1)]
    dx = next_point[0] - prev_point[0]
    dy = next_point[1] - prev_point[1]

    if abs(dx) <= 1 and abs(dy) >= 1:
        return "|"
    if dx > 0 and dy < 0:
        return "/"
    if dx < 0 and dy < 0:
        return "\\"
    if dx > 0 and dy > 0:
        return "\\"
    if dx < 0 and dy > 0:
        return "/"
    if abs(dx) >= 1 and abs(dy) <= 1:
        return "-"
    return "|"


def _choose_soft_point(
    trunk_field: dict[Point, int],
    bounds,
    allowed_points: set[Point],
    preferred_points: set[Point],
    target_x: int,
    target_y: int,
    x_window: int,
    y_window: int,
    max_soft_distance: int,
) -> Point:
    candidates = [
        point
        for point in trunk_field
        if point in allowed_points
        and bounds.contains(*point)
        and abs(point[0] - target_x) <= x_window
        and abs(point[1] - target_y) <= y_window
        and trunk_field[point] <= max_soft_distance + 4
    ]
    if not candidates:
        candidates = [
            point
            for point in trunk_field
            if point in allowed_points
            and bounds.contains(*point)
            and trunk_field[point] <= max_soft_distance + 4
        ]
    if not candidates:
        return (target_x, target_y)

    return min(
        candidates,
        key=lambda p: (
            trunk_field.get(p, max_soft_distance + 4) * 3
            + (0 if p in preferred_points else 2.0),
            abs(p[0] - target_x),
            abs(p[1] - target_y),
            -p[1],
        ),
    )


def _follow_soft_ridge(
    trunk_field: dict[Point, int],
    bounds,
    allowed_points: set[Point],
    preferred_points: set[Point],
    start: Point,
    target_y: int,
    preferred_x: int,
    max_soft_distance: int,
) -> list[Point]:
    current = start
    points: list[Point] = [current]
    visited: set[Point] = {current}
    step_y = -1 if target_y < current[1] else 1

    while current[1] != target_y and len(points) < bounds.width * bounds.height:
        candidates: list[tuple[tuple[float, float, float, float, float], Point]] = []
        for dx in (-1, 0, 1):
            nxt = (current[0] + dx, current[1] + step_y)
            if not bounds.contains(*nxt) or nxt in visited or nxt not in allowed_points:
                continue
            distance = trunk_field.get(nxt, max_soft_distance + 8)
            if distance > max_soft_distance:
                continue
            score = (
                float(distance) * 3.0 + (0 if nxt in preferred_points else 1.5),
                abs(nxt[0] - preferred_x),
                abs(nxt[1] - target_y),
                abs(nxt[0] - current[0]),
                abs(nxt[1] - current[1]),
            )
            candidates.append((score, nxt))

        if not candidates:
            for dx in (-2, -1, 0, 1, 2):
                nxt = (current[0] + dx, current[1] + step_y)
                if not bounds.contains(*nxt) or nxt in visited or nxt not in allowed_points:
                    continue
                distance = trunk_field.get(nxt, max_soft_distance + 8)
                score = (
                    float(distance) * 3.5 + (0 if nxt in preferred_points else 1.5),
                    abs(nxt[0] - preferred_x),
                    abs(nxt[1] - target_y),
                    abs(nxt[0] - current[0]),
                    abs(nxt[1] - current[1]),
                )
                candidates.append((score, nxt))

        if not candidates:
            break

        _, current = min(candidates, key=lambda item: item[0])
        visited.add(current)
        if points[-1] != current:
            points.append(current)

    if points[-1][1] != target_y:
        fallback_end = (preferred_x, target_y)
        points.extend(_line_points(points[-1], fallback_end)[1:])

    return points


def _follow_soft_path(
    trunk_field: dict[Point, int],
    bounds,
    allowed_points: set[Point],
    preferred_points: set[Point],
    start: Point,
    end: Point,
    preferred_side: str,
    max_soft_distance: int,
) -> list[Point]:
    if start == end:
        return [start]

    current = start
    points: list[Point] = [current]
    visited: set[Point] = {current}
    side_bias = -1 if preferred_side == "left" else 1
    remaining_steps = max(1, bounds.width + bounds.height)

    while current != end and remaining_steps > 0:
        remaining_steps -= 1
        candidates: list[tuple[tuple[float, float, float, float], Point]] = []
        for dx, dy in (
            (1, 0), (-1, 0), (0, 1), (0, -1),
            (1, -1), (-1, -1), (1, 1), (-1, 1),
        ):
            nxt = (current[0] + dx, current[1] + dy)
            if not bounds.contains(*nxt) or nxt in visited or nxt not in allowed_points:
                continue
            distance = trunk_field.get(nxt, max_soft_distance + 8)
            if distance > max_soft_distance + 2:
                continue
            score = (
                float(distance) * 2.75 + (0 if nxt in preferred_points else 1.25),
                abs(nxt[0] - end[0]),
                abs(nxt[1] - end[1]),
                abs(nxt[0] - current[0]) + abs(nxt[1] - current[1]),
            )
            if preferred_side == "left" and nxt[0] <= current[0]:
                score = (score[0] - 0.45, score[1], score[2], score[3])
            elif preferred_side == "right" and nxt[0] >= current[0]:
                score = (score[0] - 0.45, score[1], score[2], score[3])
            if (dx == side_bias and abs(dy) <= 1) or dy < 0:
                score = (score[0] - 0.15, score[1], score[2], score[3])
            candidates.append((score, nxt))

        if not candidates:
            break

        _, current = min(candidates, key=lambda item: item[0])
        visited.add(current)
        if points[-1] != current:
            points.append(current)

    if points[-1] != end:
        points.extend(_line_points(points[-1], end)[1:])
    return points


def _choose_branch_tip(
    trunk_field: dict[Point, int],
    bounds,
    allowed_points: set[Point],
    preferred_points: set[Point],
    fork_point: Point,
    target_x: int,
    target_y: int,
    side: str,
    max_soft_distance: int,
) -> Point:
    if side == "left":
        candidates = [
            point
            for point in trunk_field
            if bounds.contains(*point)
            and point in allowed_points
            and point[0] <= fork_point[0] - 1
            and trunk_field[point] <= max_soft_distance + 3
        ]
    else:
        candidates = [
            point
            for point in trunk_field
            if bounds.contains(*point)
            and point in allowed_points
            and point[0] >= fork_point[0] + 1
            and trunk_field[point] <= max_soft_distance + 3
        ]

    if not candidates:
        return (target_x, target_y)

    return min(
        candidates,
        key=lambda p: (
            trunk_field.get(p, max_soft_distance + 3) * 2.5
            + (0 if p in preferred_points else 1.5),
            abs(p[0] - target_x),
            abs(p[1] - target_y),
        ),
    )


def _smooth_polyline(points: list[Point]) -> list[Point]:
    if len(points) <= 2:
        return points

    smoothed: list[Point] = [points[0]]
    for point in points[1:]:
        if len(smoothed) >= 2 and _is_collinear(smoothed[-2], smoothed[-1], point):
            smoothed[-1] = point
            continue
        if len(smoothed) >= 2 and smoothed[-2] == point:
            smoothed.pop()
            continue
        smoothed.append(point)
    return smoothed


def _keep_connected_component(
    cells: dict[Point, ScaffoldCell],
    start: Point | None,
) -> dict[Point, ScaffoldCell]:
    if not cells:
        return cells
    if start is None or start not in cells:
        return _largest_connected_component(cells)

    queue = [start]
    keep: set[Point] = {start}
    while queue:
        x, y = queue.pop()
        for nxt in (
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y + 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
            (x - 1, y - 1),
        ):
            if nxt in cells and nxt not in keep:
                keep.add(nxt)
                queue.append(nxt)

    return {point: cells[point] for point in keep}


def _largest_connected_component(cells: dict[Point, ScaffoldCell]) -> dict[Point, ScaffoldCell]:
    if not cells:
        return cells

    seen: set[Point] = set()
    best: set[Point] = set()
    for start in cells:
        if start in seen:
            continue
        queue = [start]
        component: set[Point] = {start}
        seen.add(start)
        while queue:
            x, y = queue.pop()
            for nxt in (
                (x + 1, y),
                (x - 1, y),
                (x, y + 1),
                (x, y - 1),
                (x + 1, y + 1),
                (x - 1, y + 1),
                (x + 1, y - 1),
                (x - 1, y - 1),
            ):
                if nxt in cells and nxt not in seen:
                    seen.add(nxt)
                    component.add(nxt)
                    queue.append(nxt)
        if len(component) > len(best):
            best = component

    return {point: cells[point] for point in best}


def _is_collinear(a: Point, b: Point, c: Point) -> bool:
    return (b[0] - a[0]) * (c[1] - b[1]) == (b[1] - a[1]) * (c[0] - b[0])


def _line_points(start: Point, end: Point) -> list[Point]:
    x1, y1 = start
    x2, y2 = end
    dx = x2 - x1
    dy = y2 - y1
    steps = max(abs(dx), abs(dy))
    if steps == 0:
        return [start]

    points: list[Point] = []
    for step in range(steps + 1):
        t = step / steps
        x = int(round(x1 + dx * t))
        y = int(round(y1 + dy * t))
        point = (x, y)
        if not points or points[-1] != point:
            points.append(point)
    return points


def _inside_bounds(x: int, y: int, bounds) -> bool:
    return bounds.x <= x < bounds.x + bounds.width and bounds.y <= y < bounds.y + bounds.height
