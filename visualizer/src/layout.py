from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from pathlib import Path

from PIL import Image

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
    ivy_bounds: Rect
    no_go_zones: tuple[Rect, ...]
    hero_raw_mask_cells: frozenset[tuple[int, int]]
    hero_mask_boundary_cells: frozenset[tuple[int, int]]
    hero_mask_cells: frozenset[tuple[int, int]]
    allowed_cells: frozenset[tuple[int, int]]
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
    info_base_x = size.columns - layout_cfg["info_width"] - margin_x
    info_x = info_base_x + layout_cfg["info_offset_x"]
    info_y = margin_y + layout_cfg["info_offset_y"]
    min_info_x = hero.right + info_gap
    max_info_x = max(margin_x, size.columns - layout_cfg["info_width"] - margin_x)
    info = Rect(
        x=max(min_info_x, min(info_x, max_info_x)),
        y=info_y,
        width=layout_cfg["info_width"],
        height=layout_cfg["info_height"],
    )

    ivy_bounds = Rect(
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
    info_zone = no_go_zones[1]
    for y in range(1, max(1, ivy_bounds.height - 1)):
        for x in range(1, max(1, ivy_bounds.width - 1)):
            if info_zone.contains(x, y):
                continue
            if hero_blocked_cells:
                if (x, y) in hero_blocked_cells:
                    continue
            elif _hero_zone_blocks_cell(x, y, no_go_zones[0], layout_cfg):
                continue
            allowed_cells.add((x, y))

    region_cells = _build_region_cells(allowed_cells, ivy_bounds, hero)

    return SceneLayout(
        hero=hero,
        info=info,
        hero_guide=hero,
        info_guide=info,
        ivy_bounds=ivy_bounds,
        no_go_zones=no_go_zones,
        hero_raw_mask_cells=frozenset(hero_raw_mask_cells),
        hero_mask_boundary_cells=frozenset(_boundary_cells(hero_raw_mask_cells)),
        hero_mask_cells=frozenset(hero_blocked_cells),
        allowed_cells=frozenset(allowed_cells),
        region_cells=region_cells,
        warning=warning,
    )


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
        world_x = hero.x + local_x
        world_y = hero.y + local_y
        raw_cells.add((world_x, world_y))
        for dy in range(-pad_y, pad_y + 1):
            for dx in range(-pad_x, pad_x + 1):
                blocked.add((world_x + dx, world_y + dy))
    return raw_cells, blocked


def _load_hero_mask_from_config(
    layout_cfg: dict,
    width: int,
    height: int,
    threshold: int,
    scale_x: float,
    scale_y: float,
) -> frozenset[tuple[int, int]]:
    primary_path = _resolve_mask_path(layout_cfg.get("hero_mask_path", "assets/hero_mask.png"))
    fallback_value = layout_cfg.get("hero_mask_scaled_fallback_path", "")
    fallback_path = _resolve_mask_path(fallback_value) if fallback_value else None

    primary_cells = _load_mask_variant(primary_path, width, height, threshold, scale_x, scale_y)
    if primary_cells and not _mask_is_degenerate(primary_cells, width, height):
        return primary_cells

    if fallback_path:
        fallback_cells = _load_mask_variant(fallback_path, width, height, threshold, 1.0, 1.0)
        if fallback_cells:
            return fallback_cells

    return primary_cells


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


def _mask_is_degenerate(mask_cells: frozenset[tuple[int, int]], width: int, height: int) -> bool:
    if not mask_cells:
        return True
    coverage = len(mask_cells) / max(1, width * height)
    xs = [x for x, _ in mask_cells]
    ys = [y for _, y in mask_cells]
    touches_edges = (
        min(xs) <= 0
        and max(xs) >= width - 2
        and min(ys) <= 0
        and max(ys) >= height - 2
    )
    return coverage >= 0.72 or touches_edges


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

    top_left_size = layout_cfg.get("hero_corner_trim_top_left", 3)
    bottom_left_size = layout_cfg.get("hero_corner_trim_bottom_left", 5)
    bottom_right_size = layout_cfg.get("hero_corner_trim_bottom_right", 5)

    # Soft-trim the upper-left corner and both bottom corners so growth can
    # creep slightly into the hero boundary without fully breaking readability.
    if _corner_trim_allows_cell(x, y, hero_zone, corner="top_left", size=top_left_size):
        return False
    if _corner_trim_allows_cell(x, y, hero_zone, corner="bottom_left", size=bottom_left_size):
        return False
    if _corner_trim_allows_cell(x, y, hero_zone, corner="bottom_right", size=bottom_right_size):
        return False

    return True


def _corner_trim_allows_cell(x: int, y: int, rect: Rect, *, corner: str, size: int) -> bool:
    if size <= 0:
        return False

    if corner == "top_left":
        local_x = x - rect.x
        local_y = y - rect.y
        if not (0 <= local_x < size and 0 <= local_y < size):
            return False
        dx = size - (local_x + 0.5)
        dy = size - (local_y + 0.5)
        return dx * dx + dy * dy > size * size

    if corner == "bottom_left":
        local_x = x - rect.x
        local_y = rect.bottom - 1 - y
        if not (0 <= local_x < size and 0 <= local_y < size):
            return False
        dx = size - (local_x + 0.5)
        dy = size - (local_y + 0.5)
        return dx * dx + dy * dy > size * size

    if corner == "bottom_right":
        local_x = rect.right - 1 - x
        local_y = rect.bottom - 1 - y
        if not (0 <= local_x < size and 0 <= local_y < size):
            return False
        dx = size - (local_x + 0.5)
        dy = size - (local_y + 0.5)
        return dx * dx + dy * dy > size * size

    return False


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
