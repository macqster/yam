use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::layers::modal::{paint_modal_shell, ModalFooter, ModalFrame};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::ui::state::UiState;
use crate::weather::render::sprite_inspection_lines;
use ratatui::text::Line;

pub struct WeatherPopupLayer;

const H_PADDING: u16 = 4;
const V_PADDING: u16 = 2;

impl Layer for WeatherPopupLayer {
    fn z_index(&self) -> i32 {
        398
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        if !ui.show_dev_surfaces() || !ui.meta.weather_popup_open {
            return LayerOutput { grid, mask: None };
        }

        let lines = sprite_inspection_lines(ui.weather_locale);
        let content_width = lines.iter().map(line_width).max().unwrap_or(0) as u16;
        let content_height = lines.len() as u16;
        let frame = ModalFrame::centered(
            width,
            height,
            content_width
                .saturating_add(H_PADDING * 2)
                .saturating_add(2),
            content_height
                .saturating_add(V_PADDING * 2)
                .saturating_add(2),
        );
        paint_modal_shell(
            &mut grid,
            frame,
            "[W]eather",
            Some(ModalFooter {
                left: "sprite atlas review",
                right: "? ⎋",
            }),
        );

        let body_x = frame.x + 1 + H_PADDING;
        let body_y = frame.y + 1 + V_PADDING;
        for (row, line) in lines.iter().enumerate() {
            let mut cursor_x = body_x;
            let cursor_y = body_y + row as u16;
            for span in &line.spans {
                write_string(
                    &mut grid,
                    cursor_x,
                    cursor_y,
                    span.content.as_ref(),
                    span.style,
                );
                cursor_x += unicode_width::UnicodeWidthStr::width(span.content.as_ref()) as u16;
            }
        }

        LayerOutput { grid, mask: None }
    }
}

fn line_width(line: &Line<'_>) -> usize {
    line.spans
        .iter()
        .map(|span| unicode_width::UnicodeWidthStr::width(span.content.as_ref()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::WeatherPopupLayer;
    use crate::core::world::WorldState;
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::WorldPos;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::ui::state::UiState;
    use ratatui::prelude::Rect;

    #[test]
    fn weather_popup_requires_dev_mode_and_open_state() {
        let layer = WeatherPopupLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
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
        let mut ui = UiState::new();

        let closed = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        assert_eq!(closed.grid.cells[closed.grid.index(62, 16)].symbol, ' ');

        ui.meta.dev_mode = true;
        ui.meta.weather_popup_open = true;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("[W]eather"));
        assert!(text.contains("weather sprites"));
        assert!(text.contains("dev popup atlas review"));
        assert!(text.contains("sprite atlas review"));
        assert!(text.contains("? ⎋"));
        let center = open.grid.cells[open.grid.index(62, 16)].style.bg;
        assert_eq!(center, Some(crate::theme::palette::MODAL_BG));
    }
}
