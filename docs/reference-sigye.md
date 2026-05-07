# Reference Study: sigye

This note captures the parts of [`sigye`](https://github.com/am2rican5/sigye) that are worth learning from for YAM.

The goal is not to copy its whole application shape. The goal is to identify subsystems that transfer cleanly into YAM:

- wide FIGlet font support
- transparent ASCII-art rendering
- clean hotkey hint formatting such as `↑↓ nav  ←→ change`

## What sigye Is Doing Well

### 1. It splits the app into small crates

`sigye` uses a workspace with narrowly scoped crates:

- `sigye-core`
- `sigye-config`
- `sigye-fonts`
- `sigye-background`
- `sigye`

That split matters because the FIGlet system is not entangled with the TUI shell. The fonts live in their own crate and can be reasoned about independently.

For YAM, the lesson is not “make a workspace right now.” The lesson is:

- treat font parsing and font selection as a subsystem
- keep it out of scene layers as much as possible

### 2. FIGlet support is a real subsystem, not an inline trick

In `sigye`, the font path is straightforward:

- bundled `.flf` files are embedded with `include_str!`
- a parser reads `.flf` / `.tlf`
- a `Font` type stores character definitions
- a `FontRegistry` owns loaded fonts and font lookup
- rendering asks the selected `Font` to render text

Key files:

- `crates/sigye-fonts/src/parser.rs`
- `crates/sigye-fonts/src/font.rs`
- `crates/sigye-fonts/src/registry.rs`
- `crates/sigye-fonts/src/bundled.rs`

The core idea is simple and good:

- parse once
- store structured character rows
- render text from the selected font

That is a much healthier direction than hand-maintaining one-off ASCII title literals forever.

### 3. Rendering stays transparent

`sigye`’s shared text renderer writes directly into the frame buffer and skips spaces:

- non-space glyphs get foreground color
- spaces do not erase the background

That is very aligned with YAM’s current loading-screen direction.

The main helper is in:

- `crates/sigye/src/render.rs`

The important lesson is not the exact API. The lesson is:

- “ASCII title rendering” should be treated as a renderer concern with transparent semantics

### 4. Hotkey hints are composed from styled spans

This is one of the cleanest parts of `sigye`.

Its dialogs build help lines like:

- accent-colored bold tokens: `↑↓`, `←→`, `Enter`, `Esc`
- dimmed descriptive glue: `nav`, `change`, `save`, `cancel`

Examples:

- `crates/sigye/src/settings.rs`
- `crates/sigye/src/mode_dialog.rs`
- `crates/sigye/src/countdown_dialog.rs`

The result is readable because:

- controls and meaning are visually distinct
- spacing is deliberate
- the help string is treated as a composed line, not a raw sentence

This maps directly onto YAM.

## What We Should Emulate In YAM

### A. A dedicated FIGlet subsystem

Smallest good YAM shape:

- `src/render/figlet/` or `src/render/figlet.rs`
- `FigletFont`
- `FigletRegistry`
- `parse_flf(...)`
- `render_text(...) -> Vec<String>`

Initial scope should stay intentionally small:

- bundled fonts only
- ASCII `32..=126`
- transparent rendering semantics
- no live external font loading yet

That would already be enough to stop treating the boot wordmark as a fragile literal.

### B. A reusable text-art render helper

YAM should have one shared helper for transparent ASCII-art drawing, conceptually similar to `sigye`’s renderer:

- center or anchor a multiline text block
- skip spaces
- write only visible glyphs
- leave the world/background intact underneath

That helper should be usable for:

- boot/loading titles
- future labels
- possibly greenhouse room titles or ceremonial overlays

### C. A structured hotkey-hint formatter

YAM’s modal/help lines should stop being treated as plain text strings wherever richer formatting matters.

Smallest good shape:

- a helper that takes pairs like:
  - `("↑↓", "nav")`
  - `("←→", "change")`
  - `("Enter", "edit")`
  - `("Esc", "close")`
- emits a styled line with:
  - accent style for tokens
  - dim style for descriptions
  - fixed spacing between pairs

That would give us cleaner hints across:

- settings
- move
- hotkeys
- future greenhouse/lab tools

## What Not To Copy Blindly

There are also boundaries worth keeping.

### 1. YAM does not need sigye’s whole app topology

`sigye` is a multi-mode application with:

- clock
- pomodoro
- timer
- stopwatch
- world clock
- countdown

YAM should not inherit that mode taxonomy. The useful transfer is infrastructural, not conceptual.

### 2. YAM should keep its own scene/layer contract

`sigye` renders through direct ratatui frame composition for its application shell.

YAM already has stronger scene ownership rules:

- `RenderState`
- `Scene`
- explicit layers
- world / HUD / overlay split

We should preserve that and only borrow subsystem ideas.

### 3. External font loading can wait

`sigye` supports loading custom fonts from config directories.

That is nice, but not needed for the first YAM slice.

For YAM, first wins are:

- bundled fonts
- deterministic selection
- one trusted boot-title rendering path

## Recommended YAM Implementation Order

### Phase 1: FIGlet core

- add a tiny parser and font model
- embed a small initial font set
- render text to `Vec<String>`

### Phase 2: Transparent FIGlet renderer

- add a shared render helper for multiline transparent text
- migrate boot/loading title rendering to use it

### Phase 3: Hotkey-hint formatter

- add a reusable styled-hint helper
- update settings/help/hotkeys overlays to use formatted hints like:
  - `↑↓ nav  ←→ change`

### Phase 4: Settings integration

- add a font row in the appropriate settings tab
- allow cycling through bundled FIGlet fonts for the loading title or other eligible text-art surfaces

## Current Conclusion

`sigye` is worth treating as a reference for subsystem shape, not as a template for the whole YAM application.

The strongest transfer ideas are:

1. FIGlet support as a real registry/parser/render subsystem
2. transparent direct glyph rendering that skips spaces
3. composed hotkey help lines with strong token/description contrast

Those three are enough to justify a proper YAM follow-up slice.
