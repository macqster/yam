use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Cell, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::layers::modal::{paint_modal_shell, ModalFrame};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::{SettingsAxisField, SettingsTab, UiState};

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
        ctx: &RenderState,
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
        draw_tab_body(&mut grid, body_x, body_y, frame.width - 4, ui, ctx);
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

fn draw_tab_body(grid: &mut Grid, x: u16, y: u16, width: u16, ui: &UiState, ctx: &RenderState) {
    let camera_locked =
        crate::scene::viewport_covers_full_world(ctx.hud.camera.width, ctx.hud.camera.height);
    let lines = match ui.meta.settings_tab {
        SettingsTab::Positions => vec![
            if camera_locked {
                "camera: [locked in fullscreen]".to_string()
            } else {
                format_axis_line("camera", ui.offsets.camera_x, ui.offsets.camera_y, ui, 0)
            },
            format_axis_line("hero offset", ui.offsets.hero_dx, ui.offsets.hero_dy, ui, 1),
            format_axis_line(
                "clock offset",
                ui.offsets.clock_dx as i32,
                ui.offsets.clock_dy as i32,
                ui,
                2,
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
        let row_y = y + row as u16;
        let selected = ui.meta.selected_settings_row() == row as u16;
        let row_disabled =
            ui.meta.settings_tab == SettingsTab::Positions && row == 0 && camera_locked;
        let style = if row_disabled && selected {
            theme_style::settings_disabled_row_selected()
        } else if row_disabled {
            theme_style::settings_disabled_row()
        } else if selected {
            theme_style::settings_selected_row()
        } else {
            theme_style::panel_text()
        };
        if selected {
            fill_row(grid, x, row_y, width, style);
        }
        let clipped = if line.chars().count() as u16 > width {
            line.chars().take(width as usize).collect::<String>()
        } else {
            line.clone()
        };
        write_string(grid, x, row_y, &clipped, style);
        if selected && ui.meta.settings_tab == SettingsTab::Positions && !row_disabled {
            maybe_draw_active_axis_field(grid, x, row_y, width, ui, row as u16);
        }
    }
}

fn format_axis_line(label: &str, x: i32, y: i32, ui: &UiState, row: u16) -> String {
    if ui.settings_edit.active && ui.settings_edit.row == row {
        format!(
            "{}: x = {}, y = {}",
            label, ui.settings_edit.x_buffer, ui.settings_edit.y_buffer
        )
    } else {
        format!("{}: x = {}, y = {}", label, x, y)
    }
}

fn maybe_draw_active_axis_field(
    grid: &mut Grid,
    x: u16,
    y: u16,
    width: u16,
    ui: &UiState,
    row: u16,
) {
    if !ui.settings_edit.active || ui.settings_edit.row != row {
        return;
    }

    let label = match row {
        0 => "camera",
        1 => "hero offset",
        2 => "clock offset",
        _ => return,
    };
    let prefix_len = format!("{}: ", label).chars().count() as u16;
    let x_label_len = "x = ".chars().count() as u16;
    let x_value_len = ui.settings_edit.x_buffer.chars().count() as u16;
    let y_label_len = ", y = ".chars().count() as u16;

    let (field_x, field_width, field_text) = match ui.settings_edit.field {
        SettingsAxisField::X => (
            x + prefix_len + x_label_len,
            x_value_len.max(1),
            ui.settings_edit.x_buffer.as_str(),
        ),
        SettingsAxisField::Y => (
            x + prefix_len + x_label_len + x_value_len + y_label_len,
            ui.settings_edit.y_buffer.chars().count() as u16,
            ui.settings_edit.y_buffer.as_str(),
        ),
    };

    if field_x >= x + width {
        return;
    }

    fill_row(
        grid,
        field_x,
        y,
        field_width.max(1).min((x + width).saturating_sub(field_x)),
        theme_style::settings_active_field(),
    );
    write_string(
        grid,
        field_x,
        y,
        field_text,
        theme_style::settings_active_field(),
    );
}

fn fill_row(grid: &mut Grid, x: u16, y: u16, width: u16, style: ratatui::style::Style) {
    for dx in 0..width {
        grid.set(x + dx, y, Cell { symbol: ' ', style });
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

    #[test]
    fn active_settings_row_uses_a_background_highlight() {
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
        ui.meta.dev_mode = true;
        ui.meta.settings_open = true;
        ui.meta.settings_tab = crate::ui::state::SettingsTab::Positions;
        ui.meta.settings_cursor.positions = 1;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let selected_bg = open.grid.cells[open.grid.index(30, 13)].style.bg;
        let unselected_bg = open.grid.cells[open.grid.index(30, 12)].style.bg;

        assert_eq!(selected_bg, Some(crate::theme::palette::CAMERA_THUMB));
        assert_eq!(unselected_bg, Some(crate::theme::palette::MODAL_BG));
    }

    #[test]
    fn positions_tab_uses_human_friendly_axis_labels() {
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
        ui.meta.dev_mode = true;
        ui.meta.settings_open = true;
        ui.meta.settings_tab = crate::ui::state::SettingsTab::Positions;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("camera: x ="));
        assert!(text.contains("hero offset: x ="));
        assert!(text.contains("clock offset: x ="));
    }

    #[test]
    fn camera_row_is_dimmed_when_viewport_already_covers_the_full_world() {
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
                    x: 0,
                    y: 0,
                    width: 212,
                    height: 56,
                },
                viewport_rect: Rect::new(0, 0, 212, 56),
                camera: Camera {
                    x: -106,
                    y: -28,
                    width: 212,
                    height: 56,
                    follow_hero: false,
                },
            },
        };
        let mut ui = UiState::new();
        ui.meta.dev_mode = true;
        ui.meta.settings_open = true;
        ui.meta.settings_tab = crate::ui::state::SettingsTab::Positions;
        ui.meta.settings_cursor.positions = 0;

        let open = layer.render_to_grid(212, 57, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("camera: [locked in fullscreen]"));
        let row_bg = open.grid.cells[open.grid.index(74, 25)].style.bg;
        assert_eq!(row_bg, Some(crate::theme::palette::MODAL_BG));
    }
}
