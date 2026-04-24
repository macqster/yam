use crate::core::world::WorldState;
use crate::render::clock::clock_lines;
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
        let hero_visual_anchor = hero_visual_anchor(hero_world, ui);
        let clock_pos = clock_screen_pos(hero_visual_anchor, ui);
        if is_visible(clock_pos, width, height, &lines) {
            for (i, line) in lines.iter().enumerate() {
                let y = clock_pos.y + i as i32;
                if y < 0 || y >= height as i32 {
                    continue;
                }
                let x = clock_pos.x.max(0) as u16;
                write_string(&mut grid, x, y as u16, line, Style::default());
            }
        }
        LayerOutput { grid, mask: None }
    }
}

fn hero_world_pos(ui: &UiState) -> WorldPos {
    WorldPos {
        x: ui.hero.x,
        y: ui.hero.y,
    }
}

fn hero_visual_anchor(hero_world: WorldPos, ui: &UiState) -> WorldPos {
    anchor_to_world(
        hero_world,
        WorldPos {
            x: ui.offsets.hero_dx,
            y: ui.offsets.hero_dy,
        },
    )
}

fn clock_screen_pos(hero_visual_anchor: WorldPos, ui: &UiState) -> WorldPos {
    anchor_to_world(
        hero_visual_anchor,
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
