Now we’re talking—this pass is grounded in the actual docs, not just the TODO abstraction. I’ll be direct and surgical.

---

# 1. Overall Assessment

**Verdict: Architecturally coherent, but still “spec-incomplete” at critical execution seams.**

Across:
-  [oai_citation:0‡TODO.md](sediment://file_00000000711c71f4b8f2fe910bb91c79) (TODO)
-  [oai_citation:1‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88) (SCENE_MODEL)
-  [oai_citation:2‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f) (ARCHITECTURE)

…the system is **internally consistent** and aligned around:
- single projection path
- deterministic pipeline
- strict space separation
- renderer isolation

👉 That alignment is **real**, not superficial.

However:

> ⚠️ The backlog is still **too declarative where the architecture requires enforceable mechanics**

The gap is no longer conceptual—it’s **contract precision and execution guarantees**.

---

# 2. Key Strengths

## 🧱 2.1 Strong cross-doc alignment (rare, and you achieved it)

- TODO → references SCENE_MODEL + ARCHITECTURE as source of truth
- SCENE_MODEL → defines pipeline + systems + layering
- ARCHITECTURE → enforces module boundaries + render path

There are **no major contradictions** across these files.

That’s a big milestone.

---

## 🧭 2.2 The scene pipeline is consistent across all layers

All three docs converge on:

```
state → primitives → masks → projection → layers → composition → render
```

- SCENE_MODEL defines it explicitly  [oai_citation:3‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  
- TODO repeats it  [oai_citation:4‡TODO.md](sediment://file_00000000711c71f4b8f2fe910bb91c79)  
- ARCHITECTURE enforces it via `render_scene` + `LayerOutput`  [oai_citation:5‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  

👉 This is clean and stable.

---

## 🧩 2.3 World / HUD / Overlay split is consistently enforced

Across all docs:

- SCENE_MODEL → conceptual separation  
- ARCHITECTURE → implementation contract (`resolve_world_ui`, `resolve_hud_ui`)  
- TODO → backlog guardrails  

👉 No drift here. This is solid.

---

## 🔒 2.4 Anti-patterns + forbidden coupling are explicit

Between:
- SCENE_MODEL anti-patterns
- ARCHITECTURE forbidden coupling
- TODO contract debt

You’ve effectively built:
- a **negative spec (what must not happen)**

That’s extremely valuable for preventing regression.

---

## 🔁 2.5 Stabilization-first workflow is correctly enforced

The work order in TODO is **conceptually correct**:

```
UI → stabilize → hero → stabilize → vines → stabilize
```

And LOG confirms you already started enforcing this discipline  [oai_citation:6‡LOG.md](sediment://file_00000000ff4071f4a180012d0292a5f9)

👉 This is the right approach for a rendering system.

---

# 3. Risks / Inconsistencies

## ⚠️ 3.1 Projection is still not a formal contract (despite being central)

Across docs:

- TODO: “Keep projection defined in one place”
- SCENE_MODEL: “Projection is part of pipeline”
- ARCHITECTURE: “projection should live in one explicit place”

But:

> ❌ Nowhere is projection defined as a **function-level contract**

Missing:

- input definition
- output definition
- guarantees (purity, determinism, no masking, etc.)

👉 This is the **#1 architectural risk**

Right now:
- projection is a rule
- not a **mechanism that can be violated and detected**

---

## ⚠️ 3.2 Layering model vs z-index model is slightly divergent

- SCENE_MODEL: strict fixed order (Background → Scaffold → Ivy → Hero…)  [oai_citation:7‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  
- ARCHITECTURE: numeric z-index system  [oai_citation:8‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  

These are *compatible*, but:

> ⚠️ You now have **two representations of ordering**

Risk:
- future dev introduces dynamic z-index logic
- breaks SCENE_MODEL’s “no dynamic reordering”

👉 You need to unify:
- either “fixed enum order”
- or “z-index but frozen”

---

## ⚠️ 3.3 “RenderState” is critical but underspecified

ARCHITECTURE says:

- computed once
- split into `world` and `hud`
- read-only

But:

> ❌ No definition of:
- required fields
- ownership lifecycle
- invalidation rules

And TODO relies heavily on it.

👉 This is a hidden coupling hotspot.

---

## ⚠️ 3.4 Hero rendering is directionally correct but operationally undefined

TODO says:
- prefer cached frames
- reduce quantization drift
- maybe replace chafa

ARCHITECTURE shows:
- chafa path already exists
- partial fixes applied

But:

> ❌ No decision gate, no evaluation criteria, no fallback plan

👉 This is a sequencing risk before vines.

---

## ⚠️ 3.5 Greenhouse mode is concept-only

TODO:
- “separate world mode”
- “not tab UI”

But:

> ❌ No integration with:
- Scene
- RenderState
- projection
- camera semantics

👉 This is currently **non-implementable without interpretation**

---

## ⚠️ 3.6 “Scene stabilisation” is still too vague

Exit criteria (TODO):

- “resize does not change semantics”
- “camera does not produce multiple projection meanings”

These are good—but:

> ❌ Not measurable, not testable

---

# 4. Missing Items / Gaps

## 🧪 4.1 No formal invariant list (despite tests existing)

LOG shows:
- invariance tests already exist  [oai_citation:9‡LOG.md](sediment://file_00000000ff4071f4a180012d0292a5f9)  

But TODO does not define:

- what invariants must always hold

👉 You need a **canonical invariants section**

---

## 🧪 4.2 No explicit failure taxonomy in active backlog

Archive contains:
- bug categories
- projection failures
- drift cases  [oai_citation:10‡REFERENCE_ARCHIVE.md](sediment://file_00000000df647243acb0860c6a79e254)  

TODO only references them loosely.

👉 These should become **test targets**

---

## 🧪 4.3 No instrumentation requirements

SCENE_MODEL defines debug layer—but:

> ❌ No requirement to expose:
- projection outputs
- mask overlays
- layer boundaries

👉 Stabilisation without instrumentation is guesswork.

---

## 🧪 4.4 No definition of determinism scope

SCENE_MODEL says:
> “same input → same output”  [oai_citation:11‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

But missing:
- what counts as input?
- does resize count?
- does terminal size count?

---

## 🧪 4.5 No performance / caching expectations

Given:
- hero frames
- vines
- full-frame composition

👉 You need:
- frame cost expectations
- caching rules

---

# 5. Recommended Reorderings / Rewrites

## 🔧 5.1 Add a “Projection Contract” section (mandatory)

Add to TODO:

```md
### Projection Contract

Projection is a pure function:

(world_pos, camera, viewport) -> screen_pos

Rules:
- no side effects
- no masking
- no layering
- deterministic
- single implementation path

All world → screen conversion must pass through this function.
```

---

## 🔧 5.2 Unify layering model

Pick one:

### Option A (recommended):
```md
Layer order is fixed and mapped to static z-indices.
No dynamic z-index is allowed.
```

---

## 🔧 5.3 Rewrite stabilisation exit criteria (make them testable)

Replace vague wording with:

```md
Exit Criteria:

- identical frame buffer across N frames with no input changes
- world-space positions unchanged under resize
- projection output changes only due to viewport
- no sub-cell jitter across frames
- HUD remains pixel-stable under camera movement
```

---

## 🔧 5.4 Promote invariants to first-class section

Add:

```md
### Core Invariants

- world-space is resolution independent
- projection is singular
- HUD is camera-independent
- overlay is top-z and modal
- masks are applied before projection output is final
```

---

## 🔧 5.5 Add hero decision gate

```md
Hero Rendering Decision Gate:

Before vines:
- choose between:
  - chafa streaming
  - cached frame grid

Must evaluate:
- geometry stability
- color stability
- resize determinism

Decision required before proceeding.
```

---

## 🔧 5.6 Define greenhouse integration

```md
Greenhouse Mode Contract:

- uses same projection system
- does not introduce new layout model
- may swap world state
- camera semantics must remain identical
```

---

# 6. Suggested Test Plan

## 🧪 Core tests

- projection invariance (world → screen consistency)
- resize invariance
- anchor integrity (hero ↔ clock)
- frame determinism (buffer equality)
- layer ordering (no z drift)
- mask correctness (no “empty pixel” behavior)

---

## 🧪 Advanced

- frame diff testing
- jitter detection
- rapid resize stress
- camera pan stress

---

# 7. Archive Ideas to Promote

From  [oai_citation:12‡REFERENCE_ARCHIVE.md](sediment://file_00000000df647243acb0860c6a79e254):

## ✅ Promote to TODO

- canonical projection function
- bug taxonomy → convert into tests
- enforcement layer design
- spatial ruleset

## ❌ Keep archived

- experimental UI ideas
- alternative layouts
- early debugging logs

---

# 8. Best-Practice Notes

## 🧭 8.1 Separate rules from execution

Right now TODO mixes:
- architecture rules
- tasks

👉 Split into:
- `CONTRACTS.md` (rules)
- `TODO.md` (work)

---

## 🧭 8.2 Every phase must have:

- entry condition  
- exit condition  
- validation method  

You have 2/3—add validation.

---

## 🧭 8.3 Eliminate ambiguous language

Replace:
- “clean”
- “stable”
- “correct”

with measurable conditions.

---

## 🧭 8.4 Treat TODO as executable spec

If a task cannot be:
- tested
- validated
- falsified

→ it’s not ready.

---

# Final Take

You’ve crossed the hardest threshold:

> ✅ The system is **architecturally aligned across all documents**

But you’re now at the **danger zone**:

> ⚠️ Transition from *conceptual correctness* → *mechanical enforcement*

What’s missing:
- formal contracts (projection, RenderState)
- measurable stabilisation
- decision gates (hero path)
- explicit invariants

Once those are added, this backlog becomes:
- enforceable
- testable
- safe for iterative development

Right now it’s **very good—but still slightly too trusting of interpretation**


Here’s the **precise gap analysis**, distilled after actually aligning `TODO.md`, `SCENE_MODEL.md`, and `ARCHITECTURE.md`.

No fluff—just what is *missing* in terms of making this system executable and safe.

---

# 1. Missing: A **formal projection contract (the core hole)**

You reference projection everywhere:

- TODO → “Keep projection defined in one place”  [oai_citation:0‡TODO.md](sediment://file_00000000711c71f4b8f2fe910bb91c79)  
- SCENE_MODEL → projection is a pipeline stage  [oai_citation:1‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  
- ARCHITECTURE → “projection should live in one explicit place”  [oai_citation:2‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  

But what’s missing:

### ❌ No definition of:
- exact function signature
- allowed inputs
- allowed outputs
- forbidden responsibilities

### Why this matters
Right now projection is:
> a *rule everyone agrees on*  

But not:
> a *thing that can be violated and detected*

### What’s missing (concretely)
You need something like:

```rust
fn project(world_pos, camera, viewport) -> screen_pos
```

with guarantees:
- no masking
- no layering
- no state mutation
- deterministic

👉 Without this, “single projection path” is not enforceable.

---

# 2. Missing: A **canonical invariants layer**

You *implicitly* rely on invariants across all docs, but nowhere are they centralized.

Examples scattered across docs:

- world is resolution-independent  [oai_citation:3‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  
- HUD is screen-attached  [oai_citation:4‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  
- camera must not mutate world  [oai_citation:5‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

But:

> ❌ There is no single list of invariants that must always hold

### Why this matters
You already have tests (per LOG)  [oai_citation:6‡LOG.md](sediment://file_00000000ff4071f4a180012d0292a5f9)  
…but no canonical spec to validate against.

### What’s missing
A section like:

```md
Core invariants:

- world position of entities is independent of viewport
- projection is the only world→screen mapping
- HUD is camera-independent
- overlay always sits above HUD and world
- masks do not delete geometry
```

👉 Without this, tests are not grounded.

---

# 3. Missing: **Measurable stabilization criteria**

TODO defines stabilization like:

- “clean”
- “flicker-free”
- “layout-stable”  [oai_citation:7‡TODO.md](sediment://file_00000000711c71f4b8f2fe910bb91c79)  

And even exit criteria:

- “Resize does not change world attachment semantics”
- “Camera does not produce multiple projection meanings”

These are **good intentions—but not testable**.

### What’s missing

You don’t define:
- how to detect flicker
- how to verify layout stability
- how to prove determinism

### What it should be
You need measurable signals:

- frame buffer equality across frames
- coordinate equality checks
- jitter thresholds (0 cell movement allowed)

👉 Right now stabilization is **subjective**, not enforceable.

---

# 4. Missing: A **fully specified RenderState contract**

ARCHITECTURE defines:

- RenderState exists
- split into `world` and `hud`
- read-only per frame  [oai_citation:8‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  

But missing:

### ❌ No definition of:
- required fields
- who writes it
- when it is constructed
- immutability guarantees

### Why this matters
RenderState is:
> the *only shared truth across layers*

But currently:
> it’s structurally undefined

### Consequence
High risk of:
- hidden coupling
- recomputation drift
- inconsistent projection snapshots

---

# 5. Missing: A **layering model unification**

You currently have:

### SCENE_MODEL:
- fixed order list (Background → Scaffold → Ivy…)  [oai_citation:9‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

### ARCHITECTURE:
- numeric z-index system  [oai_citation:10‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  

These are compatible, but:

> ❌ Not formally tied together

### What’s missing
A rule like:

> “z-index is a static encoding of fixed layer order; no dynamic reordering allowed”

Otherwise:
- someone introduces dynamic z-index
- breaks determinism

---

# 6. Missing: **Hero rendering decision gate**

TODO correctly identifies:

- chafa instability
- need for cached frames  [oai_citation:11‡TODO.md](sediment://file_00000000711c71f4b8f2fe910bb91c79)  

But:

> ❌ No decision criteria  
> ❌ No evaluation plan  
> ❌ No “must decide before X” enforcement

### Why this matters
Hero sits at:
- center of projection
- center of masking
- center of layering

👉 If this is unstable, everything downstream (vines) collapses.

---

# 7. Missing: **Greenhouse mode integration contract**

You say:

- “separate world mode”
- “not tab UI”  [oai_citation:12‡TODO.md](sediment://file_00000000711c71f4b8f2fe910bb91c79)  

But:

> ❌ No connection to:
- Scene
- RenderState
- projection
- camera rules

### What’s missing
You need to answer:

- Does greenhouse reuse projection? (it should)
- Does it reuse camera? (it should)
- Is it a different WorldState or a variant?

👉 Right now it’s conceptual, not implementable.

---

# 8. Missing: **Failure taxonomy → active test mapping**

Archive contains:

- bug maps
- projection failures
- drift cases  [oai_citation:13‡REFERENCE_ARCHIVE.md](sediment://file_00000000df647243acb0860c6a79e254)  

TODO says:
> “keep bug taxonomy around”

But:

> ❌ Not promoted into actionable tests

### What’s missing
Explicit mapping:

| Failure | Test |
|--------|------|
| coordinate drift | resize invariance test |
| double projection | projection path assertion |
| mask-as-empty bug | occlusion test |

👉 You’re sitting on gold here but not operationalizing it.

---

# 9. Missing: **Instrumentation requirements**

SCENE_MODEL defines debug layer:

- bounding boxes
- masks
- grids  [oai_citation:14‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

But:

> ❌ No requirement that these must exist for stabilization

### Why this matters
Without instrumentation:
- you can’t verify projection correctness
- you can’t debug mask issues

👉 Stabilization without visibility = guesswork.

---

# 10. Missing: **Determinism scope definition**

SCENE_MODEL says:

> “same input → same output”  [oai_citation:15‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

But:

> ❌ What counts as input?

- terminal size?
- camera?
- time?
- animation frame?

### Why this matters
Without defining input:
- determinism is ambiguous
- tests can pass while system is still unstable

---

# 11. Missing: **Explicit dependency chain enforcement**

You imply ordering:

- projection → hero → vines

But nowhere do you say:

> ❌ “Do not implement vines until hero + projection invariants are proven”

### Why this matters
This is where regressions usually happen.

---

# 12. Missing: **Performance / caching policy**

You already have:
- full-frame grid composition
- animated hero
- future vines/particles

But:

> ❌ No performance expectations  
> ❌ No caching rules  

### Why this matters
You will hit:
- frame drops
- recomputation overhead
- inconsistent timing behavior

---

# Bottom Line

## What’s actually missing (compressed)

You don’t lack ideas—you lack **enforceable contracts**.

### The critical missing layers:

1. **Projection = function-level contract**
2. **Invariants = centralized + testable**
3. **Stabilization = measurable, not descriptive**
4. **RenderState = fully specified structure**
5. **Layering = single authoritative model**
6. **Hero path = forced decision gate**
7. **Greenhouse = integrated into scene system**
8. **Bug taxonomy = converted into tests**
9. **Instrumentation = required, not optional**
10. **Determinism = explicitly scoped**

---

# Final framing

Right now your system is:

> ✅ Architecturally correct  
> ❌ Not yet mechanically enforceable  

The missing pieces are exactly the ones that:
- prevent regression
- allow safe iteration
- make Codex (or any dev) unable to “accidentally break things”

Fix those—and this becomes a **very high-quality, production-grade rendering architecture**.


Yes—this is the right moment to add **second-order structure**. You’ve already identified what’s missing at the contract level; now the question is:

> *What extra scaffolding will prevent you from drifting again once implementation resumes?*

Below are **high-leverage propositions**—not fixes, but *structural upgrades* that will pay off immediately in execution.

---

# 1. Introduce a **“Frame Authority Model”** (this is the biggest unlock)

Right now you *implicitly* assume:

- Scene owns truth  
- RenderState snapshots it  
- Layers consume it  

But this is not formalized.

## 🔧 Proposition

Add a concept:

```md
Frame Authority Model:

- WorldState → authoritative simulation state
- Scene → authoritative spatial composition
- RenderState → authoritative per-frame snapshot
- FrameBuffer → authoritative terminal output

No system may bypass its authority layer.
```

## Why this matters

This aligns perfectly with how ratatui actually works:

- everything renders into a single buffer per frame  [oai_citation:0‡Ratatui](https://ratatui.rs/concepts/rendering/under-the-hood/?utm_source=chatgpt.com)  
- final output is just the resolved buffer  

👉 You want:
> one authoritative owner per stage

Without this, bugs will always sneak in via:
- “temporary helpers”
- debug shortcuts
- UI leaking into scene

---

# 2. Add a **“Render Ownership Rule” per system**

Right now SCENE_MODEL says:
> systems emit primitives  [oai_citation:1‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

That’s good—but incomplete.

## 🔧 Proposition

Define explicitly:

```md
Each system must declare:

- what state it owns
- what primitives it emits
- what it depends on (RenderState fields)
- what it must NOT access
```

Example:

```md
Hero:
- owns: frame sequence, anchor
- emits: glyph grid
- depends on: projection, mask
- must not access: HUD, viewport layout
```

## Why this matters

You eliminate:
- hidden dependencies
- cross-layer leakage
- accidental coupling

---

# 3. Introduce a **“No Backflow Rule”**

You already forbid rendering inside logic.

But you don’t forbid the reverse:

> rendering influencing logic

## 🔧 Proposition

Add:

```md
No Backflow Rule:

Render output must never influence:
- world state
- camera
- system logic

All logic must derive from WorldState, not from rendered output.
```

## Why this matters

This prevents:
- “measure rendered width → adjust logic”
- “mask visibility affects simulation”

👉 This is a common failure mode in terminal renderers.

---

# 4. Add a **“Terminal Reality Constraint” layer**

You’re treating terminal as framebuffer (correct).

But ratatui is *not* a pixel renderer:

- it diff-compares buffers
- every cell write is expensive  [oai_citation:2‡GitHub](https://github.com/madebyaris/native-cli-ai/blob/main/docs/research/rust-ratatui-optimization.md?utm_source=chatgpt.com)  

## 🔧 Proposition

Add a section:

```md
Terminal Constraints:

- rendering is cell-based, not pixel-based
- full-frame redraw is expensive
- diffing is O(n) per frame
- unicode width affects layout

Implications:
- prefer stable geometry
- minimize per-frame changes
- cache where possible
```

## Why this matters

This will directly affect:
- hero rendering decision
- vine animation strategy
- debug overlays

---

# 5. Introduce a **“Determinism Envelope”**

You define determinism—but not its boundaries.

## 🔧 Proposition

Define explicitly:

```md
Determinism Envelope:

Deterministic given:
- WorldState
- camera
- viewport size
- frame index (for animation)

Not required deterministic across:
- different terminal sizes
- different fonts/renderers
```

## Why this matters

Prevents:
- over-constraining the system
- false test failures
- confusion about resize behavior

---

# 6. Add a **“State Mutation Map”**

Right now ARCHITECTURE defines forbidden coupling (excellent).

But you don’t define:

> who *is allowed* to mutate what

## 🔧 Proposition

Create a table:

| Layer     | Can mutate | Cannot mutate |
|----------|------------|--------------|
| core     | data only  | everything else |
| systems  | WorldState | render/UI |
| scene    | nothing    | everything |
| render   | nothing    | everything |
| ui       | UI state   | WorldState |

## Why this matters

Prevents:
- accidental write paths
- “temporary hacks” becoming permanent

---

# 7. Add a **“Feature Gate Discipline”**

You already have phase ordering.

Now enforce *entry conditions*.

## 🔧 Proposition

Before each phase:

```md
Entry Gate: Hero Phase

- projection invariants pass
- resize invariance verified
- anchor stability confirmed
```

## Why this matters

Prevents:
- cascading bugs
- premature feature layering

---

# 8. Introduce a **“Minimal Frame Definition”**

Right now:

- layers emit full-frame grids
- Scene merges them

But:

> ❌ You don’t define what a “valid frame” is

## 🔧 Proposition

```md
Valid Frame:

- covers entire terminal area
- every cell has a defined value
- layering resolved deterministically
- no undefined regions
```

## Why this matters

This aligns with ratatui’s requirement:

> every draw must fully reconstruct the frame  [oai_citation:3‡Ratatui](https://ratatui.rs/concepts/rendering/under-the-hood/?utm_source=chatgpt.com)  

---

# 9. Add a **“Mask Semantics Contract” (tighten this)**

You already say:
- masks are not empty pixels  [oai_citation:4‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

But missing:

## 🔧 Proposition

```md
Mask Rules:

- mask = occlusion operator, not deletion
- masked area still exists in world space
- masking occurs before final composition
- masking does not affect system logic
```

## Why this matters

This is critical for:
- vines wrapping hero
- scaffold interactions
- future collision systems

---

# 10. Add a **“Complexity Budget” (yes, seriously)**

You’re about to add:
- hero animation
- vines
- particles

Without constraints, this explodes.

## 🔧 Proposition

```md
Complexity Budget:

- max systems active per frame
- max primitives per system
- max frame recomposition cost

Prefer:
- static + incremental updates
- cached representations
```

## Why this matters

Ratatui is not GPU-backed. You’re CPU + terminal IO bound.

---

# 11. Add a **“One-way Data Flow Diagram”**

You have it implicitly.

Make it explicit:

```text
WorldState
   ↓
Systems
   ↓
Scene (composition)
   ↓
RenderState
   ↓
Layers
   ↓
FrameBuffer
   ↓
Terminal
```

👉 And explicitly forbid reverse arrows.

---

# 12. Add a **“Kill-switch rule” for instability**

This is practical, not theoretical.

## 🔧 Proposition

```md
If any new feature introduces:
- frame jitter
- projection inconsistency
- anchor drift

→ feature must be disabled until stabilised
```

## Why this matters

Protects your core invariants aggressively.

---

# Final Take

You don’t need more ideas—you need **structural enforcement layers**.

## The most impactful additions:

1. Frame Authority Model  
2. Projection Contract (from earlier)  
3. Determinism Envelope  
4. Render Ownership Rules  
5. Feature Gate Discipline  

Everything else is secondary.

---

If you want, next step we can:
👉 convert these into a **clean patch for TODO.md / SCENE_MODEL.md** so it becomes executable, not just descriptive.


Alright—this is where we get *practical and opinionated*. You don’t need a random list of crates; you need a **tight, architecture-aligned tooling stack** that reinforces your contracts instead of fighting them.

Below is a **curated, role-based tooling stack** specifically for *your* yam-rust architecture (scene → projection → layers → framebuffer → ratatui).

---

# 🧠 First principle (important before tools)

[ratatui](chatgpt://generic-entity?number=0) is **not a framework—it’s a rendering library**  [oai_citation:0‡Ratatui](https://ratatui.rs/faq/?utm_source=chatgpt.com)  

👉 That’s perfect for you:
- you *want* full control over scene + projection
- you *do NOT want* framework-imposed layout logic

So the stack should:
> reinforce your architecture, not replace it

---

# 🧱 1. Core runtime + terminal layer (non-negotiable)

## ✅ Required

### 🟢 Crossterm
- input handling
- terminal control
- backend for ratatui

👉 Already standard pairing with ratatui  [oai_citation:1‡docs.rs](https://docs.rs/ratatui/latest/ratatui/?utm_source=chatgpt.com)  

### 🟢 color-eyre (or anyhow)
- structured error handling for runtime loop

---

# 🎯 2. Scene / rendering alignment tools (high priority)

These directly support your architecture.

---

## 🟣 `ansi-to-tui`

👉 From ecosystem list  [oai_citation:2‡GitHub](https://github.com/ratatui/awesome-ratatui?utm_source=chatgpt.com)  

### Why you want it
- converts ANSI → ratatui `Text`
- perfect bridge for your **chafa → renderer migration**

### Use cases
- current hero pipeline stabilization
- debug overlay rendering
- transition step toward cached frames

---

## 🟣 `tachyonfx`

👉 Shader-like effects system  [oai_citation:3‡GitHub](https://github.com/ratatui/awesome-ratatui?utm_source=chatgpt.com)  

### Why it matters for yam
- lets you do:
  - fade
  - dissolve
  - growth effects

### Critical note
Use ONLY:
- after projection is stable
- as **post-composition effect layer**

👉 fits your “overlay / final stage” model nicely

---

## 🟣 `termprofile`

👉 Terminal capability detection  [oai_citation:4‡GitHub](https://github.com/ratatui/awesome-ratatui?utm_source=chatgpt.com)  

### Why you need it
Your system depends on:
- color fidelity
- glyph width (braille, unicode)

This crate lets you:
- adapt palette
- detect truecolor vs limited color

👉 prevents “why does it look different on another terminal” bugs

---

# 🧩 3. Input / interaction layer (careful selection)

You want **control**, not a framework.

---

## 🟢 `tui-input`

👉 headless input system  [oai_citation:5‡GitHub](https://github.com/ratatui/awesome-ratatui?utm_source=chatgpt.com)  

### Why it fits
- does NOT impose UI model
- keeps input separate from rendering

👉 aligns with:
> “ui must not mutate world state directly”

---

## 🟡 `ratatui-input-manager` (optional)

### Only if:
- you want declarative input handling

⚠️ Risk:
- can push toward “framework thinking”

👉 Use carefully or skip entirely.

---

# 🎨 4. Styling / theming (important for your BTAS direction)

---

## 🟣 `opaline`

👉 token-based theme engine  [oai_citation:6‡GitHub](https://github.com/ratatui/awesome-ratatui?utm_source=chatgpt.com)  

### Why it’s perfect for yam

You already want:
- palette control
- consistent color semantics
- theme system

This gives:
- reusable tokens
- theme switching

👉 way better than hardcoding colors in layers

---

## 🟣 `coolor` / `color-to-tui`

### Use for:
- palette conversion
- dynamic color adjustment

---

# 🧪 5. Testing + determinism (you *must* add this)

---

## 🔴 `ratatui-testlib` (critical)

👉 built specifically for TUI testing  [oai_citation:7‡crates.io](https://crates.io/crates/ratatui-testlib?utm_source=chatgpt.com)  

### Why this is huge for you

It enables:
- frame buffer assertions
- deterministic rendering tests

👉 This directly supports:
- your stabilization checkpoints
- projection invariants
- “same input → same frame”

---

# 🧰 6. Layout / composition helpers (use sparingly)

---

## 🟡 `ratatui-garnish`

👉 composition system  [oai_citation:8‡GitHub](https://github.com/ratatui/awesome-ratatui?utm_source=chatgpt.com)  

### When useful
- composing widgets cleanly

### When dangerous
- if it starts replacing your Scene model

👉 Recommendation:
Use only for HUD widgets, NOT world rendering.

---

## 🟡 `tui-scrollview`

### Good for:
- HUD scroll areas
- debug panels

👉 Never for world.

---

# 🧪 7. Development tooling (very underrated)

---

## 🟣 `tui-pantry`

👉 “storybook for TUI”  [oai_citation:9‡GitHub](https://github.com/ratatui/awesome-ratatui?utm_source=chatgpt.com)  

### Why this is extremely useful

You can:
- test widgets in isolation
- preview HUD elements
- debug styling

👉 prevents polluting main scene with test UI

---

# 🚫 8. Tools you should **NOT use**

These will break your architecture.

---

## ❌ `tui-realm`

👉 it’s a framework  [oai_citation:10‡Ratatui](https://ratatui.rs/faq/?utm_source=chatgpt.com)  

### Why not
- enforces its own state model
- conflicts with your Scene/RenderState system

---

## ❌ `cursive`

### Why not
- event-driven UI framework
- panel/dashboard oriented

👉 violates:
> “terminal is framebuffer, not dashboard”

---

## ❌ high-level layout engines

Anything that:
- auto-manages layout
- owns render loop

👉 will fight your projection contract

---

# 🧠 9. Your **ideal stack (opinionated)**

Here’s the clean setup for yam:

---

## 🔵 Core
- ratatui
- crossterm
- color-eyre

---

## 🟣 Rendering / bridge
- ansi-to-tui (temporary, during transition)
- tachyonfx (later stage only)

---

## 🟣 Input
- tui-input

---

## 🟣 Styling
- opaline
- coolor

---

## 🔴 Testing (mandatory)
- ratatui-testlib

---

## 🟡 Optional
- tui-pantry (dev)
- termprofile (portability)

---

# ⚠️ Final strategic advice

## 1. Don’t over-adopt
You already have a **strong architecture**.

👉 Tools should:
- support it
- not replace it

---

## 2. The most important addition is NOT visual

It’s:

> 🧪 `ratatui-testlib`

Because your whole system depends on:
- determinism
- invariance
- frame correctness

---

## 3. Sequence matters

Introduce tooling in this order:

1. testlib → lock invariants  
2. ansi-to-tui → stabilize hero  
3. theme system → unify styling  
4. effects → AFTER stabilization  

---

# 🚀 If you want next step

We can:
👉 map each of these tools to **specific yam modules (render/, scene/, ui/)**  
👉 and decide exactly where they plug in without breaking contracts

That’s where this gets really powerful.

Good—now let’s **wire this into your actual repo architecture**, not just list tools.

I’ll map each tool to:
- your modules (`scene/`, `render/`, `ui/`, `runtime`)
- your contracts (projection, RenderState, layering)
- your current backlog phases

This is the part that makes the stack *usable*, not just “nice to know”.

---

# 🧭 0. Ground truth (your architecture constraints)

From your docs:

- Scene owns composition  [oai_citation:0‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  
- RenderState is per-frame contract  [oai_citation:1‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  
- ratatui = final renderer only  [oai_citation:2‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

👉 So tooling must plug into:

```text
WorldState → Scene → RenderState → Layers → FrameBuffer → ratatui
```

---

# 🧱 1. `ansi-to-tui` → **Hero pipeline bridge (render/ only)**

### 📍 Where it goes
```
render/
  hero/
    chafa_adapter.rs
    ansi_bridge.rs   ← HERE
```

### 🎯 Role
- Convert ANSI output → ratatui `Text`  [oai_citation:3‡docs.rs](https://docs.rs/ansi-to-tui?utm_source=chatgpt.com)  
- Transitional layer for your current chafa pipeline

### 🧠 How it fits your contracts

| Contract | Behavior |
|--------|--------|
| Scene purity | stays in `render/`, not `scene/` |
| Projection | already resolved before rendering |
| Layering | outputs primitives, not final draw |

### ⚠️ Critical constraint

> ❌ Never let ANSI parsing leak into Scene

This tool is **temporary infrastructure**, not core architecture.

---

# 🧠 2. `ratatui-testlib` → **Stabilisation engine (global)**

### 📍 Where it goes
```
tests/
  render_tests.rs
  projection_tests.rs
  invariants/
```

### 🎯 Role
- full TUI integration testing
- snapshot + PTY-based validation  [oai_citation:4‡docs.rs](https://docs.rs/ratatui-testlib?utm_source=chatgpt.com)  

### 🧠 How it maps to your backlog

Directly powers:

- TODO §5 stabilization checkpoints  [oai_citation:5‡TODO.md](sediment://file_00000000711c71f4b8f2fe910bb91c79)  
- SCENE_MODEL determinism  [oai_citation:6‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

### 🧪 What you should test

| System | Test |
|------|------|
| projection | world→screen invariance |
| scene | layer ordering |
| hero | frame stability |
| HUD | camera independence |

### 🚨 This is your most important tool

Without this:
> your “stabilisation phases” are not enforceable

---

# 🎨 3. `opaline` (or equivalent) → **theme system (ui/ + render/)**

### 📍 Where it goes
```
theme/
  tokens.rs
  palette.rs
```

### 🎯 Role
- central color + style authority

### 🧠 Why it matters for you

Right now:
- hero uses chafa palette
- UI uses hardcoded styles
- debug uses ad hoc colors

👉 This WILL drift.

### 🔧 With opaline-style system:

```rust
theme.hero.primary
theme.hud.footer
theme.debug.overlay
```

### 🧠 Architectural fit

| Contract | Behavior |
|--------|--------|
| separation | no styling in scene |
| determinism | no runtime color guessing |
| portability | terminal-aware themes |

---

# 🧩 4. `tui-input` → **input layer isolation (runtime/ + ui/)**

### 📍 Where it goes
```
runtime/
  input.rs
ui/
  input_mapping.rs
```

### 🎯 Role
- event normalization
- input state handling

### 🧠 Why it matters

You explicitly want:

> UI must not mutate world state directly  [oai_citation:7‡TODO.md](sediment://file_00000000711c71f4b8f2fe910bb91c79)  

This tool helps enforce:
- input → intent
- intent → system mutation (NOT UI)

---

# 🌈 5. `coolor` / `color-to-tui` → **color pipeline normalization**

### 📍 Where it goes
```
render/
  color_pipeline.rs
```

### 🎯 Role
- unify:
  - chafa colors
  - theme colors
  - debug colors

### 🧠 Why this matters

Right now your risks:

- hero uses ANSI palette
- vines will use custom colors
- UI uses ratatui styles

👉 mismatch = visual inconsistency + bugs

---

# 🧪 6. `tui-pantry` → **isolated UI lab (dev only)**

### 📍 Where it goes
```
tools/
  ui_lab/
```

### 🎯 Role
- preview widgets independently
- test HUD without scene

### 🧠 Why this is huge for you

Prevents:

- polluting scene with test UI
- debugging UI inside full render loop

---

# ⚡ 7. `tachyonfx` → **post-composition effects layer (LATE STAGE ONLY)**

### 📍 Where it goes
```
render/
  effects/
```

### 🎯 Role
- apply effects AFTER composition

### 🧠 Correct integration point

```text
FrameBuffer → Effects → Final Output
```

### 🚨 Important

DO NOT use it:
- before projection is stable
- inside Scene

👉 It must be **post-pipeline only**

---

# 🧪 8. `termprofile` → **environment normalization (runtime)**

### 📍 Where it goes
```
runtime/
  terminal_profile.rs
```

### 🎯 Role
- detect:
  - color support
  - unicode capabilities

### 🧠 Why you need it

Your system depends on:
- braille glyphs
- truecolor gradients

Without this:
> behavior varies across terminals

---

# 🧱 9. What your **final architecture with tooling looks like**

```text
WorldState
   ↓
Systems
   ↓
Scene (pure)
   ↓
RenderState
   ↓
Layers
   ↓
FrameBuffer
   ↓
[ tachyonfx (optional, late) ]
   ↓
ratatui
```

### Supporting layers:

| Layer | Tool |
|------|------|
| input | tui-input |
| color | coolor |
| theme | opaline |
| hero bridge | ansi-to-tui |
| testing | ratatui-testlib |
| dev UI | tui-pantry |
| terminal | termprofile |

---

# 🚫 What you must **NOT integrate into core**

From ecosystem list  [oai_citation:8‡GitHub](https://github.com/ratatui/awesome-ratatui?utm_source=chatgpt.com)  

Avoid in core architecture:

- tui-realm  
- cursive  
- rat-widget  
- any “layout engine”  

👉 These introduce:
- alternative state models
- competing render pipelines

---

# ⚠️ Critical sequencing (tie to your TODO phases)

## Phase 1 — UI + stabilization
- ✅ tui-input
- ✅ ratatui-testlib

## Phase 2 — hero
- ✅ ansi-to-tui (temporary)
- ✅ color pipeline

## Phase 3 — stabilization
- 🔴 expand test coverage

## Phase 4 — vines
- (no new tools)

## Phase 5 — polish
- tachyonfx
- theme system refinement

---

# 🧠 Final synthesis

You don’t need many tools—you need **correctly placed tools**.

## The only *must-haves*:

- `ratatui-testlib` → enforces your architecture  
- `ansi-to-tui` → stabilizes hero transition  
- `tui-input` → protects UI boundaries  

Everything else is:
> optimization or ergonomics

---

# If you want next step

We can now:
👉 draft **exact module layout + file structure** for `render/`, `scene/`, `ui/` with these tools embedded cleanly  

That’s where this becomes truly production-grade.


Good—this is exactly the layer you should be thinking about now.  
You’ve already got **render architecture under control**; what’s missing is a **clean “engine-side discipline”** so the system doesn’t collapse as you add behavior (vines, particles, greenhouse, etc.).

Below is a **focused engine-side stack + practices**, aligned with your architecture (NOT generic ECS hype).

---

# 🧠 0. First: what “engine-side” means in *your* system

From your docs:

- `core/` → data only  
- `systems/` → mutate world  
- `scene/` → composition  
- `render/` → output  [oai_citation:0‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  

👉 So “engine-side” = everything **before Scene**:

```text
WorldState → Systems → (then Scene)
```

---

# 🧱 1. You need a **lightweight ECS discipline (NOT a framework)**

## ❌ Do NOT use:
- Bevy ECS
- Specs
- Legion

👉 Too heavy, wrong abstraction, will fight your design.

---

## ✅ Instead: adopt **“ECS-lite by convention”**

### 🔧 Pattern

```rust
struct WorldState {
    hero: Hero,
    vines: Vec<Vine>,
    particles: Vec<Particle>,
}
```

```rust
trait System {
    fn update(&mut self, world: &mut WorldState, dt: f32);
}
```

👉 That’s enough.

---

## 🧠 Why this is correct for you

- deterministic
- simple ownership
- matches your current repo structure

👉 You don’t need dynamic entity graphs—you need **predictable simulation**.

---

# ⚙️ 2. Add a **System Scheduler (critical missing piece)**

Right now you have systems—but no formal execution order.

---

## 🔧 Proposition

```rust
enum SystemStage {
    Input,
    Simulation,
    PostSimulation,
}
```

```rust
scheduler.run(stage, world, dt);
```

---

## 🧠 Why this matters

Prevents:
- hidden dependencies
- order-based bugs
- “hero updates after vines” issues

---

## 🎯 Minimal stages you need

```text
Input → Simulation → Cleanup
```

Later:
```text
Input → Simulation → Constraint → Cleanup
```

---

# 🧩 3. Introduce a **State Transition Model (important for greenhouse)**

You already hint at:
- `WorldMode::Main`
- `WorldMode::Greenhouse`  [oai_citation:1‡TODO.md](sediment://file_00000000711c71f4b8f2fe910bb91c79)  

But you don’t define transitions.

---

## 🔧 Add:

```rust
enum EngineState {
    Main,
    Greenhouse,
    SettingsOverlay,
}
```

```rust
fn transition(from, to) -> EngineState
```

---

## 🧠 Why this matters

Prevents:
- mode leakage
- mixed input handling
- UI/scene conflicts

---

# 🌿 4. Add a **Spatial Query Layer (you WILL need this for vines)**

Right now:
- vines will need proximity
- masks need spatial checks
- anchors depend on positions

---

## 🔧 Minimal solution

```rust
fn query_near(world: &WorldState, pos: Vec2, radius: f32) -> Vec<EntityId>
```

---

## 🔧 Optional upgrade

Use:
- `rstar` crate (R-tree spatial index)

---

## 🧠 Why this matters

Without it:
- O(n²) checks
- messy coupling
- unpredictable behavior

---

# 🎯 5. Add a **Deterministic Tick Model**

SCENE_MODEL requires determinism  [oai_citation:2‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

But engine-side doesn’t enforce it.

---

## 🔧 Add:

```rust
struct Tick {
    frame: u64,
    dt: f32,
    seed: u64,
}
```

---

## 🧠 Rules

- all randomness must be seeded
- no system reads system time directly
- dt is fixed or clamped

---

## Why this matters

You want:
- reproducibility
- testability
- stable animation

---

# 🧪 6. Add **Simulation-Level Testing (separate from render tests)**

You already plan render tests.

You ALSO need:

---

## 🔧 System tests

```rust
#[test]
fn vines_do_not_penetrate_hero_mask() {}
```

```rust
#[test]
fn hero_anchor_remains_constant() {}
```

---

## 🧠 Why this matters

Prevents:
- logic bugs from reaching render stage
- debugging via visuals only

---

# 🧱 7. Add a **Constraint System (future-proofing vines)**

You will need:

- vines avoid hero
- particles respect bounds
- anchors maintain offsets

---

## 🔧 Pattern

```rust
trait Constraint {
    fn apply(&self, world: &mut WorldState);
}
```

---

## Example

```rust
HeroCollisionConstraint
VineBoundaryConstraint
```

---

## 🧠 Why this matters

Separates:
- behavior (systems)
- rules (constraints)

👉 massively improves clarity

---

# 🧠 8. Add a **Data Ownership Discipline**

Right now implicit.

Make it explicit:

---

## 🔧 Rule

```text
Each system owns:
- its data
- its update logic

No system mutates another system’s internal state directly.
```

---

## 🧠 Why this matters

Prevents:
- tight coupling
- cascading bugs

---

# 🧩 9. Introduce a **“Simulation Snapshot” (engine-side equivalent of RenderState)**

You already have RenderState for rendering.

You need:

---

## 🔧

```rust
struct SimulationSnapshot {
    hero_pos: Vec2,
    vine_segments: Vec<...>,
}
```

---

## 🧠 Why this matters

Separates:
- mutable world
- read-only view for scene

👉 same principle as RenderState, earlier in pipeline

---

# 🔧 10. Add **Debug Introspection Hooks (engine-side)**

Right now debug is visual.

You also need:

---

## 🔧

```rust
fn debug_dump(world: &WorldState) -> DebugInfo
```

---

## Output:
- entity counts
- spatial distribution
- system timings

---

## 🧠 Why this matters

Prevents:
- relying only on visual debugging
- invisible logic bugs

---

# ⚡ 11. Optional but powerful: `tracing`

## 🟣 Use:
- `tracing`
- `tracing-subscriber`

---

## 📍 Where

```text
systems/
runtime/
```

---

## 🧠 Why

- structured logs
- performance tracing
- system-level visibility

---

# 🚫 What NOT to introduce

---

## ❌ Full ECS frameworks

They will:
- break determinism
- add indirection
- conflict with your Scene model

---

## ❌ Game engines (Bevy etc.)

They:
- own render loop
- own ECS
- incompatible with ratatui architecture

---

# 🧠 Final Engine Stack (clean)

---

## Core patterns (mandatory)

- ECS-lite (manual structs + systems)
- system scheduler
- deterministic tick model
- simulation snapshot
- constraint system

---

## Optional crates

- `tracing` → debugging
- `rstar` → spatial queries

---

# 🚀 Final synthesis

You now have:

### Render side:
- Scene
- RenderState
- Layer system
- ratatui

### Engine side (after this):
- WorldState
- Systems
- Scheduler
- Constraints
- SimulationSnapshot

---

## The key insight

Your architecture becomes:

```text
Simulation (engine)
   ↓
Scene (composition)
   ↓
Render (terminal)
```

👉 clean, deterministic, testable

---

# If you want next step

We can now:
👉 design the **exact `systems/` module layout + scheduler implementation**  
👉 or define **vine system architecture properly (growth, collision, masks)**

That’s the next real breakpoint.

Perfect—this is exactly the right move.  
Before vines, before effects, before anything visual—you need a **platform that cannot drift**.

What you’re building now is not a feature layer. It’s a **failure-resistant execution substrate**.

I’ll define **what that platform must consist of**, aligned with your current docs, and what to implement *now* so vines later “just plug in”.

---

# 🧠 1. Define the platform goal (very explicitly)

Your platform must guarantee:

```text
Same world state + same inputs → same frame output
```

Across:
- resize
- camera movement
- frame progression

👉 If this holds, vines will not introduce chaos.

---

# 🧱 2. The minimal stable platform (what must exist before vines)

You already have parts of this. What’s missing is **tight coupling between them**.

---

## 🔷 Layer 1 — Simulation Core (engine-side)

```text
WorldState → Systems → SimulationSnapshot
```

### MUST guarantee:
- deterministic updates
- no rendering influence
- no cross-system mutation leaks

---

## 🔷 Layer 2 — Scene Composition (already strong)

```text
SimulationSnapshot → Scene → RenderState
```

From your SCENE_MODEL:
- projection
- masking
- layering  [oai_citation:0‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  

👉 This is your strongest layer already.

---

## 🔷 Layer 3 — Rendering (ratatui)

```text
RenderState → Layers → FrameBuffer → ratatui
```

From ARCHITECTURE:
- single frame buffer
- no side effects  [oai_citation:1‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  

---

# 🧩 3. What’s missing for “robust platform”

These are the **non-negotiable gaps** to fill before vines.

---

# 🔴 3.1 Projection must become a *hard API*, not a rule

Right now:
> “projection should live in one place”  [oai_citation:2‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  

That’s not enforceable.

---

## 🔧 You need:

```rust
pub fn project(
    world: Vec2,
    camera: Camera,
    viewport: Rect
) -> ScreenPos
```

### And enforce:

- ONLY this function converts world → screen
- no inline math anywhere else

---

## 🧠 Why this matters for vines

Vines will:
- grow in world space
- be masked
- be projected

If projection is inconsistent:
> vines will jitter, clip, or drift

---

# 🔴 3.2 Introduce a **SimulationSnapshot (critical missing layer)**

Right now Scene reads directly from world-ish state.

That’s fragile.

---

## 🔧 Add:

```rust
struct SimulationSnapshot {
    hero_pos: Vec2,
    vine_segments: Vec<Segment>,
    particles: Vec<Particle>,
}
```

### Rules:
- immutable
- built once per frame
- consumed by Scene

---

## 🧠 Why this matters

Prevents:
- mid-frame mutation bugs
- inconsistent reads
- order-dependent behavior

---

# 🔴 3.3 Lock the **execution order (scheduler)**

You currently *assume* order.

You need to enforce it.

---

## 🔧 Minimal scheduler:

```rust
enum Stage {
    Input,
    Simulation,
    Constraints,
    Snapshot,
}
```

---

## Execution:

```text
Input → Simulation → Constraints → Snapshot → Scene → Render
```

---

## 🧠 Why this matters

Vines will depend on:
- hero position
- masks
- collision rules

Without strict ordering:
> you get frame-to-frame inconsistency

---

# 🔴 3.4 Add a **constraint system (before vines, not after)**

You will need:

- vines avoid hero
- particles respect bounds
- anchors stay valid

---

## 🔧 Add now:

```rust
trait Constraint {
    fn apply(&self, world: &mut WorldState);
}
```

---

## 🧠 Why BEFORE vines

If you don’t:
- vine logic will embed constraints
- system becomes tangled

---

# 🔴 3.5 Introduce **frame determinism enforcement**

SCENE_MODEL says deterministic  [oai_citation:3‡SCENE_MODEL.md](sediment://file_000000008f6471f4af6ca274e23b3b88)  
But you don’t enforce it.

---

## 🔧 Add:

```rust
struct Tick {
    frame: u64,
    seed: u64,
    dt: f32,
}
```

### Rules:
- all randomness seeded
- no system reads system clock
- dt is fixed or clamped

---

## 🧠 Why this matters

Vines are stochastic-like systems:
- growth patterns
- branching
- randomness

Without determinism:
> impossible to debug or test

---

# 🔴 3.6 Add **frame-level invariance tests (MANDATORY)**

You already started this (LOG confirms)  [oai_citation:4‡LOG.md](sediment://file_00000000ff4071f4a180012d0292a5f9)  

Now formalize.

---

## 🔧 Tests you must have

### 1. Resize invariance
```text
world_pos(hero) unchanged after resize
```

### 2. Projection invariance
```text
project(world_pos) stable for same camera
```

### 3. Frame determinism
```text
same input → identical frame buffer
```

### 4. Anchor integrity
```text
clock follows hero across all frames
```

---

## 🧠 Why this matters

These are your **platform guarantees**.

---

# 🔴 3.7 Add **debug instrumentation (engine + scene)**

You already have visual debug layer.

Now enforce:

---

## 🔧 Must be visible:

- world coords
- projected coords
- mask overlays
- layer boundaries

---

## 🧠 Why this matters

Without it:
- you debug via guesswork
- vines will be impossible to tune

---

# 🔴 3.8 Lock **data ownership boundaries**

From ARCHITECTURE:

- systems mutate world
- render does not  [oai_citation:5‡ARCHITECTURE.md](sediment://file_00000000d00071f49e60bad069dfe22f)  

Good—but not enforced at system level.

---

## 🔧 Add rule:

```text
Systems may only mutate their own domain data.
```

Example:

- vine system → vine data only
- hero system → hero only

---

## 🧠 Why this matters

Prevents:
- cross-system side effects
- unpredictable interactions

---

# 🧱 4. What this gives you (important)

After implementing above:

### You get:

- deterministic simulation
- stable projection
- clean scene composition
- testable frame output

---

### THEN vines become:

```text
just another system + primitives emitter
```

👉 not a destabilizing feature

---

# 🧭 5. Implementation order (tightened)

Refine your TODO order:

---

## Phase 1 — Platform lock

- projection API
- scheduler
- SimulationSnapshot
- determinism model

---

## Phase 2 — Validation

- invariance tests
- frame diff tests
- debug overlays

---

## Phase 3 — Hero stabilization

- cached frames or stable pipeline
- no geometry drift

---

## Phase 4 — Only now: vines

---

# 🚀 Final takeaway

You don’t need more features.

You need:

> a system where **nothing unexpected can happen**

---

## The platform is stable when:

- every frame is explainable  
- every change is intentional  
- every bug is reproducible  

---

Once that’s true:

👉 vines won’t “break things”  
👉 they’ll simply “exist inside the system”

---

If you want next step, we can:
👉 design the **exact scheduler + SimulationSnapshot implementation**  
👉 or define **vine system architecture on top of this stable base**


