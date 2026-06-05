<div align="center">

<img src="assets/isthatayam_github.gif" alt="YAM animated terminal diorama preview">

  <h1>yam</h1>

  <p>Rust + Ratatui terminal diorama engine</p>

  <p>
    <img src="https://img.shields.io/badge/release-0.3.9-6f89a2" alt="release 0.3.9" />
    <img src="https://img.shields.io/badge/next-0.4%20greenhouse-7fa36b" alt="next 0.4 greenhouse" />
    <img src="https://img.shields.io/badge/runtime-Rust%20%2B%20Ratatui-8fb8c8" alt="Rust and Ratatui runtime" />
    <img src="https://img.shields.io/badge/posture-personal%20sandbox-879078" alt="personal sandbox posture" />
  </p>
</div>

<!-- cspell:ignore Arbaro asciiquarium aseprite Bacall braille cbonsai Chafa Dini DitherArt eco figlets Ghostty GreenLab HighGrow Infantino isthatayam Kanigher Krieg likness LinuxPorn Moldoff Moebius Naylor pplant Ratatui Riba Stanwyck Timm TNBA unixart unixporn Wray Wrigh wttr XBIN twimc rewatching -->

I was dealing with some dark stuff, so I tried to focus on something gentle enough to let my mind rest a little. Voilà: `yam`, a Rust/Ratatui terminal screensaver-like diorama sandbox engine: a Chafa-driven animated Ivy clip, world-space scene projection, compact companion surfaces, BTAS-inspired UI, and a maintenance-first path toward richer procedural scene life.

First I just wanted some benign animation on my screen to blankly stare at, but, as I said, I needed some distraction and it became a pet project of mine. At that time I was rewatching DC animations, reading stuff by Paul Dini and Bruce Timm, and trying to keep up with the awesome contemporary CLI/TUI scene – hence `yam`.

For some nostalgic reference ghosts in here, intentional or otherwise, check the acknowledgements section at the end; I hope nothing was stolen too explicitly. Feel free to yell at me if it was.

Codex is doing the hard work; life is unfortunately consuming too much time and attention, so my headspace to hand-write all this is limited.

> - `yam` is not meant as an end-user product or any kind of dashboard framework
> - `yam` is – and probably will be – in constant development, so nothing here is set in stone and everything is subject to change
> - the only real readers are me, Codex, and you, dear curious person, so keep in mind that the rest of this README is only a compact, rather machine-readable orientation sheet

<div align="center">

# twimc: some systematized info

</div>

current release: `0.3.9`

<div align="center">

## snapshot

</div>

- `next_track`: `0.4 greenhouse ecosystem expansion`
- `repo_mode`: `personal sandbox / controlled stabilization`
- `canonical_runtime`: `cargo run --release`
- `canonical_maintenance_gate`: `bash scripts/verify.sh`
- `project_type`: `Rust + Ratatui terminal diorama engine`
- `primary_goal`: `coherent world-space visualizer, not widget pileup`

<div align="center">

## current state

</div>

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

<div align="center">

## commands

</div>

clone and run the default world:

```bash
git clone https://github.com/macqster/yam.git
cd yam
cargo run --release
```

run the sandbox world:

```bash
cargo run --release -- --sandbox
```

check runtime identity:

```bash
cargo run --release -- --identity
```

run the full maintenance gate:

```bash
bash scripts/verify.sh
```

<div align="center">

## local launchers

</div>

```bash
bash scripts/update.sh
```

<div align="center">

| command | role |
| --- | --- |
| `yam` | canonical local launcher |
| `yam-sandbox` | sparse sandbox launcher |
| `yam-rust` | direct Rust runtime launcher |
| `yam-install` | rebuild/reinstall wrapper flow |
| `yam-diagnostics` | local diagnostics reader/tailer |

</div>

wrapper behavior that matters:

- `yam` and `yam-sandbox` prefer the installed `yam-rust` binary
- if repo runtime inputs are newer, wrappers refresh through `scripts/update.sh`
- `YAM_USE_REPO_RUN=1` forces the older direct `cargo run --release` path
- `YAM_DIAGNOSTICS=1` writes local NDJSON diagnostics

