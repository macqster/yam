# YAM v2 Morphology and Shapes

This document records the current translation path from organisms to renderable forms.

## Morphology

- organisms become `Node` instances
- each organism produces a single-node `Axis`
- each organism becomes an `Organ` anchored to that node

## Shapes

- morphology nodes become `ShapeInstance` objects
- hero organisms gain a second accent glyph above the anchor
- seed organisms gain a small subordinate glyph below the anchor

## Constraints

- morphology does not render directly
- shape generation does not mutate engine state
- render code consumes shapes only
