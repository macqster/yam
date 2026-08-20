# Chafa Drop Rule

How `--bg` decides which of a hero asset's colors get drawn at all, why that
silently deleted every dark region until 0.4.2, and the procedure for choosing
`HeroSource::absent_color` when new art is registered.

Read this before changing `absent_color`, before adding a hero source, and
before concluding that a color "cannot be rendered".

## The Mechanism

Under `--fg-only`, `--bg` is not a background fill and is never painted.
Measured: chafa emits zero `48;2` background codes at every `--bg` value
tested, against 410 when the flag is removed.

What it actually is: the color chafa treats as *already on screen*. Any art
resembling it is judged redundant and discarded rather than drawn. So `--bg`
is a cull control. The closer it sits to the palette, the more of the palette
disappears - not desaturated, not dimmed, omitted.

This is the whole trap. The name says "background", the behavior is "delete
art that looks like this", and a plausible-looking value silently removes
regions that no test at the time was checking for.

## How It Presented

`HERO_DISPLAY_BG` was `#100100` - a dark red. It was therefore instructing
chafa to discard dark red art. Symptom: the hero's hair shadow, the leggings,
and every outline were missing, for months, while frame counts and coverage
tests passed.

The 2026-07-22 pass fixed two real and separate defects (an opaque matte
canvas, and the `average` extractor) and was believed to have fixed this. It
had not. The drop rule was untouched, which is why dark reds still did not
render afterward.

Demonstrated on a fully opaque single-color image, no transparency involved:

| color | mean RGB | dots lit |
| --- | --- | --- |
| line art `#090404` | 6 | 0 |
| darkest red `#430003` | 23 | 0 |
| dark red `#7c0307` | 45 | 0 |
| leggings `#332a29` | 45 | 0 |
| bright red `#b8170c` | 73 | all |
| purple `#5e4762` | 88 | all |

A hard step, not a gradient. Not hue-dependent either: `rgb(184,3,7)` shares
the bright red's red channel and renders nothing.

## The Boundary Is Not Predictable

Ray-cast from two backgrounds, 24 directions each, measuring the crossing
distance in several metrics:

| from | mean-RGB | Euclidean | L1 | luma |
| --- | --- | --- | --- | --- |
| `#000000` | 1.2x | 1.6x | - | 5.9x |
| `#336699` | 2.9x | 3.7x | 5.8x | - |

From a neutral background the rule approximates sum-of-channels. From a
chromatic one every simple metric collapses: moving *toward red* from
`#336699` crosses at Euclidean 60, moving toward green at 218.

**Do not model this. Measure it.** No clearance number predicts which colors
survive a given background.

A corollary worth keeping: `--bg` decides *whether* a color renders, never
*what* color it renders as. Worst emitted deviation across 216 backgrounds on
opaque art was 1. Color bleed exists only on partially transparent cells,
where the value mixes into chafa's foreground pick, and it grows with distance
from the palette.

## Measurement Harness

Testing one color per chafa run is too slow to explore with. Batch it: lay
colors out as a patch grid, render once, read interior cells only so patch
boundaries cannot contaminate the sample.

- source image sized so one cell is 12x24 px at `--size 96x48`
- one patch is 4x4 cells, giving 24x12 = 288 patches per render
- for patch `(px, py)` read cells `(px*4+1..2, py*4+1..2)`
- a patch "renders" if any interior cell has lit braille dots

That harness produced every number in this note. Two traps it must avoid:

- **Parse indexed color.** At `--colors=256` chafa emits `38;5;N`, not
  `38;2;R;G;B`. A parser that reads only truecolor scores every cell as blank
  and yields plausible-looking but meaningless results.
- **Filter by cell position, not color proximity.** A mode that shifts a color
  far enough will fall outside a proximity filter and read as "fewer cells
  rendered" when it is actually "same cells, wrong color".

## Choosing `absent_color` For New Art