<div align="center">

## current priorities

</div>

1. `stability`
2. `efficiency`
3. `runtime/docs contract cleanup`
4. `new surface work only behind stable seams`

pressure points:

- `core::spatial` consolidation
- hero startup and cache efficiency
- theme and surface convergence
- runtime identity and docs sync
- flora and greenhouse work only behind stable scene/render ownership

<div align="center">

## canonical docs

</div>

read these first:

- [AGENTS.md](AGENTS.md) - repo-local agent contract
- [docs/README.md](docs/README.md) - docs routing map
- [TODO.md](TODO.md) - active execution backlog
- [known_issues.md](known_issues.md) - concrete unresolved issues only
- [docs/audit.md](docs/audit.md) - current risk and drift snapshot
- [docs/LOG.md](docs/LOG.md) - append-only project history

core contracts:

- [docs/architecture.md](docs/architecture.md) - ownership and coupling rules
- [docs/scene-model.md](docs/scene-model.md) - scene/world behavior
- [docs/rendering.md](docs/rendering.md) - layer and UI/render contracts
- [docs/glossary.md](docs/glossary.md) - shared terminology
- [docs/hygiene.md](docs/hygiene.md) - repo hygiene rules

active surface docs:

- [docs/greenhouse-roadmap.md](docs/greenhouse-roadmap.md) - greenhouse 0.4 plan
- [docs/main-scene-scaffold.md](docs/main-scene-scaffold.md) - hero support scaffold direction
- [docs/vines.md](docs/vines.md) - vine ownership and readiness
- [docs/weather-widget.md](docs/weather-widget.md) - weather ownership contract
- [docs/hero-cache.md](docs/hero-cache.md) - hero cache/runtime path

<div align="center">

## repo shape

</div>

<div align="center">

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
| `docs/` | active contracts, notes, archive entry points |

</div>

<div align="center">

## working rules

</div>

- keep terminology aligned with [docs/glossary.md](docs/glossary.md)
- keep active behavior contracts under `docs/`
- keep `TODO.md` execution-focused
- keep `known_issues.md` issue-focused
- keep `docs/audit.md` risk-focused
- keep `docs/LOG.md` historical and append-only
- keep build output and runtime cache artifacts out of the repo
- update README claims only when runtime behavior and docs support them

<div align="center">

## environment notes

</div>

- UTF-8 braille support is required for hero rendering
- full-color terminal output is recommended
- tested mostly in Kitty-family and Ghostty-like terminals
- local reference window: Ghostty `120x31`, roughly `124x32` usable cells on
  the current macOS setup

<div align="center">

## acknowledgements

</div>

