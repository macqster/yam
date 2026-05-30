use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::layers::modal::{paint_modal_shell, ModalFooter, ModalFrame};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;

pub struct MoveLayer;

impl Layer for MoveLayer {
    fn z_index(&self) -> i32 {
        395
    }

    fn should_render(&self, ui: &UiState) -> bool {
        ui.show_dev_surfaces() && ui.meta.move_mode_open
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
        if !self.should_render(ui) {
            return LayerOutput { grid, mask: None };
        }

        let frame = bottom_move_frame(width, height);
        paint_modal_shell(
            &mut grid,
            frame,
            "[m]ove",
            Some(ModalFooter {
                left: "↑ ↓ ← →  ──  ⇥",
                right: "? ⎋",
            }),
        );
        let tabs_y = frame.y + 2;
        draw_target_tabs(&mut grid, frame, tabs_y, ui);

        LayerOutput { grid, mask: None }
    }
}

fn bottom_move_frame(width: u16, height: u16) -> ModalFrame {
    let desired_width = 68u16.min(width.saturating_sub(12));
    let desired_height = 5u16;
    let x = (width.saturating_sub(desired_width)) / 2;
    let y = height.saturating_sub(desired_height + 2);
    ModalFrame {
        x,
        y,
        width: desired_width,
        height: desired_height,
    }
}

fn draw_target_tabs(grid: &mut Grid, frame: ModalFrame, y: u16, ui: &UiState) {
    let tabs = [
        ("hero", crate::ui::state::MoveTarget::Hero),
        ("clock", crate::ui::state::MoveTarget::Clock),
        ("weather", crate::ui::state::MoveTarget::Weather),
        ("date", crate::ui::state::MoveTarget::Date),
    ];
    let formatted_tabs: Vec<(String, ratatui::style::Style)> = tabs
        .into_iter()
        .map(|(label, target)| {
            let formatted = if target == ui.meta.move_target {
                format!("[{}]", label)
            } else {
                format!(" {} ", label)
            };
            let style = if target == ui.meta.move_target {
                theme_style::settings_tab_active()
            } else {
                theme_style::settings_tab_inactive()
            };
            (formatted, style)
        })
        .collect();
    let total_width: u16 = formatted_tabs
        .iter()
        .map(|(label, _)| label.chars().count() as u16)
        .sum::<u16>()
        + formatted_tabs.len().saturating_sub(1) as u16 * 2;
    let body_x = frame.x + 2;
    let body_width = frame.width.saturating_sub(4);
    let mut cursor = body_x + body_width.saturating_sub(total_width) / 2;
    for (formatted, style) in formatted_tabs {
        write_string(grid, cursor, y, &formatted, style);
        cursor = cursor.saturating_add(formatted.chars().count() as u16 + 2);
    }
}

#[cfg(test)]
mod tests {
    use super::MoveLayer;
    use crate::core::spatial::SpatialPoint as WorldPos;
    use crate::core::world::WorldState;
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::ui::state::UiState;
    use ratatui::prelude::Rect;

    fn render_state() -> RenderState {
        RenderState {
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
        }
    }

    #[test]
    fn move_overlay_requires_dev_mode_and_open_state() {
        let layer = MoveLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
        let render_state = render_state();
        let mut ui = UiState::new();

        let closed = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        assert_eq!(closed.grid.cells[closed.grid.index(62, 16)].symbol, ' ');

        ui.meta.dev_mode = true;
        ui.meta.move_mode_open = true;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("[m]ove"));
        assert!(text.contains("[hero]"));
        assert!(text.contains(" clock "));
        assert!(text.contains(" weather "));
        assert!(text.contains(" date "));
        assert!(!text.contains(" calendar "));
        assert!(text.contains("? ⎋"));
        let frame = super::bottom_move_frame(124, 32);
        let interior = open.grid.cells[open.grid.index(frame.x + 3, frame.y + 3)]
            .style
            .bg;
        assert_eq!(interior, Some(crate::theme::palette::MODAL_BG));
    }

    #[test]
    fn move_overlay_keeps_visible_inner_padding_around_target_tabs() {
        let frame = super::bottom_move_frame(124, 32);
        assert_eq!(frame.width, 68);
    }
}
