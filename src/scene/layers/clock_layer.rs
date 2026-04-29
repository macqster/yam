use crate::core::world::WorldState;
use crate::render::clock::clock_lines;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::coords::WorldPos;
use crate::scene::{Layer, LayerOutput, RenderState};
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
        ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let lines = clock_lines(ui, fonts);
        let origin = clock_origin(ui.meta.anchored_clock, width, height, ctx, &lines);
        if let Some((start_x, start_y)) = origin {
            for (i, line) in lines.iter().enumerate() {
                write_string(
                    &mut grid,
                    start_x,
                    start_y + i as u16,
                    line,
                    Style::default(),
                );
            }
        }
        LayerOutput { grid, mask: None }
    }
}

fn clock_origin(
    anchored_clock: bool,
    width: u16,
    height: u16,
    ctx: &RenderState,
    lines: &[String],
) -> Option<(u16, u16)> {
    if anchored_clock {
        let screen_pos = ctx.clock_screen();
        if is_visible(screen_pos, width, height, lines) {
            Some((screen_pos.x.max(0) as u16, screen_pos.y.max(0) as u16))
        } else {
            None
        }
    } else {
        let clock_width = lines
            .iter()
            .map(|l| l.chars().count() as i32)
            .max()
            .unwrap_or(0);
        Some((width.saturating_sub(clock_width as u16 + 1), 0))
    }
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

#[cfg(test)]
mod tests {
    use super::{clock_origin, is_visible};
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::WorldPos;
    use crate::scene::viewport::Viewport;
    use ratatui::prelude::Rect;

    #[test]
    fn anchored_clock_uses_projection_while_hud_clock_stays_screen_attached() {
        let lines = vec!["12:34".to_string()];
        let render_state = RenderState {
            world: WorldFrame {
                hero_world: WorldPos { x: 50, y: 30 },
                hero_visual_anchor: WorldPos { x: 40, y: 20 },
                clock_world: WorldPos { x: 45, y: 25 },
            },
            hud: HudFrame {
                viewport: Viewport {
                    x: 30,
                    y: 10,
                    width: 124,
                    height: 32,
                },
                viewport_rect: Rect::new(0, 0, 124, 32),
                camera: Camera {
                    x: 30,
                    y: 10,
                    width: 124,
                    height: 32,
                    follow_hero: false,
                },
            },
        };

        let anchored = clock_origin(true, 124, 32, &render_state, &lines);
        let hud = clock_origin(false, 124, 32, &render_state, &lines);

        assert!(is_visible(render_state.clock_screen(), 124, 32, &lines));
        assert_eq!(anchored, Some((15, 15)));
        assert_eq!(hud, Some((118, 0)));
    }
}
