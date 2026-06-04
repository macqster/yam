<div align="center">

<img src="assets/isthatayam_github.gif" alt="Alt text">

  <h1>yam</h1>

  <p>Rust + Ratatui terminal diorama engine</p>

  <p>
    <img src="https://img.shields.io/badge/release-0.3.9-6f89a2" alt="release 0.3.9" />
    <img src="https://img.shields.io/badge/next-0.4%20greenhouse-7fa36b" alt="next 0.4 greenhouse" />
    <img src="https://img.shields.io/badge/runtime-Rust%20%2B%20Ratatui-8fb8c8" alt="Rust and Ratatui runtime" />
    <img src="https://img.shields.io/badge/posture-personal%20sandbox-879078" alt="personal sandbox posture" />
  </p>
</div>

<!-- cspell:ignore Dini Timm twimc isthatayam -->

I was dealing with some dark stuff, so I tried to focus on something gentle enough to let my mind rest a little. Voilà: yam, a Rust/Ratatui terminal screensaver-like diorama sandbox engine: a Chafa-driven animated Ivy clip, world-space scene projection, compact companion surfaces, BTAS-inspired UI, and a maintenance-first path toward richer procedural scene life.

First I just wanted some benign animation on my screen to stare at blankly. But, as I said, I needed distractions, so it became a pet project of mine. At that time I was watching DC animation again, reading stuff by Paul Dini and Bruce Timm, and trying to keep up with the awesome contemporary CLI/TUI scene. Hence yam.

Some nostalgic reference ghosts in here, intentional or otherwise: lo-fi girl animation loops, the cbonsai generative plant idea, HighGrow controlled greenhouse environment and plant lifecycle, ricing culture, terminal art toys, and many, many more. I hope nothing was stolen too explicitly. Feel free to yell at me if it was.

Codex is doing the hard work; life is unfortunately consuming too much time and attention, so the headspace to hand-write all this is limited.

> `yam` is not meant as an end-user product or any kind of dashboard framework <br>
The real readers are me, Codex, and you, dear curious person, so keep in mind, that the rest of this README is only a compact orientation sheet.

***

# twimc: some systematized info

Current release: `0.3.9`

## snapshot

- `next_track`: `0.4 greenhouse ecosystem expansion`
- `repo_mode`: `personal sandbox / controlled stabilization`
- `canonical_runtime`: `cargo run --release`
- `canonical_maintenance_gate`: `bash scripts/verify.sh`
- `project_type`: `Rust + Ratatui terminal diorama engine`
- `primary_goal`: `coherent world-space visualizer, not widget pileup`

## current state

- `worlds`: `boot`, `main`, `sandbox`
- `hero`: `Chafa-backed animated hero`
- `scene`: `world-space projection with read-only render layers`
- `companions`: `clock`, `weather`, `Polish date`
- `framing`: `scaffold` plus `vine` prototype
- `dev_surfaces`: `help`, `move`, `settings`, `palette`, `weather`, `quit confirm`
- `future_surfaces`:
  - `calendar companion seam`
  - `greenhouse/lab spaces`
  - `broader flora beyond current vine/scaffold prototype`
  - `terminal-art asset compilation and inspection workflows`

## commands

```bash
git clone https://github.com/macqster/yam.git
cd yam
cargo run --release
```

```bash
cargo run --release -- --sandbox
```

```bash
cargo run --release -- --identity
```

```bash
bash scripts/verify.sh
```

## local launchers

```bash
bash scripts/update.sh
```

| command | role |
| --- | --- |
| `yam` | canonical local launcher |
| `yam-sandbox` | sparse sandbox launcher |
| `yam-rust` | direct Rust runtime launcher |
| `yam-install` | rebuild/reinstall wrapper flow |
| `yam-diagnostics` | local diagnostics reader/tailer |

Wrapper behavior that matters:

- `yam` and `yam-sandbox` prefer the installed `yam-rust` binary
- if repo runtime inputs are newer, wrappers refresh through `scripts/update.sh`
- `YAM_USE_REPO_RUN=1` forces the older direct `cargo run --release` path
- `YAM_DIAGNOSTICS=1` writes local NDJSON diagnostics

## current priorities

1. `stability`
2. `efficiency`
3. `runtime/docs contract cleanup`
4. `new surface work only behind stable seams`

Pressure points:

- `core::spatial` consolidation
- hero startup and cache efficiency
- theme and surface convergence
- runtime identity and docs sync
- flora and greenhouse work only behind stable scene/render ownership

## canonical docs

Read these first:

- [AGENTS.md](AGENTS.md) - repo-local agent contract
- [docs/README.md](docs/README.md) - docs routing map
- [TODO.md](TODO.md) - active execution backlog
- [known_issues.md](known_issues.md) - concrete unresolved issues only
- [docs/audit.md](docs/audit.md) - current risk and drift snapshot
- [docs/LOG.md](docs/LOG.md) - append-only project history

Core contracts:

- [docs/architecture.md](docs/architecture.md) - ownership and coupling rules
- [docs/scene-model.md](docs/scene-model.md) - scene/world behavior
- [docs/rendering.md](docs/rendering.md) - layer and UI/render contracts
- [docs/glossary.md](docs/glossary.md) - shared terminology
- [docs/hygiene.md](docs/hygiene.md) - repo hygiene rules

Active surface docs:

- [docs/greenhouse-roadmap.md](docs/greenhouse-roadmap.md) - greenhouse 0.4 plan
- [docs/main-scene-scaffold.md](docs/main-scene-scaffold.md) - hero support scaffold direction
- [docs/vines.md](docs/vines.md) - vine ownership and readiness
- [docs/weather-widget.md](docs/weather-widget.md) - weather ownership contract
- [docs/hero-cache.md](docs/hero-cache.md) - hero cache/runtime path

## repo shape

| path | role |
| --- | --- |
| `src/core/` | world, spatial, guide, organism, greenhouse, flora primitives |
| `src/render/` | compositor, Chafa integration, hero cache, draw helpers |
| `src/scene/` | scene orchestration, camera/viewport, render layers |
| `src/ui/` | UI state, anchors, layer assembly, widgets |
| `src/weather/` | weather model, provider, normalization, layout, rendering |
| `src/theme/` | BTAS palette, glyphs, style, semantic render helpers |
| `src/systems/` | early simulation seams for growth, aging, density, constraints |
| `assets/` | runtime visual/font assets |
| `scripts/` | maintenance, verification, update utilities |
| `docs/` | active contracts, notes, archive entrypoints |

## working rules

- keep terminology aligned with [docs/glossary.md](docs/glossary.md)
- keep active behavior contracts under `docs/`
- keep `TODO.md` execution-focused
- keep `known_issues.md` issue-focused
- keep `docs/audit.md` risk-focused
- keep `docs/LOG.md` historical and append-only
- keep build output and runtime cache artifacts out of the repo
- update README claims only when runtime behavior and docs support them

## environment notes

- UTF-8 braille support is required for hero rendering
- full-color terminal output is recommended
- tested mostly in Kitty-family and Ghostty-like terminals
- local reference window: Ghostty `120x31`, roughly `124x32` usable cells on
  the current macOS setup
