# Visualizer Config Manual

This document is the field manual for [visualizer.json](/Users/maciejkuster/yam/visualizer/config/visualizer.json).

It describes each setting by purpose, expected effect, and common tradeoffs.

The visualizer is still under active development, so some settings are more stable than others. When in doubt:

- `chafa` and `timing` are straightforward
- `layout` is powerful but can create accidental collisions
- `ivy` is the most experimental section

## System Model (Conceptual)

The visualizer is composed of layered systems:

- input + frame pipeline (GIF → Chafa → frames)
- layout (placement + collision mask)
- growth engine (behavior + routing)
- rendering (ornament + glyphs)
- debug layer (validation tools)

Most config sections map to one of these layers.

## `chafa`

Controls the hero animation source and how Chafa converts it into terminal symbols.

### `source_gif`

Path to the preferred source GIF.

Effect:
- used as the primary hero animation source
- if missing, the fallback image path is used to synthesize a GIF
- paths are resolved relative to the visualizer repo root unless already absolute

Tradeoff:
- higher quality source material matters more than any downstream tuning

### `fallback_image`

Path to a still image used when `source_gif` is unavailable.

Effect:
- keeps the visualizer runnable even without the intended GIF
- also resolved relative to the visualizer repo root unless already absolute

### `cache_dir_raw`

Directory for extracted raw image frames.

Effect:
- stores decoded source frames before Chafa conversion
- paths are resolved relative to the visualizer repo root unless already absolute

### `cache_dir_chafa`

Directory for cached ANSI/symbol-rendered Chafa frames.

Effect:
- avoids rerendering the hero frames every run
- paths are resolved relative to the visualizer repo root unless already absolute

### `frame_count`

Target number of hero frames kept in the loop.

Effect:
- fewer frames: calmer loop, less motion detail
- more frames: smoother motion, more visual activity

### `width`

Hero frame width in terminal cells.

Effect:
- directly determines hero footprint
- also changes layout pressure on ivy and panel

### `height`

Hero frame height in terminal rows.

Effect:
- one of the strongest composition controls
- smaller height opens more crawl space for ivy

Current baseline:
- the active hero footprint is `72x36`
- if you change `height`, recheck mask fit and trunk route behavior immediately

Current route model:
- trunk routing is phase-based rather than purely scalar-biased
- the current intended sequence is:
  - `approach`
  - `hero_top`
  - `hero_exit`
  - `post_top`

### `align`

Chafa alignment string.

Effect:
- determines how the source content is aligned inside the Chafa frame box

### `symbols`

Chafa symbol set, currently `braille`.

Effect:
- affects texture density and perceived resolution

### `fill`

Background fill behavior inside Chafa.

Effect:
- `none` keeps the hero airy and avoids a boxed look

### `colors`

Chafa color mode.

Effect:
- `full` preserves the richest color information

### `color_space`

Color interpretation for Chafa conversion.

Effect:
- controls how color distances are evaluated during terminal conversion

### `color_extractor`

How Chafa samples source colors.

Effect:
- influences local color stability from frame to frame

### `fg_only`

Whether Chafa emits foreground-only color.

Effect:
- `true` avoids heavy block backgrounds and keeps the hero more transparent in scene composition

### `bg`

Background color supplied to Chafa.

Effect:
- matters mainly when conversion decisions need a background assumption

### `threshold`

Transparency / preprocessing threshold input to Chafa.

Effect:
- higher values usually reduce fringe noise
- too high can erase subtle details

### `preprocess`

Enables Chafa preprocessing.

Effect:
- usually improves the stability of keyed or difficult source material

### `dither`

Dithering mode.

Effect:
- `none` keeps the hero calmer and less noisy

Note:
- Chafa output is cached and may pass through additional preprocessing (e.g. dithering prepass)
- changes to source or parameters may require cache invalidation to take effect

## `timing`

Controls scene cadence.

### `render_fps`

How often the terminal scene is redrawn.

Effect:
- higher values make motion feel smoother
- lower values make debugging frame relationships easier

Tradeoff:
- too high can make terminal redraw artifacts more visible

### `hero_fps`

How often the hero frame advances.

Effect:
- independent from `render_fps`
- lower values make the hero feel calm and intentional

### `ivy_tick_seconds`

How often the ivy engine advances.

Effect:
- smaller values make ivy growth faster and easier to inspect during development
- larger values make ivy feel ambient instead of aggressive

### `info_refresh_seconds`

How often the panel is regenerated.