1. **Extract the exact opaque palette** across all frames, with per-color pixel
   counts. Ignore colors that never reach half a rendered cell in any single
   frame - those are GIF-export anti-aliasing fringe, not art, and policing
   them means policing the exporter. On `hero_gif_1` that fringe is 108 of 249
   distinct colors.

2. **Decide what should be culled.** If the answer is nothing, pick a color
   absent from the palette and stop. If the art is flat-filled and a faithful
   render would be a solid mass, culling the darkest tiers may be wanted - see
   Solid Cells below.

3. **Sweep candidates and record the keep-set.** Do not reason from distance.
   Across 216 backgrounds the current default asset yields 25 distinct color
   outcomes, and 111 of those backgrounds render all ten authored colors.

4. **Prefer the least clearance that achieves the outcome.** Edge bleed grows
   with distance from the palette. Measured on `hero_gif_2`: `#ffffff`
   (clearance 5) 0 off-palette cells but drops near-white highlights,
   `#00e000` (141) 8, `#00ff00` (170) 15, `#00f0b0` (176) 50.

5. **Set the descriptor, bump `cache_revision`.** Every rendered frame changes,
   so an existing cache would serve pre-change frames. Check the cache
   directory first - a revision number used by an earlier experiment can still
   be present and would be accepted as fresh.

6. **If the value deliberately overlaps the palette**, list the source in
   `ACCEPTED_OVERLAP` in `absent_color_is_actually_absent_from_every_source`
   with its measured overlap. The gate then pins that number instead of
   requiring separation, so drift in the palette, the radius, or the chosen
   color still fails.

7. **Verify live against a fresh cache**, not only by test:

   ```sh
   XDG_CACHE_HOME=$(mktemp -d) cargo run --release -- --hard-reset
   ```

For iterating on the value itself, chafa run directly on the asset is
byte-identical to the runtime pipeline (verified, 15192 bytes both ways),
so it needs no build and no cache:

```sh
chafa assets/hero_gif_2.gif --size 96x48 --format=symbols --symbols=braille \
  --colors=full --color-space=rgb --color-extractor=average --dither=none \
  --fg-only --bg=#336699
```

## Reference Data For Current Assets

`hero_gif_2` is authored as ten flat colors. The exported GIF carries 212
distinct opaque colors; 93.8% of opaque pixels are those ten and the remaining
6.16% is anti-aliasing fringe.

Darkest survivable value per family at `--bg=#336699`, pinned to 0.012 HSV:

| color | anchor V | floor V | headroom |
| --- | --- | --- | --- |
| red `b8170c` | 0.72 | 0.31 | 57% |
| skin light `bba381` | 0.73 | 0.31 | 58% |
| skin `9a7b59` | 0.60 | 0.31 | 49% |
| dark red `7c0307` | 0.49 | 0.32 | 34% |
| purple light `7f6e87` | 0.53 | 0.37 | 30% |
| green `395f0b` | 0.37 | 0.34 | 8% |
| purple `5e4762` | 0.38 | 0.37 | 4% |

Purple is the only family whose floor depends on saturation, and it does so
steeply - S=0.15 floors at V=0.35, S=1.00 at V=0.65. **A more saturated purple
must be lighter to survive**, because `#336699` is itself a saturated blue. To
author a darker purple, desaturate it.

Red, green and skin have flat floors; saturation does not move them.
`--color-extractor` does not move any boundary either - `median` and `average`
give identical tables - so that setting can only change how faithfully a
surviving color is reproduced, never whether it survives.

Why the value is chromatic rather than a dark neutral: `7c0307` and `332a29`
have identical RGB sums (134 each). No grey or black separates them, and
raising a neutral loses the bright red before it recovers the dark one. Only a
chromatically opposed value keeps one while dropping the other.

## Solid Cells

