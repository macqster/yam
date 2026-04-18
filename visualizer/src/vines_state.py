from __future__ import annotations

from dataclasses import dataclass, field

from vines_types import Direction, GrowthTip, Point


@dataclass
class VinesState:
    frame: int = 0
    stems: set[Point] = field(default_factory=set)
    stem_birth: dict[Point, int] = field(default_factory=dict)
    trunk_cells: set[Point] = field(default_factory=set)
    trunk_birth: dict[Point, int] = field(default_factory=dict)
    tips: list[GrowthTip] = field(default_factory=list)
    terminal_leaves: set[Point] = field(default_factory=set)
    active_leaf_positions: set[Point] = field(default_factory=set)
    active_leaf_dirs: dict[Point, Direction] = field(default_factory=dict)
    leaf_stamps: dict[Point, str] = field(default_factory=dict)
    dead_leaf_stamps: dict[Point, str] = field(default_factory=dict)
    thickened_wood: dict[Point, str] = field(default_factory=dict)
    flower_stamps: dict[Point, str] = field(default_factory=dict)
    flower_birth: dict[Point, int] = field(default_factory=dict)
    flower_parent: dict[Point, Point] = field(default_factory=dict)
    info_hanger_spawned: bool = False
    hero_top_commit_active: bool = False
    trunk_route_phase: str = "approach"
    debug_stats: dict[str, object] = field(
        default_factory=lambda: {
            "spawn_origin_counts": {},
            "failed_move_counts": {},
            "region_coverage": {},
            "stem_count": 0,
            "ornament_count": 0,
        }
    )
