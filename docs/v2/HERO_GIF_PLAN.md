# YAM v2 Hero GIF Plan

This note tracks the preliminary plan for the hero GIF pipeline.

## Goal

Render the hero as a terminal image layer in the v2 scene while keeping the clock, UI, and engine boundaries separate.

## Working Assumption

- Use **Chafa** for the hero image pipeline first.
- Keep the clock on the FIGlet path.
- Keep the hero renderer independent from the clock renderer.

## Why Chafa

- Chafa is designed for terminal graphics and animated image input.
- The old visualizer already used a Chafa-based hero pipeline, so the repository has prior art here.
- Chafa gives a practical first implementation path without writing a custom GIF-to-terminal converter.

## Proposed Rollout

1. Add a dedicated hero source field to the v2 scene config if needed.
2. Add a hero renderer module in `hero/` that converts GIF frames into terminal art.
3. Cache rendered frames by terminal size and render settings.
4. Compose the hero layer into the existing v2 scene using the current layering rules.
5. Keep hero cadence separate from clock cadence.
6. Add a golden or snapshot check for the hero render path.
7. Document the final contract in the v2 docs once the first slice is stable.

## Integration Boundary

- Hero rendering should stay outside the ecosystem simulation.
- Hero rendering should stay outside clock typography.
- Scene composition should only combine pre-rendered layers.

## When to Use Codex Planning Mode

Use Codex planning mode **right before the first hero renderer implementation**:

- after the tool choice is settled
- after the source asset and layout target are known
- before editing code or moving files

That is the point where we want a short implementation plan, file ownership, and a verification sequence before making changes.

## Notes

- Treat this as a preliminary plan, not a final specification.
- If Chafa hits a clear boundary, reassess bindings or a native Go path later.
