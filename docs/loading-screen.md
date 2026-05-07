# Loading Screen

## Purpose

This note records the current YAM boot/loading-screen implementation, the visual direction we tried, what worked, and what kept fighting us. It exists so future work can resume from a clear architectural baseline instead of re-learning the same lessons through ad hoc tuning.

## Current Intent

The loading screen is not meant to be a modal popup over an already-live main scene. It is meant to be a short boot ceremony with its own empty scene underneath:

- boot starts from an empty `WorldKind::Boot`
- the loading art is rendered transparently over that empty scene
- normal startup should begin in a clean non-dev state
- the boot sequence should feel intentional, not merely technical

## Current Visual States

The boot/loading presentation now has three distinct visible states:

### 1. Active Loading

The active loading state is composed from:

- a small caption: `a very special`
- a central ASCII YAM wordmark
- a version/build line
- `loading...`
- a progress bar using filled `■` cells and unfilled `⋄` cells

The bar and text are no longer hard-coded one-off literals. The caption, wordmark, version/build line, loading label, and bar are positioned as one centered assembly with a few small deliberate per-line offsets for visual tuning.

### 2. Await Start

Once the bar phase completes, the loading label and bar are replaced with:

- two empty rows beneath the version/build line
- a centered `press [space] to continue` prompt

This state is intentionally quiet. It is meant to communicate that boot work is complete and the app is now waiting on the user, not hung.

### 3. Dissolve + Hold

When `[space]` is pressed:

- the visible `press [space] to continue` state is what dissolves away
- after the dissolve, the boot world holds for `0.5s` as a genuinely empty screen
- only after that hold does YAM hand off into the first real world

This continuity matters. Dissolving the old `loading...` composition instead of the prompt was a real regression we hit and then corrected.

## Current Runtime Sequence

Boot currently follows this staged path:

1. frame `0`: empty/near-empty boot world
2. `tachyonfx` coalesce for `1s`
3. loading-bar animation for `3s`
4. silent wait for `[space]`
5. dissolve the visible `press [space] to continue` state for `1s`
6. empty-screen hold for `0.5s`
7. handoff into the first real world

This richer sequence is for startup boot only. World switching keeps a shorter transition path and should not inherit the same blocking ceremony by default.

## Current Implementation

Relevant code paths:

- [src/core/world.rs](/Users/mcq/_git/yam/src/core/world.rs): `WorldKind::Boot` and empty boot-world construction
- [src/ui/state.rs](/Users/mcq/_git/yam/src/ui/state.rs): staged loading state machine and boot phases
- [src/runtime.rs](/Users/mcq/_git/yam/src/runtime.rs): boot sequencing, input gating, and `tachyonfx` effect application
- [src/scene/mod.rs](/Users/mcq/_git/yam/src/scene/mod.rs): scene-level suppression of non-loading layers while loading is active
- [src/scene/layers/loading_layer.rs](/Users/mcq/_git/yam/src/scene/layers/loading_layer.rs): transparent loading-art rendering
- [src/scene/layers/status_layer.rs](/Users/mcq/_git/yam/src/scene/layers/status_layer.rs): minimal footer during loading
- [src/render/fonts.rs](/Users/mcq/_git/yam/src/render/fonts.rs): bundled FIGlet registry and font selection
- [src/render/figlet.rs](/Users/mcq/_git/yam/src/render/figlet.rs): shared FIGlet-to-lines rendering helper

Important structural decisions that are now settled:

- the boot wordmark is now generated from the shared FIGlet subsystem rather than maintained as a one-off ASCII literal
- loading glyphs use foreground-only styling with no background fill
- scene composition hides unrelated layers while loading is active
- the footer keeps only minimal boot-time controls such as `[q]uit` and `[d]ev`, and those controls appear only once the app reaches the wait-for-space phase
- the footer version stamp is suppressed during loading to avoid duplicate version labeling

## What Worked Well

These choices proved valuable and should be preserved:

- separating boot loading from ordinary world-switch loading
- introducing a dedicated boot world instead of showing a half-alive main scene
- suppressing dev/debug surfaces at scene-composition level while loading is active
- using `tachyonfx` as a presentation layer on top of a regular ratatui scene instead of inventing a one-off animation path
- keeping the loading layer transparent instead of boxing it into a modal shell
- promoting FIGlet rendering into a real shared subsystem instead of treating the loading title as a one-off manual asset

In other words, the architecture is healthy even though the art itself is not yet visually final.

