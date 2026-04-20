
# YAM v2 — EMITTER

---

## PURPOSE

Define the Emitter as the **final stage that converts the framebuffer into terminal output (ANSI stream)**.

This document specifies:

- ANSI generation
- output strategies
- cursor control
- performance techniques

---

## CORE PRINCIPLE

The emitter is:

> a pure projection layer from framebuffer → terminal

It does not:

- perform rendering
- modify state
- apply logic

It only:

> serializes the framebuffer into ANSI output efficiently

---

## POSITION IN ARCHITECTURE

```text
Render → Framebuffer → Emitter → Terminal
```

---

## INPUT / OUTPUT

### Input

```text
Framebuffer (2D grid of cells)
```

### Output

```text
ANSI string (or stream)
```

---

## CELL → ANSI MAPPING

Each cell is converted to ANSI sequences:

```text
Cell(char, fg, bg) → "\x1b[...m" + char
```

Where:

- fg → foreground color code
- bg → background color code

---

## COLOR HANDLING

Supported modes:

- 16 color
- 256 color
- truecolor (24-bit)

Rule:

```text
prefer highest supported mode
fallback gracefully
```

---

## CURSOR STRATEGY

Emitter controls cursor position.

### Full redraw mode

```text
move cursor to (0,0)
write full frame
```

---

### Diff mode (optional)

```text
only update changed cells
move cursor selectively
```

---

## OUTPUT MODES

### 1. FULL FRAME (DEFAULT)

```text
clear
write all cells
```

Advantages:

- simple
- deterministic

---

### 2. DIFF-BASED (OPTIMIZED)

```text
compare previous frame
emit only changes
```

Advantages:

- reduced output
- better performance on large frames

---

## LINE BUFFERING

Emitter builds output line-by-line:

```text
for y in rows:
  build string
  append newline
```

Optimization:

- minimize ANSI resets
- group same color runs

---

## ANSI OPTIMIZATION

Techniques:

- avoid redundant color codes
- reuse last fg/bg state
- batch writes

Example:

```text
if fg == last_fg:
  skip fg code
```

---

## CLEARING STRATEGY

Options:

### Hard clear

```text
\x1b[2J
```

---

### Soft overwrite

```text
overwrite full frame without clearing
```

Preferred:

```text
soft overwrite (less flicker)
```

---

## PERFORMANCE RULES

- minimize ANSI sequences
- reduce cursor moves
- prefer contiguous writes

---

## FRAME SYNC

Emitter runs once per frame:

```text
Update → Render → Emit → Flush
```

Optional:

- frame rate limiting
- vsync-like timing (tick-based)

---

## DEBUG SUPPORT

Emitter can expose:

- frame size stats
- ANSI length
- diff efficiency

---

## ERROR HANDLING

Emitter must:

- avoid broken ANSI sequences
- fallback to safe output

---

## DESIGN CONSTRAINTS

Avoid:

- logic in emitter
- branching complexity
- per-cell expensive operations

Prefer:

- streaming approach
- linear processing
- simple state tracking

---

## FINAL PRINCIPLE

```text
Emitter turns the framebuffer into visible reality
```

---

## NEXT

→ UI / TUI system (Bubble Tea panels)
