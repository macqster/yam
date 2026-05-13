use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::{Layer, RenderState};
use crate::ui::state::UiState;
use ratatui::prelude::*;

pub struct FieldLayer;

impl Layer for FieldLayer {
    fn z_index(&self) -> i32 {
        0
    }

    fn is_field_layer(&self) -> bool {
        true
    }

    fn render_into_grid(
        &self,
        grid: &mut Grid,
        world: &WorldState,
        _ui: &UiState,
        _fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> Option<crate::render::mask::Mask> {
        for y in 0..ctx.hud.viewport_rect.height.min(grid.height) {
            let mut line = String::new();
            for x in 0..ctx.hud.viewport_rect.width.min(grid.width) {
                let wx = ctx.hud.viewport.x + x as i32;
                let wy = ctx.hud.viewport.y + y as i32;
                if wx >= 0
                    && wy >= 0
                    && (wx as usize) < world.grid.width as usize
                    && (wy as usize) < world.grid.height as usize
                {
                    if let Some(cell) = world.grid.get(wx as usize, wy as usize) {
                        let idx = world.grid.index(wx as u16, wy as u16);
                        let value = world.fields.density[idx];
                        let _ = cell;
                        line.push(density_to_char(value));
                    } else {
                        line.push(' ');
                    }
                } else {
                    line.push(' ');
                }
            }
            write_string(
                grid,
                ctx.hud.viewport_rect.x,
                ctx.hud.viewport_rect.y + y,
                &line,
                Style::default(),
            );
        }
        None
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> crate::scene::LayerOutput {
        let mut grid = Grid::new(width, height);
        let mask = self.render_into_grid(&mut grid, world, ui, fonts, ctx);
        crate::scene::LayerOutput { grid, mask }
    }
}

fn density_to_char(v: f32) -> char {
    match v {
        v if v > 0.75 => '█',
        v if v > 0.5 => '▓',
        v if v > 0.25 => '▒',
        v if v > 0.1 => '░',
        _ => ' ',
    }
}