Effect:
- usually only needs to be `1.0`

## `layout`

Controls scene composition and collision geometry.

There are two different ideas in this section:

- placement:
  - where the hero and panel are drawn
- collision:
  - what area ivy is forbidden from entering

Those are related but not identical.

Hero collision uses the mask as the canonical geometry:

- if `hero_mask_path` exists, the silhouette mask defines blocked hero cells (source of truth)
- `hero_safe_pad_*` expands that blocked silhouette
- `hero_collision_trim_*` exists only as fallback behavior when the mask is unavailable
- growth guidance may reference the visible hero block, but must never override mask-based collision

This model ensures that all growth behavior respects the actual visible silhouette of the hero.

Current working rule:
- treat the hero mask as the real temporary border of vine sprawl
- use trim values only for readability and fallback behavior, not as the primary silhouette tool

### `min_terminal_columns`

Minimum supported terminal width before the warning line appears.

### `min_terminal_rows`

Minimum supported terminal height before the warning line appears.

### `outer_margin_x`

Base horizontal margin used by layout anchors.

Effect:
- larger values pull all content inward

### `outer_margin_y`

Base vertical margin used by layout anchors.

Effect:
- larger values move the whole composition lower

### `hero_anchor`

Horizontal anchor for the hero block.

Expected values:
- `left`
- `center`
- `right`

### `hero_offset_x`

Signed horizontal adjustment applied after hero anchoring.

Effect:
- negative values move the hero left
- positive values move it right

### `hero_offset_y`

Signed vertical adjustment applied after base margin placement.

Effect:
- one of the most visible composition controls
- increasing it moves the hero lower and often opens headroom above

### `hero_mask_path`

Path to the black/white hero silhouette mask.

Effect:
- white pixels are treated as blocked hero shape
- black pixels remain passable
- should match the current hero frame aspect and visual silhouette as closely as possible

Current baseline:
- the current repo treats the cleaned hero mask asset as the temporary source of truth
- both the GIF and the mask may still be replaced later

### `hero_mask_scaled_fallback_path`

Optional path to a pre-scaled mask that already matches the terminal-cell hero footprint.

Effect:
- used only if the primary mask rasterizes into a degenerate near-full-frame footprint
- useful while iterating on mask assets or when the source mask needs manual terminal-grid compensation
- should generally point to a mask already sized to the configured hero `width` and `height`

Practical note:
- this is a safety net, not the preferred asset path

### `hero_mask_threshold`

Brightness threshold used to interpret the hero mask.

Effect:
- higher values block only brighter/whiter mask pixels
- lower values block more aggressively
- useful when the mask contains antialiasing or soft edges

### `hero_mask_scale_x`

Horizontal compensation applied before the primary hero mask is rasterized onto the terminal grid.

Effect:
- values below `1.0` compress the mask horizontally before per-cell sampling
- useful when the source mask was authored in pixel space but the rendered hero is displayed on terminal cells with different aspect behavior

Current baseline:
- this is still a legitimate correction knob
- but the current workflow assumes a manually cleaned mask first, then only modest scaling adjustment

### `hero_mask_scale_y`

Vertical compensation applied before the primary hero mask is rasterized onto the terminal grid.

Effect:
- values below `1.0` compress the mask vertically before per-cell sampling
- usually needs smaller adjustment than `hero_mask_scale_x`

Current baseline:
- vertical correction should stay conservative unless the source mask changes materially

### `hero_mask_alignment_margin`

Number of terminal cells to keep clear between the visible hero frame edge and the raw rasterized mask.

Effect:
- trims the rasterized mask inward on all four sides after sampling
- useful when the mask is visually correct but still lands slightly too close to the Chafa frame border
- good default for this project is `1`

Intent:
- this is alignment tolerance, not shape design
- if a large margin seems necessary, the mask asset itself is probably wrong

### `hero_collision_trim_left`

How much of the left side of the hero frame is excluded from the collision mask before padding.

Effect:
- primarily fallback/secondary when mask-based collision is active
- larger value means ivy can legally come closer to the left visual edge of the hero

### `hero_collision_trim_top`

Top trim of the hero collision mask.

Effect:
- primarily fallback/secondary when mask-based collision is active
- larger value means less protected space above the visible hero

### `hero_collision_trim_right`

Right trim of the hero collision mask.

Effect:
- primarily fallback/secondary when mask-based collision is active
- useful if the Chafa frame has visually empty right-side area

### `hero_collision_trim_bottom`

Bottom trim of the hero collision mask.

