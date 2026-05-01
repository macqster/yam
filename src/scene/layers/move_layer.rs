use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::layers::modal::{paint_modal_shell, ModalFrame};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;

pub struct MoveLayer;

impl Layer for MoveLayer {
    fn z_index(&self) -> i32 {
        395
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        if !ui.meta.dev_mode || !ui.meta.move_mode_open {
            return LayerOutput { grid, mask: None };
        }

        let frame = ModalFrame::centered(width, height, 68, 12);
        paint_modal_shell(&mut grid, frame, "[m]ove");

        let lines = [
            format!(
                "mode: {}",
                if ui.meta.move_mode_open { "on" } else { "off" }
            ),
            format!("target: {}", ui.meta.move_target.title()),
            "[1] hero".to_string(),
            "[2] clock".to_string(),
            "[3] weather (future)".to_string(),
            "hjkl move selected target".to_string(),
            "[Esc] or [m] exit move mode".to_string(),
        ];
        let (body_x, body_y) = frame.body_origin();
        for (row, line) in lines.iter().enumerate() {
            write_string(
                &mut grid,
                body_x,
                body_y + row as u16,
                line,
                theme_style::panel_text(),
            );
        }

        LayerOutput { grid, mask: None }
    }
}

#[cfg(test)]
mod tests {
    use super::MoveLayer;
    use crate::core::world::WorldState;
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::WorldPos;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::ui::state::UiState;
    use ratatui::prelude::Rect;

    #[test]
    fn move_overlay_requires_dev_mode_and_open_state() {
        let layer = MoveLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
        let render_state = RenderState {
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
        };
        let mut ui = UiState::new();

        let closed = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        assert_eq!(closed.grid.cells[closed.grid.index(62, 16)].symbol, ' ');

        ui.meta.dev_mode = true;
        ui.meta.move_mode_open = true;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("[m]ove"));
        assert!(text.contains("target: hero"));
        assert!(text.contains("[3] weather (future)"));
        let center = open.grid.cells[open.grid.index(62, 16)].style.bg;
        assert_eq!(center, Some(crate::theme::palette::MODAL_BG));
    }
}
