use crate::build_info;
use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::{FigletFontId, FontRegistry};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct LoadingLayer;

impl Layer for LoadingLayer {
    fn z_index(&self) -> i32 {
        950
    }

    fn visible_during_loading(&self) -> bool {
        true
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        _world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        _ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        if !ui.loading.active {
            return LayerOutput { grid, mask: None };
        }
        if matches!(
            ui.loading.mode,
            crate::ui::state::LoadingMode::Boot(crate::ui::state::BootLoadingPhase::Hold)
        ) {
            return LayerOutput { grid, mask: None };
        }

        let art_lines = loading_art_lines(fonts);
        let version_line = format!("{} ({})", build_info::VERSION, build_info::build_hash());
        let now = Instant::now();
        let label_line = ui.loading.label.clone();
        let bar_line = progress_bar(ui.loading.bar_progress(now), 20);
        let showing_start_prompt = ui.loading.showing_start_prompt();
        let tail_rows = 5;

        let center_x = width / 2;
        let origin_y = height
            .saturating_sub(art_lines.len() as u16 + tail_rows)
            .saturating_div(2)
            .saturating_sub(1);

        for (row, line) in art_lines.iter().enumerate() {
            let row_center_x = if row == 0 {
                center_x.saturating_sub(3)
            } else {
                center_x
            };
            let row_y = if row == 0 {
                origin_y + 1
            } else {
                origin_y + row as u16
            };
            write_centered_line(
                &mut grid,
                row_center_x,
                row_y,
                line,
                theme_style::loading_text(),
            );
        }

        write_centered_line(
            &mut grid,
            center_x + 4,
            origin_y + art_lines.len() as u16,
            &version_line,
            theme_style::loading_text(),
        );

        if showing_start_prompt {
            write_centered_line(
                &mut grid,
                center_x,
                origin_y + art_lines.len() as u16 + 3,
                "press [space] to continue",
                theme_style::loading_prompt(prompt_pulse(now)),
            );
        } else {
            write_centered_line(
                &mut grid,
                center_x + 1,
                origin_y + art_lines.len() as u16 + 3,
                &label_line,
                theme_style::loading_text(),
            );
            write_centered_line(
                &mut grid,
                center_x,
                origin_y + art_lines.len() as u16 + 5,
                &bar_line,
                theme_style::loading_text(),
            );
        }

        LayerOutput { grid, mask: None }
    }
}

fn prompt_pulse(_now: Instant) -> f32 {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs_f32())
        .unwrap_or(0.0);
    let cycle = (seconds / 3.0).fract();
    let ping_pong = if cycle < 0.5 {
        cycle * 2.0
    } else {
        (1.0 - cycle) * 2.0
    };
    ping_pong * ping_pong * (3.0 - 2.0 * ping_pong)
}

fn loading_art_lines(fonts: &FontRegistry) -> Vec<String> {
    let mut lines = vec!["a very special".to_string()];
    lines.extend(fonts.render(FigletFontId::Standard, "yam"));
    lines
}

fn write_centered_line(
    grid: &mut Grid,
    center_x: u16,
    y: u16,
    line: &str,
    style: ratatui::style::Style,
) {
    let line_width = line.chars().count() as u16;
    let start_x = center_x.saturating_sub(line_width / 2);
    write_string(grid, start_x, y, line, style);
}

fn progress_bar(progress: f32, width: usize) -> String {
    let filled = (progress.clamp(0.0, 1.0) * width as f32).round() as usize;
    let mut bar = String::with_capacity(width);
    for index in 0..width {
        if index < filled {
            bar.push('■');
        } else {
            bar.push('⋄');
        }
    }
    bar.pop();
    bar
}

#[cfg(test)]
mod tests {
    use super::LoadingLayer;
    use crate::core::world::WorldState;
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::WorldPos;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::ui::state::UiState;
    use ratatui::prelude::Rect;
    use std::time::Instant;
    fn render_state() -> RenderState {
        RenderState {
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
        }
    }

    #[test]
    fn loading_overlay_requires_active_loading_state() {
        let layer = LoadingLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
        let ui = UiState::new();

        let closed = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        assert_eq!(closed.grid.cells[closed.grid.index(62, 16)].symbol, ' ');
    }

    #[test]
    fn loading_overlay_renders_ascii_art_version_and_loading_bar() {
        let layer = LoadingLayer;
        let world = WorldState::for_boot();
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.start_loading_boot();

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("a very special"));
        assert!(text.contains("_   _  __ _ _ __ ___"));
        assert!(text.contains("| |_| | (_| | | | | | |"));
        assert!(text.contains("loading"));
        assert!(text.contains("■"));
        assert!(text.contains("⋄"));
    }

    #[test]
    fn loading_overlay_replaces_bar_with_space_prompt_while_awaiting_start() {
        let layer = LoadingLayer;
        let world = WorldState::for_boot();
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.start_loading_boot();
        ui.loading.mode =
            crate::ui::state::LoadingMode::Boot(crate::ui::state::BootLoadingPhase::AwaitStart);

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("press [space] to continue"));
        assert!(!text.contains("loading..."));
        assert!(!text.contains("■"));
        assert!(!text.contains("⋄"));
    }

    #[test]
    fn loading_overlay_keeps_space_prompt_visible_during_dissolve() {
        let layer = LoadingLayer;
        let world = WorldState::for_boot();
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.start_loading_boot();
        ui.loading.mode =
            crate::ui::state::LoadingMode::Boot(crate::ui::state::BootLoadingPhase::Dissolve);
        ui.loading.started_at = Some(Instant::now());
        ui.loading.duration = crate::ui::state::LoadingState::BOOT_DISSOLVE;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("press [space] to continue"));
        assert!(!text.contains("loading..."));
    }

    #[test]
    fn loading_overlay_renders_nothing_during_post_dissolve_hold() {
        let layer = LoadingLayer;
        let world = WorldState::for_boot();
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.start_loading_boot();
        ui.loading.mode =
            crate::ui::state::LoadingMode::Boot(crate::ui::state::BootLoadingPhase::Hold);
        ui.loading.started_at = Some(Instant::now());
        ui.loading.duration = crate::ui::state::LoadingState::BOOT_HOLD;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(!text.contains("a very special"));
        assert!(!text.contains("loading"));
        assert!(!text.contains("press [space] to continue"));
        assert!(!text.contains("■"));
        assert!(!text.contains("⋄"));
    }

    #[test]
    fn loading_overlay_keeps_background_transparent() {
        let layer = LoadingLayer;
        let world = WorldState::for_boot();
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.start_loading_boot();

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let corner = &open.grid.cells[open.grid.index(0, 0)];

        assert_eq!(corner.symbol, ' ');
        assert_eq!(corner.style.bg, None);
    }

    #[test]
    fn loading_overlay_glyphs_do_not_paint_panel_background() {
        let layer = LoadingLayer;
        let world = WorldState::for_boot();
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.start_loading_boot();

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let glyph_cell = open
            .grid
            .cells
            .iter()
            .find(|cell| cell.symbol == 'a')
            .expect("loading art should include a visible glyph");

        assert_eq!(glyph_cell.style.bg, None);
    }
}
