use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::layers::modal::{paint_modal_shell, ModalFrame};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;

pub struct HotkeysLayer;

impl Layer for HotkeysLayer {
    fn z_index(&self) -> i32 {
        390
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
        if !ui.meta.dev_mode || !ui.meta.hotkeys_open {
            return LayerOutput { grid, mask: None };
        }

        let frame = ModalFrame::centered(width, height, 68, 17);
        paint_modal_shell(&mut grid, frame, "[h]otkeys");

        let (body_x, body_y) = frame.body_origin();
        let lines = [
            "[q] quit app",
            "[d] toggle dev mode",
            "[m] toggle move mode",
            "  [1] hero",
            "  [2] clock",
            "  [3] weather (future)",
            "[h/j/k/l] move selected target",
            "[C] store camera home",
            "[c] recall camera home",
            "[p] toggle pointer probe",
            "[s] toggle settings popup",
            "[F5] next font",
            "[space] play/pause",
            "[.] step animation",
        ];
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
    use super::HotkeysLayer;
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
    fn hotkeys_overlay_requires_dev_mode_and_open_state() {
        let layer = HotkeysLayer;
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
        ui.meta.hotkeys_open = true;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("[h]otkeys"));
        assert!(text.contains("[C] store camera home"));
        assert!(text.contains("[c] recall camera home"));
        assert!(text.contains("[p] toggle pointer probe"));
        assert!(text.contains("[s] toggle settings popup"));
        assert!(text.contains("[m] toggle move mode"));
        assert!(text.contains("[1] hero"));
        assert!(text.contains("[2] clock"));
        assert!(text.contains("[space] play/pause"));
    }
}
