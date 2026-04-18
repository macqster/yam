#!/bin/zsh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ASSET_PATH="$SCRIPT_DIR/../assets/ives_yam.png"
RUNTIME_ASSET_PATH="$SCRIPT_DIR/assets/ives_yam.png"

#############
### input ###
#############

if [[ -z "${IMAGE_PATH:-}" ]]; then
  if [[ -f "$REPO_ASSET_PATH" ]]; then
    IMAGE_PATH="$REPO_ASSET_PATH"
  elif [[ -f "$RUNTIME_ASSET_PATH" ]]; then
    IMAGE_PATH="$RUNTIME_ASSET_PATH"
  else
    IMAGE_PATH="$REPO_ASSET_PATH"
  fi
fi
# IMAGE_PATH="${IMAGE_PATH:-$HOME/Desktop/calibration_palette.png}"

#########################
### output / playback ###
#########################

MODE="${MODE:-animated}"          # animated | still
WIDTH="${WIDTH:-62}"
HEIGHT="${HEIGHT:-12}"
SPEED="${SPEED:-0.5}"

########################
### symbol selection ###
########################

SYMBOLS="${SYMBOLS:-braille}"
FILL="${FILL:-none}"               # none | stipple | symbol class list

#######################
### color selection ###
#######################

COLORS="${COLORS:-full}"                        # full | 256 | 240 | 16 | 16/8
COLOR_SPACE="${COLOR_SPACE:-rgb}"               # rgb | din99d
COLOR_EXTRACTOR="${COLOR_EXTRACTOR:-average}"   # average | median

####################################
### transparency / preprocessing ###
####################################

FG_ONLY="${FG_ONLY:-1}"        # 1 = preserve dotted look, 0 = allow blocky cell backgrounds
PREPROCESS="${PREPROCESS:-off}" # on | off
THRESHOLD="${THRESHOLD:-0.95}"  # 0.0 - 1.0
BG_COLOR="${BG_COLOR:-#111318}" # background model for transparency handling

#####################
### render effort ###
#####################

OPTIMIZE="${OPTIMIZE:-9}"  # output compression effort
WORK="${WORK:-9}"          # approximation effort

#################
### dithering ###
#################

DITHER="${DITHER:-none}"                  # none | diffusion | ordered | noise
DITHER_GRAIN="${DITHER_GRAIN:-1x1}"       # matrix size
DITHER_INTENSITY="${DITHER_INTENSITY:-0}" # 0.0 - 1.0+

###############
### utility ###
###############

LOGO_MODE="${LOGO_MODE:-0}"  # 1 = raw art only, suitable for fastfetch --file-raw

##########################################################################################

cmd=(
  chafa
  # --fg=green                  # force foreground color
  # --invert                    # invert input lightness

  # diagnostic output
  --verbose=on

  # core output mode
  --format=symbols
  --symbols="$SYMBOLS"
  --fill="$FILL"

  # color handling
  --colors="$COLORS"
  --color-space="$COLOR_SPACE"
  --color-extractor="$COLOR_EXTRACTOR"

  # image preprocessing / alpha handling
  --preprocess="$PREPROCESS"
  --threshold="$THRESHOLD"
  --bg="$BG_COLOR"

  # dithering
  --dither="$DITHER"
  --dither-grain="$DITHER_GRAIN"
  --dither-intensity="$DITHER_INTENSITY"

  # render effort / geometry
  --optimize="$OPTIMIZE"
  --work="$WORK"
  --size="${WIDTH}x${HEIGHT}"
)

##########################################################################################

if [[ "$MODE" == "animated" ]]; then
  cmd+=(--animate=on --duration=inf --speed "$SPEED")
else
  cmd+=(--animate=off)
fi

if [[ "$FG_ONLY" == "1" ]]; then
  cmd+=(--fg-only)
fi

cmd+=("$IMAGE_PATH")

if [[ "$LOGO_MODE" != "1" ]]; then
  print -r -- ""
  print -r -- "Rendering with:"
  printf '  %q' "${cmd[@]}"
  printf '\n\n'
fi

##########################################################################################

"${cmd[@]}"
