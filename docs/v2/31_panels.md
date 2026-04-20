
# YAM v2 — PANELS

---

## PURPOSE

Define the Panel system as the **modular UI building blocks** used by the TUI runtime.

This document specifies:

- panel contracts
- panel lifecycle
- rendering responsibilities
- interaction patterns

---

## CORE PRINCIPLE

Panels are:

> independent UI components that manage their own state and rendering

They do not:

- control the engine
- perform rendering of the world directly

They only:

> present and interact with data

---

## POSITION IN ARCHITECTURE

```text
TUI Runtime → Panels → View Composition
```

Panels exist inside the **UI layer**, separate from the engine and render pipeline.

---

## PANEL CONTRACT

Each panel follows a consistent interface.

```text
Panel:
  Init() → optional commands
  Update(msg) → new state + commands
  View() → string
```

---

## PANEL MODEL

Each panel owns its own model:

```text
PanelModel:
  local_state
  derived_data
  config
```

Rule:

```text
panels do not mutate global state directly
```

---

## PANEL TYPES

### 1. VIEWPORT PANEL

Displays the main rendered output.

Responsibilities:

- display emitter output
- respect layout bounds

Rules:

- read-only
- no interaction logic

---

### 2. INSPECTOR PANEL

Displays detailed information about selected entities.

Responsibilities:

- show node/axis data
- display lifecycle state

---

### 3. PARAMETERS PANEL

Allows live modification of system parameters.

Responsibilities:

- adjust species parameters
- tweak environment values

Rules:

- changes routed through TUI runtime

---

### 4. INFO PANEL

Displays global system info.

Examples:

- FPS
- entity counts
- performance stats

---

## PANEL LIFECYCLE

```text
Init → Update → View → repeat
```

Panels react to messages and update independently.

---

## MESSAGE FLOW

```text
KeyMsg → TUI Runtime → focused panel → Update()
```

Panels may emit commands:

```text
Update() → (newModel, Cmd)
```

---

## FOCUS BEHAVIOR

Only one panel is focused.

```text
focused_panel_id
```

Rules:

- focused panel receives input
- unfocused panels are passive

---

## RENDERING

Panels render themselves as strings.

```text
panel.View() → string
```

TUI runtime composes them into layout.

---

## LAYOUT INTEGRATION

Panels are placed using layout system:

```text
bounds:
  x
  y
  width
  height
```

Panels must:

- respect bounds
- clip output if necessary

---

## DATA ACCESS

Panels receive data via:

- props from runtime
- derived snapshots of engine state

Rule:

```text
panels observe state, not own it
```

---

## INTERACTION PATTERNS

Examples:

```text
Inspector:
  arrow keys → navigate entities

Parameters:
  +/- → adjust values

Viewport:
  space → toggle overlay
```

---

## DEBUG PANELS

Special panels may exist for:

- mask visualization
- layer toggling
- z-index inspection

---

## PERFORMANCE RULES

- avoid heavy computation in View()
- cache derived data when possible
- keep rendering lightweight

---

## DESIGN CONSTRAINTS

Avoid:

- coupling panels together
- direct engine mutation
- complex internal logic

Prefer:

- simple components
- clear responsibilities
- predictable behavior

---

## FINAL PRINCIPLE

```text
Panels are the UI surface of the system
They observe and interact, but do not control the core
```

---

## NEXT

→ 32_focus_and_input.md
