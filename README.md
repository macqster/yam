# yam

Personal macOS terminal scene repo: Ghostty + Fastfetch startup rice, plus a dedicated visualizer tuned for Ghostty.

This repo preserves the current working terminal rice and keeps the moving parts
small enough to reinstall and modify without reverse-engineering it later.

The setup is intentionally macOS-specific and personal.

## Visualizer mode

The repo now also includes a separate visualizer app under [visualizer/](visualizer).

This is intentionally not part of shell startup. It is a standalone terminal scene that combines:

- cached Chafa hero animation from a GIF
- slow procedural vines ornament
- compact time/date info panel

Run it from the repo root with:

```bash
./visualizer/run_visualizer.sh
```

After `./install.sh`, launch it with:

```bash
yam
```

See [visualizer/README.md](visualizer/README.md) for setup and tuning details.
See [visualizer/STATUS.md](visualizer/STATUS.md) for the current maintenance snapshot and vines-engine status.
See [visualizer/VOCABULARY.md](visualizer/VOCABULARY.md) for the canonical visualizer dictionary.

Current visualizer direction:
- tuned for Ghostty

## Ghostty organization

Ghostty is split into two layers in this repo:

- `ghostty/config` for behavior and shell integration
- `ghostty/themes/yam-dark` for the color theme

There is no Ghostty plugin layer here.
Shell plugins and prompt behavior live in the shell profile, not in the Ghostty repo.
tmux uses the terminal's raw `Ctrl+A` prefix directly; Ghostty should not synthesize a tmux prefix on top of it.
Ghostty split actions are Cmd-based by default: `Cmd+D` and `Cmd+Shift+D` split panes, while the Ctrl/Option bindings are for navigation and resizing.

## Fastfetch organization

Fastfetch is split into two layers in this repo:

- `fastfetch/config.jsonc` for the visible system panel
- `fastfetch/startup.zsh` for the startup gate and invocation

The startup hook runs from the shell profile when Ghostty starts an interactive shell.
The repo also tracks the Fastfetch mapping in [docs/fastfetch-map.md](docs/fastfetch-map.md).

## What this repo is

When Ghostty opens:

1. `zsh` starts.
2. `~/.config/fastfetch/startup.zsh` runs only in interactive Ghostty shells.
3. That startup hook calls `~/.local/bin/fastfetch-chafa`.
4. `fastfetch-chafa` renders a still ANSI logo using `chafa/chafa_lab.sh`.
5. The ANSI logo is passed to Fastfetch as a raw logo.
6. Fastfetch renders the boxed system-info panel beside it.
7. Control returns to the shell prompt.

The startup logo is static on purpose.
Animation is for manual Chafa testing, not shell startup.

## Repo layout

- `ghostty/`
  - `config`
  - `README.md`
  - `themes/`
    - `README.md`
    - `yam-dark`
- `fastfetch/`
  - `config.jsonc`
  - `startup.zsh`
  - `README.md`
- `docs/`
  - `fastfetch-map.md`
  - `startup-bridge.md`
  - `chafa-map.md`
  - `launcher-map.md`
- `bin/`
  - `fastfetch-chafa`
  - `yam`
- `chafa/`
  - `chafa_lab.sh`
- `assets/`
  - `ives_yam.png`
- `visualizer/`
  - standalone Ghostty visualizer app
- `install.sh`
- `Brewfile`

## File ownership

This section is the most important part of the repo.

### `ghostty/config`

Owns:
- Ghostty font
- Ghostty theme selection
- Ghostty padding
- Ghostty opacity
- Ghostty palette
- Ghostty cursor behavior

Edit this file when you want to change:
- the baseline Ghostty look and feel
- the terminal environment that `yam` is tuned against

Installed runtime path on macOS:
- `~/Library/Application Support/com.mitchellh.ghostty/config.ghostty`

The installer now symlinks this file back to the repo copy through App Support on macOS.

See also:
- [ghostty/README.md](ghostty/README.md)
- [ghostty/themes/README.md](ghostty/themes/README.md)

### `ghostty/themes/yam-dark`

Owns:
- the terminal color palette
- cursor color
- selection colors
- theme-level appearance values

Edit this file when you want to change:
- the baseline dark palette used by Ghostty
- the color identity of the terminal

