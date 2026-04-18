# External Inspiration Comparison

This table condenses the useful takeaways from the external repos into one view.

| Project | What it is | Main lesson | What YAM should borrow |
|---|---|---|---|
| `ruscii` | Small terminal graphics/game runtime | Keep the app loop, input, timing, and terminal lifecycle explicit | A cleaner app skeleton, example-first docs, stderr debug logging |
| `Term-Graphics` | Header-only terminal graphics library | Make shapes, text, sprites, and hot reload part of the core workflow | Small launch surface, explicit runtime plumbing, reusable examples |
| `pyTermTk` | Python TUI toolkit | Layout systems are easier to use when they are explicit and composable | Composable layout concepts, example-driven docs, sandbox-style preview ideas |
| `ascii-art` | ANSI composition library | Treat terminal content as a composited pipeline with width-aware output | Explicit layering, width-aware composition, modular small-purpose tools |

## Common pattern

Across the set, the recurring theme is:

- make the runtime loop explicit
- make layout and composition first-class
- keep debug output separate from rendered output
- use examples and presets as the main onboarding path

## YAM implication

YAM should stay focused on:

- scene composition
- mask/field-driven layout
- reproducible presets
- debug/introspection tooling

It should not drift toward being a generic terminal toolkit or converter.
