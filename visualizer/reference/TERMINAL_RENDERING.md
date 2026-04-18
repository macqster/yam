# Terminal Rendering Takeaways

This is the focused extract from the broader research note. It captures the parts that matter most for scene composition, glyph strategy, and terminal constraints.

## Core pipeline

The useful terminal-visual pipeline is:

```text
input -> preprocess -> resize/color reduce -> glyph map -> ANSI output
```

The main lesson is that image quality is mostly decided before ANSI emission. The renderer matters, but preprocessing, palette choice, symbol choice, and thresholding matter more.

## Chafa framing

Treat Chafa as a signal-compression stage, not just a renderer.

Useful control axes:

- symbol set selection
- truecolor vs palette-constrained output
- perceptual color space choice
- dithering mode
- preprocessing and thresholding
- background handling

Practical implication for YAM:

- hero quality should be tuned at the source/conversion layer
- layout and masking should not try to compensate for a weak hero footprint
- the renderer should be treated as the last stage of a larger visual pipeline

## Glyph density

The useful glyph spectrum is:

```text
ASCII -> block -> braille
```

Tradeoff:

- ASCII: readable, low fidelity
- block: stronger color fill, coarser detail
- braille: highest density, best for terminal-native detail

Useful synthesis:

- treat glyphs as density carriers
- map continuous fields to discrete glyphs
- mix glyph classes when structure, fill, and detail need different treatments

## Field-based rendering

The best mental model for YAM remains:

```text
field -> glyph
```

instead of:

```text
geometry -> glyph
```

That means:

- density fields should exist explicitly
- soft masks should exist explicitly
- layers should write into a shared render field before final glyph selection

## Dual-color cells

A useful terminal trick is to treat one cell as two vertically stacked samples:

- foreground color = top sample
- background color = bottom sample

This can increase perceived vertical resolution and improve edge smoothness.

## Terminal constraints

Keep these in mind:

- character cells are not square
- glyph appearance depends on font support
- ANSI escape overhead can become a performance factor
- truecolor support is terminal-dependent
- gamma and brightness differ by terminal

Practical implication for YAM:

- test against the real baseline terminal
- do not treat output as device-independent
- build for readable structure first, then refine density

## Architecture implications

The research points toward a terminal-native scene system, not just a converter.

The missing pieces in the ecosystem are:

- authoring
- preprocessing
- compositing
- procedural behavior
- runtime rendering

That supports YAM’s current direction:

- keep investing in scene composition
- keep using masks and fields for placement
- keep debug overlays accurate and aligned with runtime state
