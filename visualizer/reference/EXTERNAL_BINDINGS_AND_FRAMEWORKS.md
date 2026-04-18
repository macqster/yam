# External Bindings and Frameworks

This table records the most useful implementation and workflow lessons from the newer set of external repos.

## Bindings and adapters

| Project | What it is | Main lesson | What YAM should borrow |
|---|---|---|---|
| `chafa.py` | Python bindings for Chafa | A thin, readable API around Chafa makes terminal image rendering easier to use from Python | Small wrapper surfaces, simple examples, readable conversion APIs |
| `chafa-wasm` | WebAssembly module for Chafa | The same renderer can be adapted to multiple runtimes with a clean common core | Separation between core conversion logic and runtime adapters |
| `chafa-go` | Go bindings for Chafa | A no-CGO binding layer lowers friction for adoption | Low-friction bindings, dependency-light adapters, clear examples |
| `img_term` | Simple ANSI image/video/camera tool | A tiny CLI with explicit width/palette knobs is enough for many use cases | Small command surface, obvious options, easy media replay |

## App frameworks and terminal engines

| Project | What it is | Main lesson | What YAM should borrow |
|---|---|---|---|
| `batgrl` | Terminal graphics library / widget-oriented runtime | A graphics toolkit can serve games, simulations, and full apps if the runtime is explicit | Clear app skeleton, docs/examples, and explicit scene/runtime separation |
| `TermGL` | 2D/3D terminal graphics engine | Higher-level rendering features need a corresponding demo/programming model | Example-driven feature discovery, clear compile/runtime flags, and demo-first docs |
| `AppCUI-rs` | Cross-platform Rust CUI/TUI framework | Strong layout, widgets, and galleries are a big part of toolkit usability | Explicit composable layout, gallery-style examples, and robust runtime primitives |

## Common pattern

Across these projects, the recurring pattern is:

- keep the core API small and explicit
- make examples and docs first-class
- separate conversion/adaptation from the core renderer when possible
- expose width, color, and layout controls directly
- keep runtime loops and input handling visible instead of hidden

## YAM implication

YAM should keep investing in:

- scene composition and layering
- thin runtime adapters where needed
- explicit layout and width handling
- examples and presets for repeatability
- small command surfaces for conversion-style tasks

It should not drift toward a generic toolkit or a monolithic engine.
