use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::layers::modal::{paint_modal_shell, ModalFooter, ModalFrame};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;

pub struct QuitConfirmLayer;

impl Layer for QuitConfirmLayer {
    fn z_index(&self) -> i32 {
        405
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
        if !ui.quit_confirm_active() {
            return LayerOutput { grid, mask: None };
        }

        let frame = ModalFrame::centered(width, height, 54, 10);
        paint_modal_shell(
            &mut grid,
            frame,
            "quit",
            Some(ModalFooter {
                left: "",
                right: "",
            }),
        );
        write_centered_quit_footer(
            &mut grid,
            frame,
            "[s]ave and quit • [d]iscard and quit • ⎋ cancel",
        );

        let (body_x, body_y) = frame.body_origin();
        let body_width = frame.width.saturating_sub(4);
        let lines = [
            (0u16, "do you really want to quit yam?"),
            (2u16, "unsaved tweaks were detected,"),
            (3u16, "save recent settings before quitting?"),
        ];
        for (row, line) in lines {
            write_centered_body_line(&mut grid, body_x, body_y + row, body_width, line);
        }

        LayerOutput { grid, mask: None }
    }
}

fn write_centered_quit_footer(grid: &mut Grid, frame: ModalFrame, text: &str) {
    let footer = format!("  {}  ", text);
    let footer_width = footer.chars().count() as u16;
    let x = frame.x + frame.width.saturating_sub(footer_width) / 2;
    let y = frame.y + frame.height - 1;

    for (offset, ch) in footer.chars().enumerate() {
        if let Some(cell) = grid.cell_mut(x + offset as u16, y) {
            cell.symbol = ch;
            cell.style = theme_style::panel_text();
        }
    }
}

fn write_centered_body_line(grid: &mut Grid, body_x: u16, y: u16, body_width: u16, line: &str) {
    let line_width = line.chars().count() as u16;
    let x = body_x + body_width.saturating_sub(line_width) / 2;
    write_string(grid, x, y, line, theme_style::panel_text());
}

#[cfg(test)]
mod tests {
    use super::QuitConfirmLayer;
    use crate::core::world::WorldState;
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::WorldPos;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::ui::state::UiState;
    use ratatui::prelude::Rect;

    fn render_state() -> RenderState {
        RenderState {
            world: WorldFrame {
                hero_world: WorldPos { x: 50, y: 30 },
                hero_visual_anchor: WorldPos { x: 40, y: 20 },
                clock_world: WorldPos { x: 45, y: 25 },
                weather_world: WorldPos { x: 55, y: 26 },
                date_world: WorldPos { x: 45, y: 23 },
                calendar_world: WorldPos { x: 60, y: 22 },
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
    fn quit_confirm_overlay_requires_open_state() {
        let layer = QuitConfirmLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
        let render_state = render_state();
        let mut ui = UiState::new();

        let closed = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        assert_eq!(closed.grid.cells[closed.grid.index(62, 16)].symbol, ' ');

        ui.quit_confirm_open = true;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("quit"));
        assert!(text.contains("do you really want to quit yam?"));
        assert!(text.contains("unsaved tweaks were detected,"));
        assert!(text.contains("save recent settings before quitting?"));
        assert!(text.contains("[s]ave and quit • [d]iscard and quit • ⎋ cancel"));
        assert!(!text.contains("? ⎋"));
    }
}
