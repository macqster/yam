# Palette and Dithering Takeaways

This extract captures the color and texture lessons from the research note.

## Palette control matters

Useful palette sources:

- curated palettes
- sampled palettes
- manual palettes

The important point is that palette choice is a composition decision, not just a conversion detail.

## Color space matters

Useful lesson:

- perceptual spaces matter
- RGB distance is not always what the eye sees

Practical implication for YAM:

- if color becomes a major part of the scene language, add explicit palette and perceptual tuning hooks
- do not assume raw RGB matching is enough for good terminal output

## Dithering is style

Dithering affects:

- texture
- perceived density
- stability over time
- directional artifacts

Useful models:

- Floyd–Steinberg for detail
- Atkinson for softer diffusion
- Bayer / ordered dithering for predictable structure
- serpentine diffusion to reduce directional bias

Practical implication for YAM:

- dithering choices should be scene-specific
- debug and presentation modes may want different style profiles
- noise is not always a defect; it can be part of the look

## Thresholding and preprocessing

The conversion layer is sensitive to preprocessing and threshold choices.

Useful rule:

- tune thresholding at the source/conversion layer
- avoid using downstream mask logic to fix conversion artifacts

## Actionable takeaway

If the visual output feels muddy, unstable, or oddly flat, the first place to look is:

1. palette
2. color space
3. dithering mode
4. thresholding
5. source image quality

That order is usually more productive than tweaking the final ANSI emission step.
