use crate::core::world::WorldState;
use crate::render::clock::{clock_lines, draw_clock_at};
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::coords::{anchor_to_world, WorldPos};
use crate::scene::viewport::Viewport;
use crate::scene::{Layer, LayerOutput};
use crate::ui::state::UiState;
use ratatui::prelude::*;

pub struct ClockLayer;

impl Layer for ClockLayer {
    fn z_index(&self) -> i32 {
        100
    }

    fn render(
        &self,
        frame: &mut Frame<'_>,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        viewport: &Viewport,
        _viewport_rect: Rect,
    ) {
        let screen = frame.area();
        let hero_world = hero_world_pos(ui);
        let hero_visual_anchor = hero_visual_anchor(ui, hero_world);
        let lines = clock_lines(ui, fonts);
        let clock_pos = clock_screen_pos(hero_visual_anchor, ui);
        let clock_visible = is_visible(clock_pos, screen.width, screen.height, &lines);
        ui.set_hero_anchor(hero_world.x, hero_world.y);
        ui.set_clock_final(clock_pos.x, clock_pos.y);
        if clock_visible {
            draw_clock_at(
                frame,
                world,
                clock_pos.x.max(0) as u16,
                clock_pos.y.max(0) as u16,
                ui,
                fonts,
            );
        }
        draw_scrollbars(frame, screen, viewport, world);
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        _world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        _viewport: &Viewport,
        _viewport_rect: Rect,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let lines = clock_lines(ui, fonts);
        let hero_world = hero_world_pos(ui);
        let hero_visual_anchor = hero_visual_anchor(ui, hero_world);
        let clock_pos = clock_screen_pos(hero_visual_anchor, ui);
        let clock_visible = is_visible(clock_pos, width, height, &lines);
        ui.set_hero_anchor(hero_world.x, hero_world.y);
        if clock_visible {
            for (i, line) in lines.iter().enumerate() {
                let y = clock_pos.y + i as i32;
                if y < 0 || y >= height as i32 {
                    continue;
                }
                let x = clock_pos.x.max(0) as u16;
                write_string(&mut grid, x, y as u16, line, Style::default());
            }
        }
        ui.set_clock_final(clock_pos.x, clock_pos.y);
        LayerOutput { grid, mask: None }
    }
}

fn hero_world_pos(ui: &UiState) -> WorldPos {
    if ui.offsets.hero_world_x == 0 && ui.offsets.hero_world_y == 0 {
        WorldPos {
            x: ui.hero.x,
            y: ui.hero.y,
        }
    } else {
        WorldPos {
            x: ui.offsets.hero_world_x,
            y: ui.offsets.hero_world_y,
        }
    }
}

fn clock_screen_pos(hero_screen: WorldPos, ui: &UiState) -> WorldPos {
    anchor_to_world(
        hero_screen,
        WorldPos {
            x: ui.offsets.clock_dx as i32,
            y: ui.offsets.clock_dy as i32,
        },
    )
}

fn is_visible(pos: WorldPos, viewport_width: u16, viewport_height: u16, lines: &[String]) -> bool {
    let clock_width = lines
        .iter()
        .map(|l| l.chars().count() as i32)
        .max()
        .unwrap_or(0);
    let clock_height = lines.len() as i32;
    let max_x = viewport_width as i32 - clock_width;
    let max_y = viewport_height as i32 - clock_height;
    pos.x >= 0 && pos.y >= 0 && pos.x <= max_x && pos.y <= max_y
}

fn hero_visual_anchor(ui: &UiState, hero_world: WorldPos) -> WorldPos {
    let cached = ui.hero_visual_anchor.get();
    if cached != (0, 0) {
        return WorldPos {
            x: cached.0,
            y: cached.1,
        };
    }
    WorldPos {
        x: hero_world.x + ui.offsets.hero_dx,
        y: hero_world.y + ui.offsets.hero_dy,
    }
}

#[allow(dead_code)]
fn draw_scrollbars(
    frame: &mut Frame,
    viewport_rect: Rect,
    viewport: &Viewport,
    world: &WorldState,
) {
    if viewport_rect.width == 0 || viewport_rect.height == 0 {
        return;
    }

    let max_x = world.grid.width.saturating_sub(viewport.width).max(1) as f32;
    let max_y = world.grid.height.saturating_sub(viewport.height).max(1) as f32;
    let ratio_x = (viewport.x.max(0) as f32 / max_x).clamp(0.0, 1.0);
    let ratio_y = (viewport.y.max(0) as f32 / max_y).clamp(0.0, 1.0);

    let thumb_x =
        viewport_rect.x + ((viewport_rect.width.saturating_sub(1)) as f32 * ratio_x) as u16;
    let thumb_y =
        viewport_rect.y + ((viewport_rect.height.saturating_sub(1)) as f32 * ratio_y) as u16;
    let right_x = viewport_rect.x + viewport_rect.width.saturating_sub(1);
    let bottom_y = viewport_rect.y + viewport_rect.height.saturating_sub(1);

    if let Some(cell) = frame.buffer_mut().cell_mut((thumb_x, bottom_y)) {
        cell.set_symbol("▓").set_fg(Color::DarkGray);
    }
    if let Some(cell) = frame.buffer_mut().cell_mut((right_x, thumb_y)) {
        cell.set_symbol("▓").set_fg(Color::DarkGray);
    }
}
