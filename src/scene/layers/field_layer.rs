use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::viewport::Viewport;
use crate::scene::{Layer, LayerOutput};
use crate::ui::state::UiState;
use ratatui::{prelude::*, widgets::Paragraph};

pub struct FieldLayer;

impl Layer for FieldLayer {
    fn z_index(&self) -> i32 {
        0
    }

    fn is_field_layer(&self) -> bool {
        true
    }

    fn render(
        &self,
        frame: &mut Frame<'_>,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        viewport: &Viewport,
        viewport_rect: Rect,
    ) {
        let _ = (ui, fonts);
        let mut lines = Vec::new();
        for y in 0..viewport_rect.height {
            let mut line = String::new();
            for x in 0..viewport_rect.width {
                let wx = viewport.x + x as i32;
                let wy = viewport.y + y as i32;
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
            lines.push(line);
        }

        frame.render_widget(Paragraph::new(lines.join("\n")), viewport_rect);
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        viewport: &Viewport,
        viewport_rect: Rect,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let _ = (ui, fonts);
        for y in 0..viewport_rect.height.min(grid.height) {
            let mut line = String::new();
            for x in 0..viewport_rect.width.min(grid.width) {
                let wx = viewport.x + x as i32;
                let wy = viewport.y + y as i32;
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
                &mut grid,
                viewport_rect.x,
                viewport_rect.y + y,
                &line,
                Style::default(),
            );
        }
        LayerOutput { grid, mask: None }
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
