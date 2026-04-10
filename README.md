# Kitty + Fastfetch + Chafa

Personal macOS startup setup for:

- Kitty as the terminal
- Fastfetch as the system-info panel
- a static Chafa-generated ANSI portrait as the Fastfetch logo

The live startup scene is:
- Kitty window opens
- zsh sources a small Fastfetch startup hook
- `fastfetch-chafa` renders a still ANSI logo from `chafa_lab.sh`
- Fastfetch displays the boxed system panel beside it

## Repo layout

- `kitty/`
  - `kitty.conf`
  - `current-theme.conf`
- `fastfetch/`
  - `config.jsonc`
  - `startup.zsh`
- `bin/`
  - `fastfetch-chafa`
- `chafa/`
  - `chafa_lab.sh`
- `install.sh`

## What each file does

- `kitty.conf`
  Main Kitty settings: font, window size, padding, titlebar, keybindings.
- `current-theme.conf`
  Active Kitty color theme included by `kitty.conf`.
- `config.jsonc`
  Fastfetch layout and module configuration.
- `startup.zsh`
  Runs Fastfetch once in interactive Kitty shells.
- `fastfetch-chafa`
  Generates the raw ANSI logo and launches Fastfetch.
- `chafa_lab.sh`
  Chafa study/render script used as the source of truth for the logo.

## Install on a fresh macOS system

1. Install dependencies:

   ```bash
   brew install kitty fastfetch chafa
   ```

2. Install the Nerd Font used by Kitty:

   - `JetBrainsMono Nerd Font`

3. Run the installer:

   ```bash
   git clone <your-private-repo-url> ~/kitty-fastfetch-chafa
   cd ~/kitty-fastfetch-chafa
   ./install.sh
   ```

4. Reopen Kitty.

## Notes

- The live startup logo uses the stable path:
  - `~/.local/share/fastfetch-chafa/chafa_lab.sh`
- The shell startup hook is sourced from:
  - `~/.config/fastfetch/startup.zsh`
- This keeps the setup portable and avoids depending on `Downloads`.
- The logo source image path is configured inside `chafa/chafa_lab.sh`.
- The installer is intentionally conservative: it copies the current known-good files into place and appends one source line to `~/.zshrc` only if needed.

## Recommended repo hygiene

- Keep this repo personal and macOS-specific.
- Commit screenshots separately if you want visual version history.
- Keep experiments out of the main files unless they are accepted into the live setup.
