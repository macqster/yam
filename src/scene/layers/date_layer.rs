use chrono::{Datelike, Local, NaiveDate, Weekday};
use unicode_width::UnicodeWidthStr;

use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::coords::WorldPos;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;
use crate::weather::render::{compact_widget_lines, line_width, COMPACT_WEATHER_WIDTH};

pub struct DateLayer;

impl Layer for DateLayer {
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

        let text = polish_date_label(Local::now().date_naive());
        let screen_pos = centered_date_screen_pos(&text, ui, ctx);
        if is_visible(screen_pos, grid.width, grid.height, &text) {
            write_string(
                grid,
                screen_pos.x.max(0) as u16,
                screen_pos.y.max(0) as u16,
                &text,
                theme_style::weather_text(),
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

fn centered_date_screen_pos(text: &str, ui: &UiState, ctx: &RenderState) -> WorldPos {
    let weather_width = ui
        .weather_snapshot
        .as_ref()
        .map(|snapshot| {
            compact_widget_lines(snapshot, ui.weather_locale, ui.weather_layout)
                .iter()
                .map(line_width)
                .max()
                .unwrap_or(COMPACT_WEATHER_WIDTH)
        })
        .unwrap_or(COMPACT_WEATHER_WIDTH) as i32;
    let date_width = UnicodeWidthStr::width(text) as i32;
    let clock_left = ctx.clock_screen().x;
    let weather_right = ctx.weather_screen().x + weather_width;
    let companion_center = (clock_left + weather_right) / 2;

    WorldPos {
        x: companion_center - date_width / 2 - 1,
        y: ctx.date_screen().y,
    }
}

fn polish_date_label(date: NaiveDate) -> String {
    format!(
        "{}, {} {}",
        polish_weekday(date.weekday()),
        date.day(),
        polish_month_genitive(date.month())
    )
}

fn polish_weekday(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "poniedziałek",
        Weekday::Tue => "wtorek",
        Weekday::Wed => "środa",
        Weekday::Thu => "czwartek",
        Weekday::Fri => "piątek",
        Weekday::Sat => "sobota",
        Weekday::Sun => "niedziela",
    }
}

fn polish_month_genitive(month: u32) -> &'static str {
    match month {
        1 => "stycznia",
        2 => "lutego",
        3 => "marca",
        4 => "kwietnia",
        5 => "maja",
        6 => "czerwca",
        7 => "lipca",
        8 => "sierpnia",
        9 => "września",
        10 => "października",
        11 => "listopada",
        12 => "grudnia",
        _ => "miesiąca",
    }
}

fn is_visible(pos: WorldPos, viewport_width: u16, viewport_height: u16, text: &str) -> bool {
    let width = text.chars().count() as i32;
    let max_x = viewport_width as i32 - width;
    let max_y = viewport_height as i32 - 1;
    pos.x >= 0 && pos.y >= 0 && pos.x <= max_x && pos.y <= max_y
}

#[cfg(test)]
mod tests {
    use super::{centered_date_screen_pos, is_visible, polish_date_label};
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
    use crate::weather::render::WeatherLayout;
    use chrono::NaiveDate;
    use ratatui::prelude::Rect;
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn polish_date_label_uses_nominative_weekday_and_genitive_month() {
        let date = NaiveDate::from_ymd_opt(2026, 5, 11).expect("known date should be valid");
        assert_eq!(polish_date_label(date), "poniedziałek, 11 maja");
    }

    #[test]
    fn date_uses_projection_from_the_shared_render_state() {
        let label = "poniedziałek, 11 maja";
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

        assert!(is_visible(render_state.date_screen(), 124, 32, label));
        assert_eq!(render_state.date_screen(), WorldPos { x: 15, y: 18 });
    }

    #[test]
    fn date_layer_renders_a_single_polish_line() {
        let layer = super::DateLayer;
        let world = WorldState::new();
        let mut ui = UiState::new();
        ui.weather_snapshot = Some(
            StaticWeatherProvider
                .snapshot(&WeatherLocation::named("Krakow, Poland"))
                .expect("static weather provider should always return a snapshot"),
        );
        ui.weather_locale = WeatherLocale::Pl;
        ui.weather_layout = WeatherLayout::WttrCompact;
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

        let grid = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid;
        let label = polish_date_label(chrono::Local::now().date_naive());
        let pos = centered_date_screen_pos(&label, &ui, &ctx);
        let idx = grid.index(pos.x as u16, pos.y as u16);
        let first = label
            .chars()
            .next()
            .expect("rendered date label should not be empty");

        assert_eq!(grid.cells[idx].symbol, first);
    }

    #[test]
    fn date_self_centers_between_clock_and_weather_companions() {
        let mut ui = UiState::new();
        ui.weather_snapshot = Some(
            StaticWeatherProvider
                .snapshot(&WeatherLocation::named("Krakow, Poland"))
                .expect("static weather provider should always return a snapshot"),
        );
        ui.weather_locale = WeatherLocale::Pl;
        ui.weather_layout = WeatherLayout::WttrCompact;
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
        let short = centered_date_screen_pos("wtorek, 12 maja", &ui, &ctx);
        let long = centered_date_screen_pos("poniedziałek, 11 września", &ui, &ctx);

        let short_center = short.x + UnicodeWidthStr::width("wtorek, 12 maja") as i32 / 2;
        let long_center = long.x + UnicodeWidthStr::width("poniedziałek, 11 września") as i32 / 2;

        assert_eq!(short_center, long_center);
    }
}
