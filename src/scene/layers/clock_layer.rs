use crate::core::spatial::SpatialScreenPoint as ScreenPos;
use crate::core::world::WorldState;
use crate::render::clock::clock_lines;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;

pub struct ClockLayer;

impl Layer for ClockLayer {
    fn z_index(&self) -> i32 {
        100
    }

    fn render_into_grid(
        &self,
        grid: &mut Grid,
        _world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> Option<crate::render::mask::Mask> {
        if !ui.companions_visible_in_active_world() {
            return None;
        }
        let lines = clock_lines(ui, fonts);
        let screen_pos = ctx.clock_screen();
        if is_visible(screen_pos, grid.width, grid.height, &lines) {
            for (i, line) in lines.iter().enumerate() {
                write_string(
                    grid,
                    screen_pos.x.max(0) as u16,
                    screen_pos.y.max(0) as u16 + i as u16,
                    line,
                    theme_style::clock_text(),
                );
            }
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
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let mask = self.render_into_grid(&mut grid, world, ui, fonts, ctx);
        LayerOutput { grid, mask }
    }
}

fn is_visible(pos: ScreenPos, viewport_width: u16, viewport_height: u16, lines: &[String]) -> bool {
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
    use crate::core::spatial::{SpatialPoint as WorldPos, SpatialScreenPoint as ScreenPos};
    use crate::core::world::WorldState;
    use crate::render::compositor::merge_grid;
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::theme::style as theme_style;
    use crate::ui::state::UiState;
    use ratatui::prelude::Rect;

    #[test]
    fn clock_uses_projection_from_the_shared_render_state() {
        let lines = vec!["12:34".to_string()];
        let render_state = RenderState {
            world: WorldFrame {
                hero_world: WorldPos { x: 50, y: 30 },
                hero_visual_anchor: WorldPos { x: 40, y: 20 },
                clock_world: WorldPos { x: 45, y: 25 },
                weather_world: WorldPos { x: 55, y: 26 },
                date_world: WorldPos { x: 45, y: 23 },
                calendar_world: WorldPos { x: 60, y: 22 },
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
        assert_eq!(render_state.clock_screen(), ScreenPos { x: 15, y: 16 });
    }

    #[test]
    fn clock_glyphs_keep_their_own_foreground_when_merged_over_styled_content() {
        let layer = super::ClockLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
        let ctx = RenderState {
            world: WorldFrame {
                hero_world: WorldPos { x: 50, y: 30 },
                hero_visual_anchor: WorldPos { x: 40, y: 20 },
                clock_world: WorldPos { x: 45, y: 25 },
                weather_world: WorldPos { x: 55, y: 26 },
                date_world: WorldPos { x: 45, y: 23 },
                calendar_world: WorldPos { x: 60, y: 22 },
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
        let ui = UiState::new();
        let mut base = crate::render::compositor::Grid::new(124, 32);
        let clock = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid;
        let idx = clock
            .cells
            .iter()
            .position(|cell| cell.symbol != ' ')
            .expect("clock grid should render at least one visible glyph");
        base.cells[idx].symbol = '#';
        base.cells[idx].style.fg = Some(ratatui::style::Color::Rgb(178, 78, 46));

        merge_grid(&mut base, &clock, None);

        assert_eq!(base.cells[idx].symbol, clock.cells[idx].symbol);
        assert_eq!(base.cells[idx].style.fg, theme_style::clock_text().fg);
    }
}