Installed runtime path:
- `~/.config/ghostty/themes/yam-dark`

The base Ghostty config selects this theme by name.

### `fastfetch/config.jsonc`

Owns:
- Fastfetch module layout
- Fastfetch box drawing / custom sections
- module keys / icons / labels
- module colors
- logo padding inside Fastfetch
- command-based custom module rendering

Edit this file when you want to change:
- what system stats are shown
- how the right-side panel is grouped
- section titles
- icons
- box decorations
- value formatting

This is the main file for the visible startup panel on the right.

See also:
- [fastfetch/README.md](fastfetch/README.md)
- [docs/fastfetch-map.md](docs/fastfetch-map.md)

### `fastfetch/startup.zsh`

Owns:
- when startup rendering happens
- how many blank lines appear before / after startup
- gating by shell conditions

Edit this file when you want to change:
- whether startup runs at all
- whether it runs only in Ghostty
- prompt spacing around startup

Do not put large rendering logic in `~/.zshrc`.
Keep the logic here and source it from `.zshrc`.

See also:
- [fastfetch/README.md](fastfetch/README.md)
- [docs/fastfetch-map.md](docs/fastfetch-map.md)
- [docs/startup-bridge.md](docs/startup-bridge.md)

### `bin/fastfetch-chafa`

Owns:
- how the startup logo is generated
- which Chafa script is called
- the still-logo render size used for startup
- conversion of the Chafa output into a raw Fastfetch logo file

Edit this file when you want to change:
- startup logo size
- whether startup uses still mode
- which Chafa script/path is used

This is the file to edit when the startup portrait is too big or too small.

See also:
- [docs/chafa-map.md](docs/chafa-map.md)

### `bin/yam`

Owns:
- launcher selection for the visualizer
- repo-copy-first behavior for active iteration
- fallback to the installed runtime bundle when the repo copy is unavailable

Edit this file when you want to change:
- how the `yam` command finds the visualizer
- whether the repo copy or runtime bundle should win

Installed runtime path:
- `~/.local/bin/yam`

See also:
- [docs/launcher-map.md](docs/launcher-map.md)

### `chafa/chafa_lab.sh`

Owns:
- the Chafa study/render baseline
- source image path
- Chafa symbol settings
- color-extraction settings
- transparency / threshold / fill behavior
- manual testing behavior

Edit this file when you want to change:
- how the portrait is rendered
- which image is used as the source
- Chafa symbol vocabulary
- threshold / color extraction / fg-only behavior

This is the source of truth for the portrait rendering style.

### `assets/ives_yam.png`

Owns:
- the default source portrait used by the Chafa script

Edit or replace this file when you want to change:
- the source image behind the startup portrait

## Current live paths after install

The installer installs files into these runtime locations:

- `~/Library/Application Support/com.mitchellh.ghostty/config.ghostty` (symlink to repo file)
- `~/.config/ghostty/themes/yam-dark` (symlink to repo file)
- `~/.config/fastfetch/config.jsonc`
- `~/.config/fastfetch/startup.zsh`
- `~/.local/bin/fastfetch-chafa`
- `~/.local/bin/yam`
- `~/.local/share/fastfetch-chafa/chafa_lab.sh`
- `~/.local/share/fastfetch-chafa/assets/ives_yam.png`

The shell profile also sources `~/.config/fastfetch/startup.zsh`, which is what starts the Ghostty panel when an interactive shell opens.

The repo remains the editable source of truth.
The Ghostty config is repo-backed through macOS App Support and the Ghostty theme is repo-backed by symlink; the other runtime files are copies.

## Reference screenshot

Current known-good startup reference:
- `screenshots/startup-reference.png`

## Verified on

This repo was last verified on:
- macOS Tahoe 26.4
- Ghostty target baseline
- Fastfetch 2.61.0
- Chafa 1.18.1
- Homebrew 5.1.5
- Font: JetBrainsMono Nerd Font

Minor rendering differences may still happen across macOS, font, or package versions.

## Dependencies

Install with Homebrew:

```bash
brew install ghostty fastfetch chafa
```

Or from the repo root:

```bash
brew bundle
```

Required font:
- `JetBrainsMono Nerd Font`

