use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::render::mask::Mask;
use crate::scene::coords::world_to_screen;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::ui::state::UiState;
use ratatui::text::{Line, Span};

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
            let clipped_row = clip_line(&row, clip_cols);
            let mut cursor_x = draw_x;
            for span in clipped_row.spans {
                let content = span.content.as_ref();
                if content.is_empty() {
                    continue;
                }
                write_string(&mut grid, cursor_x, draw_y, content, span.style);
                for (col_idx, ch) in content.chars().enumerate() {
                    let x = cursor_x as usize + col_idx;
                    let y = draw_y as usize;
                    if x >= mask.width || y >= mask.height {
                        continue;
                    }
                    if ch != ' ' {
                        mask.set(x, y, false);
                    }
                }
                cursor_x = cursor_x.saturating_add(content.chars().count() as u16);
            }
        }

        LayerOutput {
            grid,
            mask: Some(mask),
        }
    }
}

fn normalize_lines(lines: Vec<Line<'static>>, width: u16, height: u16) -> Vec<Line<'static>> {
    let mut normalized = Vec::with_capacity(height as usize);
    for line in lines.into_iter().take(height as usize) {
        normalized.push(normalize_line(line, width));
    }
    while normalized.len() < height as usize {
        normalized.push(Line::from(vec![Span::raw(" ".repeat(width as usize))]));
    }
    if normalized.len() > height as usize {
        normalized.truncate(height as usize);
    }
    debug_assert_eq!(normalized.len(), height as usize);
    normalized
}

fn normalize_line(line: Line<'static>, width: u16) -> Line<'static> {
    let mut remaining = width as usize;
    let mut spans = Vec::new();
    for span in line.spans {
        if remaining == 0 {
            break;
        }

        let mut chars = String::new();
        for ch in span.content.chars() {
            let ch = match ch {
                '\0' | '\r' | '\t' => ' ',
                other => other,
            };
            chars.push(ch);
            if chars.chars().count() >= remaining {
                break;
            }
        }

        if chars.is_empty() {
            continue;
        }

        let count = chars.chars().count();
        remaining = remaining.saturating_sub(count);
        spans.push(Span::styled(chars, span.style));
    }

    if remaining > 0 {
        spans.push(Span::raw(" ".repeat(remaining)));
    }

    Line::from(spans)
}

fn clip_line(line: &Line<'static>, skip_cols: usize) -> Line<'static> {
    let mut remaining = skip_cols;
    let mut spans = Vec::new();

    for span in &line.spans {
        let content = span.content.as_ref();
        let content_width = content.chars().count();
        if remaining >= content_width {
            remaining -= content_width;
            continue;
        }

        let clipped = content.chars().skip(remaining).collect::<String>();
        remaining = 0;
        if !clipped.is_empty() {
            spans.push(Span::styled(clipped, span.style));
        }
    }

    Line::from(spans)
}
