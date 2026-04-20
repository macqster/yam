
# YAM v2 — KEYBINDINGS

---

## PURPOSE

Define the Keybindings system as the **semantic mapping layer between raw input and actions**.

This document specifies:

- key → action mapping
- mode-based bindings
- panel overrides
- configurability

---

## CORE PRINCIPLE

Keybindings are:

> a declarative mapping from input → intent

They do not:

- handle input directly
- execute logic

They only:

> assign meaning to keys

---

## POSITION IN ARCHITECTURE

```text
KeyMsg → Routing → Keybinding → Action → Update()
```

---

## CORE MODEL

```text
Keybinding:
  keys[]
  action
  description
  context
```

---

## ACTION MODEL

Actions are symbolic identifiers.

```text
Action:
  Quit
  Pause
  ToggleMode
  FocusNext
  FocusPrev
  IncreaseValue
  DecreaseValue
  SpawnEntity
  ToggleDebug
```

Rule:

```text
actions are resolved in Update(), not in keybinding layer
```

---

## DEFAULT KEYMAP

### Global

```text
q / ctrl+c → Quit
p → Pause
space → ToggleMode
tab → FocusNext
shift+tab → FocusPrev
```

---

### Navigation

```text
arrow keys / hjkl → move
enter → select
esc → back
```

---

### Parameter Control

```text
+ / = → increase
- / _ → decrease
r → reset
```

---

### Debug

```text
d → toggle debug
l → toggle layers
m → toggle masks
z → z-index overlay
```

---

## MODE-BASED KEYMAPS

Bindings can vary by mode.

```text
Visualizer:
  navigation + toggles

Greenhouse:
  parameter editing + spawning

Debug:
  inspection tools
```

Rule:

```text
mode modifies meaning, not routing
```

---

## PANEL OVERRIDES

Panels can extend keymaps.

Example:

```text
Inspector:
  up/down → navigate entities

Parameters:
  +/- → adjust selected value
```

Rule:

```text
panel bindings only apply when focused
```

---

## BINDING STRUCTURE (BUBBLE TEA)

Bubble Tea supports structured key bindings using `key.Binding` citeturn0search4

Example:

```go
type KeyMap struct {
  Quit key.Binding
}
```

---

## HELP INTEGRATION

Bindings include descriptions:

```text
key → action → help text
```

Used for:

- help panel
- inline hints

---

## CONFIGURABILITY

Keybindings should be configurable.

```text
config:
  keymap.json / yaml
```

Features:

- remapping
- multiple presets
- user customization

---

## CONFLICT RESOLUTION

Rules:

- global bindings override panel bindings
- focused panel overrides non-focused
- last match wins

---

## DESIGN CONSTRAINTS

Avoid:

- hardcoded key logic in Update()
- duplicate bindings
- hidden shortcuts

Prefer:

- centralized keymap
- declarative structure
- consistent patterns

---

## FINAL PRINCIPLE

```text
Keybindings define meaning
Input system delivers events
Runtime executes actions
```

---

## NEXT

→ optional: themes / layout / presets