Optional tools sometimes used during development:
- `perl`
- `sed`
- `awk`
- `system_profiler`
- `df`

These are already standard on macOS except the Nerd Font.

## Install on a fresh system

1. Clone the repo:

```bash
git clone <your-private-repo-url> ~/_git/yam
cd ~/_git/yam
```

2. Install dependencies:

```bash
brew bundle
```

Fallback if you do not want to use `Brewfile`:

```bash
brew install ghostty fastfetch chafa
```

3. Install the font:
- `JetBrainsMono Nerd Font`

4. Run the installer:

```bash
./install.sh
```

5. Reopen Ghostty.

## How the installer works

`install.sh` does only a few things:

- symlinks Ghostty config into `~/Library/Application Support/com.mitchellh.ghostty/`
- copies Fastfetch config into `~/.config/fastfetch/`
- copies the wrapper into `~/.local/bin/`
- copies the Chafa script into `~/.local/share/fastfetch-chafa/`
- copies the source portrait image into `~/.local/share/fastfetch-chafa/assets/`
- copies the visualizer runtime bundle into `~/.local/share/yam-visualizer/`
- makes the scripts executable
- creates the visualizer venv in `~/.local/share/yam-visualizer/.venv`
- appends one source line to `~/.zshrc` only if it is not already there

It is intentionally conservative.

## Safe editing guide

Use this when you forget where to change something.

### Change startup portrait size

Edit:
- `bin/fastfetch-chafa`

Look for:

```zsh
LOGO_MODE=1 MODE=still WIDTH=... HEIGHT=...
```

### Change startup portrait style

Edit:
- `chafa/chafa_lab.sh`

Look for variables such as:
- `SYMBOLS`
- `FILL`
- `COLOR_SPACE`
- `COLOR_EXTRACTOR`
- `FG_ONLY`
- `THRESHOLD`

### Change startup info modules

Edit:
- `fastfetch/config.jsonc`

Look for:
- `"modules": [ ... ]`

### Change startup spacing above / below prompt

Edit:
- `fastfetch/startup.zsh`

Look for:

```zsh
printf '\n\n'
...
printf '\n'
```

### Change Ghostty window padding / geometry / font

Edit:
- `ghostty/config`

### Change Ghostty colors / palette

Edit:
- `ghostty/themes/yam-dark`

## Known design decisions

- Startup logo is static, not animated.
  - This keeps shell startup fast and non-blocking.
- Fastfetch is used as the startup frame and system panel.
  - We are not replacing it with a custom compositor.
- Chafa rendering is optimized for the current portrait aesthetic, not for general image fidelity.
- The default portrait image is resolved relative to `chafa_lab.sh`, so the live setup does not depend on `Desktop` or `Downloads`.
- The repo stays personal and macOS-specific.

## Known caveats

- Some Fastfetch custom modules shell out to Fastfetch itself and then strip formatting.
  - This is intentional where built-in module formatting was too noisy.
- The right-side panel is decorative and boxed on purpose.
  - It is not trying to be the minimal stock Fastfetch look.
- If a future config drift makes things confusing, check this README first and restore from repo files rather than guessing from installed copies.

## Avoid drift

There are two copies of this setup at any given time:

- the repo copy in `~/_git/yam`
- the installed runtime copy under `~/.config` and `~/.local`

The repo copy is the canonical one.

That means:
- edit repo files first whenever possible
- run `./install.sh` to sync the runtime copy
- if you hot-fix a live file under `~/.config` or `~/.local`, copy that change back into the repo before committing

Do not assume the installed files are safe to edit and forget.
If you do that, the repo and the live setup will drift apart and future debugging gets confusing fast.

One specific expected difference is `chafa/chafa_lab.sh` asset resolution:
- the script supports the repo layout and the installed runtime layout
- that dual-path behavior is intentional because the directory layouts are different


## Recommended workflow

1. Edit files in this repo.
2. Copy them into live locations by rerunning:

```bash
./install.sh
```

3. Reopen Ghostty.
4. Commit only after the live setup looks correct.

## Git notes

Suggested next commands:

```bash
cd ~/_git/yam
git branch -m main
git remote add origin <your-private-repo-url>
git push -u origin main
```