Effect:
- primarily fallback/secondary when mask-based collision is active
- useful when the frame box extends lower than the meaningful hero pixels

### `hero_safe_pad_x`

Extra horizontal safety padding added after collision trimming.

Effect:
- larger values keep ivy farther from visible hero pixels

### `hero_safe_pad_y`

Extra vertical safety padding added after collision trimming.

Effect:
- larger values reduce crowding above and below the hero

### `info_width`

Panel width in terminal cells.

### `info_height`

Panel height in rows.

### `info_gap`

Minimum horizontal separation between the hero block and the info panel block.

Effect:
- larger values make the composition breathe more
- smaller values tighten the scene

### `info_offset_x`

Signed horizontal offset applied to the right-anchored panel position.

Effect:
- negative values move the panel left toward the hero

### `info_offset_y`

Signed vertical offset for the info panel.

Effect:
- positive values move the panel lower

### `info_collision_trim_left`

Left trim of the panel collision mask before padding.

Use carefully:
- if too large, ivy can appear to push panel text

### `info_collision_trim_top`

Top trim of the panel collision mask.

### `info_collision_trim_right`

Right trim of the panel collision mask.

### `info_collision_trim_bottom`

Bottom trim of the panel collision mask.

### `info_safe_pad_x`

Extra horizontal panel protection after trims.

Effect:
- larger values are the main tool for preventing ivy from visually pushing text

### `info_safe_pad_y`

Extra vertical panel protection after trims.

Effect:
- useful when vines visually crowd the title or timezone rows

## `ivy`

Controls the procedural organism.

This section is the least stable and should be treated as experimental.

Important:
- density, clustering, and overall plant shape must be controlled by engine parameters
- ornament/rendering should not be used to compensate for weak growth behavior

Conceptually:
- many ivy parameters act as spatial “forces” influencing movement decisions
- trunk routing is now also phase/state-driven, not just weight-driven

Current trunk route sequence:
- `approach`
  - from the lower-right origin toward the hero
- `hero_top`
  - commit to the upper hero band and traverse left
- `hero_exit`
  - leave the top band deliberately, biased left and down-left away from the panel corridor
- `post_top`
  - freer downstream behavior after clearing the hero-top segment

### `max_tips`

Maximum number of live growth tips at once.

Effect:
- lower values keep the organism legible
- higher values increase branching and clutter

### `max_structural_cells`

Cap for stem cells.

Effect:
- higher values allow broader sprawl
- too high can fill the scene aggressively

### `max_ornament_cells`

Cap for non-structural ornament cells.

Includes:
- leaf stamps
- death clusters
- thickened wood fillers

### `trunk_life`

Starting lifespan of the main trunk.

Effect:
- higher values allow longer travel before the organism settles

### `trunk_decay`

Life consumed per tick by trunk tips.

Effect:
- higher values shorten trunk travel
- lower values make trunk movement more persistent

### `branch_life_min`

Minimum lifespan for new branches.

### `branch_life_max`

Maximum lifespan for new branches.

Effect:
- together these define how long side growth can persist

### `branch_decay`

Life consumed per tick by branch tips.

Effect:
- larger values make branches die faster into ornament

### `branch_chance`

Probability that a trunk move spawns a branch.

Effect:
- one of the main density controls

### `forward_bonus`

Reward for continuing in the current direction.

Effect:
- high values create rails and long straight segments
- low values create looser motion

### `turn_penalty`

Penalty for changing direction.

Effect:
- higher values suppress jitter but can make growth too rigid

### `organic_variation`

Small random noise added to move scoring.

Effect:
- higher values make growth less predictable

### `trunk_seed_offset_x`

How far right of the hero collision box the initial seed target sits.

Effect:
- larger values push the origin farther into the lower-right field

### `trunk_seed_bottom_margin`

Bottom margin used when selecting the initial trunk seed.

Effect:
- smaller values start growth closer to the terminal bottom edge

### `support_band_above`

Distance above the hero collision box used when the engine can form an upper support band.

### `support_band_height`

Height of the support band.

Effect:
- larger values create a broader support traversal zone

### `support_min_headroom`

Minimum space required above the hero before the engine uses the upper support band.

Effect:
- avoids forcing a top-edge rail when there is not enough real headroom

### `support_span_left`

How far the support band extends to the left of the hero collision box.

### `support_span_right`

How far the support band extends to the right of the hero collision box.

Effect:
- these define how wide the wrap zone around the hero can be

### `trunk_climb_bonus`

