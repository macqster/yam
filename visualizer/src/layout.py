from __future__ import annotations

from dataclasses import dataclass

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
    ivy_bounds: Rect
    no_go_zones: tuple[Rect, ...]
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
    hero_collision = hero.inset(
        layout_cfg.get("hero_collision_trim_left", 0),
        layout_cfg.get("hero_collision_trim_top", 0),
        layout_cfg.get("hero_collision_trim_right", 0),
        layout_cfg.get("hero_collision_trim_bottom", 0),
    )
    info_collision = info.inset(
        layout_cfg.get("info_collision_trim_left", 0),
        layout_cfg.get("info_collision_trim_top", 0),
        layout_cfg.get("info_collision_trim_right", 0),
        layout_cfg.get("info_collision_trim_bottom", 0),
    )
    no_go_zones = (
        hero_collision.inflate(hero_pad_x, hero_pad_y),
        info_collision.inflate(info_pad_x, info_pad_y),
    )

    allowed_cells: set[tuple[int, int]] = set()
    for y in range(1, max(1, ivy_bounds.height - 1)):
        for x in range(1, max(1, ivy_bounds.width - 1)):
            if any(zone.contains(x, y) for zone in no_go_zones):
                continue
            allowed_cells.add((x, y))

    hero_zone = no_go_zones[0]
    region_cells = _build_region_cells(allowed_cells, ivy_bounds, hero_zone)

    return SceneLayout(
        hero=hero,
        info=info,
        ivy_bounds=ivy_bounds,
        no_go_zones=no_go_zones,
        allowed_cells=frozenset(allowed_cells),
        region_cells=region_cells,
        warning=warning,
    )


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
