use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::viewport::Viewport;
use crate::scene::{Layer, LayerOutput, WORLD_HALF_H, WORLD_HALF_W};
use crate::ui::debug::draw_layout_debug;
use crate::ui::state::UiState;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub struct DebugLayer;

impl Layer for DebugLayer {
    fn z_index(&self) -> i32 {
        300
    }

    fn render(
        &self,
        frame: &mut Frame<'_>,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _viewport: &Viewport,
        viewport_rect: Rect,
    ) {
        if !ui.debug_layout {
            return;
        }

        draw_layout_debug(frame, viewport_rect);

        let frame_area = frame.area();
        let hero = &ui.hero;
        let hero_anchor = ui.hero_anchor.get();
        let hero_visual_anchor = ui.hero_visual_anchor.get();
        let clock_final = ui.clock_final.get();
        let clock_anchor = (
            clock_final.0 - ui.offsets.clock_dx as i32,
            clock_final.1 - ui.offsets.clock_dy as i32,
        );
        let clock_visible = clock_final.0 >= 0
            && clock_final.1 >= 0
            && clock_final.0 < frame_area.width as i32
            && clock_final.1 < frame_area.height as i32;
        let cam_x = ui.camera.x;
        let cam_y = ui.camera.y;
        let center_x = frame_area.width as i32 / 2;
        let center_y = frame_area.height as i32 / 2;
        let cam_dx = cam_x - center_x;
        let cam_dy = cam_y - center_y;
        let telemetry = Paragraph::new(format!(
            "FPS: {:.1}\nHero FPS: {:.1}\nFrame: {} / {}\nPlaying: {}\nHero anchor: ({}, {})\nHero visual anchor: ({}, {})\nHero offset: ({}, {})\nClock anchor: ({}, {})\nClock offset: ({}, {})\nClock final: ({}, {})\nClock visible: {}\nCamera: ({}, {})\nCamera Δ: ({}, {})",
            ui.fps,
            ui.offsets.hero_fps,
            hero.current_frame,
            hero.frames.len(),
            hero.playing,
            hero_anchor.0,
            hero_anchor.1,
            hero_visual_anchor.0,
            hero_visual_anchor.1,
            ui.offsets.hero_dx,
            ui.offsets.hero_dy,
            clock_anchor.0,
            clock_anchor.1,
            ui.offsets.clock_dx,
            ui.offsets.clock_dy,
            clock_final.0,
            clock_final.1,
            clock_visible,
            cam_x,
            cam_y,
            cam_dx,
            cam_dy
        ))
        .style(Style::default().fg(Color::Green));

        frame.render_widget(
            telemetry,
            Rect::new(viewport_rect.x + 10, viewport_rect.y + 5, 52, 10),
        );
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _viewport: &Viewport,
        _viewport_rect: Rect,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        if !ui.debug_layout {
            return LayerOutput { grid, mask: None };
        }

        let panel_x = 10u16;
        let panel_y = 5u16;
        let panel_width = 52u16;
        let panel_height = 10u16;

        let cam_x = ui.camera.x;
        let cam_y = ui.camera.y;
        let screen_w = width as i32;
        let screen_h = height as i32;
        let world_left = -WORLD_HALF_W;
        let world_right = WORLD_HALF_W - 1;
        let world_top = -WORLD_HALF_H;
        let world_bottom = WORLD_HALF_H - 1;
        let exclude_x0 = panel_x.saturating_sub(2);
        let exclude_y0 = panel_y.saturating_sub(2);
        let exclude_x1 = panel_x + panel_width + 2;
        let exclude_y1 = panel_y + panel_height + 2;
        for sy in 0..screen_h {
            for sx in 0..screen_w {
                let world_x = cam_x + sx;
                let world_y = cam_y + sy;
                let is_debug_zone = sx >= exclude_x0 as i32
                    && sx <= exclude_x1 as i32
                    && sy >= exclude_y0 as i32
                    && sy <= exclude_y1 as i32;
                let is_border = world_x == world_left
                    || world_x == world_right
                    || world_y == world_top
                    || world_y == world_bottom;
                if is_border && !is_debug_zone {
                    let x = sx as u16;
                    let y = sy as u16;
                    if let Some(cell) = grid.cell_mut(x, y) {
                        cell.symbol = '•';
                        cell.style = Style::default().fg(Color::DarkGray);
                    }
                }
            }
        }
        let hero = &ui.hero;
        let hero_anchor = if ui.offsets.hero_world_x == 0 && ui.offsets.hero_world_y == 0 {
            (hero.x, hero.y)
        } else {
            (ui.offsets.hero_world_x, ui.offsets.hero_world_y)
        };
        let hero_visual_anchor = ui.hero_visual_anchor.get();
        let clock_final = ui.clock_final.get();
        let clock_screen = clock_final;
        let clock_anchor = hero_visual_anchor;
        let clock_visible = clock_final.0 >= 0
            && clock_final.1 >= 0
            && clock_final.0 < width as i32
            && clock_final.1 < height as i32;
        let center_x = width as i32 / 2;
        let center_y = height as i32 / 2;
        let cam_dx = cam_x - center_x;
        let cam_dy = cam_y - center_y;
        let lines = [
            format!("FPS: {:.1}", ui.fps),
            format!("Hero FPS: {:.1}", ui.offsets.hero_fps),
            format!("Frame: {} / {}", hero.current_frame, hero.frames.len()),
            format!("Playing: {}", hero.playing),
            format!("Hero anchor: ({}, {})", hero_anchor.0, hero_anchor.1),
            format!(
                "Hero visual anchor: ({}, {})",
                hero_visual_anchor.0, hero_visual_anchor.1
            ),
            format!(
                "Hero offset: ({}, {})",
                ui.offsets.hero_dx, ui.offsets.hero_dy
            ),
            format!(
                "Clock world: ({}, {})",
                hero_anchor.0 + ui.offsets.clock_dx as i32,
                hero_anchor.1 + ui.offsets.clock_dy as i32
            ),
            format!("Clock screen: ({}, {})", clock_screen.0, clock_screen.1),
            format!("Clock anchor: ({}, {})", clock_anchor.0, clock_anchor.1),
            format!(
                "Clock offset: ({}, {})",
                ui.offsets.clock_dx, ui.offsets.clock_dy
            ),
            format!("Clock final: ({}, {})", clock_final.0, clock_final.1),
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
