from __future__ import annotations

import random
from dataclasses import dataclass

from layout import SceneLayout
from terminal import RESET


Direction = tuple[int, int]
Point = tuple[int, int]

BROWN = "\x1b[38;5;94m"
DARK_BROWN = "\x1b[38;5;58m"
GREEN = "\x1b[38;5;34m"
LIGHT_GREEN = "\x1b[38;5;82m"
OLIVE = "\x1b[38;5;106m"

NEIGHBORS_4: tuple[Direction, ...] = ((1, 0), (-1, 0), (0, 1), (0, -1))


@dataclass
class GrowthTip:
    x: int
    y: int
    dx: int
    dy: int
    life: float
    max_life: float
    is_trunk: bool


class IvyEngine:
    def __init__(self, config: dict, seed: int = 7) -> None:
        self.config = config["ivy"]
        self.debug_config = self.config.get("debug", {})
        self.rng = random.Random(seed)
        self.size = None
        self.layout: SceneLayout | None = None
        self.frame = 0

        self.stems: set[Point] = set()
        self.stem_birth: dict[Point, int] = {}
        self.tips: list[GrowthTip] = []
        self.terminal_leaves: set[Point] = set()
        self.active_leaf_positions: set[Point] = set()
        self.active_leaf_dirs: dict[Point, Direction] = {}
        self.leaf_stamps: dict[Point, str] = {}
        self.dead_leaf_stamps: dict[Point, str] = {}
        self.thickened_wood: dict[Point, str] = {}
        self.debug_stats: dict[str, object] = {}

        self.leaf_patterns = self._load_leaf_patterns()

    def reset(self, size, layout: SceneLayout) -> None:
        self.size = size
        self.layout = layout
        self.frame = 0
        self.stems = set()
        self.stem_birth = {}
        self.tips = []
        self.terminal_leaves = set()
        self.active_leaf_positions = set()
        self.active_leaf_dirs = {}
        self.leaf_stamps = {}
        self.dead_leaf_stamps = {}
        self.thickened_wood = {}
        self.debug_stats = {
            "spawn_origin_counts": {},
            "failed_move_counts": {},
            "region_coverage": {},
            "stem_count": 0,
            "ornament_count": 0,
        }

        origin = self._initial_trunk_seed(layout)
        if origin is None:
            return

        x, y = origin
        self.stems.add((x, y))
        self.stem_birth[(x, y)] = self.frame
        trunk_life = float(self.config["trunk_life"])
        self.tips = [GrowthTip(x=x, y=y, dx=0, dy=-1, life=trunk_life, max_life=trunk_life, is_trunk=True)]
        self._update_debug_stats(layout)

    def tick(self, layout: SceneLayout) -> None:
        if self.layout != layout or self.size is None:
            return
        if not self.tips and len(self.stems) >= int(self.config["max_structural_cells"]):
            return

        self.frame += 1
        max_structural = int(self.config["max_structural_cells"])
        max_tips = int(self.config["max_tips"])
        branch_chance = float(self.config["branch_chance"])
        branch_life_min = int(self.config["branch_life_min"])
        branch_life_max = int(self.config["branch_life_max"])

        new_tips: list[GrowthTip] = []
        active_leaf_positions: set[Point] = set()
        active_leaf_dirs: dict[Point, Direction] = {}

        for tip in self.tips:
            if len(self.stems) >= max_structural:
                break

            if tip.is_trunk:
                tip.life -= float(self.config["trunk_decay"])
            else:
                tip.life -= float(self.config["branch_decay"])

            if tip.life <= 0:
                self.terminal_leaves.add((tip.x, tip.y))
                if not tip.is_trunk:
                    self._stamp_death_cluster(tip.x, tip.y, tip.dx, tip.dy, layout)
                continue

            move = self._select_move(tip, layout)
            if move is None:
                self.terminal_leaves.add((tip.x, tip.y))
                if not tip.is_trunk:
                    self._stamp_death_cluster(tip.x, tip.y, tip.dx, tip.dy, layout)
                continue

            new_x, new_y, dx, dy = move
            self.stems.add((new_x, new_y))
            self.stem_birth[(new_x, new_y)] = self.frame
            self.terminal_leaves.discard((tip.x, tip.y))
            active_leaf_positions.add((new_x, new_y))
            active_leaf_dirs[(new_x, new_y)] = (dx, dy)
            new_tip = GrowthTip(
                x=new_x,
                y=new_y,
                dx=dx,
                dy=dy,
                life=tip.life,
                max_life=tip.max_life,
                is_trunk=tip.is_trunk,
            )
            new_tips.append(new_tip)

            if tip.is_trunk and self.rng.random() < branch_chance:
                branch_direction = self._branch_direction(dx, dy, new_x, new_y, layout)
                if branch_direction is not None:
                    branch_life = float(self.rng.randint(branch_life_min, branch_life_max))
                    new_tips.append(
                        GrowthTip(
                            x=new_x,
                            y=new_y,
                            dx=branch_direction[0],
                            dy=branch_direction[1],
                            life=branch_life,
                            max_life=branch_life,
                            is_trunk=False,
                        )
                    )
                    self._count_debug("spawn_origin_counts", "branch")

        self.tips = self._limit_tips(new_tips, max_tips)
        self.active_leaf_positions = self.terminal_leaves | active_leaf_positions
        self.active_leaf_dirs = active_leaf_dirs
        self._rebuild_leaf_stamps(layout)
        self._rebuild_thickened_wood(layout)
        self._trim_ornaments()
        self._update_debug_stats(layout)

    def get_segments(self) -> dict[Point, str]:
        merged: dict[Point, str] = {}

        for point in self.stems:
            merged[point] = f"{self._wood_color_for(point)}{self._wood_char_for_cell(*point)}{RESET}"

        for point, glyph in self.thickened_wood.items():
            merged[point] = glyph

        for point, glyph in self.dead_leaf_stamps.items():
            merged[point] = glyph

        for point, glyph in self.leaf_stamps.items():
            merged[point] = glyph

        for point in self.active_leaf_positions:
            if point not in self._occupied_points(include_active=False):
                merged[point] = f"{LIGHT_GREEN}{self._stable_choice(['*', 'o', '+', '·'], *point, 11)}{RESET}"

        if self.debug_config.get("stem_only_view"):
            return {point: glyph for point, glyph in merged.items() if point in self.stems or point in self.thickened_wood}

        return merged

    def _initial_trunk_seed(self, layout: SceneLayout) -> Point | None:
        hero_zone = layout.no_go_zones[0]
        target_x = min(
            layout.ivy_bounds.width - 2,
            hero_zone.right + int(self.config["trunk_seed_offset_x"]),
        )
        start_y = layout.ivy_bounds.height - max(3, int(self.config["trunk_seed_bottom_margin"]))
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
            self._count_debug("spawn_origin_counts", "trunk_seed")
        return best

    def _select_move(self, tip: GrowthTip, layout: SceneLayout) -> tuple[int, int, int, int] | None:
        scored_moves: list[tuple[float, int, int]] = []
        for dx, dy in self._candidate_moves(tip):
            new_x = tip.x + dx
            new_y = tip.y + dy
            if not self._is_open(new_x, new_y, layout):
                self._record_failed_move(new_x, new_y, layout)
                continue
            score = self._move_score(tip, new_x, new_y, dx, dy, layout)
            scored_moves.append((score, dx, dy))

        if not scored_moves:
            return None

        scored_moves.sort(reverse=True)
        limit = min(3, len(scored_moves))
        weights = [max(0.05, scored_moves[index][0]) for index in range(limit)]
        choice_index = self.rng.choices(range(limit), weights=weights, k=1)[0]
        _, dx, dy = scored_moves[choice_index]
        return (tip.x + dx, tip.y + dy, dx, dy)

    def _candidate_moves(self, tip: GrowthTip) -> list[Direction]:
        if tip.is_trunk:
            raw_moves = [
                (tip.dx, tip.dy),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (1, 0),
                (1, -1),
                (0, 1),
            ]
        else:
            raw_moves = [
                (tip.dx, tip.dy),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (1, 0),
                (1, 1),
                (0, -1),
            ]

        moves: list[Direction] = []
        for move in raw_moves:
            if move not in moves:
                moves.append(move)
        return moves

    def _move_score(self, tip: GrowthTip, x: int, y: int, dx: int, dy: int, layout: SceneLayout) -> float:
        score = 0.0

        if (dx, dy) == (tip.dx, tip.dy):
            score += float(self.config["forward_bonus"])
        else:
            score -= float(self.config["turn_penalty"])

        if tip.is_trunk:
            score += self._trunk_guidance_score(tip, x, y, dx, dy, layout)
        else:
            score += self._branch_guidance_score(tip, x, y, dx, dy, layout)

        distance = self._distance_to_structure(x, y)
        if tip.is_trunk:
            score += min(distance, 3) * 0.25
        else:
            score += min(distance, 2) * 0.1

        neighbor_bonus = sum((x + ox, y + oy) in self.stems for ox, oy in NEIGHBORS_4) * 0.25
        score += neighbor_bonus
        score += self.rng.random() * float(self.config["organic_variation"])
        return score

    def _trunk_guidance_score(self, tip: GrowthTip, x: int, y: int, dx: int, dy: int, layout: SceneLayout) -> float:
        hero_zone = layout.no_go_zones[0]
        support_top, support_bottom, support_left, support_right = self._support_geometry(layout)

        score = 0.0
        if tip.y > support_bottom:
            if dy == -1:
                score += float(self.config["trunk_climb_bonus"])
            if dx == -1 and dy == -1:
                score += float(self.config["trunk_diagonal_bonus"])
            if abs(dx) == 1:
                score += 0.5
            if dy == 1:
                score -= float(self.config["trunk_reverse_penalty"])
            if y < tip.y:
                score += 3.0
        elif support_top <= tip.y <= support_bottom:
            if dx == -1:
                score += float(self.config["support_traverse_bonus"])
            if dy == -1:
                score += 0.75
            if dy == 1:
                score += 0.25
            if x < support_left or x > support_right:
                score -= 3.5
            if y < support_top or y > support_bottom + 1:
                score -= 2.0
            if x < tip.x:
                score += 2.5
        else:
            if dy == 1:
                score += float(self.config["settle_down_bonus"])
            if dx == -1:
                score += 2.0

        if x > hero_zone.right and dx == -1:
            score += 1.25
        if x < hero_zone.x and dx == 1:
            score -= 0.75

        score += self._contour_follow_score(x, y, dx, dy, layout)
        return score

    def _branch_guidance_score(self, tip: GrowthTip, x: int, y: int, dx: int, dy: int, layout: SceneLayout) -> float:
        hero_zone = layout.no_go_zones[0]
        score = 0.0
        if dy == 1:
            score += float(self.config["branch_gravity_bonus"])
        if dy == -1:
            score -= 0.75

        if y >= hero_zone.bottom and dx == 0:
            score += 0.5

        nearest_zone_x = hero_zone.x if x < hero_zone.x else hero_zone.right
        distance_from_hero = abs(x - nearest_zone_x)
        if distance_from_hero <= int(self.config["hero_contour_attraction"]):
            score += 0.75

        if (x, y) in layout.region_cells.get("below_hero", frozenset()):
            score += 0.5

        score += 0.75 * self._contour_follow_score(x, y, dx, dy, layout)
        return score

    def _branch_direction(self, dx: int, dy: int, x: int, y: int, layout: SceneLayout) -> Direction | None:
        if dy != 0:
            midpoint = layout.ivy_bounds.width // 2
            if x > midpoint:
                branch_dx, branch_dy = (1, 0)
            else:
                branch_dx, branch_dy = (-1, 0)
        else:
            branch_dx, branch_dy = (0, 1)

        if self._is_open(x + branch_dx, y + branch_dy, layout):
            return (branch_dx, branch_dy)
        for candidate in ((-1, 0), (1, 0), (0, 1), (0, -1)):
            if self._is_open(x + candidate[0], y + candidate[1], layout):
                return candidate
        return None

    def _support_geometry(self, layout: SceneLayout) -> tuple[int, int, int, int]:
        hero_zone = layout.no_go_zones[0]
        band_height = int(self.config["support_band_height"])
        above_gap = max(0, hero_zone.y - 1)
        below_gap = max(0, layout.ivy_bounds.height - hero_zone.bottom - 2)

        if above_gap >= band_height:
            support_top = max(1, hero_zone.y - int(self.config["support_band_above"]))
            support_bottom = max(support_top, support_top + band_height - 1)
        elif below_gap >= band_height:
            support_top = min(layout.ivy_bounds.height - 2, hero_zone.bottom + 1)
            support_bottom = min(layout.ivy_bounds.height - 2, support_top + band_height - 1)
        else:
            support_top = max(1, min(layout.ivy_bounds.height - 2, hero_zone.y + band_height))
            support_bottom = min(layout.ivy_bounds.height - 2, max(support_top, hero_zone.bottom - 2))

        support_left = max(1, hero_zone.x - int(self.config["support_span_left"]))
        support_right = min(layout.ivy_bounds.width - 2, hero_zone.right + int(self.config["support_span_right"]))
        return support_top, support_bottom, support_left, support_right

    def _contour_follow_score(self, x: int, y: int, dx: int, dy: int, layout: SceneLayout) -> float:
        score = 0.0
        attraction = float(self.config["hero_contour_attraction"])
        for zone in layout.no_go_zones:
            horizontal_gap = min(abs(x - zone.x), abs(x - zone.right))
            vertical_gap = min(abs(y - zone.y), abs(y - zone.bottom))

            near_vertical_face = zone.y <= y <= zone.bottom and horizontal_gap <= attraction
            near_horizontal_face = zone.x <= x <= zone.right and vertical_gap <= attraction

            if near_vertical_face and dy != 0:
                score += 2.0
            if near_horizontal_face and dx != 0:
                score += 2.0

            if zone.contains(x + dx, y + dy):
                score -= 6.0

        return score

    def _limit_tips(self, tips: list[GrowthTip], max_tips: int) -> list[GrowthTip]:
        if len(tips) <= max_tips:
            return tips

        trunk_tips = [tip for tip in tips if tip.is_trunk]
        branch_tips = [tip for tip in tips if not tip.is_trunk]
        kept: list[GrowthTip] = []
        if trunk_tips:
            kept.extend(trunk_tips[:1])
        remaining = max_tips - len(kept)
        if remaining > 0 and branch_tips:
            if len(branch_tips) <= remaining:
                kept.extend(branch_tips)
            else:
                kept.extend(self.rng.sample(branch_tips, remaining))
        return kept

    def _rebuild_leaf_stamps(self, layout: SceneLayout) -> None:
        self.leaf_stamps = {}
        if not self.active_leaf_positions:
            return

        chance = float(self.config["leaf_stamp_chance"])
        for x, y in self.active_leaf_positions:
            if self.rng.random() >= chance:
                continue
            direction = self.active_leaf_dirs.get((x, y))
            if direction is not None:
                self._stamp_oriented_leaf(x, y, direction[0], direction[1], layout)
            else:
                pattern = self.rng.choice(self.leaf_patterns)
                self._stamp_leaf(x, y, pattern, layout)

    def _rebuild_thickened_wood(self, layout: SceneLayout) -> None:
        self.thickened_wood = {}
        min_age = int(self.config["thickening_min_age"])
        full_age = int(self.config["thickening_full_age"])
        spread_chance = float(self.config["thickening_spread_chance"])

        for x, y in self.stems:
            age = self.frame - self.stem_birth.get((x, y), self.frame)
            if age < min_age:
                continue

            left = (x - 1, y) in self.stems
            right = (x + 1, y) in self.stems
            up = (x, y - 1) in self.stems
            down = (x, y + 1) in self.stems

            side_cells: list[tuple[int, int, str]] = []
            if up or down:
                side_cells.extend([(x - 1, y, "|"), (x + 1, y, "|")])
            if left or right:
                side_cells.extend([(x, y - 1, "-"), (x, y + 1, "-")])

            for sx, sy, wood_char in side_cells:
                if not self._is_ornament_open(sx, sy, layout):
                    continue
                if age >= full_age or self.rng.random() < spread_chance:
                    if self._stable_index(sx, sy, 41, 100) < 65:
                        leaf = self._stable_choice(["·", "'", "+", "•", "~"], sx, sy, 43)
                        self.thickened_wood[(sx, sy)] = f"{OLIVE}{leaf}{RESET}"
                    else:
                        color = self._stable_choice([BROWN, DARK_BROWN], sx, sy, 47)
                        self.thickened_wood[(sx, sy)] = f"{color}{wood_char}{RESET}"

    def _stamp_leaf(self, center_x: int, center_y: int, pattern: list[tuple[int, int, str]], layout: SceneLayout) -> None:
        for dx, dy, char in pattern:
            sx = center_x + dx
            sy = center_y + dy
            if not self._is_ornament_open(sx, sy, layout):
                continue
            if (sx, sy) in self.leaf_stamps:
                continue
            color = self._stable_choice([GREEN, LIGHT_GREEN, OLIVE], sx, sy, 23)
            self.leaf_stamps[(sx, sy)] = f"{color}{char}{RESET}"

    def _stamp_oriented_leaf(self, center_x: int, center_y: int, dx: int, dy: int, layout: SceneLayout) -> None:
        if dx != 0:
            side = self.rng.choice([-1, 1])
            pattern = [
                (0, side, "·"),
                (1, side, "~"),
                (2, side, "~"),
                (3, side, "*"),
                (1, side * 2, "."),
                (2, side * 2, "+"),
            ]
        else:
            side = self.rng.choice([-1, 1])
            pattern = [
                (side, 0, "·"),
                (side, 1, "~"),
                (side, 2, "~"),
                (side, 3, "*"),
                (side * 2, 1, "."),
                (side * 2, 2, "+"),
            ]
        self._stamp_leaf(center_x, center_y, pattern, layout)

    def _stamp_death_cluster(self, center_x: int, center_y: int, dx: int, dy: int, layout: SceneLayout) -> None:
        if dx != 0:
            side = self.rng.choice([-1, 1])
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
            side = self.rng.choice([-1, 1])
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
            if not self._is_ornament_open(sx, sy, layout):
                continue
            if (sx, sy) in self.dead_leaf_stamps:
                continue
            color = self._stable_choice([GREEN, OLIVE, LIGHT_GREEN], sx, sy, 37)
            self.dead_leaf_stamps[(sx, sy)] = f"{color}{char}{RESET}"

    def _trim_ornaments(self) -> None:
        max_ornaments = int(self.config["max_ornament_cells"])
        combined = list(self.leaf_stamps.items()) + list(self.dead_leaf_stamps.items()) + list(self.thickened_wood.items())
        if len(combined) <= max_ornaments:
            return

        overflow = len(combined) - max_ornaments
        for store in (self.leaf_stamps, self.dead_leaf_stamps, self.thickened_wood):
            if overflow <= 0:
                break
            keys = list(store.keys())
            self.rng.shuffle(keys)
            for key in keys[: min(len(keys), overflow)]:
                store.pop(key, None)
                overflow -= 1
                if overflow <= 0:
                    break

    def _is_open(self, x: int, y: int, layout: SceneLayout) -> bool:
        return (x, y) in layout.allowed_cells

    def _is_ornament_open(self, x: int, y: int, layout: SceneLayout) -> bool:
        if (x, y) not in layout.allowed_cells:
            return False
        occupied = self._occupied_points(include_active=True)
        return (x, y) not in occupied

    def _occupied_points(self, include_active: bool) -> set[Point]:
        occupied = set(self.stems)
        occupied.update(self.leaf_stamps)
        occupied.update(self.dead_leaf_stamps)
        occupied.update(self.thickened_wood)
        if include_active:
            occupied.update(self.active_leaf_positions)
        return occupied

    def _wood_char_for_cell(self, x: int, y: int) -> str:
        if (x - 1, y) in self.stems and (x + 1, y) in self.stems:
            return "-"
        if (x, y - 1) in self.stems and (x, y + 1) in self.stems:
            return "|"
        if (x - 1, y) in self.stems and (x, y - 1) in self.stems:
            return "/"
        if (x - 1, y) in self.stems and (x, y + 1) in self.stems:
            return "\\"
        return "+"

    def _wood_color_for(self, point: Point) -> str:
        return self._stable_choice([BROWN, DARK_BROWN], point[0], point[1], 53)

    def _distance_to_structure(self, x: int, y: int) -> int:
        nearest = 4
        for ox, oy in ((1, 0), (-1, 0), (0, 1), (0, -1), (2, 0), (-2, 0), (0, 2), (0, -2)):
            if (x + ox, y + oy) in self.stems:
                nearest = min(nearest, abs(ox) + abs(oy))
        return nearest

    def _record_failed_move(self, x: int, y: int, layout: SceneLayout) -> None:
        if self.debug_config.get("enabled") is not True:
            return

        if x <= 0 or y <= 0 or x >= layout.ivy_bounds.width - 1 or y >= layout.ivy_bounds.height - 1:
            reason = "bounds"
        elif (x, y) in self.stems:
            reason = "stem"
        elif any(zone.contains(x, y) for zone in layout.no_go_zones):
            reason = "no_go"
        else:
            reason = "blocked"
        self._count_debug("failed_move_counts", reason)

    def _update_debug_stats(self, layout: SceneLayout) -> None:
        region_coverage: dict[str, dict[str, int]] = {}
        for name, cells in layout.region_cells.items():
            stem_count = sum(1 for point in self.stems if point in cells)
            region_coverage[name] = {
                "allowed": len(cells),
                "stems": stem_count,
            }
        self.debug_stats["region_coverage"] = region_coverage
        self.debug_stats["stem_count"] = len(self.stems)
        self.debug_stats["ornament_count"] = (
            len(self.active_leaf_positions) + len(self.leaf_stamps) + len(self.dead_leaf_stamps) + len(self.thickened_wood)
        )

    def _count_debug(self, bucket: str, key: str) -> None:
        counts = self.debug_stats.setdefault(bucket, {})
        if isinstance(counts, dict):
            counts[key] = int(counts.get(key, 0)) + 1

    def _stable_index(self, x: int, y: int, salt: int, count: int) -> int:
        return ((x * 92821) + (y * 68917) + salt) % count

    def _stable_choice(self, values: list[str], x: int, y: int, salt: int) -> str:
        return values[self._stable_index(x, y, salt, len(values))]

    def _load_leaf_patterns(self) -> list[list[tuple[int, int, str]]]:
        return [
            [(-2, 0, "-"), (-1, 0, "~"), (0, 0, "~"), (1, 0, "|"), (2, 0, "\\"), (3, 0, "*")],
            [(-2, 0, "-"), (-1, 0, "/"), (0, 0, "~"), (1, 0, "~"), (2, 0, "|"), (3, 0, ">"), (4, 0, ">")],
            [(-1, -1, "."), (0, -1, "/"), (1, -1, "~"), (-1, 0, "-"), (0, 0, "|"), (1, 0, "~"), (2, 0, "*")],
            [(-1, 0, "<"), (0, 0, "<"), (1, 0, "~"), (2, 0, "|"), (3, 0, "/"), (4, 0, ".")],
            [(-2, 0, "-"), (-1, 0, "~"), (0, 0, "~"), (1, 0, "|"), (2, 0, "/"), (3, 0, "*"), (0, 1, "'"), (1, 1, ".")],
            [(0, -1, "."), (1, -1, "+"), (0, 0, "|"), (1, 0, "~"), (2, 0, "~"), (3, 0, "~"), (4, 0, "|"), (5, 0, ">"), (6, 0, ">")],
            [(-2, 0, "-"), (-1, 0, "-"), (0, 0, "|"), (1, 0, "~"), (2, 0, "~"), (3, 0, "/"), (4, 0, "*"), (5, 0, ".")],
            [(-1, -1, "."), (0, -1, "+"), (1, -1, "."), (-1, 0, "-"), (0, 0, "~"), (1, 0, "|"), (2, 0, "\\"), (3, 0, "*")],
        ]
