use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::coords::WorldPos;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::ui::state::UiState;
use crate::weather::render::{compact_widget_lines, line_width};

pub struct WeatherLayer;

impl Layer for WeatherLayer {
    fn z_index(&self) -> i32 {
        100
    }

    fn render_into_grid(
        &self,
        grid: &mut Grid,
        world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> Option<crate::render::mask::Mask> {
        if !world.kind.has_main_scene_composition() {
            return None;
        }

        let snapshot = ui.weather_snapshot.as_ref()?;
        let lines = compact_widget_lines(snapshot, ui.weather_locale, ui.weather_layout);
        let screen_pos = ctx.weather_screen();
        if is_visible(screen_pos, grid.width, grid.height, &lines) {
            write_lines(
                grid,
                screen_pos.x.max(0) as u16,
                screen_pos.y.max(0) as u16,
                &lines,
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
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let mask = self.render_into_grid(&mut grid, world, ui, fonts, ctx);
        LayerOutput { grid, mask }
    }
}

fn write_lines(grid: &mut Grid, x: u16, y: u16, lines: &[ratatui::text::Line<'_>]) {
    for (row, line) in lines.iter().enumerate() {
        let mut cursor_x = x;
        let cursor_y = y + row as u16;
        for span in &line.spans {
            write_string(grid, cursor_x, cursor_y, span.content.as_ref(), span.style);
            cursor_x =
                cursor_x.saturating_add(
                    unicode_width::UnicodeWidthStr::width(span.content.as_ref()) as u16,
                );
        }
    }
}

fn is_visible(
    pos: WorldPos,
    viewport_width: u16,
    viewport_height: u16,
    lines: &[ratatui::text::Line<'_>],
) -> bool {
    let widget_width = lines.iter().map(line_width).max().unwrap_or(0) as i32;
    let widget_height = lines.len() as i32;
    let max_x = viewport_width as i32 - widget_width;
    let max_y = viewport_height as i32 - widget_height;
    pos.x >= 0 && pos.y >= 0 && pos.x <= max_x && pos.y <= max_y
}

#[cfg(test)]
mod tests {
    use super::is_visible;
    use crate::core::world::WorldKind;
    use crate::core::world::WorldState;
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::WorldPos;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::ui::state::UiState;
    use crate::weather::model::{WeatherLocale, WeatherLocation};
    use crate::weather::provider::{StaticWeatherProvider, WeatherProvider};
    use crate::weather::render::{compact_widget_lines, WeatherLayout};
    use ratatui::prelude::Rect;

    #[test]
    fn weather_uses_projection_from_the_shared_render_state() {
        let provider = StaticWeatherProvider;
        let snapshot = provider
            .snapshot(&WeatherLocation::named("Sulkowice"))
            .expect("static weather provider should always return a snapshot");
        let lines = compact_widget_lines(&snapshot, WeatherLocale::En, WeatherLayout::WttrCompact);
        let render_state = RenderState {
            world: WorldFrame {
                hero_world: WorldPos { x: 50, y: 30 },
                hero_visual_anchor: WorldPos { x: 40, y: 20 },
                clock_world: WorldPos { x: 45, y: 25 },
                weather_world: WorldPos { x: 55, y: 26 },
                date_world: WorldPos { x: 55, y: 20 },
                calendar_world: WorldPos { x: 65, y: 20 },
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

        assert!(is_visible(render_state.weather_screen(), 124, 32, &lines));
        assert_eq!(render_state.weather_screen(), WorldPos { x: 25, y: 15 });
    }

    #[test]
    fn weather_layer_renders_a_compact_widget() {
        let layer = super::WeatherLayer;
        let world = WorldState::new();
        let mut ui = UiState::new();
        ui.weather_snapshot = Some(
            StaticWeatherProvider
                .snapshot(&WeatherLocation::named("Sulkowice"))
                .expect("static weather provider should always return a snapshot"),
        );
        ui.weather_locale = WeatherLocale::En;
        ui.weather_layout = WeatherLayout::WttrCompact;
        let fonts = FontRegistry::new();
        let ctx = RenderState {
            world: WorldFrame {
                hero_world: WorldPos { x: 50, y: 30 },
                hero_visual_anchor: WorldPos { x: 40, y: 20 },
                clock_world: WorldPos { x: 45, y: 25 },
                weather_world: WorldPos { x: 55, y: 26 },
                date_world: WorldPos { x: 55, y: 20 },
                calendar_world: WorldPos { x: 65, y: 20 },
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

        let grid = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid;
        let text: String = grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(grid.cells.iter().any(|cell| cell.symbol != ' '));
        assert!(text.contains("Sulkowice") || text.contains("overcast") || text.contains("km/h"));
    }

    #[test]
    fn sandbox_world_no_longer_renders_the_palette_sheet_in_world_space() {
        let layer = super::WeatherLayer;
        let world = WorldState::for_kind(WorldKind::Sandbox);
        let mut ui = UiState::new();
        ui.weather_locale = WeatherLocale::En;
        let fonts = FontRegistry::new();
        let ctx = RenderState {
            world: WorldFrame {
                hero_world: WorldPos { x: 50, y: 30 },
                hero_visual_anchor: WorldPos { x: 40, y: 20 },
                clock_world: WorldPos { x: 45, y: 25 },
                weather_world: WorldPos { x: 55, y: 26 },
                date_world: WorldPos { x: 55, y: 20 },
                calendar_world: WorldPos { x: 65, y: 20 },
            },
            hud: HudFrame {
                viewport: Viewport {
                    x: 0,
                    y: 0,
                    width: 124,
                    height: 32,
                },
                viewport_rect: Rect::new(0, 0, 124, 32),
                camera: Camera {
                    x: 0,
                    y: 0,
                    width: 124,
                    height: 32,
                    follow_hero: false,
                },
            },
        };

        let grid = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid;

        assert_eq!(grid.cells[grid.index(2, 2)].symbol, ' ');
        assert_eq!(grid.cells[grid.index(2, 5)].symbol, ' ');
        assert_eq!(grid.cells[grid.index(2, 18)].symbol, ' ');
    }
}
