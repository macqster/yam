use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::render::hero::draw_hero_debug_at;
use crate::render::mask::Mask;
use crate::scene::{Layer, LayerOutput};
use crate::ui::state::UiState;
use ratatui::prelude::*;

pub struct HeroLayer;

impl Layer for HeroLayer {
    fn z_index(&self) -> i32 {
        10
    }

    fn render(
        &self,
        frame: &mut Frame<'_>,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _viewport: &crate::scene::viewport::Viewport,
        _viewport_rect: Rect,
    ) {
        let hero = &ui.hero;
        let hero_world_x = hero_world_pos(ui).x;
        let hero_world_y = hero_world_pos(ui).y;
        let hero_x = hero_world_x - ui.camera.x + ui.offsets.hero_dx;
        let hero_y = hero_world_y - ui.camera.y + ui.offsets.hero_dy;
        let hero_visual_anchor_x = hero_x + hero.width as i32 / 2;
        let hero_visual_anchor_y = hero_y + hero.height as i32 / 3;
        let normalized = normalize_lines(hero.frame().clone(), hero.width, hero.height);
        assert_eq!(normalized.len() as u16, hero.height);
        if let Some(first) = normalized.first() {
            assert_eq!(first.chars().count() as u16, hero.width);
        }
        if ui.debug_layout {
            draw_hero_debug_at(
                frame,
                hero,
                hero_x,
                hero_y,
                ui.offsets.hero_dx,
                ui.offsets.hero_dy,
            );
        }
        ui.set_hero_anchor(hero_world_x, hero_world_y);
        ui.set_hero_visual_anchor(hero_visual_anchor_x, hero_visual_anchor_y);
        draw_normalized_hero(frame, &normalized, hero_x, hero_y, hero.width, hero.height);
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _viewport: &crate::scene::viewport::Viewport,
        _viewport_rect: Rect,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let hero = &ui.hero;
        let hero_world = hero_world_pos(ui);
        let hero_world_x = hero_world.x;
        let hero_world_y = hero_world.y;
        let hero_x = hero_world_x - ui.camera.x + ui.offsets.hero_dx;
        let hero_y = hero_world_y - ui.camera.y + ui.offsets.hero_dy;
        let hero_visual_anchor_x = hero_x + hero.width as i32 / 2;
        let hero_visual_anchor_y = hero_y + hero.height as i32 / 3;
        ui.set_hero_anchor(hero_world_x, hero_world_y);
        ui.set_hero_visual_anchor(hero_visual_anchor_x, hero_visual_anchor_y);
        let normalized = normalize_lines(hero.frame().clone(), hero.width, hero.height);
        debug_assert_eq!(normalized.len() as u16, hero.height);
        let mut mask = Mask::new(width as usize, height as usize);

        for (row_idx, row) in normalized.into_iter().enumerate() {
            let py = hero_y + row_idx as i32;
            if py < 0 {
                continue;
            }
            if py >= grid.height as i32 {
                break;
            }
            let clip_cols = hero_x.clamp(i32::MIN, 0).unsigned_abs() as usize;
            let draw_x = hero_x.max(0) as u16;
            let draw_y = py as u16;
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

#[allow(dead_code)]
fn draw_normalized_hero(
    frame: &mut Frame<'_>,
    lines: &[String],
    start_x: i32,
    start_y: i32,
    width: u16,
    height: u16,
) {
    let area = frame.area();
    for row in 0..height as usize {
        let y = start_y + row as i32;
        if y < area.y as i32 || y >= area.bottom() as i32 {
            continue;
        }
        let x = start_x.max(area.x as i32) as u16;
        let y = y as u16;
        let line = lines
            .get(row)
            .cloned()
            .unwrap_or_else(|| " ".repeat(width as usize));
        frame.buffer_mut().set_string(x, y, line, Style::default());
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
        normalized.push(padded_line(width));
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

fn padded_line(width: u16) -> String {
    " ".repeat(width as usize)
}

fn hard_lock_text(text: &mut String, width: u16) {
    if text.chars().count() > width as usize {
        *text = text.chars().take(width as usize).collect::<String>();
    }
    if text.chars().count() < width as usize {
        text.push_str(&" ".repeat(width as usize - text.chars().count()));
    }
}

fn hero_world_pos(ui: &UiState) -> crate::scene::coords::WorldPos {
    if ui.offsets.hero_world_x == 0 && ui.offsets.hero_world_y == 0 {
        crate::scene::coords::WorldPos {
            x: ui.hero.x,
            y: ui.hero.y,
        }
    } else {
        crate::scene::coords::WorldPos {
            x: ui.offsets.hero_world_x,
            y: ui.offsets.hero_world_y,
        }
    }
}
