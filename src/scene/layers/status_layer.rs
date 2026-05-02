use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;
use crate::ui::widgets::status::build_status_label;

pub struct StatusLayer;

impl Layer for StatusLayer {
    fn z_index(&self) -> i32 {
        1000
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
        let footer_y = footer_row(height);
        paint_footer_row(&mut grid, footer_y, width);
        let left_text = if ui.meta.dev_mode {
            "  [q]uit   •   [d]ev mode   •   [h]otkeys   •   [m]ove   •   [p]ointer   •   [C]/[c] camera home"
        } else {
            "  [q]uit   •   [d]ev mode"
        };
        let right_text = build_status_label();
        let footer_style = theme_style::footer_bar();
        write_string(&mut grid, 0, footer_y, left_text, footer_style);
        let stamp_width = right_text.chars().count() as u16 + 2;
        let x = width.saturating_sub(stamp_width);
        write_string(&mut grid, x, footer_y, &right_text, footer_style);
        LayerOutput { grid, mask: None }
    }
}

fn footer_row(height: u16) -> u16 {
    height.saturating_sub(1)
}

fn paint_footer_row(grid: &mut Grid, y: u16, width: u16) {
    let style = theme_style::footer_bar();
    for x in 0..width {
        if let Some(cell) = grid.cell_mut(x, y) {
            cell.symbol = ' ';
            cell.style = style;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{build_status_label, footer_row, StatusLayer};
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
    fn footer_uses_bottom_row_for_any_height() {
        assert_eq!(footer_row(57), 56);
        assert_eq!(footer_row(36), 35);
        assert_eq!(footer_row(1), 0);
        assert_eq!(footer_row(0), 0);
    }

    #[test]
    fn footer_row_is_color_highlighted_across_the_full_width() {
        let layer = StatusLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
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
        let ui = UiState::new();
        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let row = footer_row(32);
        let first = &output.grid.cells[output.grid.index(0, row)];
        let middle = &output.grid.cells[output.grid.index(50, row)];

        assert_eq!(first.style.bg, Some(crate::theme::palette::FOOTER_BG));
        assert_eq!(middle.style.bg, Some(crate::theme::palette::FOOTER_BG));
        assert_eq!(first.style.fg, Some(crate::theme::palette::FOOTER_FG));
        assert_eq!(middle.style.fg, Some(crate::theme::palette::FOOTER_FG));
    }

    #[test]
    fn default_footer_shows_clean_dev_hint_and_version_stamp() {
        let layer = StatusLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
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
        let ui = UiState::new();
        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = output.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("  [q]uit   •   [d]ev mode"));
        assert!(text.contains(&build_status_label()));
        assert!(!text.contains("space - play/pause"));
        assert!(!text.contains(". - step"));
    }

    #[test]
    fn dev_footer_keeps_the_same_compact_grammar() {
        let layer = StatusLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
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
        let mut ui = UiState::new();
        ui.meta.dev_mode = true;
        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = output.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("  [q]uit   •   [d]ev mode"));
        assert!(text.contains("[h]otkeys"));
        assert!(text.contains("[m]ove"));
        assert!(text.contains("[p]ointer"));
        assert!(text.contains("[C]/[c] camera home"));
        assert!(!text.contains("[space] play/pause"));
        assert!(!text.contains("[.] step"));
    }
}
