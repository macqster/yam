
# YAM v2 — TUI RUNTIME (BUBBLE TEA)

---

## PURPOSE

Define the TUI Runtime as the **interactive control layer built on Bubble Tea** that manages input, panels, and system orchestration.

This document specifies:

- runtime structure
- panel system
- input routing
- focus management
- integration with engine and renderer

---

## CORE PRINCIPLE

The TUI runtime is:

> a control surface for the ecosystem

It does not:

- simulate the world
- render visuals directly

It only:

> routes input, manages UI, and triggers updates

---

## POSITION IN ARCHITECTURE

```text
User Input → TUI Runtime → Engine + Render Pipeline → Emitter → Terminal
```

TUI Runtime is implemented using **Bubble Tea (MVU architecture)**.

---

## MVU MODEL (BUBBLE TEA)

```text
Model → application state
Update → handles messages
View → returns UI string
```

---

## YAM MODEL STRUCTURE

```text
AppModel:
  engine_state
  framebuffer

  ui_state
  panels
  focus

  mode
  debug_flags
```

---

## MODES

Runtime supports multiple modes:

```text
Visualizer → main ecosystem view
Greenhouse → controlled simulation
Debug → inspection overlays
```

Rules:

- mode affects Update behavior
- rendering pipeline remains unchanged

---

## PANEL SYSTEM

UI is composed of panels.

```text
Panels:
  Viewport (main render)
  Inspector
  Parameters
  Info
```

---

### Panel Properties

```text
Panel:
  id
  bounds
  visible
  focused
  component_model
```

---

## LAYOUT SYSTEM

Panels are arranged in a layout grid.

Example:

```text
+----------------------+
| Viewport             |
|                      |
+----------+-----------+
| Inspector| Parameters|
+----------+-----------+
```

---

## VIEWPORT PANEL

Special panel that displays the framebuffer.

Rules:

- always present
- renders emitter output
- respects layout bounds

---

## INPUT SYSTEM

Input is handled via Bubble Tea messages.

### Core Inputs

```text
q → quit
p → pause
space → toggle modes
```

---

### Navigation Inputs

```text
tab → switch focus
arrow keys → navigate panels
```

---

### Action Inputs

```text
+/- → adjust parameters
r → reset
```

---

## INPUT ROUTING

```text
KeyMsg → TUI Runtime
       → determine focused panel
       → route to panel or global handler
```

Rules:

- focused panel receives priority
- global shortcuts always available

---

## FOCUS SYSTEM

Only one panel is focused at a time.

```text
focus = panel_id
```

Behavior:

- focused panel receives input
- visual highlight indicates focus

---

## UPDATE FLOW

```text
Update(msg):
  handle global input
  route to focused panel
  update engine (tick)
  update UI state
```

---

## VIEW GENERATION

```text
View():
  render panels
  compose layout
  return string
```

---

## ENGINE INTEGRATION

TUI triggers engine updates:

```text
TickMsg → engine.update()
```

Rules:

- engine runs independently of UI
- UI observes, not controls logic directly

---

## GREENHOUSE INTEGRATION

Greenhouse mode allows:

- spawning species
- freezing time
- stepping simulation
- modifying parameters live

---

## DEBUG FEATURES

TUI supports:

- toggling layers
- mask visualization
- z-index overlay
- performance stats

---

## PERFORMANCE RULES

- minimize string allocations
- reuse panel buffers
- avoid blocking Update()

---

## DESIGN CONSTRAINTS

Avoid:

- embedding engine logic in UI
- tight coupling between panels
- complex layout recalculation

Prefer:

- simple routing
- modular panels
- predictable behavior

---

## FINAL PRINCIPLE

```text
TUI Runtime controls interaction
But does not interfere with simulation or rendering
```

---

## NEXT

→ 31_panels.md