- front gif was lifted from a brief shot in [*Batman and Harley Quinn*](https://www.imdb.com/title/tt6556890/reference/), directed by Sam Liu, written by Jim Krieg and Bruce Timm
- `Chafa`-rendered gif was hand-redrawn from one shot in [*The New Batman Adventures*](https://www.imdb.com/title/tt0118266/reference/) (s01e01 — "Holiday Knights"), written by Paul Dini and directed by Dan Riba
- for Poison Ivy herself, credit to Robert Kanigher and Carmine Infantino, with Sheldon Moldoff important to her first published appearance
- with all due respect to Lynne Naylor's Ivy design from ~1991, the absolute best rendition of our beloved eco-terrorist came from Bruce Timm's magic fingers during the broad character redesign between BTAS and TNBA; Timm's talent for capturing the charisma of figures such as Lauren Bacall, Veronica Lake, and/or Rita Hayworth worked here much better than Naylor's attempt to render the likeness of figures such as Barbara Stanwyck, Teresa Wright, and/or Fay Wray
- `yam` is personal glue code, but it mostly stands on other people's tools, ideas, interfaces, and visual languages: direct technical thanks to [Rust](https://www.rust-lang.org/) and the wider Rust ecosystem, [Ratatui](https://ratatui.rs/) for the terminal UI foundation, [Chafa](https://hpjansson.org/chafa/) for making terminal image and animation rendering practical enough to build around, [wttr.in](https://github.com/chubin/wttr.in) for the console-weather lineage and weather-widget inspiration, and [`tachyonfx`](https://github.com/junkdog/tachyonfx) and the broader Ratatui ecosystem for terminal presentation patterns; terminal-art and source-workflow nods to [`cbonsai`](https://gitlab.com/jallbrit/cbonsai), [`rbonsai`](https://github.com/mattn/rbonsai), [`asciiquarium`](https://robobunny.com/projects/asciiquarium/html/), [Moebius / MoebiusXBIN](https://github.com/blocktronics/moebius), [REXPaint](https://www.gridsagegames.com/rexpaint/), [FIGlet](http://www.figlet.org/), ANSI/ASCII/braille-art workflows, ricing culture, screensavers, terminal art toys, tiny idle worlds, and all the small digital things people build mostly because they are pleasant to look at
- greenhouse and plant-simulation reference nods to [`HighGrow`](https://highgrow.informer.com/4.2/), [Viridi](https://store.steampowered.com/app/375950/Viridi/), [OpenAlea](https://openalea.readthedocs.io/), [L-Py](https://github.com/openalea/lpy), [Arbaro](https://sourceforge.net/projects/arbaro/), [GreenLab](https://en.wikipedia.org/wiki/Simulated_growth_of_plants), [PowerPlant / pplant](https://sourceforge.net/projects/pplant/), [Algorithmic Botany](http://algorithmicbotany.org/), [L-systems](https://en.wikipedia.org/wiki/L-system), space-colonization growth, cellular automata, and agent-based growth literature as deferred plant-form and growth-rule lineage
- once again, aesthetic and conceptual nods to [Paul Dini](https://en.wikipedia.org/wiki/Paul_Dini), [Bruce Timm](https://en.wikipedia.org/wiki/Bruce_Timm), the broader [DC animated-universe](https://en.wikipedia.org/wiki/DC_Animated_Universe) design lineage, BTAS/TNBA palette discipline, cel-animation color restraint, and lo-fi animation loop creators
- and thanks to all the wonderful — and not so wonderful, but still helpful — redditors I have learned from on [r/ASCII](https://www.reddit.com/r/ASCII/), [r/aseprite](https://www.reddit.com/r/aseprite/), [r/CLI](https://www.reddit.com/r/CLI/), [r/codex](https://www.reddit.com/r/codex/), [r/commandline](https://www.reddit.com/r/commandline/), [r/coolgithubprojects](https://www.reddit.com/r/coolgithubprojects/), [r/DitherArt](https://www.reddit.com/r/DitherArt/), [r/fastfetch](https://www.reddit.com/r/fastfetch/), [r/Ghostty](https://www.reddit.com/r/Ghostty/), [r/learnrust](https://www.reddit.com/r/learnrust/), [r/linuxmemes](https://www.reddit.com/r/linuxmemes/), [r/LinuxPorn](https://www.reddit.com/r/LinuxPorn/), [r/low_poly](https://www.reddit.com/r/low_poly/), [r/PixelArt](https://www.reddit.com/r/PixelArt/), [r/programming](https://www.reddit.com/r/programming/), [r/programminggames](https://www.reddit.com/r/programminggames/), [r/terminal_porn](https://www.reddit.com/r/terminal_porn/), [r/textmode](https://www.reddit.com/r/textmode/), [r/tui](https://www.reddit.com/r/tui/), [r/unixart](https://www.reddit.com/r/unixart/), [r/unixporn](https://www.reddit.com/r/unixporn/), [r/webdev](https://www.reddit.com/r/webdev/), and [r/zsh](https://www.reddit.com/r/zsh/)
- nothing here claims affiliation, endorsement, or ownership over those works or communities, and if something feels credited badly, named poorly, or stolen too explicitly, please yell at me
