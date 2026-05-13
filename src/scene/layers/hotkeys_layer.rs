use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::layers::modal::{paint_modal_shell, ModalFooter, ModalFrame};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;
use ratatui::style::Style;

pub struct HotkeysLayer;

impl Layer for HotkeysLayer {
    fn z_index(&self) -> i32 {
        390
    }

    fn should_render(&self, ui: &UiState) -> bool {
        ui.show_help_surface()
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

        let frame = ModalFrame::centered(width, height, 76, 17);
        paint_modal_shell(
            &mut grid,
            frame,
            "[?] help",
            Some(ModalFooter {
                left: "",
                right: "? ⎋",
            }),
        );

        let (body_x, body_y) = frame.body_origin();
        let (left_lines, right_lines) = if ui.meta.dev_mode {
            (
                vec![
                    HelpLine::section("core"),
                    HelpLine::item("[q] quit app"),
                    HelpLine::item("[d] toggle dev mode"),
                    HelpLine::item("[?] open help"),
                    HelpLine::blank(),
                    HelpLine::section("inspect"),
                    HelpLine::item("[s] settings popup"),
                    HelpLine::item("[P] palette popup"),
                    HelpLine::item("[W] weather popup"),
                    HelpLine::item("[p] pointer probe"),
                    HelpLine::item("[v] vines"),
                    HelpLine::item("[F5] next font"),
                ],
                vec![
                    HelpLine::section("move"),
                    HelpLine::item("[m] move popup"),
                    HelpLine::item("[Tab/Shift+Tab] cycle target"),
                    HelpLine::item("[arrow keys] move target"),
                    HelpLine::item("[C] store camera home"),
                    HelpLine::item("[c] recall camera home"),
                    HelpLine::blank(),
                    HelpLine::section("animation"),
                    HelpLine::item("[space] play/pause"),
                    HelpLine::item("[.] step animation"),
                ],
            )
        } else {
            (
                vec![
                    HelpLine::section("available now"),
                    HelpLine::item("[q] quit app"),
                    HelpLine::item("[d] enter dev mode"),
                    HelpLine::item("[?] open help"),
                    HelpLine::blank(),
                    HelpLine::section("in dev mode"),
                    HelpLine::item("[s] settings popup"),
                    HelpLine::item("[m] move popup"),
                    HelpLine::item("[P] palette popup"),
                    HelpLine::item("[W] weather popup"),
                    HelpLine::item("[p] pointer probe"),
                ],
                vec![
                    HelpLine::section("dev mode tools"),
                    HelpLine::item("[Tab/Shift+Tab] cycle move target"),
                    HelpLine::item("[arrow keys] move target"),
                    HelpLine::item("[C] store camera home"),
                    HelpLine::item("[c] recall camera home"),
                    HelpLine::item("[v] vines"),
                    HelpLine::item("[F5] next font"),
                    HelpLine::blank(),
                    HelpLine::section("animation"),
                    HelpLine::item("[space] play/pause"),
                    HelpLine::item("[.] step animation"),
                ],
            )
        };
        let body_width = frame.width.saturating_sub(4);
        let gap = 4u16;
        let column_width = body_width.saturating_sub(gap) / 2;
        let right_x = body_x + column_width + gap;

        for (row, line) in left_lines.iter().enumerate() {
            write_clipped_help_line(&mut grid, body_x, body_y + row as u16, column_width, line);
        }
        for (row, line) in right_lines.iter().enumerate() {
            write_clipped_help_line(&mut grid, right_x, body_y + row as u16, column_width, line);
        }

        LayerOutput { grid, mask: None }
    }
}

#[derive(Clone, Copy)]
struct HelpLine {
    text: &'static str,
    style: Style,
}

impl HelpLine {
    fn section(text: &'static str) -> Self {
        Self {
            text,
            style: theme_style::settings_tab_inactive(),
        }
    }

    fn item(text: &'static str) -> Self {
        Self {
            text,
            style: theme_style::panel_text(),
        }
    }

    fn blank() -> Self {
        Self::item("")
    }
}

fn write_clipped_help_line(grid: &mut Grid, x: u16, y: u16, width: u16, line: &HelpLine) {
    let clipped = if line.text.chars().count() as u16 > width {
        line.text.chars().take(width as usize).collect::<String>()
    } else {
        line.text.to_string()
    };
    write_string(grid, x, y, &clipped, line.style);
}

#[cfg(test)]
mod tests {
    use super::HotkeysLayer;
    use crate::core::world::WorldState;
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::WorldPos;
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
    fn hotkeys_overlay_requires_dev_mode_and_open_state() {
        let layer = HotkeysLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
        let render_state = render_state();
        let mut ui = UiState::new();

        let closed = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        assert_eq!(closed.grid.cells[closed.grid.index(62, 16)].symbol, ' ');

        ui.meta.dev_mode = true;
        ui.meta.hotkeys_open = true;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("[?] help"));
        assert!(text.contains("core"));
        assert!(text.contains("inspect"));
        assert!(text.contains("move"));
        assert!(text.contains("animation"));
        assert!(text.contains("[P] palette popup"));
        assert!(text.contains("[W] weather popup"));
        assert!(text.contains("[C] store camera home"));
        assert!(text.contains("[c] recall camera home"));
        assert!(text.contains("[p] pointer probe"));
        assert!(text.contains("[v] vines"));
        assert!(text.contains("[s] settings popup"));
        assert!(text.contains("[m] move popup"));
        assert!(text.contains("[Tab/Shift+Tab] cycle target"));
        assert!(text.contains("[arrow keys] move target"));
        assert!(!text.contains("[1/2/3/4/5] select target"));
        assert!(text.contains("[space] play/pause"));
        assert!(text.contains("? ⎋"));
    }

    #[test]
    fn hotkeys_overlay_is_available_from_main_scene_when_dev_mode_is_off() {
        let layer = HotkeysLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
        let render_state = render_state();
        let mut ui = UiState::new();
        ui.meta.hotkeys_open = true;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("[?] help"));
        assert!(text.contains("available now"));
        assert!(text.contains("[d] enter dev mode"));
        assert!(text.contains("in dev mode"));
        assert!(text.contains("[m] move popup"));
        assert!(text.contains("[Tab/Shift+Tab] cycle move target"));
        assert!(!text.contains("inspect"));
    }
}
