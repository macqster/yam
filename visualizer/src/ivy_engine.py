from __future__ import annotations

import random

from ivy_growth import (
    branch_direction,
    initial_trunk_seed,
    limit_tips,
    select_move,
)
from ivy_ornament import (
    count_debug,
    merge_segments,
    rebuild_leaf_stamps,
    rebuild_thickened_wood,
    stamp_death_cluster,
    trim_ornaments,
    update_debug_stats,
)
from ivy_state import IvyState
from ivy_types import GrowthTip
from layout import SceneLayout


class IvyEngine:
    def __init__(self, config: dict, seed: int = 7) -> None:
        self.config = config["ivy"]
        self.debug_config = self.config.get("debug", {})
        self.rng = random.Random(seed)
        self.size = None
        self.layout: SceneLayout | None = None
        self.state = IvyState()

    @property
    def stems(self):
        return self.state.stems

    def reset(self, size, layout: SceneLayout) -> None:
        self.size = size
        self.layout = layout
        self.state = IvyState()

        origin = initial_trunk_seed(self.config, layout, self.rng, self.state)
        if origin is None:
            return

        x, y = origin
        self.state.stems.add((x, y))
        self.state.stem_birth[(x, y)] = self.state.frame
        trunk_life = float(self.config["trunk_life"])
        self.state.tips = [GrowthTip(x=x, y=y, dx=0, dy=-1, life=trunk_life, max_life=trunk_life, is_trunk=True)]
        update_debug_stats(self.state, layout)

    def tick(self, layout: SceneLayout) -> None:
        if self.layout != layout or self.size is None:
            return
        if not self.state.tips and len(self.state.stems) >= int(self.config["max_structural_cells"]):
            return

        self.state.frame += 1
        max_structural = int(self.config["max_structural_cells"])
        max_tips = int(self.config["max_tips"])
        branch_chance = float(self.config["branch_chance"])
        branch_life_min = int(self.config["branch_life_min"])
        branch_life_max = int(self.config["branch_life_max"])

        new_tips: list[GrowthTip] = []
        active_leaf_positions: set[tuple[int, int]] = set()
        active_leaf_dirs: dict[tuple[int, int], tuple[int, int]] = {}

        for tip in self.state.tips:
            if len(self.state.stems) >= max_structural:
                break

            if tip.is_trunk:
                tip.life -= float(self.config["trunk_decay"])
            else:
                tip.life -= float(self.config["branch_decay"])

            if tip.life <= 0:
                self.state.terminal_leaves.add((tip.x, tip.y))
                if not tip.is_trunk:
                    stamp_death_cluster(self.state, tip.x, tip.y, tip.dx, tip.dy, layout, self.rng)
                continue

            move = select_move(tip, self.state, self.config, layout, self.rng, self.debug_config)
            if move is None:
                self.state.terminal_leaves.add((tip.x, tip.y))
                if not tip.is_trunk:
                    stamp_death_cluster(self.state, tip.x, tip.y, tip.dx, tip.dy, layout, self.rng)
                continue

            new_x, new_y, dx, dy = move
            self.state.stems.add((new_x, new_y))
            self.state.stem_birth[(new_x, new_y)] = self.state.frame
            self.state.terminal_leaves.discard((tip.x, tip.y))
            active_leaf_positions.add((new_x, new_y))
            active_leaf_dirs[(new_x, new_y)] = (dx, dy)
            new_tips.append(
                GrowthTip(
                    x=new_x,
                    y=new_y,
                    dx=dx,
                    dy=dy,
                    life=tip.life,
                    max_life=tip.max_life,
                    is_trunk=tip.is_trunk,
                )
            )

            if tip.is_trunk and self.rng.random() < branch_chance:
                new_branch_direction = branch_direction(self.config, new_x, new_y, dy, layout)
                if new_branch_direction is not None:
                    branch_life = float(self.rng.randint(branch_life_min, branch_life_max))
                    new_tips.append(
                        GrowthTip(
                            x=new_x,
                            y=new_y,
                            dx=new_branch_direction[0],
                            dy=new_branch_direction[1],
                            life=branch_life,
                            max_life=branch_life,
                            is_trunk=False,
                        )
                    )
                    count_debug(self.state, "spawn_origin_counts", "branch")

        self.state.tips = limit_tips(new_tips, max_tips, self.rng)
        self.state.active_leaf_positions = self.state.terminal_leaves | active_leaf_positions
        self.state.active_leaf_dirs = active_leaf_dirs
        rebuild_leaf_stamps(self.state, self.config, layout, self.rng)
        rebuild_thickened_wood(self.state, self.config, layout, self.rng)
        trim_ornaments(self.state, int(self.config["max_ornament_cells"]), self.rng)
        update_debug_stats(self.state, layout)

    def get_segments(self) -> dict[tuple[int, int], str]:
        return merge_segments(self.state, self.debug_config)