## What Did Not Go Smoothly

The weak point was not the staged boot logic. The weak point was the wordmark art and its exact terminal presentation.

Problems we hit:

- manually tuned ASCII lines looked correct in isolated text files but still shifted or degraded once rendered in-YAM
- Rust string-literal escapes made early manual art tuning unnecessarily error-prone
- per-line centering and later fixed-block alignment both improved some aspects while still leaving subtle horizontal/shape errors
- the lower contour of the wordmark remained especially fragile, with screenshot-visible glyph loss or apparent shortening
- repeated micro-tweaks to the art literal produced diminishing returns and poor confidence
- the wait-for-space prompt color drift is currently implemented in code but not clearly perceptible in the live terminal output, so it should be treated as unresolved rather than complete
- the dissolve handoff originally regressed to dissolving the older `loading...` state instead of the final `press [space] to continue` state, which broke continuity between the last two boot phases
- the post-dissolve empty hold originally existed only in the state machine; until it was fixed, stale loading content could still leak into that half-second and make the hold look fake
- the active/loading state and the wait-for-space state have different line counts by nature, so keeping the assembly visually stable required explicit spacer rows instead of relying on “natural” vertical centering

The screenshot that triggered this pause confirms that the current wordmark still glitches visually enough that continued manual pixel-pushing is no longer a good use of energy.

## Current Decision

Manual art tweaking is paused.

The prompt-color animation is also paused as an aesthetic problem, not a structural one:

- the code currently attempts a smooth white-to-green foreground shift
- the live terminal output does not show that shift clearly enough to count as working
- future work should re-evaluate this with stronger contrast, direct Tachyonfx foreground effects, or terminal/backend-aware color testing

The wordmark/art problem is paused for similar reasons:

- the architecture now supports plain-text assets, FIGlet-backed rendering, and per-line placement
- that is enough infrastructure for future refinement
- continuing to hand-nudge characters without a stronger offline inspection workflow is unlikely to produce trustworthy gains

That does **not** mean the loading-screen work failed. It means we should treat the current implementation as:

- architecturally successful
- visually promising
- artistically unresolved

The right next step is not more blind character nudging inside source. The right next step is to return later with a more systematic text/figlet workflow.

## Lessons Learned

1. A boot screen is best treated as its own UI state and world mode, not as a popup.
2. Transparent terminal art is much easier to live with than panel-backed loading UI for YAM’s current aesthetic direction.
3. If text art matters visually, it should be owned by a rendering subsystem, not by scattered hand-tuned literals.
4. The difference between “text file looks right” and “rendered terminal frame looks right” is real and should be expected.
5. Footer timing and dissolve continuity matter more than they first seem; users notice when the wrong state dissolves or when controls appear too early.
6. Once tweaks become one-character visual archaeology, stop and document.

## Future Directions

When we come back to this, likely better approaches include:

- using a more systematic figlet-generation pipeline instead of manual literal surgery
- building a small offline inspection tool for art width, line length, and anchor alignment
- exploring a proper font/figlet integration path rather than treating the wordmark as a one-off literal forever
- studying terminal-art projects that already handle figlets and animation cleanly

One relevant reference already noted during this pass:

- `sigye`: <https://github.com/am2rican5/sigye>

`sigye` looks useful as a reference for:

- figlet handling
- clock/text layout discipline
- restrained terminal animation patterns

It is a reference, not a dependency decision.

## Practical Resume Checklist

If future work resumes here, start with this order:

1. keep the current staged boot sequence intact
2. keep the current FIGlet-backed loading title path intact
3. keep the empty boot-world hold and prompt-dissolve continuity intact
4. inspect the rendered output with an offline helper or a more systematic figlet workflow if the shape still feels wrong
5. treat the prompt color drift as a separate problem from layout/art and solve it with deliberate contrast testing rather than bundled “visual polish”
6. only then retune in-YAM placement
7. add color/Tachyonfx polish after the static art is trustworthy

## Current Status

As of this note:

- the boot/loading architecture is good enough to keep
- the staged sequence is good enough to keep
- the footer/loading interaction is good enough to keep
- the final boot continuity is now correct: the visible `press [space] to continue` state is what dissolves away on start
- the final `0.5s` hold is now a truly empty boot screen rather than stale loading content
- the prompt color-shift idea exists but is still visually broken/unproven
- the ASCII wordmark is still not final
- world-switch loading still shares the same visual family but should remain much lighter than startup boot

That is a perfectly respectable stopping point.
