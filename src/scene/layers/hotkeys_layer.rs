use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
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

        let panel_width = width.min(44);
        let panel_height = height.min(12);
        let panel_x = (width.saturating_sub(panel_width)) / 2;
        let panel_y = (height.saturating_sub(panel_height)) / 2;

        draw_border(&mut grid, panel_x, panel_y, panel_width, panel_height);
        write_string(
            &mut grid,
            panel_x + 2,
            panel_y + 1,
            "[h]otkeys",
            theme_style::panel_text(),
        );

        let lines = [
            "[q] quit app",
            "[d] toggle dev mode",
            "[c] center camera",
            "[s] toggle settings popup",
            "[F5] next font",
            "[space] play/pause",
            "[.] step animation",
        ];
        for (row, line) in lines.iter().enumerate() {
            write_string(
                &mut grid,
                panel_x + 2,
                panel_y + 3 + row as u16,
                line,
                theme_style::panel_text(),
            );
        }

        LayerOutput { grid, mask: None }
    }
}

fn draw_border(grid: &mut Grid, x: u16, y: u16, width: u16, height: u16) {
    if width < 2 || height < 2 {
        return;
    }
    let right = x + width - 1;
    let bottom = y + height - 1;
    for cx in x..=right {
        write_border_cell(grid, cx, y, if cx == x || cx == right { '+' } else { '-' });
        write_border_cell(
            grid,
            cx,
            bottom,
            if cx == x || cx == right { '+' } else { '-' },
        );
    }
    for cy in y + 1..bottom {
        write_border_cell(grid, x, cy, '|');
        write_border_cell(grid, right, cy, '|');
    }
}

fn write_border_cell(grid: &mut Grid, x: u16, y: u16, ch: char) {
    if let Some(cell) = grid.cell_mut(x, y) {
        cell.symbol = ch;
        cell.style = theme_style::panel_text();
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
        assert!(text.contains("[c] center camera"));
        assert!(text.contains("[s] toggle settings popup"));
    }
}
