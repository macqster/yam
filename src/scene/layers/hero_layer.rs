use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::render::mask::Mask;
use crate::scene::coords::world_to_screen;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::ui::state::UiState;
use ratatui::prelude::*;

pub struct HeroLayer;

impl Layer for HeroLayer {
    fn z_index(&self) -> i32 {
        10
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
        let hero = &ui.hero;
        let hero_x = ctx.world.hero_visual_anchor.x;
        let hero_y = ctx.world.hero_visual_anchor.y;
        let cam_x = ctx.hud.camera.x;
        let cam_y = ctx.hud.camera.y;
        let normalized = normalize_lines(hero.frame().clone(), hero.width, hero.height);
        debug_assert_eq!(normalized.len() as u16, hero.height);
        let mut mask = Mask::new(width as usize, height as usize);

        for (row_idx, row) in normalized.into_iter().enumerate() {
            let py = hero_y + row_idx as i32;
            let screen = world_to_screen(
                crate::scene::coords::WorldPos { x: hero_x, y: py },
                cam_x,
                cam_y,
            );
            if screen.y < 0 {
                continue;
            }
            if screen.y >= grid.height as i32 {
                break;
            }
            let clip_cols = screen.x.clamp(i32::MIN, 0).unsigned_abs() as usize;
            let draw_x = screen.x.max(0) as u16;
            let draw_y = screen.y as u16;
            let clipped_row = row.chars().skip(clip_cols).collect::<String>();
            write_string(&mut grid, draw_x, draw_y, &clipped_row, Style::default());
            for (col_idx, ch) in clipped_row.chars().enumerate() {
                let x = draw_x as usize + col_idx;
                let y = draw_y as usize;
                if x >= mask.width || y >= mask.height {
                    continue;
                }
                if ch != ' ' {
                    mask.set(x, y, false);
                }
            }
        }

        LayerOutput {
            grid,
            mask: Some(mask),
        }
    }
}

fn normalize_lines(
    lines: Vec<ratatui::text::Line<'static>>,
    width: u16,
    height: u16,
) -> Vec<String> {
    let mut normalized = Vec::with_capacity(height as usize);
    for line in lines.into_iter().take(height as usize) {
        normalized.push(normalize_line(line, width));
    }
    while normalized.len() < height as usize {
        normalized.push(" ".repeat(width as usize));
    }
    if normalized.len() > height as usize {
        normalized.truncate(height as usize);
    }
    debug_assert_eq!(normalized.len(), height as usize);
    for line in &mut normalized {
        hard_lock_text(line, width);
    }
    normalized
}

fn normalize_line(line: ratatui::text::Line<'static>, width: u16) -> String {
    let mut chars = Vec::new();
    for span in line.spans {
        for ch in span.content.chars() {
            let ch = match ch {
                '\0' | '\r' | '\t' => ' ',
                other => other,
            };
            chars.push(ch);
            if chars.len() >= width as usize {
                break;
            }
        }
        if chars.len() >= width as usize {
            break;
        }
    }

    while chars.len() < width as usize {
        chars.push(' ');
    }

    chars.into_iter().collect::<String>()
}

fn hard_lock_text(text: &mut String, width: u16) {
    if text.chars().count() > width as usize {
        *text = text.chars().take(width as usize).collect::<String>();
    }
    if text.chars().count() < width as usize {
        text.push_str(&" ".repeat(width as usize - text.chars().count()));
    }
}