Reward for upward trunk movement during the climb phase.

Effect:
- too high makes the organism over-prioritize ascent

### `trunk_diagonal_bonus`

Extra reward for upper-left diagonal trunk movement.

Effect:
- increases the “from lower-right toward upper-left” feel

### `trunk_reverse_penalty`

Penalty for moving downward during trunk ascent.

### `support_traverse_bonus`

Reward for leftward traversal in the support zone.

### `support_wrap_bonus`

Extra reward for leftward-downward wrap motion around obstacles.

Effect:
- useful for crawling around the hero/panel
- too high can trap the vine in local obstacle orbits

### `settle_down_bonus`

Reward for downward movement after the trunk has passed the support phase.

Effect:
- encourages spill and descent instead of endless climbing

### `branch_gravity_bonus`

Reward for downward branch movement.

Effect:
- larger values create hanging, falling side growth

### `hero_contour_attraction`

How strongly the organism is attracted to obstacle contours.

Effect:
- larger values make the vine hug hero/panel boundaries
- too high makes it feel pushy and crowded

### `top_edge_soft_limit`

Rows near the top edge where soft penalties begin.

Effect:
- helps suppress hard top-rail growth

### `top_edge_penalty`

Penalty applied to growth too close to the top edge.

Effect:
- larger values reduce top-edge accumulation

### `leaf_stamp_chance`

Chance of generating a decorative leaf stamp from an active leaf position.

Effect:
- high values create dense foliage quickly
- lower values keep the plant more skeletal

### `thickening_min_age`

Minimum stem age before thickened wood can appear.

Effect:
- larger values delay wood mass buildup

### `thickening_full_age`

Age where thickening becomes much more likely.

### `thickening_spread_chance`

Chance of adding thickened wood before the full-age threshold.

Effect:
- lower values keep thickening restrained

### `trunk_thickening_min_age`

Minimum age before main-trunk cells begin using the stronger trunk-thickening path.

Effect:
- lower values make the base and older scaffold bulk up earlier
- higher values keep the trunk visually lean for longer

### `trunk_thickening_bonus`

Extra maturity multiplier applied to trunk-lineage cells during thickening.

Effect:
- values above `1.0` make old trunk segments thicken faster and read as more woody than ordinary branches

### `trunk_thickening_info_margin`

Horizontal allowance beyond the info-panel guide where trunk-specific thickening is still considered part of the old main line.

Effect:
- lower values confine heavy trunk wood to the origin-to-panel corridor
- higher values let the thick trunk treatment continue farther right

### `trunk_thickening_core_bias`

Base bias for adding dense wood strokes directly adjacent to mature trunk cells.

Effect:
- higher values produce a heavier trunk spine
- too high can make the plant look boxed-in instead of woody

### `debug.enabled`

Enables debug accounting hooks.

Current use:
- failed move counts
- region coverage tracking
- spawn origin counts

Note:
- debug overlays are part of the core development workflow
- use them to validate mask alignment, layout geometry, and growth behavior before tuning parameters

### `debug.stem_only_view`

Renderer-side debug view that suppresses most ornament and shows the structural scaffold more clearly.

## `panel`

Controls the utility block on the right.

### `enabled`

Reserved toggle for panel display behavior.

Current note:
- the panel is effectively assumed on in the current composition

### `title`

Panel title string.

### `timezone`

Timezone mode for the info panel.

Current value:
- `local`

### `show_weather_placeholder`

Reserved switch for future weather-related UI.

Current note:
- weather is not implemented

## Practical Tuning Guidance

If the ivy feels too aggressive:

- lower `branch_chance`
- lower `leaf_stamp_chance`
- lower `hero_contour_attraction`
- lower `support_wrap_bonus`

If the ivy feels too stiff:

- lower `forward_bonus`
- lower `turn_penalty`
- increase `organic_variation`

If the ivy crowds the hero:

- increase `hero_safe_pad_x`
- increase `hero_safe_pad_y`
- reduce `hero_collision_trim_*` aggressiveness

If the ivy crowds the info panel:

- increase `info_safe_pad_x`
- increase `info_safe_pad_y`
- reduce `info_collision_trim_*`

If the top edge fills too much:

- increase `top_edge_penalty`
- increase `top_edge_soft_limit`
- reduce `support_traverse_bonus`

If the lower-right origin is too weak:

- increase `trunk_seed_offset_x`
- reduce `trunk_seed_bottom_margin`
- increase `trunk_diagonal_bonus`
