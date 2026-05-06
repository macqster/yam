use crate::core::world::{WorldKind, WorldState};
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;

pub struct WorldLabelLayer;

impl Layer for WorldLabelLayer {
    fn z_index(&self) -> i32 {
        305
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        if !ui.meta.dev_mode || height == 0 || width == 0 {
            return LayerOutput { grid, mask: None };
        }

        let label = match world.kind {
            WorldKind::MainScene => "MAIN SCENE",
            WorldKind::Sandbox => "SANDBOX",
        };
        let label_width = label.chars().count() as u16;
        let x = width.saturating_sub(label_width) / 2;
        write_string(&mut grid, x, 0, label, theme_style::debug_text());

        LayerOutput { grid, mask: None }
    }
}

#[cfg(test)]
mod tests {
    use super::WorldLabelLayer;
    use crate::core::world::{WorldKind, WorldState};
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::WorldPos;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::ui::state::{UiState, WorldKindSnapshot};
    use ratatui::prelude::Rect;

    fn render_state() -> RenderState {
        RenderState {
            world: WorldFrame {
                hero_world: WorldPos { x: 50, y: 30 },
                hero_visual_anchor: WorldPos { x: 40, y: 20 },
                clock_world: WorldPos { x: 45, y: 25 },
            },
            hud: HudFrame {
                viewport: Viewport {
                    x: 30,
                    y: 10,
                    width: 124,
                    height: 32,
                },
                viewport_rect: Rect::new(0, 0, 124, 32),
                camera: Camera {
                    x: 30,
                    y: 10,
                    width: 124,
                    height: 32,
                    follow_hero: false,
                },
            },
        }
    }

    #[test]
    fn dev_mode_shows_centered_main_scene_label() {
        let layer = WorldLabelLayer;
        let world = WorldState::for_kind(WorldKind::MainScene);
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.meta.dev_mode = true;
        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let text: String = output.grid.cells[0..124]
            .iter()
            .map(|cell| cell.symbol)
            .collect();

        assert!(text.contains("MAIN SCENE"));
    }

    #[test]
    fn dev_mode_shows_centered_sandbox_label() {
        let layer = WorldLabelLayer;
        let world = WorldState::for_kind(WorldKind::Sandbox);
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.meta.dev_mode = true;
        ui.meta.active_world = WorldKindSnapshot::Sandbox;
        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let text: String = output.grid.cells[0..124]
            .iter()
            .map(|cell| cell.symbol)
            .collect();

        assert!(text.contains("SANDBOX"));
    }
}
