# Palette Sheet Reference

This document is YAM's local handoff note for the shared BTAS/TNBA palette sheet source.

The actual shareable sheet source currently lives in:

- `~/_git/legacy/archive-dotfiles/themes/btas-tnba_reference_sheet.md`

The currently adopted exported packet asset now also lives in:

- `docs/btas_tnba_color_system_canonical_packet.pdf`

YAM keeps this local note so palette-sheet work remains visible from the runtime repo without duplicating the whole source of the sheet in two places.

## Why this exists

The older one-page palette-sheet experiment was not safe enough as a shared reference because it merged:

- extracted source data
- renamed visual labels
- and role suggestions

into one authoritative-looking artifact.

The revised shared sheet source fixes that by keeping three layers distinct:

1. extracted source SSOT
2. curated workstation master palette
3. runtime semantic interpretation

## Current source chain

For YAM work, the relevant chain is:

1. `~/_git/home/themes/palette/yam_btas_extracted_swatches.md`
2. `~/_git/home/themes/palette/btas-tnba_color_palette.md`
3. `~/_git/home/themes/palette/btas-tnba_reference_guide.md`
4. `~/_git/legacy/archive-dotfiles/themes/btas-tnba_reference_sheet.md`
5. `docs/palette-reference.md`
6. `docs/theme.md`

Interpretation:

- the extracted swatch file preserves provenance
- the curated palette file preserves the current shared palette values
- the reference guide explains the palette for humans
- the reference sheet is the PDF/layout-ready source
- the canonical packet PDF is the currently adopted shareable export
- YAM docs explain runtime-facing usage

## Rule

If YAM needs:

- source provenance: use the extracted swatches
- shared curated colors: use the master palette
- human-readable quick orientation: use the reference guide
- printable/shareable visual sheet: use the reference sheet
- runtime role meaning: use YAM's local theme and palette docs
