from __future__ import annotations

import random
from dataclasses import dataclass

from layout import Rect, SceneLayout
from terminal import RESET


Direction = tuple[int, int]


@dataclass
class VineHead:
    x: int
    y: int
    dx: int
    dy: int
    energy: float


class IvyEngine:
    def __init__(self, config: dict, seed: int = 7) -> None:
        self.config = config["ivy"]
        self.rng = random.Random(seed)
        self.size = None
        self.layout = None
        self.segments: dict[tuple[int, int], str] = {}
        self.heads: list[VineHead] = []

    def reset(self, size, layout: SceneLayout) -> None:
        self.size = size
        self.layout = layout
        self.segments = {}
        self.heads = self._spawn_heads(layout)

    def _spawn_heads(self, layout: SceneLayout) -> list[VineHead]:
        heads: list[VineHead] = []
        seeds = max(1, self.config["seed_count"])
        width = layout.ivy_bounds.width
        height = layout.ivy_bounds.height
        candidates = [
            (0, self.rng.randint(1, max(1, height - 2)), 1, 0),
            (width - 1, self.rng.randint(1, max(1, height - 2)), -1, 0),
            (self.rng.randint(1, max(1, width - 2)), 0, 0, 1),
            (self.rng.randint(1, max(1, width - 2)), height - 1, 0, -1),
            (0, 0, 1, 1),
            (width - 1, 0, -1, 1),
            (0, height - 1, 1, -1),
            (width - 1, height - 1, -1, -1),
        ]
        self.rng.shuffle(candidates)
        for x, y, dx, dy in candidates[:seeds]:
            sx = min(max(1, x), width - 2)
            sy = min(max(1, y), height - 2)
            heads.append(VineHead(x=sx, y=sy, dx=dx, dy=dy, energy=1.0))
        return heads

    def _spawn_head_from_existing(self, layout: SceneLayout) -> VineHead | None:
        hero_corridor_count = self._count_segments_near_hero_corridor(layout)
        hero_corridor_target = self.config["hero_corridor_min_segments"]
        hero_band_head = self._spawn_head_near_hero_corridor(layout)
        if hero_band_head is not None and hero_corridor_count < hero_corridor_target:
            return hero_band_head
        if hero_band_head is not None and self.rng.random() < self.config["hero_corridor_respawn_bias"]:
            return hero_band_head

        open_head = self._spawn_head_in_open_space(layout)
        if open_head is not None and self.rng.random() < self.config["open_space_respawn_bias"]:
            return open_head

        edge_head = self._spawn_head_from_edge(layout)
        if edge_head is not None and self.rng.random() < self.config["edge_respawn_bias"]:
            return edge_head

        if not self.segments:
            spawned = self._spawn_heads(layout)
            return spawned[0] if spawned else None

        viable_positions = []
        preferred_half = self._preferred_half(layout)
        for x, y in self.segments.keys():
            if preferred_half is not None:
                midpoint = layout.ivy_bounds.width // 2
                if preferred_half == "left" and x > midpoint:
                    continue
                if preferred_half == "right" and x < midpoint:
                    continue
            if any(
                self._is_open(x + dx, y + dy, layout)
                for dx, dy in [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (-1, -1),
                    (1, -1),
                    (-1, 1),
                ]
            ):
                viable_positions.append((x, y))

        if not viable_positions:
            return edge_head

        x, y = self.rng.choice(viable_positions)
        dx, dy = self.rng.choice(
            [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)]
        )
        return VineHead(x=x, y=y, dx=dx, dy=dy, energy=self.config["respawn_energy"])

    def _spawn_head_in_open_space(self, layout: SceneLayout) -> VineHead | None:
        width = layout.ivy_bounds.width
        height = layout.ivy_bounds.height
        center_x = width // 2
        band_half_width = max(8, width // 8)

        candidates: list[tuple[int, int]] = []
        for y in range(2, max(3, height - 2), max(1, height // 12)):
            for x in range(
                max(1, center_x - band_half_width),
                min(width - 1, center_x + band_half_width),
                max(1, band_half_width // 4),
            ):
                if self._is_open(x, y, layout):
                    candidates.append((x, y))

        if not candidates:
            return None

        x, y = self.rng.choice(candidates)
        dx, dy = self.rng.choice(
            [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)]
        )
        return VineHead(x=x, y=y, dx=dx, dy=dy, energy=self.config["respawn_energy"])

    def _spawn_head_near_hero_corridor(self, layout: SceneLayout) -> VineHead | None:
        hero_zone = layout.no_go_zones[0]
        width = layout.ivy_bounds.width
        height = layout.ivy_bounds.height
        x_start = max(1, hero_zone.x - self.config["hero_corridor_pad_x"])
        x_end = min(width - 1, hero_zone.right + self.config["hero_corridor_pad_x"])
        pad_y = self.config["hero_corridor_pad_y"]

        candidates: list[tuple[int, int]] = []

        top_y_start = max(2, hero_zone.y - pad_y)
        top_y_end = max(top_y_start + 1, hero_zone.y)
        for y in range(top_y_start, top_y_end, max(1, max(1, top_y_end - top_y_start) // 6)):
            for x in range(x_start, x_end, max(1, (x_end - x_start) // 10 or 1)):
                if self._is_open(x, y, layout):
                    candidates.append((x, y))

        bottom_y_start = min(height - 2, hero_zone.bottom + 1)
        bottom_y_end = min(height - 1, hero_zone.bottom + pad_y)
        for y in range(bottom_y_start, bottom_y_end, max(1, max(1, bottom_y_end - bottom_y_start) // 6)):
            for x in range(x_start, x_end, max(1, (x_end - x_start) // 10 or 1)):
                if self._is_open(x, y, layout):
                    candidates.append((x, y))

        if not candidates:
            return None

        x, y = self.rng.choice(candidates)
        if y < hero_zone.y:
            dy = self.rng.choice([-1, 0])
        elif y > hero_zone.bottom:
            dy = self.rng.choice([0, 1])
        else:
            dy = self.rng.choice([-1, 1])
        dx = self.rng.choice([-1, -1, 0, 1, 1])
        return VineHead(x=x, y=y, dx=dx, dy=dy, energy=self.config["respawn_energy"])

    def _count_segments_near_hero_corridor(self, layout: SceneLayout) -> int:
        hero_zone = layout.no_go_zones[0]
        pad_x = self.config["hero_corridor_pad_x"]
        pad_y = self.config["hero_corridor_pad_y"]
        top_band_top = max(1, hero_zone.y - pad_y)
        top_band_bottom = max(top_band_top, hero_zone.y - 1)
        bottom_band_top = hero_zone.bottom + 1
        bottom_band_bottom = min(layout.ivy_bounds.height - 2, hero_zone.bottom + pad_y)
        count = 0
        for x, y in self.segments.keys():
            if hero_zone.x - pad_x <= x <= hero_zone.right + pad_x:
                if top_band_top <= y <= top_band_bottom:
                    count += 1
                elif bottom_band_top <= y <= bottom_band_bottom:
                    count += 1
        return count

    def _spawn_head_from_edge(self, layout: SceneLayout) -> VineHead | None:
        width = layout.ivy_bounds.width
        height = layout.ivy_bounds.height
        preferred_half = self._preferred_half(layout)

        candidates: list[tuple[int, int, int, int]] = []
        y_samples = range(1, max(2, height - 1), max(1, height // 8))
        x_samples = range(1, max(2, width - 1), max(1, width // 8))

        if preferred_half in (None, "left"):
            for y in y_samples:
                candidates.extend(
                    [
                        (1, y, 1, 0),
                        (1, y, 1, self.rng.choice([-1, 0, 1])),
                    ]
                )
        if preferred_half in (None, "right"):
            for y in y_samples:
                candidates.extend(
                    [
                        (width - 2, y, -1, 0),
                        (width - 2, y, -1, self.rng.choice([-1, 0, 1])),
                    ]
                )

        for x in x_samples:
            candidates.extend(
                [
                    (x, 1, self.rng.choice([-1, 0, 1]), 1),
                    (x, height - 2, self.rng.choice([-1, 0, 1]), -1),
                ]
            )

        self.rng.shuffle(candidates)
        for x, y, dx, dy in candidates:
            if self._is_open(x, y, layout):
                return VineHead(x=x, y=y, dx=dx, dy=dy, energy=self.config["respawn_energy"])
        return None

    def _preferred_half(self, layout: SceneLayout) -> str | None:
        midpoint = layout.ivy_bounds.width // 2
        left_count = sum(1 for x, _ in self.segments.keys() if x < midpoint)
        right_count = sum(1 for x, _ in self.segments.keys() if x >= midpoint)
        imbalance = abs(left_count - right_count)
        threshold = self.config["rebalance_threshold"]
        if imbalance < threshold:
            return None
        return "left" if left_count < right_count else "right"

    def tick(self, layout: SceneLayout) -> None:
        if self.layout != layout or self.size is None:
            return
        if len(self.segments) >= self.config["max_segments"]:
            return

        if len(self.heads) < self.config["min_active_heads"]:
            while (
                len(self.heads) < self.config["min_active_heads"]
                and len(self.segments) < self.config["max_segments"]
            ):
                new_head = self._spawn_head_from_existing(layout)
                if new_head is None:
                    break
                self.heads.append(new_head)

        next_heads: list[VineHead] = []
        for head in self.heads:
            if head.energy < self.config["min_energy"]:
                continue
            step = self._advance(head, layout)
            if step is None:
                continue
            new_x, new_y, new_dx, new_dy = step
            glyph = self._stem_glyph(new_dx, new_dy)
            color = self.rng.choice(self.config["stem_colors"])
            self.segments[(new_x, new_y)] = f"{color}{glyph}{RESET}"

            if self.rng.random() < self.config["leaf_chance"]:
                leaf = self.rng.choice(self.config["leaf_glyphs"])
                leaf_color = self.rng.choice(self.config["leaf_colors"])
                leaf_pos = self._leaf_position(new_x, new_y, new_dx, new_dy)
                if leaf_pos and self._is_open(*leaf_pos, layout):
                    self.segments[leaf_pos] = f"{leaf_color}{leaf}{RESET}"

            if self.rng.random() < self.config["dot_chance"]:
                dot = self.rng.choice(self.config["dot_glyphs"])
                dot_pos = self._leaf_position(new_x, new_y, -new_dy or 1, new_dx or 1)
                if dot_pos and self._is_open(*dot_pos, layout):
                    self.segments[dot_pos] = f"\x1b[38;5;58m{dot}{RESET}"

            next_heads.append(
                VineHead(
                    x=new_x,
                    y=new_y,
                    dx=new_dx,
                    dy=new_dy,
                    energy=head.energy * self.config["branch_decay"],
                )
            )

            if self.rng.random() < self.config["split_chance"]:
                split_dx, split_dy = self._rotate(new_dx, new_dy)
                branch = self._advance(
                    VineHead(new_x, new_y, split_dx, split_dy, head.energy * 0.8),
                    layout,
                )
                if branch is not None:
                    bx, by, bdx, bdy = branch
                    self.segments[(bx, by)] = (
                        f"{self.rng.choice(self.config['stem_colors'])}"
                        f"{self._stem_glyph(bdx, bdy)}{RESET}"
                    )
                    next_heads.append(
                        VineHead(
                            x=bx,
                            y=by,
                            dx=bdx,
                            dy=bdy,
                            energy=head.energy * 0.66,
                        )
                    )

        self.heads = next_heads[: self.config["max_active_heads"]]

    def get_segments(self) -> dict[tuple[int, int], str]:
        return dict(self.segments)

    def _advance(
        self,
        head: VineHead,
        layout: SceneLayout,
    ) -> tuple[int, int, int, int] | None:
        candidates: list[Direction] = [(head.dx, head.dy)]
        if self.rng.random() < self.config["turn_chance"]:
            candidates.append(self._rotate(head.dx, head.dy))
            candidates.append(self._rotate(head.dx, head.dy, clockwise=False))
        edge_pull = self._edge_pull(head.x, head.y, layout.ivy_bounds)
        candidates.append(edge_pull)
        candidates.extend(
            self.rng.sample(
                [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)],
                k=4,
            )
        )

        for dx, dy in candidates:
            if dx == 0 and dy == 0:
                continue
            nx = head.x + dx
            ny = head.y + dy
            if self._is_open(nx, ny, layout):
                return nx, ny, dx, dy
        return None

    def _edge_pull(self, x: int, y: int, bounds: Rect) -> Direction:
        left = x
        right = bounds.width - 1 - x
        top = y
        bottom = bounds.height - 1 - y
        nearest = min(left, right, top, bottom)
        if nearest == left:
            return (self.rng.choice([0, 1]), self.rng.choice([-1, 0, 1]))
        if nearest == right:
            return (self.rng.choice([-1, 0]), self.rng.choice([-1, 0, 1]))
        if nearest == top:
            return (self.rng.choice([-1, 0, 1]), self.rng.choice([0, 1]))
        return (self.rng.choice([-1, 0, 1]), self.rng.choice([-1, 0]))

    def _rotate(self, dx: int, dy: int, clockwise: bool = True) -> Direction:
        if clockwise:
            return (-dy or dx, dx or dy)
        return (dy or dx, -dx or dy)

    def _leaf_position(self, x: int, y: int, dx: int, dy: int) -> tuple[int, int] | None:
        offsets = [(-dy, dx), (dy, -dx), (-dx, -dy)]
        self.rng.shuffle(offsets)
        for ox, oy in offsets:
            if ox == 0 and oy == 0:
                continue
            return (x + ox, y + oy)
        return None

    def _is_open(self, x: int, y: int, layout: SceneLayout) -> bool:
        if x <= 0 or y <= 0 or x >= layout.ivy_bounds.width - 1 or y >= layout.ivy_bounds.height - 1:
            return False
        if (x, y) in self.segments:
            return False
        return not any(zone.contains(x, y) for zone in layout.no_go_zones)

    def _stem_glyph(self, dx: int, dy: int) -> str:
        if dy == 0:
            return "─"
        if dx == 0:
            return "│"
        return "╱" if dx == dy else "╲"
