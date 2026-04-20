
# YAM v2 — FRAMEBUFFER

---

## PURPOSE

Define the Framebuffer as the **single source of visual truth** in YAM.

This document specifies:

- cell structure
- write rules
- layering and z-resolution
- responsibilities and constraints

---

## CORE PRINCIPLE

The framebuffer is:

> a 2D grid of cells representing the final visual state of a frame

Nothing renders directly to the terminal.

Everything writes to the framebuffer.

---

## POSITION IN ARCHITECTURE

```text
Runtime → Engine → Morphology → Shape → Render → Framebuffer → Emit
                                          ↑
                                     Framebuffer
```

Framebuffer is part of the **Rendering layer**.

---

## CORE MODEL

```text
Framebuffer:
  width
  height
  cells[][]
```

---

## CELL STRUCTURE

Each cell represents a single terminal position.

```text
Cell:
  char
  fg_color
  bg_color
  z_index
```

---

## DEFAULT STATE

On each frame:

```text
clear framebuffer:
  char = ' '
  fg = default
  bg = default
  z = -∞
```

---

## WRITE MODEL

All rendering systems write into the framebuffer.

```text
write(x, y, char, fg, bg, z)
```

---

## Z-BUFFER RESOLUTION

Conflict resolution is handled by z-index.

```text
if incoming_z >= cell.z:
  overwrite
else:
  discard
```

---

## LAYERING

Logical layers map to z-index ranges.

Example:

```text
HeroLayer        → z = 100
VinesLayer       → z = 200
OrnamentsLayer   → z = 300
ParticlesLayer   → z = 400
UILayer          → z = 1000
```

Rules:

- layers do not know about each other
- ordering is enforced via z only

---

## MASK APPLICATION

Masks are applied during write.

```text
if mask[x][y] == blocked:
  skip write
```

---

## BOUNDS CHECKING

All writes must be validated.

```text
if x < 0 or x >= width: skip
if y < 0 or y >= height: skip
```

---

## PERFORMANCE RULES

- avoid unnecessary writes
- batch writes where possible
- minimize full redraw cost

---

## CLEARING STRATEGY

Per frame:

```text
clear framebuffer
render all layers
emit frame
```

No partial persistence between frames.

---

## NO SIDE EFFECTS

Framebuffer is write-only during render.

Rules:

- no reads for logic decisions
- no mutation outside render phase

---

## DEBUG SUPPORT

Framebuffer can expose:

- z-index visualization
- layer overlays
- write heatmaps

---

## OUTPUT CONTRACT

Framebuffer produces:

```text
final 2D grid of cells
```

Consumed by:

```text
Emitter → ANSI output
```

---

## DESIGN CONSTRAINTS

Avoid:

- direct terminal writes
- implicit layering
- hidden overrides

Prefer:

- explicit writes
- deterministic resolution
- simple rules

---

## FINAL PRINCIPLE

```text
Framebuffer is the ground truth of rendering
Everything else is just input to it
```

---

## NEXT

→ 21_render_pipeline.md
