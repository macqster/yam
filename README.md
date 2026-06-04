<img width="640" height="360" alt="YAM GitHub preview" src="https://github.com/user-attachments/assets/16c637d4-d2c6-4669-a506-597ea0015bb0" />

# yam

![release](https://img.shields.io/badge/release-0.3.9-6f89a2)
![rust](https://img.shields.io/badge/Rust/Ratatui-terminal%20diorama-8fb8c8)
![status](https://img.shields.io/badge/status-personal%20sandbox-879078)
![direction](https://img.shields.io/badge/next-0.4%20greenhouse-7fa36b)

I was dealing with some dark stuff, so I tried to focus on something gentle enough to let my mind rest a little. Voilà: `yam`, a Rust/Ratatui terminal screensaver-like diorama sandbox engine: a Chafa-driven animated Ivy gif, world-space scene projection, compact companion surfaces, BTAS-inspired UI, and a maintenance-first path toward richer procedural scene life.

<!-- cspell:ignore Dini Timm -->

First I just wanted some benign animation on my screen to stare at blankly. But, as I said, I needed distractions, so it became a pet project of mine. At that time I was watching DC animation again, reading stuff by `Paul Dini` and `Bruce Timm`, and trying to keep up with the awesome contemporary CLI/TUI scene. Hence `yam`.

Some nostalgic reference ghosts in here, intentional or otherwise: lo-fi girl animation loops, the `cbonsai` generative plant idea, `HighGrow` controlled greenhouse environment and plant lifecycle, ricing culture, terminal art toys, and many, many more. I hope nothing was stolen too explicitly. Feel free to yell at me if it was.

Codex is doing the hard work; life is unfortunately consuming too much time and attention, so the headspace to hand-write all this is limited.

---

## repo.status

Current release: `0.3.9`

- release: `0.3.9`
- next_track: `0.4 greenhouse ecosystem expansion`
- repo_mode: `personal sandbox / controlled stabilization`
- canonical_runtime: `cargo run --release`
- canonical_maintenance_gate: `bash scripts/verify.sh`

## repo.identity

- project_type: `Rust + Ratatui terminal diorama engine`
- primary_goal: `coherent animated terminal scene, not widget pileup`
- current_worlds:
  - `boot`
  - `main`
  - `sandbox`
- core_runtime_features:
  - `Chafa-backed animated hero`
  - `scene projection`
  - `clock/weather/date companions`
  - `vine/scaffold framing`
  - `help/settings/palette/weather/diagnostics/quit surfaces`
- aesthetic_contract:
  - `graphite-dark background`
  - `muted blue/green structure`
  - `warm companion text`
  - `compact HUD/footer`
  - `composed scene layout over panel density`

## repo.commands

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

## repo.launchers

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

## repo.priorities

1. `stability`
2. `efficiency`
3. `runtime/docs contract cleanup`
4. `new surface work only behind stable seams`

## repo.current_pressure_points

- `spatial relation cleanup`
- `hero startup and cache efficiency`
- `theme and surface convergence`
- `runtime identity and documentation sync`
- `flora expansion only behind stable scene/render contracts`

## repo.future_surfaces

- `calendar companion seam`
- `greenhouse/lab spaces`
- `flora expansion beyond current vine/scaffold prototype`
- `terminal-art asset compilation and inspection workflows`
- roadmap pointer: [docs/greenhouse-roadmap.md](docs/greenhouse-roadmap.md)

## repo.maintenance

| gate | purpose |
| --- | --- |
| `bash scripts/check-docs.sh` | doc existence, version sync, issue-link hygiene, optional markdown/spell checks |
| `bash scripts/check.sh` | `cargo fmt`, clippy, cargo check |
| `cargo test --quiet` | test suite |
| `bash scripts/verify.sh` | full maintenance/release gate |

## repo.map

| path | role |
| --- | --- |
| `src/core/` | world, grid, spatial, entity, flora primitives |
| `src/render/` | compositor, Chafa integration, hero cache, masks, draw helpers |
| `src/scene/` | scene orchestration, camera/viewport, render layers |
| `src/ui/` | UI state, anchors, layer assembly, reusable widgets |
| `src/weather/` | weather model/provider/normalization/layout/rendering |
| `src/theme/` | BTAS palette, glyphs, style, semantic render helpers |
| `src/systems/` | early simulation seams for growth, aging, density, fields, constraints, ticks |
| `assets/` | runtime visual/font assets |
| `scripts/` | maintenance, verification, update utilities |
| `docs/` | active contracts, design notes, release model, archive entrypoints |

## repo.canonical_docs

| path | role |
| --- | --- |
| [AGENTS.md](AGENTS.md) | repo-local agent contract |
| [skills/yam-maintenance/SKILL.md](skills/yam-maintenance/SKILL.md) | maintenance workflow skill |
| [skills/yam-architecture-review/SKILL.md](skills/yam-architecture-review/SKILL.md) | architecture review workflow skill |
| [docs/README.md](docs/README.md) | documentation routing map |
| [TODO.md](TODO.md) | active execution backlog |
| [known_issues.md](known_issues.md) | unresolved issue ledger |
| [docs/audit.md](docs/audit.md) | risk and drift snapshot |
| [docs/LOG.md](docs/LOG.md) | append-only project log |
| [docs/glossary.md](docs/glossary.md) | terminology authority |

## repo.rules

- keep terminology aligned with [docs/glossary.md](docs/glossary.md)
- keep active behavior contracts under `docs/`
- keep `TODO.md` execution-focused
- keep `known_issues.md` issue-focused
- keep `docs/audit.md` risk-focused
- keep `docs/LOG.md` append-only
- keep build output and runtime cache artifacts out of the repo
- update README claims only when runtime behavior and docs support them

## repo.environment

- encoding_requirement: `UTF-8 braille support required for hero rendering`
- color_requirement: `full-color terminal output recommended`
- tested_terminal_bias: `Kitty-family and Ghostty-like terminals`
- local_reference_window:
  - `ghostty_config: 120x31`
  - `observed_usable_cells: about 124x32 on current macOS setup`
