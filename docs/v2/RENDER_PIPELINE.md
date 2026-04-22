# YAM v2 Render Pipeline

This document records the current render composition contract.

## Flow

```text
shape list
→ layer assignment
→ masked framebuffer writes
→ ANSI emission
```

## Current Layers

- `world`
- `ui`

## Current Mask Rule

- a mask is consulted before every framebuffer write
- blocked cells are skipped

## Notes

- this is a minimal structural split, not the final visual system
- the composer remains deterministic
- the retired visualizer stack is no longer part of the active repo
