# YAM v2 — COMMAND SYSTEM

---

## PURPOSE

Define the Command System as the **text-driven interaction layer** that enables direct, expressive control over the YAM ecosystem.

This document specifies:

- command syntax
- parsing model
- execution flow
- integration with TUI runtime and engine

---

## CORE PRINCIPLE

The command system is:

> a structured language for expressing user intent

It does not:

- replace keybindings
- bypass runtime

It only:

> provides a high-bandwidth interface for advanced interaction

---

## POSITION IN ARCHITECTURE

```text
User → Command Input → Parser → Action → TUI Runtime → Engine
```

---

## ACTIVATION

Command mode is entered via:

```text
: (colon)
```

Behavior:

- opens input line
- captures text input
- suspends normal keybindings

---

## INPUT MODEL

```text
:command arg1 arg2 key=value
```

Examples:

```text
:spawn vine density=high
:set growth_rate 1.2
:toggle masks
:focus inspector
:mode greenhouse
```

---

## COMMAND STRUCTURE

```text
Command:
  name
  args[]
  flags{}
```

---

## PARSING PIPELINE

```text
raw string
→ tokenize
→ parse command
→ validate
→ map to action
```

---

## ACTION MAPPING

Commands map to existing action system.

```text
:pause → TogglePause
:quit → Quit
:spawn → SpawnEntity
```

Rule:

```text
command system reuses action layer
```

---

## COMMAND TYPES

### 1. SYSTEM COMMANDS

```text
:quit
:reset
:mode
```

---

### 2. ENGINE COMMANDS

```text
:spawn
:kill
:set
:step
```

---

### 3. UI COMMANDS

```text
:focus
:toggle panel
:layout
```

---

### 4. DEBUG COMMANDS

```text
:toggle masks
:toggle layers
:show z
:stats
```

---

## ARGUMENT HANDLING

Arguments support:

```text
positional → spawn vine
named → density=high
flags → --force
```

---

## VALIDATION

Commands must be validated before execution.

```text
unknown command → error
invalid args → error
```

Errors are returned to UI.

---

## EXECUTION FLOW

```text
command parsed
→ mapped to action
→ dispatched to runtime
→ handled in Update()
```

---

## FEEDBACK SYSTEM

Command results are displayed in:

- info panel
- command line output

Example:

```text
> spawned vine (density=high)
```

---

## HISTORY

Command history is stored:

```text
previous commands
↑ / ↓ to navigate
```

---

## AUTOCOMPLETE (FUTURE)

Support:

- command suggestions
- argument hints
- inline help

---

## MODAL BEHAVIOR

While in command mode:

- normal keybindings disabled
- text input active
- enter → execute
- esc → cancel

---

## SECURITY / SAFETY

Rules:

- commands cannot corrupt state
- destructive commands require confirmation (optional)

---

## PERFORMANCE RULES

- parsing must be lightweight
- execution deferred to Update()

---

## DESIGN CONSTRAINTS

Avoid:

- overly complex syntax
- hidden commands
- inconsistent naming

Prefer:

- short, readable commands
- consistent verbs
- composable arguments

---

## FINAL PRINCIPLE

```text
Commands provide precision control over the system
Complementing keybindings and panels
```

---

## NEXT

→ themes / layout presets / implementation phase
