use crate::core::world::{WorldKind, WorldState};
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
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        if world.kind != WorldKind::MainScene {
            return LayerOutput { grid, mask: None };
        }
        let lines = clock_lines(ui, fonts);
        let screen_pos = ctx.clock_screen();
        if is_visible(screen_pos, width, height, &lines) {
            for (i, line) in lines.iter().enumerate() {
                write_string(
                    &mut grid,
                    screen_pos.x.max(0) as u16,
                    screen_pos.y.max(0) as u16 + i as u16,
                    line,
                    Style::default(),
                );
            }
        }
        LayerOutput { grid, mask: None }
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
    use super::is_visible;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::WorldPos;
    use crate::scene::viewport::Viewport;
    use ratatui::prelude::Rect;

    #[test]
    fn clock_uses_projection_from_the_shared_render_state() {
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

        assert!(is_visible(render_state.clock_screen(), 124, 32, &lines));
        assert_eq!(render_state.clock_screen(), WorldPos { x: 15, y: 16 });
    }
}