Under `--fg-only` a cell of uniform color has exactly two representations: all
eight braille dots, or none. There is no partial state, because partial dots
would misreport what is there. Flat-filled art therefore renders as solid
masses, and the airy look of a partially-culled render is the *absence* of art,
not a texture.

At `--colors=full` no chafa setting reduces solid cells without deleting art.
The only lever that produces texture at no color cost is upstream: stippling
alpha in the source art took a leggings patch from 8.0 to 1.5 dots per cell
with no solid cells and colors untouched, and it is per-region so the artist
chooses what breathes.

## Terminal-Dependent Settings

Two options are live but are not tuning knobs. Both describe the terminal, so
the right value is a fact about the display rather than a preference, and
neither belongs in the descriptor until that fact is established.

`--font-ratio=W/H` sets the assumed character cell aspect, which changes how
source pixels map into cells and therefore the hero's rendered shape:

| ratio | rendered extent | lit cells |
| --- | --- | --- |
| `1/2` (default) | 94x44 | 923 |
| `3/5` | 96x37 | 750 |
| `2/3` | 96x33 | 697 |
| `1/1` | 95x22 | 475 |

The runtime leaves this at chafa's default `1/2`, the standard monospace
assumption. If the terminal font is not 1:2, the hero renders at a subtly
wrong aspect and this is the setting that corrects it.

`--glyph-file=FILE` loads real glyph coverage from a font file instead of
chafa's built-in model. Measured against installed monospace faces:

| glyph source | lit | solid | mean dots/cell |
| --- | --- | --- | --- |
| built-in model | 923 | 527 | 6.58 |
| Agave-Regular | 900 | 573 | 6.81 |
| CascadiaMonoPL | 895 | 583 | 6.80 |
| Apple Braille | 931 | 520 | 6.56 |

Real fonts *increase* solid cells: their braille dots cover less of the cell
than the built-in model assumes, so chafa lights more of them to compensate.
That is the wrong direction if openness is wanted, and the right direction if
the goal is matching what the terminal actually paints.

Menlo returned byte-identical output. That may mean the built-in model matches
it, or that the `.ttc` failed to load and chafa fell back silently - not
verified, so do not rely on it either way.

## Settings With No Effect

Measured inert at `--colors=full`, which is what the runtime uses. Recorded so
they are not re-tested or tuned in the expectation of an effect.

| setting | evidence |
| --- | --- |
| `--dither`, `--dither-grain`, `--dither-intensity` | 72 of 72 mode/grain/intensity combinations byte-identical; chafa documents "No effect with 24-bit color" |
| `--color-space` | byte-identical across 288 colors and 4 backgrounds |
| `--fg` | identical output at white, black and red |
| `--fill` | byte-identical at `none`, `braille`, `solid`, `stipple`, `space` and `all`; it only applies when the chosen symbol set needs supplementing, which `braille` does not |
| `--preprocess`, `--optimize`, `--work`, `--threshold` | no measurable change |
| `--symbols=braille-solid` | byte-identical to `braille`; chafa's "solid" class means the block glyph, not the full braille cell |

Reduced color depth is not a usable escape from any of this. At
`--colors=256`, `rgb` renders all 199 purple cells as flat grey (`#585858`),
because a low-saturation purple sits nearer the xterm greyscale ramp than any
entry in the 6x6x6 cube. `din99d` keeps the hue but lands on a washed-out
mauve at roughly triple the error, and additionally changes the drop decision
itself - 923 lit cells become 1721, which undoes the cull entirely.

## What The Renderer Cannot Fix

- **Sub-cell features.** The iris `395f0b` peaks at 177 px in a frame against a
  253 px cell footprint, so cell averaging absorbs it regardless of `--bg`. It
  has to be larger in the art, or the hero rendered at more cells.
- **Colors merged into the transparency index.** `ffffff` has zero opaque
  pixels in `hero_gif_2`; white is the transparency index, so it is gone at
  export before yam sees the file.
