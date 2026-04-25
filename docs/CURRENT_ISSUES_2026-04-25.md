# Current Issues Report

Date: 2026-04-25

This report captures the current problems visible in the screenshots and the contract gaps they expose.

## Contract Snapshot

This is the current intended split, written as a contract instead of a preference:

- static world datum: `(0, 0)`
- world-ui: attached to world entities and allowed to move only when the world attachment changes
- hud-ui: attached to the terminal frame and allowed to move only when the viewport changes
- camera: the world-space origin of the visible crop
- viewport: the terminal-sized crop rectangle
- resize: may change the viewport, but must not change the world datum or the world-ui attachment rules
- arrow-key camera motion: may change the visible crop in windowed mode, but must not reclassify world-ui as hud-ui or the reverse
- fullscreen lock: when terminal size matches the world crop, the visible frame should be static and centered on the world datum
- windowed pan: when terminal size is smaller than the world crop, the viewport may move within the world bounds, but only as a crop change
- fullscreen lock is now enforced by a render-state rule: the stored camera may still change, but the visible crop is recentered when the terminal covers the world extent

What belongs where:

- world-ui
  - hero
  - clock
  - border probe
  - any world-attached effect or label
- hud-ui
  - debug telemetry
  - footer/status bar
  - version/build label
  - any terminal-fixed overlay

What must not drift:

- hero must stay world-pinned
- clock must stay hero-relative in world space
- border probe must stay a world-border indicator
- footer must stay hud-attached
- debug text must stay hud-attached
- camera must never become a second meaning for viewport
- fullscreen must never expose camera panning as visible motion when the crop already equals the world

## 1. Camera moves, but the view does not always change

Observed:
- debug shows `Camera` changing across frames
- some screenshots still show the world composition staying effectively fixed or only partially responding

Likely causes:
- the active projection contract is still being interpreted differently across layers
- some layers use camera projection, while others remain screen-stable
- fullscreen and windowed mode are still treated with different assumptions
- the fullscreen case still allows camera mutation even when the crop should be locked to the world datum

Impact:
- arrow-key panning feels inconsistent
- it is unclear which elements are supposed to move with camera and which are not

## 2. Static vs dynamic semantics are still overloaded

We currently have three distinct semantics, but the code and docs still drift between them:

- world-ui
  - should be attached to world entities
  - should move with the world/camera contract
- hud-ui
  - should be attached to the terminal frame
  - should stay screen-stable
- border/probe elements
  - should indicate world borders
  - should make the world bounds readable

The issue is not that these categories exist.
The issue is that they have not stayed stable long enough, so the active meaning keeps changing during implementation.

## 3. The ASCII border probe is still the most ambiguous feature

Observed:
- the border is visually present as a frame-like indicator
- it has intentional padding rows/cells
- it is being used as a debug indicator of world borders

Unclear / unstable:
- whether it is meant to be screen-fixed or world-projected
- whether it is a HUD probe, a world-space probe, or a hybrid
- whether arrow-key camera motion should move it or not

Contract problem:
- the border probe is currently the main source of confusion about static vs dynamic behavior

## 4. Hero and clock semantics are still easy to mix up

Current intent:
- hero is world-pinned
- clock is world-pinned relative to hero
- clock has its own hero-relative offset

Current risk:
- hero, clock, and camera have all been moved between screen, world, and anchor-space interpretations multiple times
- debug values and visible positions can diverge if one layer reprojects and another does not

What must stay fixed:
- hero should not become a HUD object
- clock should not become a pure screen overlay
- camera should not silently become the attachment point for either one

## 5. Viewport and camera are still conceptually close enough to confuse

Current state:
- camera = world-space origin of visible crop
- viewport = terminal-sized crop rectangle

Observed problem:
- these terms are still easy to conflate in the code and in prose
- when terminal size changes, it is not always obvious which object should change and which should remain stable

Need:
- a tighter, single-sentence contract for each term
- a hard rule about which layers may read camera and which may read viewport
- a fullscreen rule that freezes the crop at the centered world extent

## 6. Resize invariance still needs explicit enforcement

What should be invariant:
- world datum
- world-pinned hero/clock relationship
- footer HUD placement
- intentional border padding

What should change:
- viewport crop bounds
- visible world subset
- screen-space debug layout

Risk:
- resize behavior is still one of the easiest ways to reintroduce drift

## 7. World-space and screen-space are both still present, but the boundaries are not hard enough

We now have:
- world space
- world-ui
- hud-ui
- screen space

The issue:
- the semantics are correct in prose, but not yet hard enough in code
- there are still places where a feature can be reclassified by implementation accident

## 8. The border padding rules are intentional, but should be treated as first-class contract

Current accepted padding:
- top padding row: intentional
- one side padding cell: intentional
- bottom one-row padding: currently used by the footer

Issue:
- these are still easy to mistake for accidental off-by-one artifacts
- they should be named and preserved as deliberate layout reservations

## 9. The debug overlay is useful, but it can still mask contract problems

Observed:
- debug text gives camera, anchor, and offset values
- the screen can still look different from the mental model

Risk:
- debug telemetry is only useful if the terms it prints are stable
- if the underlying semantics change, the overlay becomes misleading rather than clarifying

## 10. Current top-level problem statement

The repo is still trying to answer these questions consistently:

1. What is static?
2. What is dynamic?
3. What belongs to world-ui?
4. What belongs to hud-ui?
5. What is camera?
6. What is viewport?
7. Which features should move with arrow keys?
8. Which features should remain pinned to the world datum?
9. When should the visible crop be movable?
10. When should the visible crop be static?

Until those questions stay fixed, the screenshots will keep showing visually correct pieces that still feel semantically inconsistent.

## 11. Immediate Interpretation Of The Latest Screenshots

The latest screenshots are not showing a missing render pass.
They are showing a contract split:

- the debug overlay/footer are behaving like hud-ui, which is correct
- the hero/clock/border probe are behaving like world-ui, which is also correct
- the perceived bug is that the contract is still too easy to forget when reading the screenshot
- fullscreen is still effectively pannable, so the visible crop can drift even when terminal size equals world size

The practical issue is therefore semantic, not mechanical:

- some features are static in hud space
- some features are dynamic in world space
- the repo still needs a stronger naming and test boundary so those spaces are not reinterpreted later

## Recommended next step

Do not add new rendering features yet.

First:
- freeze the contract definitions
- assign each feature to exactly one space
- write one test per feature class:
  - world-pinned
  - world-projected
  - HUD-attached
  - resize-invariant

Only after that should the render behavior be extended further.
