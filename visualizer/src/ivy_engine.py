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
    rebuild_flower_stamps,
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
        self.state.trunk_route_phase = "approach"

        origin = initial_trunk_seed(self.config, layout, self.rng, self.state)
        if origin is None:
            return

        x, y = origin
        self.state.stems.add((x, y))
        self.state.stem_birth.setdefault((x, y), self.state.frame)
        self.state.trunk_cells.add((x, y))
        self.state.trunk_birth.setdefault((x, y), self.state.frame)
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
        base_branch_chance = float(self.config["branch_chance"])
        branch_life_min = int(self.config["branch_life_min"])
        branch_life_max = int(self.config["branch_life_max"])

        # Phase 1: Structural Growth
        # --------------------------
        # This phase is responsible for advancing live growth fronts, committing
        # new stem cells into persistent structure, spawning branches, and
        # injecting explicitly scoped special-case tips such as the info hanger.
        #
        # IMPORTANT:
        # - This phase may update structural state and route state.
        # - This phase must NOT perform canopy shaping or ornament rendering.
        new_tips, active_leaf_dirs = self._run_structural_growth_phase(
            layout=layout,
            max_structural=max_structural,
            base_branch_chance=base_branch_chance,
            branch_life_min=branch_life_min,
            branch_life_max=branch_life_max,
        )
        self.state.tips = limit_tips(new_tips, max_tips, self.rng)

        # Phase 2: Foliage Host Discovery
        # --------------------------------
        # This phase derives potential foliage anchors from committed vine
        # structure. It should only discover where foliage *can* exist based on
        # maturity and connectivity; it must not yet perform regional density
        # shaping or visual ornament decisions.
        active_positions, active_dirs = self._run_foliage_host_discovery_phase(
            active_leaf_dirs=active_leaf_dirs,
        )

        # Phase 3: Spatial Shaping
        # ------------------------
        # This phase modifies discovered foliage hosts for canopy readability and
        # composition. It is allowed to enrich, thin, and reshape foliage host
        # distribution, but it must NOT modify structural vine growth.
        #
        # Enrichment: expand foliage hosts into nearby empty cells for canopy shaping.
        active_positions = self._apply_host_enrichment(
            active_positions=active_positions,
            layout=layout,
        )

        # Directional bias: bias clusters away from stems (directional foliage)
        active_dirs = self._apply_directional_foliage_bias(
            active_positions=active_positions,
            active_dirs=active_dirs,
        )

        active_positions = self._apply_canopy_jitter(
            active_positions=active_positions,
            layout=layout,
        )

        broken_positions, broken_dirs = self._filter_canopy_for_readability(
            active_positions=active_positions,
            active_dirs=active_dirs,
            layout=layout,
        )
        self.state.active_leaf_positions = broken_positions
        self.state.active_leaf_dirs = broken_dirs

        self._spawn_top_left_hanging_stems(layout)

        self._run_ornament_reconstruction_phase(layout)

    def get_segments(self) -> dict[tuple[int, int], str]:
        return merge_segments(self.state, self.debug_config)

    def _run_structural_growth_phase(
        self,
        layout: SceneLayout,
        max_structural: int,
        base_branch_chance: float,
        branch_life_min: int,
        branch_life_max: int,
    ) -> tuple[list[GrowthTip], dict[tuple[int, int], tuple[int, int]]]:
        # Phase 1: Structural Growth
        # Iterates over all currently active tips and advances the live structural
        # simulation by one tick. This helper owns:
        # - tip decay
        # - movement / failure handling
        # - stem commitment
        # - trunk route-state updates
        # - branch spawning
        # - explicitly scoped special-case tip injection
        #
        # It returns the next active tip list plus structural direction hints for
        # freshly advanced cells. Stage 2 state-flow cleanup removes the unused
        # transient position set so this helper only returns data that is actually
        # consumed downstream.
        new_tips: list[GrowthTip] = []
        active_leaf_dirs: dict[tuple[int, int], tuple[int, int]] = {}

        for tip in self.state.tips:
            if len(self.state.stems) >= max_structural:
                break

            self._process_growth_tip(
                tip=tip,
                layout=layout,
                new_tips=new_tips,
                active_leaf_dirs=active_leaf_dirs,
                base_branch_chance=base_branch_chance,
                branch_life_min=branch_life_min,
                branch_life_max=branch_life_max,
            )

        return new_tips, active_leaf_dirs

    def _run_foliage_host_discovery_phase(
        self,
        active_leaf_dirs: dict[tuple[int, int], tuple[int, int]],
    ) -> tuple[set[tuple[int, int]], dict[tuple[int, int], tuple[int, int]]]:
        # Phase 2: Foliage Host Discovery
        # Discovers where foliage can exist based on committed structure, stem
        # maturity, and local connectivity. This phase intentionally stops short
        # of regional density shaping; it should only derive host candidates plus
        # their initial structural direction hints.
        maturity_threshold = int(self.config.get("leaf_maturity_frames", 8))
        trunk_maturity_threshold = int(self.config.get("leaf_trunk_maturity_frames", max(3, maturity_threshold - 3)))

        mature_positions, stem_orientations = self._collect_mature_foliage_hosts(
            active_leaf_dirs=active_leaf_dirs,
            maturity_threshold=maturity_threshold,
            trunk_maturity_threshold=trunk_maturity_threshold,
        )

        # Keep tip accents, but let mature runs and pockets host foliage too.
        # At this stage, directions represent structural orientation hints, not
        # final foliage-emission policy.
        active_positions = mature_positions | (self.state.terminal_leaves & mature_positions)
        active_dirs = {**stem_orientations, **active_leaf_dirs}
        return active_positions, active_dirs

    def _collect_mature_foliage_hosts(
        self,
        active_leaf_dirs: dict[tuple[int, int], tuple[int, int]],
        maturity_threshold: int,
        trunk_maturity_threshold: int,
    ) -> tuple[set[tuple[int, int]], dict[tuple[int, int], tuple[int, int]]]:
        # Phase 2: Foliage Host Discovery
        # Scans committed structural cells and promotes sufficiently mature stem
        # positions into foliage hosts. Structural orientation hints are inferred
        # from local connectivity so later phases have a stable stem-direction
        # baseline before any foliage-emission policy is applied.
        mature_positions: set[tuple[int, int]] = set()
        stem_orientations: dict[tuple[int, int], tuple[int, int]] = {}

        for pos, birth in self.state.stem_birth.items():
            age = self.state.frame - birth
            is_trunk = pos in self.state.trunk_cells
            local_threshold = trunk_maturity_threshold if is_trunk else maturity_threshold
            if age < local_threshold:
                continue

            mature_positions.add(pos)
            stem_orientations[pos] = self._infer_stem_orientation(
                pos=pos,
                fallback_dirs=active_leaf_dirs,
            )

        return mature_positions, stem_orientations


    def _infer_stem_orientation(
        self,
        pos: tuple[int, int],
        fallback_dirs: dict[tuple[int, int], tuple[int, int]],
    ) -> tuple[int, int]:
        # Phase 2: Foliage Host Discovery
        # Infers the dominant structural direction at a committed stem cell using
        # local structural connectivity. This returns stem orientation only — it
        # does NOT decide final foliage-emission direction.
        #
        # WHY this helper exists:
        # Stage 2 begins unifying direction semantics by separating “what
        # direction is the stem going here?” from “where should foliage fan out?”
        # This helper is the structural side of that split.
        x, y = pos
        left = (x - 1, y) in self.state.stems
        right = (x + 1, y) in self.state.stems
        up = (x, y - 1) in self.state.stems
        down = (x, y + 1) in self.state.stems

        if left and right and not (up or down):
            return (1, 0)
        if up and down and not (left or right):
            return (0, -1)
        if right or left:
            return (1, 0) if right else (-1, 0)
        if up or down:
            return (0, -1) if up else (0, 1)
        return fallback_dirs.get(pos, (0, -1))

    def _process_growth_tip(
        self,
        tip: GrowthTip,
        layout: SceneLayout,
        new_tips: list[GrowthTip],
        active_leaf_dirs: dict[tuple[int, int], tuple[int, int]],
        base_branch_chance: float,
        branch_life_min: int,
        branch_life_max: int,
    ) -> None:
        # Phase 1: Structural Growth
        # Advances one live tip through decay, movement, structural commitment,
        # and optional branch/special-tip spawning.
        #
        # WHY this helper exists:
        # The per-tip lifecycle is the core unit of structural simulation, and
        # pulling it into a dedicated helper makes Phase 1 boundaries explicit
        # without changing behavior.
        #
        # State-flow note:
        # This helper now records only the structural direction hint that survives
        # into Phase 2. It no longer writes an unused transient position set.
        if tip.is_trunk:
            tip.life -= float(self.config["trunk_decay"])
        else:
            tip.life -= float(self.config["branch_decay"])

        if tip.life <= 0:
            self.state.terminal_leaves.add((tip.x, tip.y))
            if not tip.is_trunk:
                stamp_death_cluster(self.state, tip.x, tip.y, tip.dx, tip.dy, layout, self.rng)
            return

        move = select_move(tip, self.state, self.config, layout, self.rng, self.debug_config)
        if move is None:
            self.state.terminal_leaves.add((tip.x, tip.y))
            if not tip.is_trunk:
                stamp_death_cluster(self.state, tip.x, tip.y, tip.dx, tip.dy, layout, self.rng)
            return

        new_x, new_y, dx, dy = move
        if tip.is_trunk:
            self._update_trunk_route_state(new_x, new_y, layout)

        self.state.stems.add((new_x, new_y))
        self.state.stem_birth.setdefault((new_x, new_y), self.state.frame)
        if tip.is_trunk:
            self.state.trunk_cells.add((new_x, new_y))
            self.state.trunk_birth.setdefault((new_x, new_y), self.state.frame)

        self.state.terminal_leaves.discard((tip.x, tip.y))
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

        branch_chance = base_branch_chance
        if tip.is_trunk:
            hero_zone = layout.hero_guide
            suppression_margin = int(self.config.get("branch_suppression_margin", 10))
            suppression_factor = float(self.config.get("pre_contact_branch_factor", 0.35))
            if new_x > hero_zone.right + suppression_margin and new_y >= hero_zone.y:
                branch_chance *= suppression_factor
            if self.state.trunk_route_phase == "hero_top":
                hero_span = max(1, hero_zone.width)
                leftward_progress = (hero_zone.right - new_x) / hero_span
                leftward_progress = max(0.0, min(1.0, leftward_progress))
                base_top_factor = float(self.config.get("hero_top_branch_factor", 0.2))
                released_top_factor = float(self.config.get("hero_top_left_branch_factor", 0.8))
                branch_chance *= base_top_factor + ((released_top_factor - base_top_factor) * leftward_progress)

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

        if tip.is_trunk and not self.state.info_hanger_spawned:
            hanger = self._spawn_info_hanger(new_x, new_y, layout)
            if hanger is not None:
                new_tips.append(hanger)
                self.state.info_hanger_spawned = True
                count_debug(self.state, "spawn_origin_counts", "info_hanger")

    def _update_trunk_route_state(self, x: int, y: int, layout: SceneLayout) -> None:
        hero_zone = layout.hero_guide
        top_margin = int(self.config.get("hero_top_commit_margin", 1))
        hero_mid_x = hero_zone.x + hero_zone.width // 2
        hero_left_release_x = hero_zone.x + hero_zone.width // 3

        in_top_band = hero_zone.y - top_margin <= y <= hero_zone.y + top_margin
        if self.state.trunk_route_phase == "approach" and in_top_band and x >= hero_mid_x:
            self.state.trunk_route_phase = "hero_top"

        if self.state.trunk_route_phase == "hero_top":
            self.state.hero_top_commit_active = True
            if x <= hero_left_release_x or y > hero_zone.y + top_margin + 1 or y < hero_zone.y - top_margin - 1:
                self.state.trunk_route_phase = "post_top"
                self.state.hero_top_commit_active = False
        else:
            self.state.hero_top_commit_active = False

    def _spawn_info_hanger(self, x: int, y: int, layout: SceneLayout) -> GrowthTip | None:
        hero_zone = layout.hero_guide
        info_zone = layout.info_guide
        trigger_margin = int(self.config.get("info_hanger_trigger_margin", 0))
        vertical_margin = int(self.config.get("info_hanger_vertical_margin", 0))

        if not (info_zone.x - trigger_margin <= x <= info_zone.right + trigger_margin):
            return None
        if not (hero_zone.y - vertical_margin <= y <= info_zone.bottom + vertical_margin):
            return None

        target_x = hero_zone.right + int(self.config.get("info_hanger_target_offset", 1))
        branch_dx = 0
        if x < target_x:
            branch_dx = 1
        elif x > target_x:
            branch_dx = -1

        next_point = (x + branch_dx, y + 1)
        if next_point not in layout.allowed_cells:
            fallback_options = (
                (x, y + 1),
                (x - 1, y + 1),
                (x + 1, y + 1),
                (x - 1, y),
                (x + 1, y),
            )
            branch_dx = 0
            for candidate_x, candidate_y in fallback_options:
                if (candidate_x, candidate_y) in layout.allowed_cells:
                    if candidate_x > x:
                        branch_dx = 1
                    elif candidate_x < x:
                        branch_dx = -1
                    return GrowthTip(
                        x=x,
                        y=y,
                        dx=branch_dx,
                        dy=1 if candidate_y >= y else 0,
                        life=float(self.config.get("info_hanger_life", self.config["branch_life_max"])),
                        max_life=float(self.config.get("info_hanger_life", self.config["branch_life_max"])),
                        is_trunk=False,
                    )
            return None

        branch_life = float(self.config.get("info_hanger_life", self.config["branch_life_max"]))
        return GrowthTip(
            x=x,
            y=y,
            dx=branch_dx,
            dy=1,
            life=branch_life,
            max_life=branch_life,
            is_trunk=False,
        )

    def _apply_host_enrichment(
        self,
        active_positions: set[tuple[int, int]],
        layout: SceneLayout,
    ) -> set[tuple[int, int]]:
        # Phase 3: Spatial Shaping
        # Expands the discovered foliage-host field into nearby empty cells so the
        # canopy reads as clustered and spatially present rather than restricted to
        # only exact structural host points.
        #
        # WHY this helper exists:
        # This is the first explicit spatial-density shaping pass. Stage 2 begins
        # formalizing these shaping rules as named field policies rather than
        # leaving region bias as anonymous inline arithmetic.
        enriched_positions = set(active_positions)
        for (x, y) in list(active_positions):
            for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if (nx, ny) in layout.allowed_cells and (nx, ny) not in self.state.stems:
                    local_enrichment = self._sample_host_enrichment_field(
                        pos=(nx, ny),
                        layout=layout,
                    )
                    if ((nx * 193 + ny * 389) % 100) < local_enrichment:
                        enriched_positions.add((nx, ny))
        return enriched_positions

    def _apply_directional_foliage_bias(
        self,
        active_positions: set[tuple[int, int]],
        active_dirs: dict[tuple[int, int], tuple[int, int]],
    ) -> dict[tuple[int, int], tuple[int, int]]:
        # Phase 3: Spatial Shaping
        # Reorients foliage emission so clusters push away from the dominant local
        # stem axis. Horizontal runs emit vertically; vertical runs emit laterally.
        #
        # WHY this helper exists:
        # Directional bias is a distinct shaping policy layered on top of host
        # discovery. Extracting it keeps “where hosts exist” separate from “how
        # those hosts should fan outward.”
        directional_dirs: dict[tuple[int, int], tuple[int, int]] = {}

        for pos in active_positions:
            directional_dirs[pos] = self._infer_foliage_emission_direction(
                pos=pos,
                structural_dirs=active_dirs,
            )

        return directional_dirs


    def _infer_foliage_emission_direction(
        self,
        pos: tuple[int, int],
        structural_dirs: dict[tuple[int, int], tuple[int, int]],
    ) -> tuple[int, int]:
        # Phase 3: Spatial Shaping
        # Converts structural direction knowledge into foliage-emission policy.
        # This helper answers a different question than `_infer_stem_orientation`:
        # the structural helper asks where the stem is going, while this helper
        # asks where foliage should fan out from that stem.
        #
        # WHY this helper exists:
        # Stage 2 is separating direction semantics into two layers:
        # - Phase 2: structural orientation inference
        # - Phase 3: foliage-emission policy
        # Keeping them distinct reduces conceptual overlap even while behavior is
        # intentionally preserved.
        x, y = pos
        left = (x - 1, y) in self.state.stems
        right = (x + 1, y) in self.state.stems
        up = (x, y - 1) in self.state.stems
        down = (x, y + 1) in self.state.stems

        # horizontal stem → leaves go vertical
        if left and right and not (up or down):
            return (0, 1)

        # vertical stem → leaves go horizontal
        if up and down and not (left or right):
            return (1, 0)

        # fallback: keep existing structural or inherited direction if available
        if pos in structural_dirs:
            return structural_dirs[pos]

        return (0, 1)

    def _sample_host_enrichment_field(
        self,
        pos: tuple[int, int],
        layout: SceneLayout,
    ) -> int:
        # Phase 3: Spatial Shaping
        # Samples the current canopy-enrichment field for a candidate foliage host.
        # This makes the regional enrichment policy explicit: different parts of
        # the scene receive different encouragement strengths even though the
        # underlying behavior is still the same deterministic arithmetic as before.
        #
        # WHY this helper exists:
        # Stage 2 spatial-field formalization is turning implicit region bias into
        # named policy surfaces. This helper is the enrichment-density surface.
        nx, ny = pos
        hero_zone = layout.hero_guide
        left_canopy_limit_x = hero_zone.x - 2
        left_canopy_limit_y = hero_zone.y + max(4, hero_zone.height // 4)

        if nx <= left_canopy_limit_x and ny <= left_canopy_limit_y:
            return 26
        if nx <= hero_zone.x and ny <= hero_zone.y + 2:
            return 18
        return 12

    def _apply_canopy_jitter(
        self,
        active_positions: set[tuple[int, int]],
        layout: SceneLayout,
    ) -> set[tuple[int, int]]:
        # Phase 3: Spatial Shaping
        # Applies a light deterministic jitter pass so canopy anchors break out of
        # rigid alignment while still reading as clustered foliage rather than
        # salt-and-pepper scatter.
        #
        # WHY this helper exists:
        # Jitter is a distinct morphology-shaping pass. Extracting it clarifies
        # that small positional nudges are part of canopy shaping, not host
        # discovery or ornament rendering.
        jittered_positions: set[tuple[int, int]] = set()

        for (jx, jy) in active_positions:
            r = ((jx * 37 + jy * 53) % 100)
            left = (jx - 1, jy) in active_positions
            right = (jx + 1, jy) in active_positions
            horizontal_run = left or right

            # downward sag (primary) — only nudge isolated anchors aggressively;
            # horizontal runs should mostly stay together and droop later in the
            # dedicated top-run breakup pass.
            if (not horizontal_run) and r < 12 and (jx, jy + 1) in layout.allowed_cells:
                jittered_positions.add((jx, jy + 1))

            # slight upward lift (break symmetry) — keep very rare so clusters do
            # not dissolve into speckle.
            elif (not horizontal_run) and 12 <= r < 13 and (jx, jy - 1) in layout.allowed_cells:
                jittered_positions.add((jx, jy - 1))

            # rare horizontal nudge for isolated points only.
            elif (not horizontal_run) and 13 <= r < 15:
                if (jx + 1, jy) in layout.allowed_cells:
                    jittered_positions.add((jx + 1, jy))
                elif (jx - 1, jy) in layout.allowed_cells:
                    jittered_positions.add((jx - 1, jy))
                else:
                    jittered_positions.add((jx, jy))

            else:
                jittered_positions.add((jx, jy))

        return jittered_positions

    def _filter_canopy_for_readability(
        self,
        active_positions: set[tuple[int, int]],
        active_dirs: dict[tuple[int, int], tuple[int, int]],
        layout: SceneLayout,
    ) -> tuple[set[tuple[int, int]], dict[tuple[int, int], tuple[int, int]]]:
        # Phase 3: Spatial Shaping
        # Applies the readability and morphology filters that prevent the canopy
        # from becoming a flat, over-uniform band. This includes lower-right base
        # thinning, upper-run breakup, and a final global horizontal suppression pass.
        #
        # WHY this helper exists:
        # These filters are composition policy, not structural growth. Stage 2
        # begins formalizing them as named field/policy checks so the shaping
        # layer reads like a system rather than scattered heuristics.
        filtered_positions: set[tuple[int, int]] = set()
        filtered_dirs: dict[tuple[int, int], tuple[int, int]] = {}

        for pos in active_positions:
            x, y = pos
            if self._should_thin_for_base_readability(pos=pos, layout=layout):
                continue

            if self._apply_top_run_breakup_policy(
                pos=pos,
                active_dirs=active_dirs,
                filtered_positions=filtered_positions,
                filtered_dirs=filtered_dirs,
                layout=layout,
            ):
                continue

            filtered_positions.add(pos)
            if pos in active_dirs:
                filtered_dirs[pos] = active_dirs[pos]

        return self._apply_global_horizontal_suppression(
            filtered_positions=filtered_positions,
            filtered_dirs=filtered_dirs,
        )


    def _apply_top_run_breakup_policy(
        self,
        pos: tuple[int, int],
        active_dirs: dict[tuple[int, int], tuple[int, int]],
        filtered_positions: set[tuple[int, int]],
        filtered_dirs: dict[tuple[int, int], tuple[int, int]],
        layout: SceneLayout,
    ) -> bool:
        # Phase 3: Spatial Shaping
        # Applies the upper-canopy breakup policy for a single candidate host.
        # Returns True when the position has been fully handled by the breakup
        # logic (either removed or replaced with droop positions), and False when
        # the caller should continue with normal retention.
        #
        # WHY this helper exists:
        # The top-run breakup logic is a distinct readability policy with several
        # severity bands. Extracting it makes that policy explicit and keeps the
        # parent readability pass focused on orchestration.
        x, y = pos
        left = (x - 1, y) in self.state.stems
        right = (x + 1, y) in self.state.stems
        up = (x, y - 1) in self.state.stems
        down = (x, y + 1) in self.state.stems

        top_ceiling = self._sample_top_run_breakup_ceiling(pos=pos, layout=layout)
        if not (y <= top_ceiling and left and right and not (up or down)):
            return False

        h = ((x * 92821) + (y * 68917) + 701) % 100

        # 1) very aggressive breakage (kill most straight continuity)
        if h < 55:
            return True

        # 2) force vertical irregularity (strong sag bias)
        if 55 <= h < 85:
            # create vertical droop clusters instead of flat spans
            if (x, y + 1) in layout.allowed_cells:
                filtered_positions.add((x, y + 1))
                if pos in active_dirs:
                    filtered_dirs[(x, y + 1)] = active_dirs[pos]

            # occasionally extend droop further
            if ((x + y) % 2 == 0) and (x, y + 2) in layout.allowed_cells:
                filtered_positions.add((x, y + 2))

            return True

        # 3) uneven clustering (break uniform spacing)
        if 85 <= h < 90:
            return ((x * 13 + y * 19) % 4) != 0

        # 4) rare survivors only
        if h >= 90:
            return ((x + y) % 3) != 0

        return False

    def _apply_global_horizontal_suppression(
        self,
        filtered_positions: set[tuple[int, int]],
        filtered_dirs: dict[tuple[int, int], tuple[int, int]],
    ) -> tuple[set[tuple[int, int]], dict[tuple[int, int], tuple[int, int]]]:
        # Phase 3: Spatial Shaping
        # Applies the final global safety-net suppression against long horizontal
        # continuity. This is intentionally broader and simpler than the top-run
        # breakup policy: it exists to catch any remaining rail-like runs after the
        # more targeted readability rules have already executed.
        #
        # WHY this helper exists:
        # Global suppression is a separate readability policy from top-run breakup.
        # Naming it explicitly makes the two suppression layers easier to reason
        # about and tune independently later.
        broken_positions: set[tuple[int, int]] = set()
        broken_dirs: dict[tuple[int, int], tuple[int, int]] = {}

        for (bx, by) in filtered_positions:
            left = (bx - 1, by) in self.state.stems
            right = (bx + 1, by) in self.state.stems
            up = (bx, by - 1) in self.state.stems
            down = (bx, by + 1) in self.state.stems

            if left and right and not (up or down):
                if ((bx * 97 + by * 131) % 100) < 70:
                    continue

            broken_positions.add((bx, by))
            if (bx, by) in filtered_dirs:
                broken_dirs[(bx, by)] = filtered_dirs[(bx, by)]

        return broken_positions, broken_dirs

    def _should_thin_for_base_readability(
        self,
        pos: tuple[int, int],
        layout: SceneLayout,
    ) -> bool:
        # Phase 3: Spatial Shaping
        # Samples the lower-right readability field that protects woody structure
        # near the trunk base from being swallowed by dense foliage.
        #
        # WHY this helper exists:
        # This makes the base-readability suppression policy explicit instead of
        # burying it inside a broader filtering loop.
        x, y = pos
        info_zone = layout.info_guide
        if x <= info_zone.x or y <= info_zone.bottom + 2:
            return False
        return ((x * 92821) + (y * 68917) + 615) % 100 < 28

    def _sample_top_run_breakup_ceiling(
        self,
        pos: tuple[int, int],
        layout: SceneLayout,
    ) -> int:
        # Phase 3: Spatial Shaping
        # Samples the soft ceiling used by the top-run breakup policy. This keeps
        # the upper canopy breakup boundary slightly irregular rather than perfectly
        # flat, which helps the scene avoid a rail-like top edge.
        #
        # WHY this helper exists:
        # The breakup ceiling is part of the spatial shaping field, not just a raw
        # arithmetic trick. Naming it makes that policy legible.
        x, y = pos
        return layout.hero_guide.y - 1 + (((x * 17) + (y * 7)) % 3)

    def _spawn_top_left_hanging_stems(self, layout: SceneLayout) -> None:
        # Phase 3: Spatial Shaping / Prototype Behavior
        # Injects the current top-left hanging-stem experiment back into the live
        # tip pool on a slow cadence. This is intentionally kept as an explicit,
        # named special-case behavior rather than hidden inline inside `tick()`.
        #
        # WHY this helper exists:
        # The hanging stems are not generic engine law. Extracting them makes it
        # obvious that this is a prototype art-direction feature layered on top of
        # the main canopy system.
        if self.state.frame % 6 != 0:
            return

        spawn_band_y = max(1, layout.hero_guide.y - 6)

        for sx in range(2, layout.hero_guide.x - 4, 6):
            if ((sx * 31 + self.state.frame * 7) % 100) < 18:
                if (sx, spawn_band_y) in layout.allowed_cells:
                    self.state.tips.append(
                        GrowthTip(
                            x=sx,
                            y=spawn_band_y,
                            dx=0,
                            dy=1,  # hanging downward
                            life=float(self.config.get("branch_life_max", 18)),
                            max_life=float(self.config.get("branch_life_max", 18)),
                            is_trunk=False,
                        )
                    )

    def _run_ornament_reconstruction_phase(self, layout: SceneLayout) -> None:
        # Phase 4: Ornament Reconstruction
        # Converts final structural + foliage-host state into renderable ornament
        # stamps and refreshes derived debug statistics.
        #
        # WHY this helper exists:
        # Ornament reconstruction is the final downstream phase. Extracting it
        # keeps rendering work clearly separated from structural growth and canopy
        # shaping while preserving the existing rebuild order.
        rebuild_leaf_stamps(self.state, self.config, layout, self.rng)
        rebuild_flower_stamps(self.state, self.config, layout, self.rng)
        rebuild_thickened_wood(self.state, self.config, layout, self.rng)
        trim_ornaments(self.state, int(self.config["max_ornament_cells"]), self.rng)
        update_debug_stats(self.state, layout)