# Visualizer Recipes

These files are config overlays for common development states.

Use them with:

```bash
./visualizer/run_recipe.sh <recipe-name>
```

The launcher merges the selected recipe into the base `config/visualizer.json` and runs the visualizer against a temporary merged config.

Available recipes:

- `debug` - Show the denser debug view and speed up the tick/render loop.
- `presentation` - Slow the scene down for a calmer presentation-friendly rhythm.
- `tight_layout` - Tighten spacing around the hero and info panel for small terminals.
- `hero_heavy` - Emphasize the hero art by increasing its footprint and nudging the info panel.
- `trunk_probe` - Expose scaffold behavior with a narrow, aggressive growth probe.

Recipe files are intentionally small. They should only override the values needed for a specific tuning mode.
