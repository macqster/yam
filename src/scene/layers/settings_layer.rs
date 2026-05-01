use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::layers::modal::{paint_modal_shell, ModalFrame};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::{SettingsTab, UiState};

pub struct SettingsLayer;

impl Layer for SettingsLayer {
    fn z_index(&self) -> i32 {
        400
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
        if !ui.meta.dev_mode || !ui.meta.settings_open {
            return LayerOutput { grid, mask: None };
        }

        let frame = ModalFrame::centered(width, height, 68, 14);
        paint_modal_shell(&mut grid, frame, "[s]ettings");
        let body_x = frame.x + 2;
        let body_y = frame.y + 3;
        draw_tabs(&mut grid, body_x, frame.y + 1, ui.meta.settings_tab);
        draw_tab_body(&mut grid, body_x, body_y, frame.width - 4, ui);
        LayerOutput { grid, mask: None }
    }
}

fn draw_tabs(grid: &mut Grid, x: u16, y: u16, active: SettingsTab) {
    let tabs = [
        SettingsTab::Positions,
        SettingsTab::Widgets,
        SettingsTab::Gif,
        SettingsTab::Theme,
    ];
    let mut cursor = x;
    for tab in tabs {
        let label = if tab == active {
            format!("[{}]", tab.title())
        } else {
            format!(" {} ", tab.title())
        };
        write_string(grid, cursor, y, &label, theme_style::panel_text());
        cursor = cursor.saturating_add(label.chars().count() as u16 + 1);
    }
}

fn draw_tab_body(grid: &mut Grid, x: u16, y: u16, width: u16, ui: &UiState) {
    let lines = match ui.meta.settings_tab {
        SettingsTab::Positions => vec![
            format!("camera: ({}, {})", ui.offsets.camera_x, ui.offsets.camera_y),
            format!(
                "hero offset: ({}, {})",
                ui.offsets.hero_dx, ui.offsets.hero_dy
            ),
            format!(
                "clock offset: ({}, {})",
                ui.offsets.clock_dx, ui.offsets.clock_dy
            ),
        ],
        SettingsTab::Widgets => vec![
            format!("dev mode: {}", ui.meta.dev_mode),
            format!("settings open: {}", ui.meta.settings_open),
            format!("move mode: {}", ui.meta.move_mode_open),
            format!("move target: {}", ui.meta.move_target.title()),
            format!(
                "camera mode: {}",
                if ui.camera.follow_hero {
                    "follow-hero"
                } else {
                    "manual pan"
                }
            ),
            "clock: hero-attached world entity".to_string(),
        ],
        SettingsTab::Gif => vec![
            format!("hero fps: {:.1}", ui.offsets.hero_fps),
            format!("clock font: {}", ui.offsets.clock_font),
            "hero render: chafa-backed".to_string(),
        ],
        SettingsTab::Theme => vec![
            "theme: active runtime palette".to_string(),
            "theme editing can be wired in later".to_string(),
            "no theme ownership changed yet".to_string(),
        ],
    };

    for (row, line) in lines.iter().enumerate() {
        let clipped = if line.chars().count() as u16 > width {
            line.chars().take(width as usize).collect::<String>()
        } else {
            line.clone()
        };
        write_string(grid, x, y + row as u16, &clipped, theme_style::panel_text());
    }
}

#[cfg(test)]
mod tests {
    use super::SettingsLayer;
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
    fn settings_overlay_requires_dev_mode_and_open_state() {
        let layer = SettingsLayer;
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

        let closed = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        assert_eq!(closed.grid.cells[closed.grid.index(62, 16)].symbol, ' ');

        ui.meta.dev_mode = true;
        ui.meta.settings_open = true;
        ui.meta.settings_tab = crate::ui::state::SettingsTab::Widgets;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("widgets"));
        assert!(text.contains("dev mode: true"));
        assert!(text.contains("move mode: false"));
        assert!(text.contains("move target: hero"));
        assert!(text.contains("clock: hero-attached world entity"));
        let center = open.grid.cells[open.grid.index(62, 16)].style.bg;
        assert_eq!(center, Some(crate::theme::palette::MODAL_BG));
    }
}
