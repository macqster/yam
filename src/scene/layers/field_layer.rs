use crate::scene::Layer;
use crate::{
    core::world::WorldState, render::fonts::FontRegistry, ui::layout::LayoutRegions,
    ui::state::UiState, scene::viewport::Viewport,
};
use ratatui::{prelude::*, widgets::Paragraph};

pub struct FieldLayer;

impl Layer for FieldLayer {
    fn z_index(&self) -> i32 {
        0
    }

    fn render(
        &self,
        frame: &mut Frame<'_>,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        viewport: &Viewport,
        viewport_rect: Rect,
        _layout: &LayoutRegions,
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
