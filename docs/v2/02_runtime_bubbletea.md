# YAM v2 — RUNTIME (BUBBLE TEA)

---

## PURPOSE

Define how YAM runs as a real-time application using the Bubble Tea framework.

This document specifies:

- execution model
- message system
- update loop
- runtime responsibilities

---

## CORE PRINCIPLE

Bubble Tea is the **execution engine** of YAM.

```text
Bubble Tea controls flow
YAM defines the world
```

---

## MVU MODEL (BUBBLE TEA)

Bubble Tea applications follow the Model–Update–View architecture:

- Model → full application state
- Update → processes messages and mutates state
- View → renders output from state citeturn0search0

---

## YAM MAPPING

| Bubble Tea | YAM |
|-----------|-----|
| Model | full ecosystem + UI state |
| Update | simulation + input + environment |
| View | render → framebuffer → ANSI |

---

## RUNTIME LOOP

Bubble Tea drives the loop:

```text
Init()
→ Update(msg)
→ View()
→ repeat
```

YAM integrates into this loop:

```text
msg → Update()
     → engine update
     → render frame
     → return model
→ View()
     → emit ANSI
```

---

## MODEL STRUCTURE

The Model contains all application state:

```text
Model:
  ecosystem
  environment
  vines
  scaffold
  lifecycle
  balance

  framebuffer

  ui_state
  mode

  time
  seed
```

Rules:

- single source of truth
- no duplicated state

---

## MESSAGE SYSTEM

Bubble Tea is message-driven.

### Core messages

```text
TickMsg           → time progression
KeyMsg            → user input
ResizeMsg         → terminal resize
```

### YAM-specific messages

```text
EnvironmentMsg    → weather/time changes
DebugMsg          → toggles
ModeSwitchMsg     → mode change
```

---

## TIME SYSTEM

Time is driven by tick messages:

```go
tea.Tick(interval, func(t time.Time) tea.Msg {
    return TickMsg{}
})
```

Rules:

- no direct system time usage
- all systems derive time from ticks

---

## UPDATE PHASE

All logic happens inside Update().

```text
Update(msg):
  if TickMsg:
    update environment
    update ecosystem
    update vines
    update lifecycle
    update balance

  if KeyMsg:
    handle input

  if ResizeMsg:
    update UI state
```

Rules:

- all state mutation occurs here
- no mutation in View()

---

## RENDER PHASE

Rendering is triggered after Update().

```text
render():
  clear framebuffer
  layers write to framebuffer
```

Rules:

- no direct printing
- no side effects

---

## VIEW PHASE

View() outputs final frame:

```text
View():
  return framebuffer → ANSI string
```

Rules:

- pure function
- no state mutation

---

## MODE SYSTEM

Modes are runtime state, not separate programs.

```text
mode:
  visualizer
  greenhouse
  debug
```

Routing inside Update():

```text
if mode == greenhouse:
  use greenhouse update logic
```

---

## INPUT HANDLING

Input is processed via KeyMsg.

Examples:

```text
q → quit
p → pause
s → step
1/2/3 → switch panels
```

Rules:

- input handled in Update()
- never in View()

---

## PANEL / UI INTEGRATION

UI components are sub-models:

```text
main_model
  ├── slot_grid
  ├── inspector
  ├── parameters
  └── info
```

Each panel:

- receives messages
- updates independently

---

## COMMAND SYSTEM (ASYNC)

Bubble Tea supports async commands.

Used for:

- heavy computation
- loading assets
- background processing

Example:

```go
func loadAsset() tea.Cmd {
  return func() tea.Msg {
    return AssetLoadedMsg{}
  }
}
```

Rules:

- do not block Update()
- use commands for expensive work

---

## PERFORMANCE RULES

- Update() must be fast
- View() must be fast
- heavy work → commands

---

## ERROR HANDLING

Use message-based errors:

```text
ErrorMsg → handled in Update()
```

---

## FINAL PRINCIPLE

```text
Model = world state
Update = world evolution
View = world projection
```

---

## NEXT

→ 10_ecosystem.md
