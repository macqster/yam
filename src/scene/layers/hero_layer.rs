use crate::core::spatial::{
    SpatialPoint as WorldPos, SpatialProjection, SpatialResolver, SpatialScreenPoint,
};
use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::render::mask::Mask;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::ui::state::UiState;
use ratatui::text::{Line, Span};

pub struct HeroLayer;

#[derive(Copy, Clone)]
struct HeroProjection {
    resolver: SpatialResolver,
}

impl HeroProjection {
    fn new(camera_x: i32, camera_y: i32, viewport_width: u16, viewport_height: u16) -> Self {
        Self {
            resolver: SpatialResolver::new(SpatialProjection::new(
                camera_x,
                camera_y,
                viewport_width,
                viewport_height,
            )),
        }
    }

    fn project(self, point: WorldPos) -> SpatialScreenPoint {
        self.resolver.world_to_screen_point(point)
    }
}

impl Layer for HeroLayer {
    fn z_index(&self) -> i32 {
        10
    }

    fn render_into_grid(
        &self,
        grid: &mut Grid,
        world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> Option<Mask> {
        if !world.kind.has_main_scene_composition() {
            return None;
        }
        let hero = &ui.hero;
        let hero_x = ctx.world.hero_visual_anchor.x;
        let hero_y = ctx.world.hero_visual_anchor.y;
        let cam_x = ctx.hud.camera.x;
        let cam_y = ctx.hud.camera.y;
        let projection =
            HeroProjection::new(cam_x, cam_y, ctx.hud.camera.width, ctx.hud.camera.height);
        let normalized = normalize_lines(hero.frame().clone(), hero.width, hero.height);
        debug_assert_eq!(normalized.len() as u16, hero.height);
        let mut mask = Mask::new(grid.width as usize, grid.height as usize);

        for (row_idx, row) in normalized.into_iter().enumerate() {
            let py = hero_y - row_idx as i32;
            let screen = projection.project(WorldPos { x: hero_x, y: py });
            if screen.y >= grid.height as i32 {
                break;
            }
            let Some((draw_x, draw_y, clip_cols)) =
                hero_row_grid_position(screen, grid.width, grid.height)
            else {
                continue;
            };
            let clipped_row = clip_line(&row, clip_cols);
            let mut cursor_x = draw_x;
            for span in clipped_row.spans {
                let content = span.content.as_ref();
                if content.is_empty() {
                    continue;
                }
                write_string(grid, cursor_x, draw_y, content, span.style);
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

        Some(mask)
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let mask = self.render_into_grid(&mut grid, world, ui, fonts, ctx);
        LayerOutput { grid, mask }
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

fn hero_row_grid_position(
    screen: SpatialScreenPoint,
    grid_width: u16,
    grid_height: u16,
) -> Option<(u16, u16, usize)> {
    let draw_y = u16::try_from(screen.y).ok()?;
    if draw_y >= grid_height {
        return None;
    }

    let clip_cols = if screen.x < 0 {
        screen.x.unsigned_abs() as usize
    } else {
        0
    };
    let draw_x = if screen.x < 0 {
        0
    } else {
        u16::try_from(screen.x).ok()?
    };
    if draw_x >= grid_width {
        return None;
    }

    Some((draw_x, draw_y, clip_cols))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hero_row_position_clips_negative_x_to_grid_origin() {
        assert_eq!(
            hero_row_grid_position(SpatialScreenPoint { x: -3, y: 2 }, 10, 5),
            Some((0, 2, 3))
        );
    }

    #[test]
    fn hero_row_position_skips_oversized_positive_x_without_wrapping() {
        assert_eq!(
            hero_row_grid_position(SpatialScreenPoint { x: 70_000, y: 2 }, 80, 24),
            None
        );
    }
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
