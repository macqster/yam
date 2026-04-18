from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from collections import deque
from pathlib import Path

from PIL import Image

INFO_PANEL_MIN_HEIGHT = 9

from terminal import TerminalSize


@dataclass(frozen=True)
class Rect:
    x: int
    y: int
    width: int
    height: int

    @property
    def right(self) -> int:
        return self.x + self.width

    @property
    def bottom(self) -> int:
        return self.y + self.height

    def contains(self, x: int, y: int) -> bool:
        return self.x <= x < self.right and self.y <= y < self.bottom

    def inflate(self, padx: int, pady: int) -> "Rect":
        return Rect(
            x=max(0, self.x - padx),
            y=max(0, self.y - pady),
            width=self.width + 2 * padx,
            height=self.height + 2 * pady,
        )

    def inset(self, left: int, top: int, right: int, bottom: int) -> "Rect":
        new_x = self.x + max(0, left)
        new_y = self.y + max(0, top)
        new_right = max(new_x + 1, self.right - max(0, right))
        new_bottom = max(new_y + 1, self.bottom - max(0, bottom))
        return Rect(
            x=new_x,
            y=new_y,
            width=max(1, new_right - new_x),
            height=max(1, new_bottom - new_y),
        )


@dataclass(frozen=True)
class SceneLayout:
    hero: Rect
    info: Rect
    hero_guide: Rect
    info_guide: Rect
    vines_bounds: Rect
    no_go_zones: tuple[Rect, ...]
    hero_raw_mask_cells: frozenset[tuple[int, int]]
    hero_mask_boundary_cells: frozenset[tuple[int, int]]
    hero_mask_cells: frozenset[tuple[int, int]]
    trunk_mask_cells: frozenset[tuple[int, int]]
    trunk_field: dict[tuple[int, int], int]
    allowed_cells: frozenset[tuple[int, int]]
    ornament_cells: frozenset[tuple[int, int]]
    region_cells: dict[str, frozenset[tuple[int, int]]]
    warning: str | None


