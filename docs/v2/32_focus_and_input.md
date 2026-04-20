# YAM v2 — FOCUS & INPUT

---

## PURPOSE

Define the Focus & Input system as the **interaction layer that translates user input into structured actions within the TUI runtime**.

This document specifies:

- key handling
- focus routing
- command patterns
- interaction hierarchy

---

## CORE PRINCIPLE

Input system is:

> a message-driven routing system built on Bubble Tea

It does not:

- directly mutate engine state
- bypass runtime

It only:

> translates input → messages → routed actions

---

## POSITION IN ARCHITECTURE

```text
User Input → Bubble Tea Msg → TUI Runtime → Panels / Global Handlers
```

---

## MESSAGE MODEL

Bubble Tea delivers input as messages.

Primary type:

```text
tea.KeyMsg
```

Handled inside Update(): citeturn0search2

---

## INPUT FLOW

```text
Key press
→ Bubble Tea generates Msg
→ Update(msg)
→ runtime interprets
→ routed to target
```

---

## INPUT HIERARCHY

Input is resolved in priority order:

```text
1. Global shortcuts
2. Focused panel
3. Fallback (ignored)
```

---

## GLOBAL SHORTCUTS

Always active regardless of focus.

Examples:

```text
q / ctrl+c → quit
p → pause simulation
space → toggle mode
```

---

## PANEL INPUT

If not handled globally:

```text
route to focused panel
```

Example:

```text
Inspector:
  arrow keys → navigate nodes

Parameters:
  +/- → modify values
```

---

## FOCUS SYSTEM

Only one panel is active at a time.

```text
focus = panel_id
```

---

### Focus Switching

```text
tab → next panel
shift+tab → previous panel
```

---

### Focus Rules

- only focused panel receives input
- non-focused panels are passive
- visual highlight indicates focus

---

## KEY MAPPING

Keys can be matched by:

```text
msg.String()
msg.Type
```

Example:

```go
case tea.KeyMsg:
  switch msg.String() {
    case "q":
      return m, tea.Quit
    case "tab":
      focusNext()
  }
```

---

## COMMAND SYSTEM

Input triggers actions via commands.

```text
action → state change OR command
```

Examples:

```text
pause → toggle engine flag
spawn → emit SpawnMsg
```

---

## MODAL BEHAVIOR

Modes affect input interpretation.

```text
Visualizer mode → navigation
Greenhouse mode → parameter editing
Debug mode → toggles
```

Rule:

```text
same key may have different meaning per mode
```

---

## ADVANCED INPUT (FUTURE)

Bubble Tea supports:

- key down / key up events
- mouse input
- scroll

These enable:

- drag interactions
- continuous actions
- fine control citeturn0search0

---

## INPUT STATE

Optional tracking:

```text
pressed_keys
repeat_state
modifiers (ctrl, alt, shift)
```

---

## PERFORMANCE RULES

- keep input handling lightweight
- avoid blocking Update()
- prefer simple key matching

---

## DESIGN CONSTRAINTS

Avoid:

- deep input nesting
- panel-specific hacks
- direct engine mutation

Prefer:

- centralized routing
- consistent key mappings
- predictable behavior

---

## FINAL PRINCIPLE

```text
Input is intent
Routing gives it meaning
Focus defines its target
```

---

## NEXT

→ polish / implementation phase
