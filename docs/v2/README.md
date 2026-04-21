# YAM v2 Spec Index

This directory contains the preliminary documentation import for the YAM v2 rebuild.

## Reading order

1. [`00_overview.md`](00_overview.md)
2. [`01_architecture.md`](01_architecture.md)
3. [`02_runtime_bubbletea.md`](02_runtime_bubbletea.md)
4. [`10_ecosystem.md`](10_ecosystem.md)
5. [`11_environment.md`](11_environment.md)
6. [`12_lifecycle.md`](12_lifecycle.md)
7. [`13_species.md`](13_species.md)
8. [`14_morphology.md`](14_morphology.md)
9. [`15_vines.md`](15_vines.md)
10. [`16_balance.md`](16_balance.md)
11. [`20_framebuffer.md`](20_framebuffer.md)
12. [`21_render_pipeline.md`](21_render_pipeline.md)
13. [`22_layers.md`](22_layers.md)
14. [`23_masks.md`](23_masks.md)
15. [`24_shape_language.md`](24_shape_language.md)
16. [`25_hybrid_renderer.md`](25_hybrid_renderer.md)
17. [`26_emitter.md`](26_emitter.md)
18. [`30_tui_runtime.md`](30_tui_runtime.md)
19. [`31_panels.md`](31_panels.md)
20. [`32_focus_and_input.md`](32_focus_and_input.md)
21. [`33_keybindings.md`](33_keybindings.md)
22. [`34_command_system.md`](34_command_system.md)
23. [`40_themes.md`](40_themes.md)
24. [`41_layout_presets.md`](41_layout_presets.md)
25. [`42_renderer_rules.md`](42_renderer_rules.md)
26. [`50_general_aesthetics.md`](50_general_aesthetics.md)
27. [`51_textmode_graphical_design.md`](51_textmode_graphical_design.md)
28. [`52_glyph_system.md`](52_glyph_system.md)

## Purpose

These files form the working specification for the v2 branch:

- system identity and architecture
- engine, simulation, and ecology
- morphology, shape language, and rendering
- framebuffer and emitter rules
- TUI runtime, panels, focus, keybindings, and commands
- theme and aesthetic constraints

## Notes

- This is the imported draft spec, not yet the final canonical documentation set.
- Files are kept close to the source notes so we can refactor them into implementation docs later.
- Track rebuild progress in [`ROADMAP.md`](ROADMAP.md) and [`LOG.md`](LOG.md).
- Follow [`HYGIENE.md`](HYGIENE.md) for repo discipline while v2 is under active reconstruction.
- Read [`AUDIT.md`](AUDIT.md) before starting implementation work.
- Use [`SOURCE_MAP.md`](SOURCE_MAP.md) when adding new source files.
- Read [`VERTICAL_SLICE.md`](VERTICAL_SLICE.md) for the current runnable demo contract.
- Read [`RUNTIME_LOOP.md`](RUNTIME_LOOP.md) for the current message and tick flow.
- Read [`ENGINE_MODEL.md`](ENGINE_MODEL.md) for the current engine state contract.
- Read [`MORPHOLOGY_AND_SHAPES.md`](MORPHOLOGY_AND_SHAPES.md) for the organism-to-shape translation.
- Read [`RENDER_PIPELINE.md`](RENDER_PIPELINE.md) for the layer and mask composition split.
- Read [`GOLDEN_FRAME.md`](GOLDEN_FRAME.md) for the current baseline frame.
- Read [`VERIFICATION.md`](VERIFICATION.md) for the current snapshot check command.
- Read [`GIF_AND_CLOCK.md`](GIF_AND_CLOCK.md) for the minimal working scene target.
- Read [`CLOCK_CONTRACT.md`](CLOCK_CONTRACT.md) for the FIGlet clock contract.
- Read [`LAUNCHER.md`](LAUNCHER.md) for the current launch contract.
- Read [`RENDER_HARDENING.md`](RENDER_HARDENING.md) for the current z-band and overlay contract.
- Read [`UI_INPUT.md`](UI_INPUT.md) for the current UI separation boundary.
- Read [`THEME_AND_GLYPHS.md`](THEME_AND_GLYPHS.md) for the current minimal visual policy.
- Read [`SCENE_CONFIG.md`](SCENE_CONFIG.md) for the explicit scene config surface.
- Read [`BUBBLETEA_MIGRATION.md`](BUBBLETEA_MIGRATION.md) for the runtime migration plan.
- Read [`DEPENDENCY_MATRIX.md`](DEPENDENCY_MATRIX.md) for the upstream tool availability and adoption policy.
- Read [`FIGLET_TOOLING.md`](FIGLET_TOOLING.md) for the FIGlet library review and current clock-font decision.
- The dependency matrix is the first stop for deciding whether a third-party Bubble Tea library is core, optional, or reference-only.
- The default scene config is stored in [`../../v2/scene_config.json`](../../v2/scene_config.json).
- Use `yam --scene-config show|edit|reset` to inspect or tune the file-backed config.
- Use `yam --scene-set key=value` for one-key scene updates.
- The live app uses a real clock; the snapshot check uses a fixed one.
- The live app enters the terminal alternate screen and restores on exit.
- See [`../VERSION_MAP.md`](../VERSION_MAP.md) for the v1/v2 repo split.
