# Source Index

This is the canonical reference list for the visualizer work.

Use this whenever you need to point to the underlying sources, external inspiration repos, or the internal docs that summarize them.

## Core YAM docs

| Item | Purpose |
|---|---|
| [README.md](README.md) | Main entry point and navigation hub |
| [STATUS.md](STATUS.md) | Current maintenance snapshot |
| [VOCABULARY.md](VOCABULARY.md) | Canonical terminology |
| [CONFIG.md](CONFIG.md) | Full config manual |
| [WORKFLOW.md](WORKFLOW.md) | Day-to-day operating guide |
| [TUNING_CHECKLIST.md](TUNING_CHECKLIST.md) | Short tuning order checklist |
| [MASKS_AND_GUIDES.md](MASKS_AND_GUIDES.md) | Spatial model and field/mask notes |
| [DEV_TOOLS.md](DEV_TOOLS.md) | Development tooling roadmap |
| [PROJECT_PROCESS.md](PROJECT_PROCESS.md) | Working conventions and process |
| [reference/RESEARCH.md](reference/RESEARCH.md) | Archive of architecture and ecosystem research |

## External inspiration and reference docs

| Item | Purpose |
|---|---|
| [reference/EXTERNAL_YAM_SHOULD_COPY.md](reference/EXTERNAL_YAM_SHOULD_COPY.md) | One-page summary of what YAM should adopt |
| [reference/EXTERNAL_COMPARISON_TABLE.md](reference/EXTERNAL_COMPARISON_TABLE.md) | Compact comparison across the main external repos |
| [reference/EXTERNAL_BINDINGS_AND_FRAMEWORKS.md](reference/EXTERNAL_BINDINGS_AND_FRAMEWORKS.md) | Bindings, adapters, and framework lessons |
| [reference/EXTERNAL_TERMINAL_LIBRARIES.md](reference/EXTERNAL_TERMINAL_LIBRARIES.md) | Legacy pointer to the comparison table for terminal-graphics libs |
| [reference/EXTERNAL_TUI_TOOLKITS.md](reference/EXTERNAL_TUI_TOOLKITS.md) | Legacy pointer to the comparison table for TUI frameworks |
| [reference/EXTERNAL_ASCII_ART.md](reference/EXTERNAL_ASCII_ART.md) | Legacy pointer to the comparison table for ANSI composition libs |
| [reference/EXTERNAL_INSPIRATIONS.md](reference/EXTERNAL_INSPIRATIONS.md) | Legacy pointer to the comparison table for runtime/workflow inspiration |

## External repos studied

| Repo | What it contributed |
|---|---|
| <https://github.com/Zebbeni/ansizalizer> | Pipeline-based image-to-ANSI rendering, density mapping, alpha handling |
| <https://github.com/JVSCHANDRADITHYA/buddy> | Two-pixel cell thinking, color-first rendering, area averaging |
| <https://github.com/lemunozm/ruscii> | Explicit app loop, terminal lifecycle, example-first onboarding |
| <https://github.com/Rrrinav/Term-Graphics> | Header-only runtime, explicit shapes/text/input/window surface, hot-reload workflow |
| <https://github.com/ceccopierangiolieugenio/pyTermTk> | Explicit composable layout, widgets, sandbox/browser tooling, Unicode layout concerns |
| <https://github.com/khrome/ascii-art> | ANSI composition pipeline, width-aware output, modular composition |
| <https://github.com/GuardKenzie/chafa.py> | Readable Python bindings around Chafa |
| <https://github.com/hectorm/chafa-wasm> | Shared Chafa core adapted to multiple runtimes |
| <https://github.com/ploMP4/chafa-go> | Low-friction Go bindings without CGO |
| <https://github.com/salt-die/batgrl> | Terminal graphics runtime with app/widget framing |
| <https://github.com/wojciech-graj/TermGL> | Terminal graphics engine with 2D/3D demos and feature flags |
| <https://github.com/gdt050579/AppCUI-rs> | Cross-platform CUI/TUI framework with strong layout and galleries |
| <https://github.com/JonnoFTW/img_term> | Small ANSI image/video/camera CLI with width and palette controls |

## External source of truth

For image-to-text conversion, Chafa remains the underlying upstream project used by several of the bindings and adapters above.

## How to use this index

- start with [reference/EXTERNAL_YAM_SHOULD_COPY.md](reference/EXTERNAL_YAM_SHOULD_COPY.md) for the compressed conclusions
- use [reference/EXTERNAL_COMPARISON_TABLE.md](reference/EXTERNAL_COMPARISON_TABLE.md) to compare repos quickly
- open the repo list here when you need the original source links
- use [reference/RESEARCH.md](reference/RESEARCH.md) when you need the longer archive context

## Archive docs

| Item | Purpose |
|---|---|
| [archive/AUDIT.md](archive/AUDIT.md) | Historical audit notes |
| [archive/ENGINE_SPEC.md](archive/ENGINE_SPEC.md) | Older engine-spec framing |
| [archive/LEAF_STUDY.md](archive/LEAF_STUDY.md) | Leaf / ornament study notes |
| [archive/PATCHLOG.md](archive/PATCHLOG.md) | Historical patch log |
| [archive/REFACTOR_GUARDRAILS.md](archive/REFACTOR_GUARDRAILS.md) | Refactor safety notes |
| [archive/STATE_MODEL.md](archive/STATE_MODEL.md) | Older state-model framing |
| [archive/TUNING_MAP.md](archive/TUNING_MAP.md) | Historical tuning map |
