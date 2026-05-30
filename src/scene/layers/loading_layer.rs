use crate::build_info;
use crate::core::world::WorldState;
use crate::render::compositor::{write_ascii_string_skip_spaces, write_string, Grid};
use crate::render::fonts::{FigletFontId, FontRegistry};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;
use ratatui::style::Style;
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

        let caption_line = "a very special";
        let logo_lines = loading_logo_lines(fonts);
        let version_line = format!("{} ({})", build_info::VERSION, build_info::build_hash());
        let now = Instant::now();
        let label_segments = loading_label_segments(&ui.loading.label, now);
        let bar_segments = progress_bar_segments(ui.loading.bar_progress(now), 16);
        let showing_start_prompt = ui.loading.showing_start_prompt();
        let tail_rows = 5;

        let center_x = width / 2;
        let lower_text_center_x = center_x + 1;
        let assembly_height = 1u16 + 1u16 + logo_lines.len() as u16 + tail_rows;
        let assembly_top = height
            .saturating_sub(assembly_height)
            .saturating_div(2)
            .saturating_sub(1);
        let caption_y = assembly_top + 1;
        let logo_y = caption_y + 1;
        let version_y = logo_y + logo_lines.len() as u16;
        let prompt_y = version_y + 4;
        let bar_y = version_y + 5;

        write_centered_line(
            &mut grid,
            center_x.saturating_sub(3),
            caption_y,
            caption_line,
            theme_style::loading_meta_text(),
        );

        for (row, line) in logo_lines.iter().enumerate() {
            write_centered_ascii_line(
                &mut grid,
                center_x,
                logo_y + row as u16,
                line,
                theme_style::loading_logo(),
            );
        }

        write_centered_line(
            &mut grid,
            center_x + 4,
            version_y,
            &version_line,
            theme_style::loading_meta_text(),
        );

        if showing_start_prompt {
            write_centered_line(
                &mut grid,
                center_x + 1,
                prompt_y,
                "press [space] to continue",
                theme_style::loading_prompt(prompt_pulse(now)),
            );
        } else {
            write_centered_segments(&mut grid, lower_text_center_x, prompt_y, &label_segments);
            write_centered_segments(&mut grid, center_x, bar_y, &bar_segments);
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

fn loading_logo_lines(fonts: &FontRegistry) -> Vec<String> {
    let mut lines = fonts.render(FigletFontId::Standard, "yam");
    let first_non_blank = lines
        .iter()
        .position(|line| line.chars().any(|ch| ch != ' '))
        .unwrap_or(lines.len());
    lines.drain(..first_non_blank);
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

fn write_centered_ascii_line(
    grid: &mut Grid,
    center_x: u16,
    y: u16,
    line: &str,
    style: ratatui::style::Style,
) {
    let line_width = line.chars().count() as u16;
    let start_x = center_x.saturating_sub(line_width / 2);
    write_ascii_string_skip_spaces(grid, start_x, y, line, style);
}

fn write_centered_segments(grid: &mut Grid, center_x: u16, y: u16, segments: &[(String, Style)]) {
    let line_width: u16 = segments
        .iter()
        .map(|(text, _)| text.chars().count() as u16)
        .sum();
    let mut x = center_x.saturating_sub(line_width / 2);
    for (text, style) in segments {
        write_string(grid, x, y, text, *style);
        x = x.saturating_add(text.chars().count() as u16);
    }
}

fn loading_label_segments(label: &str, now: Instant) -> Vec<(String, Style)> {
    let trimmed = label.trim_end_matches('.');
    let had_ellipsis = trimmed.len() != label.len();
    let dot_count = if had_ellipsis {
        animated_dot_count(now)
    } else {
        0
    };

    let mut segments = vec![(trimmed.to_string(), theme_style::loading_text())];
    let suffix = format!("{}{}", ".".repeat(dot_count), " ".repeat(3 - dot_count));
    segments.push((suffix, theme_style::loading_dot()));
    segments
}

fn animated_dot_count(_now: Instant) -> usize {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    ((millis / 300) % 3 + 1) as usize
}

fn progress_bar_segments(progress: f32, slot_count: usize) -> Vec<(String, Style)> {
    let filled = ((progress.clamp(0.0, 1.0) * slot_count as f32).round() as usize)
        .clamp(0, slot_count.max(1));
    let empty = slot_count.saturating_sub(filled);

    let mut segments = Vec::new();
    if filled > 0 {
        let body = filled.saturating_sub(1);
        if body > 0 {
            segments.push(("━".repeat(body), theme_style::loading_bar_fill()));
        }
        segments.push(("╸".to_string(), theme_style::loading_bar_edge()));
    }
    if empty > 0 {
        segments.push(("┄".repeat(empty), theme_style::loading_bar_empty()));
    }
    if segments.is_empty() {
        segments.push(("┄".repeat(slot_count), theme_style::loading_bar_empty()));
    }
    segments
}

#[cfg(test)]
mod tests {
    use super::LoadingLayer;
    use crate::core::spatial::SpatialPoint as WorldPos;
    use crate::core::world::WorldState;
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::theme::{palette, style as theme_style};
    use crate::ui::state::UiState;
    use ratatui::prelude::Rect;
    use std::time::{Duration, Instant};

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

    fn grid_text(grid: &crate::render::compositor::Grid) -> String {
        grid.cells.iter().map(|cell| cell.symbol).collect()
    }

    fn find_text_position(
        grid: &crate::render::compositor::Grid,
        text: &str,
    ) -> Option<(u16, u16)> {
        let text_len = text.chars().count();
        if text_len == 0 || text_len > grid.width as usize {
            return None;
        }

        for y in 0..grid.height {
            for x in 0..=grid.width.saturating_sub(text_len as u16) {
                let candidate: String = (0..text_len)
                    .map(|offset| grid.cells[grid.index(x + offset as u16, y)].symbol)
                    .collect();
                if candidate == text {
                    return Some((x, y));
                }
            }
        }

        None
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
        ui.loading.mode =
            crate::ui::state::LoadingMode::Boot(crate::ui::state::BootLoadingPhase::Bar);
        ui.loading.started_at = Some(Instant::now() - Duration::from_millis(1500));
        ui.loading.duration = crate::ui::state::LoadingState::BOOT_BAR;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let text = grid_text(&open.grid);

        assert!(text.contains("a very special"));
        assert!(text.contains("_   _  __ _ _ __ ___"));
        assert!(text.contains("| |_| | (_| | | | | | |"));
        assert!(text.contains("loading"));
        assert!(text.contains("╸") || text.contains("┄"));
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
        let text = grid_text(&open.grid);

        assert!(text.contains("press [space] to continue"));
        assert!(!text.contains("loading..."));
        assert!(!text.contains("■"));
        assert!(!text.contains("╸"));
        assert!(!text.contains("┄"));
    }

    #[test]
    fn loading_overlay_gives_space_prompt_one_cell_visual_right_nudge() {
        let layer = LoadingLayer;
        let world = WorldState::for_boot();
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.start_loading_boot();
        ui.loading.mode =
            crate::ui::state::LoadingMode::Boot(crate::ui::state::BootLoadingPhase::AwaitStart);

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let prompt = "press [space] to continue";
        let (prompt_x, _) = find_text_position(&open.grid, prompt).expect("prompt should render");
        let prompt_center = prompt_x as i32 + prompt.chars().count() as i32 / 2;
        let visual_center = open.grid.width as i32 / 2 + 1;

        assert_eq!(prompt_center, visual_center);
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
        let text = grid_text(&open.grid);

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
        let text = grid_text(&open.grid);

        assert!(!text.contains("a very special"));
        assert!(!text.contains("loading"));
        assert!(!text.contains("press [space] to continue"));
        assert!(!text.contains("■"));
        assert!(!text.contains("╸"));
        assert!(!text.contains("┄"));
    }

    #[test]
    fn loading_overlay_uses_requested_palette_split() {
        let layer = LoadingLayer;
        let world = WorldState::for_boot();
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.start_loading_boot();
        ui.loading.mode =
            crate::ui::state::LoadingMode::Boot(crate::ui::state::BootLoadingPhase::AwaitStart);

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());
        let logo_lines = super::loading_logo_lines(&fonts);
        let center_x = open.grid.width / 2;
        let assembly_height = 1u16 + 1u16 + logo_lines.len() as u16 + 5;
        let assembly_top = open
            .grid
            .height
            .saturating_sub(assembly_height)
            .saturating_div(2)
            .saturating_sub(1);
        let version_y = assembly_top + 1 + 1 + logo_lines.len() as u16;
        let prompt_y = version_y + 4;

        let (tag_x, tag_y) =
            find_text_position(&open.grid, "a very special").expect("subtitle should render");
        let tag_probe_offset = "a very special".chars().count() as u16 - 1;
        let tag_cell = &open.grid.cells[open.grid.index(tag_x + tag_probe_offset, tag_y)];
        assert_eq!(tag_cell.style.fg, Some(palette::NEUTRAL_SLATE));

        let (logo_x, logo_y) =
            find_text_position(&open.grid, "_   _  __ _ _ __ ___").expect("logo should render");
        let logo_cell = &open.grid.cells[open.grid.index(logo_x, logo_y)];
        assert_eq!(logo_cell.style.fg, Some(palette::GREEN_DEEP));

        let version = format!(
            "{} ({})",
            crate::build_info::VERSION,
            crate::build_info::build_hash()
        );
        let (version_x, version_y) =
            find_text_position(&open.grid, &version).expect("version should render");
        let version_probe_offset = version
            .find('(')
            .expect("version line should contain build hash delimiter")
            as u16;
        let version_cell =
            &open.grid.cells[open.grid.index(version_x + version_probe_offset, version_y)];
        assert_eq!(version_cell.style.fg, Some(palette::NEUTRAL_SLATE));

        let prompt = "press [space] to continue";
        let prompt_x = center_x
            .saturating_add(1)
            .saturating_sub(prompt.chars().count() as u16 / 2);
        let prompt_cell = &open.grid.cells[open.grid.index(prompt_x, prompt_y)];
        assert_eq!(prompt_cell.style.fg, Some(palette::BLUE_DECO));
    }

    #[test]
    fn loading_overlay_uses_omp_inspired_label_and_bar_colors() {
        let layer = LoadingLayer;
        let world = WorldState::for_boot();
        let fonts = FontRegistry::new();
        let mut ui = UiState::new();
        ui.start_loading_boot();

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state());

        let (loading_x, loading_y) =
            find_text_position(&open.grid, "loading").expect("loading label should render");
        let loading_cell = &open.grid.cells[open.grid.index(loading_x, loading_y)];
        assert_eq!(loading_cell.style.fg, Some(palette::LOADING_TEXT));

        let dot_cell = &open.grid.cells[open
            .grid
            .index(loading_x + "loading".len() as u16, loading_y)];
        assert_eq!(dot_cell.style.fg, Some(palette::GREEN_MID));

        let bar_segments =
            super::progress_bar_segments(ui.loading.bar_progress(Instant::now()), 16);
        let bar_text: String = bar_segments.iter().map(|(text, _)| text.as_str()).collect();
        assert!(
            find_text_position(&open.grid, &bar_text).is_some(),
            "progress bar should render"
        );

        let role_segments = super::progress_bar_segments(0.5, 16);
        assert!(role_segments
            .iter()
            .any(|(text, style)| text.contains('╸') && *style == theme_style::loading_bar_edge()));
        assert!(role_segments
            .iter()
            .any(|(text, style)| text.contains('┄') && *style == theme_style::loading_bar_empty()));
    }

    #[test]
    fn loading_label_segments_keep_a_fixed_width_for_dot_animation() {
        let one = super::loading_label_segments("loading...", Instant::now());
        let two = super::loading_label_segments("loading...", Instant::now());
        let width_one: usize = one.iter().map(|(text, _)| text.chars().count()).sum();
        let width_two: usize = two.iter().map(|(text, _)| text.chars().count()).sum();

        assert_eq!(width_one, "loading...".chars().count());
        assert_eq!(width_one, width_two);
    }

    #[test]
    fn loading_bar_segments_use_thin_transparent_progress_grammar() {
        let segments = super::progress_bar_segments(0.5, 16);
        let text: String = segments.iter().map(|(text, _)| text.as_str()).collect();

        assert!(text.contains('╸'));
        assert!(text.contains('┄'));
        assert!(text.contains('━'));
        assert!(segments
            .iter()
            .any(|(text, style)| text.contains('╸') && *style == theme_style::loading_bar_edge()));
    }

    #[test]
    fn loading_logo_lines_trim_leading_blank_rows() {
        let fonts = FontRegistry::new();
        let logo_lines = super::loading_logo_lines(&fonts);

        assert!(!logo_lines.is_empty());
        assert!(logo_lines[0].chars().any(|ch| ch != ' '));
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
