use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::ui::state::UiState;
use ratatui::prelude::*;

pub struct DebugLayer;

impl Layer for DebugLayer {
    fn z_index(&self) -> i32 {
        300
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        if !ui.debug_layout {
            return LayerOutput { grid, mask: None };
        }

        let panel_x = 10u16;
        let panel_y = 5u16;
        let panel_width = 52u16;
        let panel_height = 10u16;

        let cam_x = ctx.camera.x;
        let cam_y = ctx.camera.y;
        let screen_w = width as i32;
        let screen_h = height as i32;
        let exclude_x0 = panel_x.saturating_sub(2);
        let exclude_y0 = panel_y.saturating_sub(2);
        let exclude_x1 = panel_x + panel_width + 2;
        let exclude_y1 = panel_y + panel_height + 2;
        let left = 1i32;
        let right = screen_w - 2;
        let top = 1i32;
        let bottom = screen_h - 2;
        let mid_x = screen_w / 2;
        let mid_y = screen_h / 2;

        let mut draw_border_cell = |x: i32, y: i32, ch: char| {
            if x < 0 || y < 0 || x >= screen_w || y >= screen_h {
                return;
            }
            if x >= exclude_x0 as i32
                && x <= exclude_x1 as i32
                && y >= exclude_y0 as i32
                && y <= exclude_y1 as i32
            {
                return;
            }
            if let Some(cell) = grid.cell_mut(x as u16, y as u16) {
                cell.symbol = ch;
                cell.style = Style::default().fg(Color::DarkGray);
            }
        };

        for x in left..=right {
            let ch = if x == left || x == mid_x || x == right {
                '+'
            } else {
                '-'
            };
            draw_border_cell(x, top, ch);
            draw_border_cell(x, mid_y, ch);
            draw_border_cell(x, bottom, ch);
        }

        for y in top + 1..bottom {
            let ch = if y == mid_y { '+' } else { '|' };
            draw_border_cell(left, y, ch);
            draw_border_cell(mid_x, y, ch);
            draw_border_cell(right, y, ch);
        }

        let hero = &ui.hero;
        let hero_anchor = ctx.hero_world;
        let hero_visual_anchor = ctx.hero_visual_anchor;
        let clock_world = ctx.clock_world;
        let clock_screen = ctx.clock_screen;
        let clock_visible = clock_screen.x >= 0
            && clock_screen.y >= 0
            && clock_screen.x < width as i32
            && clock_screen.y < height as i32;
        let center_x = width as i32 / 2;
        let center_y = height as i32 / 2;
        let cam_dx = cam_x - center_x;
        let cam_dy = cam_y - center_y;
        let lines = [
            format!("FPS: {:.1}", ui.fps),
            format!("Hero FPS: {:.1}", ui.offsets.hero_fps),
            format!("Frame: {} / {}", hero.current_frame, hero.frames.len()),
            format!("Playing: {}", hero.playing),
            format!("Hero anchor: ({}, {})", hero_anchor.x, hero_anchor.y),
            format!(
                "Hero visual anchor: ({}, {})",
                hero_visual_anchor.x, hero_visual_anchor.y
            ),
            format!(
                "Hero offset: ({}, {})",
                ui.offsets.hero_dx, ui.offsets.hero_dy
            ),
            format!("Clock world: ({}, {})", clock_world.x, clock_world.y),
            format!("Clock screen: ({}, {})", clock_screen.x, clock_screen.y),
            format!(
                "Clock anchor: ({}, {})",
                hero_visual_anchor.x, hero_visual_anchor.y
            ),
            format!(
                "Clock offset: ({}, {})",
                ui.offsets.clock_dx, ui.offsets.clock_dy
            ),
            format!("Clock final: ({}, {})", clock_screen.x, clock_screen.y),
            format!("Clock visible: {}", clock_visible),
            format!("Camera: ({}, {})", cam_x, cam_y),
            format!("Camera Δ: ({}, {})", cam_dx, cam_dy),
        ];
        for (row, line) in lines.iter().enumerate() {
            write_string(
                &mut grid,
                panel_x,
                panel_y + row as u16,
                line,
                Style::default().fg(Color::Green),
            );
        }
        LayerOutput { grid, mask: None }
    }
}