def build_layout(
    size: TerminalSize,
    config: dict,
    hero_width: int,
    hero_height: int,
) -> SceneLayout:
    layout_cfg = config["layout"]
    margin_x = layout_cfg["outer_margin_x"]
    margin_y = layout_cfg["outer_margin_y"]
    warning = None

    if (
        size.columns < layout_cfg["min_terminal_columns"]
        or size.rows < layout_cfg["min_terminal_rows"]
    ):
        warning = (
            f"Resize terminal to at least "
            f"{layout_cfg['min_terminal_columns']}x{layout_cfg['min_terminal_rows']}"
        )

    hero_anchor = layout_cfg.get("hero_anchor", "left")
    if hero_anchor == "center":
        hero_base_x = max(margin_x, (size.columns - hero_width) // 2)
    elif hero_anchor == "right":
        hero_base_x = max(margin_x, size.columns - hero_width - margin_x)
    else:
        hero_base_x = margin_x

    hero_x = hero_base_x + layout_cfg["hero_offset_x"]
    hero_y = margin_y + layout_cfg["hero_offset_y"]
    hero = Rect(x=hero_x, y=hero_y, width=hero_width, height=hero_height)

    info_gap = layout_cfg.get("info_gap", 6)
    info_width = layout_cfg["info_width"]
    info_height = max(INFO_PANEL_MIN_HEIGHT, int(layout_cfg["info_height"]))
    info_base_x = size.columns - info_width - margin_x
    info_x = info_base_x + layout_cfg["info_offset_x"]
    info_y = margin_y + layout_cfg["info_offset_y"]
    min_info_x = hero.right + info_gap
    max_info_x = max(margin_x, size.columns - info_width - margin_x)
    max_info_y = max(margin_y, size.rows - info_height - margin_y)
    info = Rect(
        x=max(min_info_x, min(info_x, max_info_x)),
        y=max(margin_y, min(info_y, max_info_y)),
        width=info_width,
        height=info_height,
    )

    vines_bounds = Rect(
        x=0,
        y=0,
        width=max(1, size.columns),
        height=max(1, size.rows),
    )
    hero_pad_x = layout_cfg.get("hero_safe_pad_x", 2)
    hero_pad_y = layout_cfg.get("hero_safe_pad_y", 1)
    info_pad_x = layout_cfg.get("info_safe_pad_x", 2)
    info_pad_y = layout_cfg.get("info_safe_pad_y", 1)
    info_collision = info.inset(
        layout_cfg.get("info_collision_trim_left", 0),
        layout_cfg.get("info_collision_trim_top", 0),
        layout_cfg.get("info_collision_trim_right", 0),
        layout_cfg.get("info_collision_trim_bottom", 0),
    )
    hero_raw_mask_cells, hero_blocked_cells = _build_hero_blocked_cells(hero, hero_pad_x, hero_pad_y, layout_cfg)
    trunk_mask_cells = _load_trunk_mask_cells(size, layout_cfg)
    trunk_field = _build_distance_field(trunk_mask_cells, vines_bounds)
    if hero_blocked_cells:
        hero_collision = _bounding_rect(hero_blocked_cells)
    else:
        hero_collision = hero.inset(
            layout_cfg.get("hero_collision_trim_left", 0),
            layout_cfg.get("hero_collision_trim_top", 0),
            layout_cfg.get("hero_collision_trim_right", 0)
            + layout_cfg.get("hero_collision_bleed_right", 3),
            layout_cfg.get("hero_collision_trim_bottom", 0),
        ).inflate(hero_pad_x, hero_pad_y)

    no_go_zones = (
        hero_collision,
        info_collision.inflate(info_pad_x, info_pad_y),
    )
    allowed_cells: set[tuple[int, int]] = set()
    ornament_cells: set[tuple[int, int]] = set()
    info_zone = no_go_zones[1]
    hero_raw_zone = hero_raw_mask_cells if hero_raw_mask_cells else set()
    for y in range(1, max(1, vines_bounds.height - 1)):
        for x in range(1, max(1, vines_bounds.width - 1)):
            if info_zone.contains(x, y):
                continue

            blocked_for_growth = False
            if hero_blocked_cells:
                if (x, y) in hero_blocked_cells:
                    blocked_for_growth = True
            elif _hero_zone_blocks_cell(x, y, no_go_zones[0], layout_cfg):
                blocked_for_growth = True

            blocked_for_ornament = False
            if hero_raw_zone:
                if (x, y) in hero_raw_zone:
                    blocked_for_ornament = True
            elif hero.contains(x, y):
                blocked_for_ornament = True

            if not blocked_for_growth:
                allowed_cells.add((x, y))
            if not blocked_for_ornament:
                ornament_cells.add((x, y))

    hero_guide = hero
    if hero_raw_mask_cells:
        hero_guide = _bounding_rect(set(hero_raw_mask_cells))

    region_cells = _build_region_cells(allowed_cells, vines_bounds, hero_guide)

    return SceneLayout(
        hero=hero,
        info=info,
        hero_guide=hero_guide,
        info_guide=info,
        vines_bounds=vines_bounds,
        no_go_zones=no_go_zones,
        hero_raw_mask_cells=frozenset(hero_raw_mask_cells),
        hero_mask_boundary_cells=frozenset(_boundary_cells(hero_raw_mask_cells)),
        hero_mask_cells=frozenset(hero_blocked_cells),
        trunk_mask_cells=frozenset(trunk_mask_cells),
        trunk_field=trunk_field,
        allowed_cells=frozenset(allowed_cells),
        ornament_cells=frozenset(ornament_cells),
        region_cells=region_cells,
        warning=warning,
    )

def _load_trunk_mask_cells(
    size: TerminalSize,
    layout_cfg: dict,
) -> set[tuple[int, int]]:
    trunk_path_value = layout_cfg.get("trunk_mask_path", "")
    if not trunk_path_value:
        return set()

    trunk_path = _resolve_mask_path(trunk_path_value)
    if not trunk_path.exists():
        return set()

    threshold = int(layout_cfg.get("trunk_mask_threshold", 200))
    scale_x = float(layout_cfg.get("trunk_mask_scale_x", 1.0))
    scale_y = float(layout_cfg.get("trunk_mask_scale_y", 1.0))
    offset_x = int(layout_cfg.get("trunk_mask_offset_x", 0))
    offset_y = int(layout_cfg.get("trunk_mask_offset_y", 0))
    image = Image.open(trunk_path).convert("RGBA")

    scaled_width = max(1, int(round(image.width * scale_x)))
    scaled_height = max(1, int(round(image.height * scale_y)))
    if scaled_width != image.width or scaled_height != image.height:
        image = image.resize((scaled_width, scaled_height), Image.Resampling.BOX)

    cells: set[tuple[int, int]] = set()
    for y in range(image.height):
        for x in range(image.width):
            r, g, b, a = image.getpixel((x, y))
            brightness = (r + g + b) / 3
            if a > 0 and brightness >= threshold:
                scene_x = x + offset_x
                scene_y = y + offset_y
                if 0 <= scene_x < size.columns and 0 <= scene_y < size.rows:
                    cells.add((scene_x, scene_y))

    return cells


def _build_distance_field(
    points: set[tuple[int, int]],
    bounds: Rect,
) -> dict[tuple[int, int], int]:
    if not points:
        return {}

    distances: dict[tuple[int, int], int] = {}
    frontier = deque()
    for point in points:
        distances[point] = 0
        frontier.append(point)

    while frontier:
        x, y = frontier.popleft()
        next_distance = distances[(x, y)] + 1
        for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
            if not bounds.contains(nx, ny):
                continue
            if (nx, ny) in distances and distances[(nx, ny)] <= next_distance:
                continue
            distances[(nx, ny)] = next_distance
            frontier.append((nx, ny))

    return distances


@lru_cache(maxsize=8)
def _load_resized_hero_mask(
    mask_path: str,
    width: int,
    height: int,
    threshold: int,
    scale_x: float,
    scale_y: float,
) -> frozenset[tuple[int, int]]:
    image = Image.open(mask_path).convert("RGBA")
    binary = Image.new("L", image.size, 0)
    for y in range(image.height):
        for x in range(image.width):
            r, g, b, a = image.getpixel((x, y))
            brightness = (r + g + b) / 3
            if a > 0 and brightness >= threshold:
                binary.putpixel((x, y), 255)

    canvas = Image.new("L", image.size, 0)
    scaled_width = max(1, int(round(image.width * scale_x)))
    scaled_height = max(1, int(round(image.height * scale_y)))
    scaled = binary.resize((scaled_width, scaled_height), Image.Resampling.BOX)
    offset_x = (image.width - scaled_width) // 2
    offset_y = (image.height - scaled_height) // 2
    canvas.paste(scaled, (offset_x, offset_y))

    image = canvas.resize((width, height), Image.Resampling.BOX)
    blocked: set[tuple[int, int]] = set()
    for y in range(height):
        for x in range(width):
            if image.getpixel((x, y)) >= 128:
                blocked.add((x, y))
    return frozenset(blocked)


def _build_hero_blocked_cells(
    hero: Rect,
    pad_x: int,
    pad_y: int,
    layout_cfg: dict,
) -> tuple[set[tuple[int, int]], set[tuple[int, int]]]:
    threshold = int(layout_cfg.get("hero_mask_threshold", 200))
    scale_x = float(layout_cfg.get("hero_mask_scale_x", 0.7))
    scale_y = float(layout_cfg.get("hero_mask_scale_y", 0.9))
    alignment_margin = max(0, int(layout_cfg.get("hero_mask_alignment_margin", 0)))
    hero_mask_offset_x = int(layout_cfg.get("hero_mask_offset_x", 0))
    hero_mask_offset_y = int(layout_cfg.get("hero_mask_offset_y", 0))
    local_cells = _load_hero_mask_from_config(
        layout_cfg,
        hero.width,
        hero.height,
        threshold,
        scale_x,
        scale_y,
    )
    if not local_cells:
        return set(), set()
    if alignment_margin > 0:
        local_cells = frozenset(
            (x, y)
            for x, y in local_cells
            if alignment_margin <= x < hero.width - alignment_margin
            and alignment_margin <= y < hero.height - alignment_margin
        )
        if not local_cells:
            return set(), set()
    raw_cells: set[tuple[int, int]] = set()
    blocked: set[tuple[int, int]] = set()
    for local_x, local_y in local_cells:
        world_x = hero.x + local_x + hero_mask_offset_x
        world_y = hero.y + local_y + hero_mask_offset_y
        raw_cells.add((world_x, world_y))
        for dy in range(-pad_y, pad_y + 1):
            for dx in range(-pad_x, pad_x + 1):
                blocked.add((world_x + dx, world_y + dy))

    raw_cells = _apply_mask_corner_trims(raw_cells, hero, layout_cfg)
    blocked = _apply_mask_corner_trims(blocked, hero, layout_cfg)
    return raw_cells, blocked


def _apply_mask_corner_trims(
    blocked: set[tuple[int, int]],
    hero: Rect,
    layout_cfg: dict,
) -> set[tuple[int, int]]:
    trimmed = set(blocked)
    corners = (
        ("top_left", int(layout_cfg.get("hero_corner_trim_top_left", 0)), int(layout_cfg.get("hero_corner_trim_top_left_y", layout_cfg.get("hero_corner_trim_top_left", 0)))),
        ("top_right", int(layout_cfg.get("hero_corner_trim_top_right", 0)), int(layout_cfg.get("hero_corner_trim_top_right_y", layout_cfg.get("hero_corner_trim_top_right", 0)))),
        ("bottom_left", int(layout_cfg.get("hero_corner_trim_bottom_left", 0)), int(layout_cfg.get("hero_corner_trim_bottom_left_y", layout_cfg.get("hero_corner_trim_bottom_left", 0)))),
        ("bottom_right", int(layout_cfg.get("hero_corner_trim_bottom_right", 0)), int(layout_cfg.get("hero_corner_trim_bottom_right_y", layout_cfg.get("hero_corner_trim_bottom_right", 0)))),
    )
    for x, y in tuple(trimmed):
        local_x = x - hero.x
        local_y = y - hero.y
        for corner, size_x, size_y in corners:
            if _corner_trim_allows_local_cell(local_x, local_y, hero.width, hero.height, corner=corner, size_x=size_x, size_y=size_y):
                trimmed.discard((x, y))
                break
    return trimmed


def _corner_trim_allows_local_cell(
    local_x: int,
    local_y: int,
    width: int,
    height: int,
    *,
    corner: str,
    size_x: int,
    size_y: int,
) -> bool:
    if size_x <= 0 or size_y <= 0:
        return False

    if corner == "top_left":
        if not (0 <= local_x < size_x and 0 <= local_y < size_y):
            return False
        dx = size_x - (local_x + 0.5)
        dy = size_y - (local_y + 0.5)
    elif corner == "top_right":
        local_x = width - 1 - local_x
        if not (0 <= local_x < size_x and 0 <= local_y < size_y):
            return False
        dx = size_x - (local_x + 0.5)
        dy = size_y - (local_y + 0.5)
    elif corner == "bottom_left":
        local_y = height - 1 - local_y
        if not (0 <= local_x < size_x and 0 <= local_y < size_y):
            return False
        dx = size_x - (local_x + 0.5)
        dy = size_y - (local_y + 0.5)
    elif corner == "bottom_right":
        local_x = width - 1 - local_x
        local_y = height - 1 - local_y
        if not (0 <= local_x < size_x and 0 <= local_y < size_y):
            return False
        dx = size_x - (local_x + 0.5)
        dy = size_y - (local_y + 0.5)
    else:
        return False

    norm_x = dx / size_x
    norm_y = dy / size_y
    return norm_x * norm_x + norm_y * norm_y > 1.0


def _load_hero_mask_from_config(
    layout_cfg: dict,
    width: int,
    height: int,
    threshold: int,
    scale_x: float,
    scale_y: float,
) -> frozenset[tuple[int, int]]:
    primary_path = _resolve_mask_path(layout_cfg.get("hero_mask_path", "assets/hero_mask.png"))
    return _load_mask_variant(primary_path, width, height, threshold, scale_x, scale_y)


def _resolve_mask_path(mask_value: str) -> Path:
    mask_path = Path(mask_value)
    if not mask_path.is_absolute():
        mask_path = Path(__file__).resolve().parents[1] / mask_path
    return mask_path


def _load_mask_variant(
    mask_path: Path,
    width: int,
    height: int,
    threshold: int,
    scale_x: float,
    scale_y: float,
) -> frozenset[tuple[int, int]]:
    if not mask_path.exists():
        return frozenset()
    return _load_resized_hero_mask(str(mask_path), width, height, threshold, scale_x, scale_y)


def _bounding_rect(points: set[tuple[int, int]]) -> Rect:
    xs = [x for x, _ in points]
    ys = [y for _, y in points]
    min_x = min(xs)
    max_x = max(xs)
    min_y = min(ys)
    max_y = max(ys)
    return Rect(x=min_x, y=min_y, width=(max_x - min_x + 1), height=(max_y - min_y + 1))


def _boundary_cells(points: set[tuple[int, int]]) -> set[tuple[int, int]]:
    if not points:
        return set()
    boundary: set[tuple[int, int]] = set()
    for x, y in points:
        if (
            (x - 1, y) not in points
            or (x + 1, y) not in points
            or (x, y - 1) not in points
            or (x, y + 1) not in points
        ):
            boundary.add((x, y))
    return boundary


def _hero_zone_blocks_cell(x: int, y: int, hero_zone: Rect, layout_cfg: dict) -> bool:
    if not hero_zone.contains(x, y):
        return False

    top_left_size = int(layout_cfg.get("hero_corner_trim_top_left", 3))
    top_left_size_y = int(layout_cfg.get("hero_corner_trim_top_left_y", top_left_size))
    top_right_size = int(layout_cfg.get("hero_corner_trim_top_right", 0))
    top_right_size_y = int(layout_cfg.get("hero_corner_trim_top_right_y", top_right_size))
    bottom_left_size = int(layout_cfg.get("hero_corner_trim_bottom_left", 5))
    bottom_left_size_y = int(layout_cfg.get("hero_corner_trim_bottom_left_y", bottom_left_size))
    bottom_right_size = int(layout_cfg.get("hero_corner_trim_bottom_right", 5))
    bottom_right_size_y = int(layout_cfg.get("hero_corner_trim_bottom_right_y", bottom_right_size))

    if _corner_trim_allows_cell(x, y, hero_zone, corner="top_left", size_x=top_left_size, size_y=top_left_size_y):
        return False
    if _corner_trim_allows_cell(x, y, hero_zone, corner="top_right", size_x=top_right_size, size_y=top_right_size_y):
        return False
    if _corner_trim_allows_cell(x, y, hero_zone, corner="bottom_left", size_x=bottom_left_size, size_y=bottom_left_size_y):
        return False
    if _corner_trim_allows_cell(x, y, hero_zone, corner="bottom_right", size_x=bottom_right_size, size_y=bottom_right_size_y):
        return False

    return True


def _corner_trim_allows_cell(
    x: int,
    y: int,
    rect: Rect,
    *,
    corner: str,
    size_x: int,
    size_y: int,
) -> bool:
    if size_x <= 0 or size_y <= 0:
        return False

    if corner == "top_left":
        local_x = x - rect.x
        local_y = y - rect.y
        if not (0 <= local_x < size_x and 0 <= local_y < size_y):
            return False
        dx = size_x - (local_x + 0.5)
        dy = size_y - (local_y + 0.5)
    elif corner == "top_right":
        local_x = rect.right - 1 - x
        local_y = y - rect.y
        if not (0 <= local_x < size_x and 0 <= local_y < size_y):
            return False
        dx = size_x - (local_x + 0.5)
        dy = size_y - (local_y + 0.5)
    elif corner == "bottom_left":
        local_x = x - rect.x
        local_y = rect.bottom - 1 - y
        if not (0 <= local_x < size_x and 0 <= local_y < size_y):
            return False
        dx = size_x - (local_x + 0.5)
        dy = size_y - (local_y + 0.5)
    elif corner == "bottom_right":
        local_x = rect.right - 1 - x
        local_y = rect.bottom - 1 - y
        if not (0 <= local_x < size_x and 0 <= local_y < size_y):
            return False
        dx = size_x - (local_x + 0.5)
        dy = size_y - (local_y + 0.5)
    else:
        return False

    norm_x = dx / size_x
    norm_y = dy / size_y
    return norm_x * norm_x + norm_y * norm_y > 1.0


def _build_region_cells(
    allowed_cells: set[tuple[int, int]],
    bounds: Rect,
    hero_zone: Rect,
) -> dict[str, frozenset[tuple[int, int]]]:
    midpoint = bounds.width // 2

    regions: dict[str, set[tuple[int, int]]] = {
        "above_hero": set(),
        "below_hero": set(),
        "left_field": set(),
        "right_field": set(),
    }

    for x, y in allowed_cells:
        if y < hero_zone.y:
            regions["above_hero"].add((x, y))
        elif y >= hero_zone.bottom:
            regions["below_hero"].add((x, y))

        if x < midpoint:
            regions["left_field"].add((x, y))
        else:
            regions["right_field"].add((x, y))

    return {name: frozenset(cells) for name, cells in regions.items()}
