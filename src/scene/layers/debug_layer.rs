use crate::core::guide_line::{classify_line, LineFamily, LinePoint};
#[cfg(test)]
use crate::core::guide_line::{rasterize_line, soft_line_glyph};
use crate::core::spatial::SpatialGuideIndex;
use crate::core::world::WorldState;
use crate::core::{flora::BORDER_VINE_GUIDE_SET_LABEL, guide::GuideState};
use crate::render::compositor::Cell;
use crate::render::compositor::{write_ascii_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::render::guide::draw_guides;
use crate::scene::coords::{project_world_to_screen, ScreenPos, WorldPos};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::{DebugPanelTab, UiState};
use ratatui::prelude::*;
use ratatui::{
    buffer::Buffer,
    widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget},
};

pub struct DebugLayer;

impl Layer for DebugLayer {
    fn z_index(&self) -> i32 {
        300
    }

    fn render_into_grid(
        &self,
        grid: &mut Grid,
        world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> Option<crate::render::mask::Mask> {
        if !ui.show_dev_surfaces() {
            return None;
        }

        if ui.meta.sliders_visible {
            draw_camera_scrollbars(grid, grid.width, grid.height, ctx);
        }
        draw_visible_guides(grid, world, ui, ctx);

        let panel_x = 3u16;
        let tabs_y = 1u16;
        let panel_y = 2u16;
        let cam_x = ctx.hud.camera.x;
        let cam_y = ctx.hud.camera.y;
        let screen_w = grid.width as i32;
        let screen_h = grid.height as i32;
        // Datum-centered world-border probe:
        // the frame is defined in world space and projected through the current camera.
        let border = border_probe_bounds();

        let hero = &ui.hero;
        let main_scene = world.kind.has_main_scene_composition();
        let hero_world = ctx.world.hero_world;
        let hero_screen = project_world_to_screen(
            ctx.world.hero_visual_anchor,
            ctx.hud.camera.x,
            ctx.hud.camera.y,
            ctx.hud.camera.height,
        );
        let hero_visible = main_scene
            && hero_screen.x >= 0
            && hero_screen.y >= 0
            && hero_screen.x < grid.width as i32
            && hero_screen.y < grid.height as i32;
        let pointer_world = WorldPos {
            x: ui.offsets.pointer_x,
            y: ui.offsets.pointer_y,
        };
        let pointer_screen = project_world_to_screen(
            pointer_world,
            ctx.hud.camera.x,
            ctx.hud.camera.y,
            ctx.hud.camera.height,
        );
        let pointer_visible = ui.meta.pointer_probe_open
            && ui.pointer_blink_on
            && pointer_screen.x >= 0
            && pointer_screen.y >= 0
            && pointer_screen.x < grid.width as i32
            && pointer_screen.y < grid.height as i32;
        let clock_world = ctx.world.clock_world;
        let clock_screen = ctx.clock_screen();
        let clock_visible = main_scene
            && clock_screen.x >= 0
            && clock_screen.y >= 0
            && clock_screen.x < grid.width as i32
            && clock_screen.y < grid.height as i32;
        let weather_world = ctx.world.weather_world;
        let weather_screen = ctx.weather_screen();
        let weather_visible = main_scene
            && weather_screen.x >= 0
            && weather_screen.y >= 0
            && weather_screen.x < grid.width as i32
            && weather_screen.y < grid.height as i32;
        let date_world = ctx.world.date_world;
        let date_screen = ctx.date_screen();
        let date_visible = main_scene
            && date_screen.x >= 0
            && date_screen.y >= 0
            && date_screen.x < grid.width as i32
            && date_screen.y < grid.height as i32;
        let vine_count = world.flora.vines.len();
        let first_vine = world.flora.vines.first();
        let vine_line = if let Some(vine) = first_vine {
            let identity = vine.identity();
            format!(
                "Vines: {} (id {}, {})",
                vine_count, identity.id, identity.species_id
            )
        } else {
            "Vines: 0".to_string()
        };
        let vine_axis_line = if let Some(vine) = first_vine {
            let segment_count: usize = vine.axes.iter().map(|axis| axis.segments.len()).sum();
            format!(
                "Vine axes: {} / segments: {} / organs: {}",
                vine.axes.len(),
                segment_count,
                vine.organs.len()
            )
        } else {
            "Vine axes: 0 / segments: 0 / organs: 0".to_string()
        };
        let vine_tip_line = if let Some(vine) = first_vine {
            let active_tips = vine
                .growth_tips
                .iter()
                .filter(|tip| matches!(tip.state, crate::core::flora::VineGrowthTipState::Active))
                .count();
            let dormant_tips = vine
                .growth_tips
                .iter()
                .filter(|tip| matches!(tip.state, crate::core::flora::VineGrowthTipState::Dormant))
                .count();
            format!(
                "Vine tips: {} active / {} dormant",
                active_tips, dormant_tips
            )
        } else {
            "Vine tips: 0 active / 0 dormant".to_string()
        };
        let vine_guide_line = if let Some(vine) = first_vine {
            let label = vine
                .axes
                .first()
                .and_then(|axis| axis.guide_set_label.as_deref())
                .unwrap_or("none");
            format!("Vine guide set: {}", label)
        } else {
            "Vine guide set: none".to_string()
        };
        let camera_mode = if ui.camera.follow_hero {
            "Camera mode: follow-hero"
        } else {
            "Camera mode: manual pan"
        };
        let move_mode = format!(
            "Move mode: {} ({})",
            if ui.meta.move_mode_open { "on" } else { "off" },
            ui.meta.move_target.title()
        );
        let center_x = grid.width as i32 / 2;
        let center_y = grid.height as i32 / 2;
        let cam_dx = cam_x - center_x;
        let cam_dy = cam_y - center_y;
        let soft_band_line = {
            let probe_start = WorldPos { x: -28, y: 22 };
            let probe_end = WorldPos { x: 36, y: 12 };
            let start = project_world_to_screen(probe_start, cam_x, cam_y, ctx.hud.camera.height);
            let end = project_world_to_screen(probe_end, cam_x, cam_y, ctx.hud.camera.height);
            let key = classify_line(
                LinePoint {
                    x: start.x,
                    y: start.y,
                },
                LinePoint { x: end.x, y: end.y },
                64,
                32,
            );
            let family = match key.family {
                LineFamily::Axis => "axis",
                LineFamily::VeryShallow => "very-shallow",
                LineFamily::Shallow => "shallow",
                LineFamily::Medium => "medium",
                LineFamily::Steep => "steep",
            };
            format!("Soft band: {} / {:?}", family, key.band)
        };
        if ui.meta.debug_info_panel_visible {
            draw_debug_tabs(grid, panel_x, tabs_y, ui.meta.debug_panel_tab);
            let lines = debug_panel_lines(
                ui.meta.debug_panel_tab,
                main_scene,
                DebugPanelFacts {
                    fps: ui.fps,
                    hero_fps: ui.offsets.hero_fps,
                    current_frame: hero.current_frame,
                    frame_count: hero.frames.len(),
                    playing: hero.playing,
                    world_title: world.kind.title(),
                    camera_mode,
                    move_mode: &move_mode,
                    pointer_probe_open: ui.meta.pointer_probe_open,
                    pointer_world,
                    pointer_visible,
                    camera_x: cam_x,
                    camera_y: cam_y,
                    camera_dx: cam_dx,
                    camera_dy: cam_dy,
                    hero_world,
                    hero_screen,
                    hero_visible,
                    hero_dx: ui.offsets.hero_dx,
                    hero_dy: ui.offsets.hero_dy,
                    soft_band_line: &soft_band_line,
                    clock_world,
                    clock_screen,
                    clock_visible,
                    weather_world,
                    weather_screen,
                    weather_visible,
                    date_world,
                    date_screen,
                    date_visible,
                    vine_line: &vine_line,
                    vine_axis_line: &vine_axis_line,
                    vine_tip_line: &vine_tip_line,
                    vine_guide_line: &vine_guide_line,
                    guide_count: world.guides.guides.len(),
                    vine_count,
                },
            );
            for (row, line) in lines.iter().enumerate() {
                write_ascii_string(
                    grid,
                    panel_x,
                    panel_y + row as u16,
                    line,
                    theme_style::debug_text(),
                );
            }
        }
        if ui.meta.world_frame_visible {
            draw_world_frame(
                grid,
                border,
                cam_x,
                cam_y,
                ctx.hud.camera.height,
                screen_w,
                screen_h,
            );
        }
        if ui.meta.world_axis_visible {
            draw_world_axis(
                grid,
                border,
                cam_x,
                cam_y,
                ctx.hud.camera.height,
                screen_w,
                screen_h,
            );
        }
        if ui.meta.world_datum_visible {
            draw_world_datum(
                grid,
                border,
                cam_x,
                cam_y,
                ctx.hud.camera.height,
                screen_w,
                screen_h,
            );
        }
        draw_pointer_probe(grid, pointer_screen, pointer_visible);
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

fn draw_visible_guides(grid: &mut Grid, world: &WorldState, ui: &UiState, ctx: &RenderState) {
    if ui.meta.vines_visible {
        draw_guides(
            grid,
            SpatialGuideIndex::new(&world.guides),
            ctx.hud.camera.x,
            ctx.hud.camera.y,
            ctx.hud.camera.height,
        );
        return;
    }

    let mut visible_guides = world.guides.clone();
    suppress_vine_guides(&mut visible_guides);
    draw_guides(
        grid,
        SpatialGuideIndex::new(&visible_guides),
        ctx.hud.camera.x,
        ctx.hud.camera.y,
        ctx.hud.camera.height,
    );
}

fn suppress_vine_guides(guides: &mut GuideState) {
    let vine_guide_ids = guides
        .guide_set(BORDER_VINE_GUIDE_SET_LABEL)
        .map(|set| set.guides.clone())
        .unwrap_or_default();
    for guide in &mut guides.guides {
        if vine_guide_ids.contains(&guide.id) {
            guide.enabled = false;
        }
    }
}

#[derive(Clone, Copy)]
struct DebugPanelFacts<'a> {
    fps: f64,
    hero_fps: f32,
    current_frame: usize,
    frame_count: usize,
    playing: bool,
    world_title: &'a str,
    camera_mode: &'a str,
    move_mode: &'a str,
    pointer_probe_open: bool,
    pointer_world: WorldPos,
    pointer_visible: bool,
    camera_x: i32,
    camera_y: i32,
    camera_dx: i32,
    camera_dy: i32,
    hero_world: WorldPos,
    hero_screen: ScreenPos,
    hero_visible: bool,
    hero_dx: i32,
    hero_dy: i32,
    soft_band_line: &'a str,
    clock_world: WorldPos,
    clock_screen: ScreenPos,
    clock_visible: bool,
    weather_world: WorldPos,
    weather_screen: ScreenPos,
    weather_visible: bool,
    date_world: WorldPos,
    date_screen: ScreenPos,
    date_visible: bool,
    vine_line: &'a str,
    vine_axis_line: &'a str,
    vine_tip_line: &'a str,
    vine_guide_line: &'a str,
    guide_count: usize,
    vine_count: usize,
}

fn draw_debug_tabs(grid: &mut Grid, x: u16, y: u16, active: DebugPanelTab) {
    let tabs = [
        DebugPanelTab::Runtime,
        DebugPanelTab::Hero,
        DebugPanelTab::Companions,
        DebugPanelTab::Vines,
    ];
    let mut cursor = x;
    for tab in tabs {
        let label = if tab == active {
            format!("[{}]", tab.title())
        } else {
            format!(" {} ", tab.title())
        };
        let style = if tab == active {
            theme_style::settings_tab_active()
        } else {
            theme_style::settings_tab_inactive()
        };
        write_ascii_string(grid, cursor, y, &label, style);
        cursor = cursor.saturating_add(label.chars().count() as u16 + 1);
    }
}

fn debug_panel_lines(
    tab: DebugPanelTab,
    main_scene: bool,
    facts: DebugPanelFacts<'_>,
) -> Vec<String> {
    match tab {
        DebugPanelTab::Runtime => {
            let mut lines = vec![
                format!("FPS: {:.1}", facts.fps),
                format!("World: {}", facts.world_title),
                facts.camera_mode.to_string(),
                facts.move_mode.to_string(),
                format!("Camera: ({}, {})", facts.camera_x, facts.camera_y),
                format!("Camera Δ: ({}, {})", facts.camera_dx, facts.camera_dy),
            ];
            if facts.pointer_probe_open {
                lines.push(format!(
                    "Pointer: on ({}, {})",
                    facts.pointer_world.x, facts.pointer_world.y
                ));
            } else {
                lines.push("Pointer: off".to_string());
            }
            if !main_scene {
                lines.push(format!("Pointer visible: {}", facts.pointer_visible));
            }
            lines
        }
        DebugPanelTab::Hero => {
            if main_scene {
                vec![
                    format!("Hero FPS: {:.1}", facts.hero_fps),
                    format!("Frame: {} / {}", facts.current_frame, facts.frame_count),
                    format!("Playing: {}", facts.playing),
                    format!(
                        "Hero world: ({}, {})",
                        facts.hero_world.x, facts.hero_world.y
                    ),
                    format!(
                        "Hero screen: ({}, {})",
                        facts.hero_screen.x, facts.hero_screen.y
                    ),
                    format!("Hero anchor visible: {}", facts.hero_visible),
                    format!("Hero offset: ({}, {})", facts.hero_dx, facts.hero_dy),
                ]
            } else {
                vec![
                    "Hero: main-scene only".to_string(),
                    format!("World: {}", facts.world_title),
                    format!("Pointer visible: {}", facts.pointer_visible),
                ]
            }
        }
        DebugPanelTab::Companions => {
            if main_scene {
                vec![
                    format!(
                        "Clock world: ({}, {})",
                        facts.clock_world.x, facts.clock_world.y
                    ),
                    format!(
                        "Clock screen: ({}, {})",
                        facts.clock_screen.x, facts.clock_screen.y
                    ),
                    format!("Clock visible: {}", facts.clock_visible),
                    format!(
                        "Weather world: ({}, {})",
                        facts.weather_world.x, facts.weather_world.y
                    ),
                    format!(
                        "Weather screen: ({}, {})",
                        facts.weather_screen.x, facts.weather_screen.y
                    ),
                    format!("Weather visible: {}", facts.weather_visible),
                    format!(
                        "Date world: ({}, {})",
                        facts.date_world.x, facts.date_world.y
                    ),
                    format!(
                        "Date screen: ({}, {})",
                        facts.date_screen.x, facts.date_screen.y
                    ),
                    format!("Date visible: {}", facts.date_visible),
                ]
            } else {
                vec![
                    "Companions: main-scene only".to_string(),
                    format!("World: {}", facts.world_title),
                    format!("Pointer visible: {}", facts.pointer_visible),
                ]
            }
        }
        DebugPanelTab::Vines => {
            let mut lines = vec![facts.soft_band_line.to_string()];
            if main_scene {
                lines.extend([
                    format!("Guides: {}", facts.guide_count),
                    facts.vine_line.to_string(),
                    facts.vine_axis_line.to_string(),
                    facts.vine_tip_line.to_string(),
                    facts.vine_guide_line.to_string(),
                ]);
            } else {
                lines.extend([
                    format!("Guides: {}", facts.guide_count),
                    format!("Vines: {}", facts.vine_count),
                ]);
            }
            lines
        }
    }
}

fn draw_world_frame(
    grid: &mut Grid,
    border: BorderProbeBounds,
    cam_x: i32,
    cam_y: i32,
    viewport_height: u16,
    screen_w: i32,
    screen_h: i32,
) {
    for wx in border.left..=border.right {
        let ch = if wx == border.left || wx == border.right {
            '+'
        } else {
            '-'
        };
        draw_border_cell(
            grid,
            wx,
            border.top,
            ch,
            cam_x,
            cam_y,
            viewport_height,
            screen_w,
            screen_h,
        );
        draw_border_cell(
            grid,
            wx,
            border.bottom,
            ch,
            cam_x,
            cam_y,
            viewport_height,
            screen_w,
            screen_h,
        );
    }

    for wy in border.bottom + 1..border.top {
        draw_border_cell(
            grid,
            border.left,
            wy,
            '|',
            cam_x,
            cam_y,
            viewport_height,
            screen_w,
            screen_h,
        );
        draw_border_cell(
            grid,
            border.right,
            wy,
            '|',
            cam_x,
            cam_y,
            viewport_height,
            screen_w,
            screen_h,
        );
    }
}

fn draw_world_axis(
    grid: &mut Grid,
    border: BorderProbeBounds,
    cam_x: i32,
    cam_y: i32,
    viewport_height: u16,
    screen_w: i32,
    screen_h: i32,
) {
    for wx in border.left..=border.right {
        let ch = if wx == border.mid_x { '+' } else { '-' };
        draw_border_cell(
            grid,
            wx,
            border.mid_y,
            ch,
            cam_x,
            cam_y,
            viewport_height,
            screen_w,
            screen_h,
        );
    }

    for wy in border.bottom + 1..border.top {
        let ch = if wy == border.mid_y { '+' } else { '|' };
        draw_border_cell(
            grid,
            border.mid_x,
            wy,
            ch,
            cam_x,
            cam_y,
            viewport_height,
            screen_w,
            screen_h,
        );
    }
}

fn draw_world_datum(
    grid: &mut Grid,
    border: BorderProbeBounds,
    cam_x: i32,
    cam_y: i32,
    viewport_height: u16,
    screen_w: i32,
    screen_h: i32,
) {
    draw_border_cell(
        grid,
        border.mid_x,
        border.mid_y,
        '+',
        cam_x,
        cam_y,
        viewport_height,
        screen_w,
        screen_h,
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_border_cell(
    grid: &mut Grid,
    wx: i32,
    wy: i32,
    ch: char,
    cam_x: i32,
    cam_y: i32,
    viewport_height: u16,
    screen_w: i32,
    screen_h: i32,
) {
    let screen = project_world_to_screen(WorldPos { x: wx, y: wy }, cam_x, cam_y, viewport_height);
    if screen.x < 0 || screen.y < 0 || screen.x >= screen_w || screen.y >= screen_h {
        return;
    }
    if let Some(cell) = grid.cell_mut(screen.x as u16, screen.y as u16) {
        cell.symbol = ch;
        cell.style = theme_style::guide_trace();
    }
}

fn draw_pointer_probe(grid: &mut Grid, pointer_screen: ScreenPos, visible: bool) {
    if !visible {
        return;
    }
    if let Some(cell) = grid.cell_mut(pointer_screen.x as u16, pointer_screen.y as u16) {
        cell.symbol = '+';
        cell.style = theme_style::pointer_probe();
    }
}

#[cfg(test)]
fn draw_soft_probe_line(
    grid: &mut Grid,
    cam_x: i32,
    cam_y: i32,
    viewport_height: u16,
    start: WorldPos,
    end: WorldPos,
) {
    let start = project_world_to_screen(start, cam_x, cam_y, viewport_height);
    let end = project_world_to_screen(end, cam_x, cam_y, viewport_height);
    let start = LinePoint {
        x: start.x,
        y: start.y,
    };
    let end = LinePoint { x: end.x, y: end.y };

    for step in rasterize_line(start, end) {
        if step.point.x < 0 || step.point.y < 0 {
            continue;
        }
        let glyph = soft_line_glyph(start, end, step.step, step.steps);
        if let Some(cell) = grid.cell_mut(step.point.x as u16, step.point.y as u16) {
            cell.symbol = glyph;
            cell.style = theme_style::guide_trace();
        }
    }
}

fn draw_camera_scrollbars(grid: &mut Grid, width: u16, height: u16, ctx: &RenderState) {
    let inset = 0;
    if width == 0 || height == 0 {
        return;
    }

    let viewport = ctx.hud.viewport;
    if viewport.width as i32 >= crate::scene::WORLD_WIDTH
        && viewport.height as i32 >= crate::scene::WORLD_HEIGHT
    {
        return;
    }
    let horizontal_min = -crate::scene::WORLD_HALF_W - crate::scene::CAMERA_OVERSCAN_CELLS;
    let vertical_min = -crate::scene::WORLD_HALF_H - crate::scene::CAMERA_OVERSCAN_CELLS;
    let horizontal_max = crate::scene::WORLD_HALF_W - 1 + crate::scene::CAMERA_OVERSCAN_CELLS
        - viewport.width as i32
        + 1;
    let vertical_max = crate::scene::WORLD_HALF_H - 1 + crate::scene::CAMERA_OVERSCAN_CELLS
        - viewport.height as i32
        + 1;
    let horizontal_content_length =
        (crate::scene::WORLD_WIDTH + crate::scene::CAMERA_OVERSCAN_CELLS * 2) as usize;
    let vertical_content_length =
        (crate::scene::WORLD_HEIGHT + crate::scene::CAMERA_OVERSCAN_CELLS * 2) as usize;
    let horizontal_area = Rect::new(inset, inset, width, 1);
    let vertical_area = Rect::new(inset, inset, 1, height);

    let horizontal_position = scrollbar_position(
        ctx.hud.camera.x,
        horizontal_min,
        horizontal_max,
        horizontal_content_length,
    );
    let vertical_position = scrollbar_position(
        ctx.hud.camera.y,
        vertical_min,
        vertical_max,
        vertical_content_length,
    );

    let mut horizontal_state = ScrollbarState::new(horizontal_content_length)
        .viewport_content_length(viewport.width as usize)
        .position(horizontal_position);
    let mut vertical_state = ScrollbarState::new(vertical_content_length)
        .viewport_content_length(viewport.height as usize)
        .position(vertical_position);

    let scrollbar_style = Scrollbar::new(ScrollbarOrientation::HorizontalTop)
        .begin_symbol(None)
        .end_symbol(None)
        .track_symbol(Some("┄"))
        .thumb_symbol("═")
        .track_style(theme_style::camera_indicator_track())
        .thumb_style(theme_style::camera_indicator_thumb());
    render_scrollbar(
        grid,
        horizontal_area,
        scrollbar_style,
        &mut horizontal_state,
    );

    let scrollbar_style = Scrollbar::new(ScrollbarOrientation::VerticalLeft)
        .begin_symbol(None)
        .end_symbol(None)
        .track_symbol(Some("┊"))
        .thumb_symbol("║")
        .track_style(theme_style::camera_indicator_track())
        .thumb_style(theme_style::camera_indicator_thumb());
    render_scrollbar(grid, vertical_area, scrollbar_style, &mut vertical_state);
}

fn scrollbar_position(
    camera_origin: i32,
    camera_min: i32,
    camera_max: i32,
    content_length: usize,
) -> usize {
    let content_max = content_length.saturating_sub(1);
    let camera_range = (camera_max - camera_min).max(0) as usize;
    if camera_range == 0 {
        return 0;
    }

    let camera_offset = camera_origin
        .saturating_sub(camera_min)
        .clamp(0, camera_range as i32) as usize;
    camera_offset.saturating_mul(content_max) / camera_range
}

fn render_scrollbar(
    grid: &mut Grid,
    area: Rect,
    scrollbar: Scrollbar<'_>,
    state: &mut ScrollbarState,
) {
    let mut buffer = Buffer::empty(area);
    scrollbar.render(area, &mut buffer, state);
    copy_buffer_to_grid(grid, &buffer);
}

fn copy_buffer_to_grid(grid: &mut Grid, buffer: &Buffer) {
    for y in buffer.area.top()..buffer.area.bottom() {
        for x in buffer.area.left()..buffer.area.right() {
            if let Some(cell) = buffer.cell((x, y)) {
                if let Some(dst) = grid.cell_mut(x, y) {
                    *dst = Cell {
                        symbol: cell.symbol().chars().next().unwrap_or(' '),
                        style: cell.style(),
                    };
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BorderProbeBounds {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    mid_x: i32,
    mid_y: i32,
}

fn border_probe_bounds() -> BorderProbeBounds {
    BorderProbeBounds {
        left: -crate::scene::WORLD_HALF_W,
        right: crate::scene::WORLD_HALF_W - 1,
        top: crate::scene::WORLD_HALF_H - 1,
        bottom: -crate::scene::WORLD_HALF_H,
        mid_x: 0,
        mid_y: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        border_probe_bounds, draw_camera_scrollbars, draw_soft_probe_line, draw_world_axis,
        draw_world_datum, draw_world_frame, scrollbar_position,
    };
    use crate::core::guide_line::{soft_line_glyph, LinePoint};
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::{project_world_to_screen, WorldPos};
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::scene::{WORLD_HALF_H, WORLD_HALF_W};
    use crate::theme::palette;
    use crate::ui::state::DebugPanelTab;
    use ratatui::prelude::Rect;

    fn world_frame() -> WorldFrame {
        WorldFrame {
            hero_world: WorldPos { x: 50, y: 30 },
            hero_visual_anchor: WorldPos { x: 40, y: 20 },
            clock_world: WorldPos { x: 45, y: 25 },
            weather_world: WorldPos { x: 55, y: 26 },
            date_world: WorldPos { x: 45, y: 23 },
            calendar_world: WorldPos { x: 60, y: 22 },
        }
    }

    fn render_state(
        viewport_x: i32,
        viewport_y: i32,
        width: u16,
        height: u16,
        camera_x: i32,
        camera_y: i32,
        follow_hero: bool,
    ) -> RenderState {
        RenderState {
            world: world_frame(),
            hud: HudFrame {
                viewport: Viewport {
                    x: viewport_x,
                    y: viewport_y,
                    width,
                    height,
                },
                viewport_rect: Rect::new(0, 0, width, height),
                camera: Camera {
                    x: camera_x,
                    y: camera_y,
                    width,
                    height,
                    follow_hero,
                },
            },
        }
    }

    #[test]
    fn border_probe_stays_datum_centered_with_one_cell_inset() {
        let border = border_probe_bounds();

        assert_eq!(border.left, -WORLD_HALF_W);
        assert_eq!(border.right, WORLD_HALF_W - 1);
        assert_eq!(border.top, WORLD_HALF_H - 1);
        assert_eq!(border.bottom, -WORLD_HALF_H);
        assert_eq!(border.mid_x, 0);
        assert_eq!(border.mid_y, 0);
        assert_eq!(border.top - border.bottom, crate::scene::WORLD_HEIGHT - 1);
        assert_eq!(border.right - border.left, crate::scene::WORLD_WIDTH - 1);
    }

    #[test]
    fn camera_scrollbars_render_on_the_outer_frame() {
        let mut grid = crate::render::compositor::Grid::new(124, 32);
        let ctx = render_state(30, 10, 124, 32, 30, 10, false);

        draw_camera_scrollbars(&mut grid, 124, 32, &ctx);

        let top_row = 0;
        let left_col = 0;
        let top_cell = &grid.cells[grid.index(left_col, top_row)];
        let side_cell = &grid.cells[grid.index(left_col, top_row + 1)];
        let thumb_present = grid
            .cells
            .iter()
            .any(|cell| cell.style.fg == Some(palette::CAMERA_THUMB));

        assert_ne!(top_cell.symbol, ' ');
        assert_ne!(side_cell.symbol, ' ');
        assert!(thumb_present);
    }

    #[test]
    fn camera_scrollbars_hide_when_viewport_already_covers_the_full_world() {
        let mut grid = crate::render::compositor::Grid::new(212, 57);
        let ctx = render_state(-106, -28, 212, 56, -106, -28, false);

        draw_camera_scrollbars(&mut grid, 212, 57, &ctx);

        assert!(grid.cells.iter().all(|cell| cell.symbol == ' '));
    }

    #[test]
    fn world_frame_and_axis_are_independent_debug_overlays() {
        let border = border_probe_bounds();
        let mut frame_grid = crate::render::compositor::Grid::new(212, 57);
        draw_world_frame(&mut frame_grid, border, -106, -28, 56, 212, 57);
        let center = project_world_to_screen(WorldPos { x: 0, y: 0 }, -106, -28, 56);
        let top_center = project_world_to_screen(
            WorldPos {
                x: 0,
                y: border.top,
            },
            -106,
            -28,
            56,
        );
        assert_eq!(
            frame_grid.cells[frame_grid.index(center.x as u16, center.y as u16)].symbol,
            ' '
        );
        assert_eq!(
            frame_grid.cells[frame_grid.index(top_center.x as u16, top_center.y as u16)].symbol,
            '-'
        );

        let mut axis_grid = crate::render::compositor::Grid::new(212, 57);
        draw_world_axis(&mut axis_grid, border, -106, -28, 56, 212, 57);
        let top_left = project_world_to_screen(
            WorldPos {
                x: border.left,
                y: border.top,
            },
            -106,
            -28,
            56,
        );
        assert_eq!(
            axis_grid.cells[axis_grid.index(center.x as u16, center.y as u16)].symbol,
            '+'
        );
        assert_eq!(
            axis_grid.cells[axis_grid.index(top_left.x as u16, top_left.y as u16)].symbol,
            ' '
        );
    }

    #[test]
    fn world_datum_is_independent_from_frame_and_axis() {
        let border = border_probe_bounds();
        let center = project_world_to_screen(WorldPos { x: 0, y: 0 }, -106, -28, 56);
        let mut datum_grid = crate::render::compositor::Grid::new(212, 57);
        draw_world_datum(&mut datum_grid, border, -106, -28, 56, 212, 57);

        assert_eq!(
            datum_grid.cells[datum_grid.index(center.x as u16, center.y as u16)].symbol,
            '+'
        );

        let top_center = project_world_to_screen(
            WorldPos {
                x: 0,
                y: border.top,
            },
            -106,
            -28,
            56,
        );
        assert_eq!(
            datum_grid.cells[datum_grid.index(top_center.x as u16, top_center.y as u16)].symbol,
            ' '
        );
    }

    #[test]
    fn debug_panel_defaults_to_runtime_tab() {
        let layer = super::DebugLayer;
        let world = crate::core::world::WorldState::new();
        let fonts = crate::render::fonts::FontRegistry::new();
        let ctx = render_state(30, 10, 124, 32, 30, 10, true);
        let mut ui = crate::ui::state::UiState::new();
        ui.camera.follow_hero = true;
        ui.meta.dev_mode = true;
        ui.meta.pointer_probe_open = true;
        ui.pointer_blink_on = true;

        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &ctx);
        let text: String = output.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("[runtime]"));
        assert!(text.contains(" hero "));
        assert!(text.contains(" companions "));
        assert!(text.contains(" vines "));
        assert!(text.contains("FPS:"));
        assert!(text.contains("Camera: (30, 10)"));
        assert!(text.contains("Camera mode: follow-hero"));
        assert!(text.contains("Move mode: off (hero)"));
        assert!(text.contains("Pointer: on (0, 0)"));
        assert!(!text.contains("Hero world:"));
        assert!(!text.contains("Hero FPS:"));
        assert!(!text.contains("Clock world:"));
        assert!(!text.contains("Vines: 1 (id 1, yam.vine.border_v1)"));
    }

    #[test]
    fn debug_panel_tabs_expose_focused_fact_groups() {
        let layer = super::DebugLayer;
        let world = crate::core::world::WorldState::new();
        let fonts = crate::render::fonts::FontRegistry::new();
        let ctx = render_state(30, 10, 124, 32, 30, 10, true);
        let mut ui = crate::ui::state::UiState::new();
        ui.meta.dev_mode = true;

        ui.meta.debug_panel_tab = DebugPanelTab::Hero;
        let hero_text: String = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid
            .cells
            .iter()
            .map(|cell| cell.symbol)
            .collect();
        assert!(hero_text.contains("[hero]"));
        assert!(hero_text.contains("Hero FPS:"));
        assert!(hero_text.contains("Frame:"));
        assert!(hero_text.contains("Playing:"));
        assert!(hero_text.contains("Hero world:"));
        assert!(hero_text.contains("Hero screen:"));
        assert!(hero_text.contains("Hero anchor visible:"));
        assert!(!hero_text.contains("Camera: (30, 10)"));
        assert!(!hero_text.contains("Soft band:"));
        assert!(!hero_text.contains("Clock world:"));

        ui.meta.debug_panel_tab = DebugPanelTab::Companions;
        let companions_text: String = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid
            .cells
            .iter()
            .map(|cell| cell.symbol)
            .collect();
        assert!(companions_text.contains("[companions]"));
        assert!(companions_text.contains("Clock world:"));
        assert!(companions_text.contains("Weather world:"));
        assert!(companions_text.contains("Date world:"));
        assert!(!companions_text.contains("Vines: 1"));

        ui.meta.debug_panel_tab = DebugPanelTab::Vines;
        let vines_text: String = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid
            .cells
            .iter()
            .map(|cell| cell.symbol)
            .collect();
        assert!(vines_text.contains("[vines]"));
        assert!(vines_text.contains("Guides: "));
        assert!(vines_text.contains("Soft band:"));
        assert!(vines_text.contains("Vines: 1 (id 1, yam.vine.border_v1)"));
        assert!(vines_text.contains("Vine axes: 1 / segments:"));
        assert!(vines_text.contains("Vine tips: 1 active / 0 dormant"));
        assert!(vines_text.contains("Vine guide set: main-scene-vine-frame"));
    }

    #[test]
    fn debug_info_panel_can_be_hidden_without_disabling_other_debug_surfaces() {
        let layer = super::DebugLayer;
        let world = crate::core::world::WorldState::new();
        let fonts = crate::render::fonts::FontRegistry::new();
        let ctx = render_state(30, 10, 124, 32, 30, 10, true);
        let mut ui = crate::ui::state::UiState::new();
        ui.meta.dev_mode = true;
        ui.meta.debug_info_panel_visible = false;

        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &ctx);
        let text: String = output.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(!text.contains("FPS:"));
        assert!(text.contains("-"));
    }

    #[test]
    fn camera_scrollbars_can_be_hidden_independently() {
        let layer = super::DebugLayer;
        let world = crate::core::world::WorldState::new();
        let fonts = crate::render::fonts::FontRegistry::new();
        let ctx = render_state(30, 10, 124, 32, 30, 10, true);
        let mut ui = crate::ui::state::UiState::new();
        ui.meta.dev_mode = true;
        ui.meta.sliders_visible = false;

        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &ctx);
        let thumb_present = output
            .grid
            .cells
            .iter()
            .any(|cell| cell.style.fg == Some(palette::CAMERA_THUMB));

        assert!(!thumb_present);
    }

    #[test]
    fn vine_guides_hide_with_vines_when_other_debug_surfaces_are_off() {
        let layer = super::DebugLayer;
        let world = crate::core::world::WorldState::new();
        let fonts = crate::render::fonts::FontRegistry::new();
        let ctx = render_state(30, 10, 124, 32, 30, 10, true);
        let mut ui = crate::ui::state::UiState::new();
        ui.meta.dev_mode = true;
        ui.meta.vines_visible = false;
        ui.meta.debug_info_panel_visible = false;
        ui.meta.sliders_visible = false;
        ui.meta.world_frame_visible = false;
        ui.meta.world_axis_visible = false;
        ui.meta.world_datum_visible = false;

        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &ctx);

        assert!(output.grid.cells.iter().all(|cell| cell.symbol == ' '));
    }

    #[test]
    fn sandbox_debug_panel_focuses_on_spatial_trials_not_main_scene_telemetry() {
        let layer = super::DebugLayer;
        let world = crate::core::world::WorldState::for_sandbox();
        let fonts = crate::render::fonts::FontRegistry::new();
        let ctx = render_state(30, 10, 124, 32, 30, 10, false);
        let mut ui = crate::ui::state::UiState::new();
        ui.meta.dev_mode = true;
        ui.meta.active_world = crate::ui::state::WorldKindSnapshot::Sandbox;
        ui.meta.pointer_probe_open = true;
        ui.pointer_blink_on = true;

        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &ctx);
        let text: String = output.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("[runtime]"));
        assert!(text.contains("World: sandbox"));
        assert!(text.contains("Pointer: on (0, 0)"));
        assert!(text.contains("Pointer visible:"));
        assert!(!text.contains("Guides: 0"));
        assert!(!text.contains("Vines: 0"));
        assert!(!text.contains("Hero: main-scene only"));
        assert!(!text.contains("Hero world:"));
        assert!(!text.contains("Hero screen:"));
        assert!(!text.contains("Clock world:"));
        assert!(!text.contains("Clock screen:"));
        assert!(!text.contains("Date world:"));
        assert!(!text.contains("Date screen:"));
        assert!(!text.contains("Hero FPS:"));
        assert!(!text.contains("Frame:"));

        ui.meta.debug_panel_tab = DebugPanelTab::Vines;
        let vines_output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &ctx);
        let vines_text: String = vines_output
            .grid
            .cells
            .iter()
            .map(|cell| cell.symbol)
            .collect();
        assert!(vines_text.contains("[vines]"));
        assert!(vines_text.contains("Soft band:"));
        assert!(vines_text.contains("Guides: 0"));
        assert!(vines_text.contains("Vines: 0"));

        ui.meta.debug_panel_tab = DebugPanelTab::Hero;
        let hero_output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &ctx);
        let hero_text: String = hero_output
            .grid
            .cells
            .iter()
            .map(|cell| cell.symbol)
            .collect();
        assert!(hero_text.contains("[hero]"));
        assert!(hero_text.contains("Hero: main-scene only"));
        assert!(hero_text.contains("World: sandbox"));
        assert!(!hero_text.contains("Hero world:"));
    }

    #[test]
    fn pointer_probe_renders_a_distinguishable_marker_on_the_world_grid() {
        let layer = super::DebugLayer;
        let world = crate::core::world::WorldState::new();
        let fonts = crate::render::fonts::FontRegistry::new();
        let ctx = render_state(30, 10, 124, 32, -63, -17, false);
        let mut ui = crate::ui::state::UiState::new();
        ui.meta.dev_mode = true;
        ui.meta.pointer_probe_open = true;
        ui.pointer_blink_on = true;
        ui.offsets.pointer_x = 0;
        ui.offsets.pointer_y = 0;

        let output = layer.render_to_grid(124, 32, &world, &ui, &fonts, &ctx);
        let pointer_screen = project_world_to_screen(
            WorldPos { x: 0, y: 0 },
            ctx.hud.camera.x,
            ctx.hud.camera.y,
            ctx.hud.camera.height,
        );
        let cell = &output.grid.cells[output
            .grid
            .index(pointer_screen.x as u16, pointer_screen.y as u16)];

        assert_eq!(cell.symbol, '+');
        assert_eq!(cell.style.fg, Some(crate::theme::palette::POINTER_PROBE));
    }

    #[test]
    fn scrollbar_position_spans_the_full_range() {
        let viewport_width = 124usize;
        let world_width =
            (crate::scene::WORLD_WIDTH + crate::scene::CAMERA_OVERSCAN_CELLS * 2) as usize;
        let camera_min = -crate::scene::WORLD_HALF_W - crate::scene::CAMERA_OVERSCAN_CELLS;
        let camera_max = crate::scene::WORLD_HALF_W - 1 + crate::scene::CAMERA_OVERSCAN_CELLS
            - viewport_width as i32
            + 1;

        assert_eq!(
            scrollbar_position(camera_min, camera_min, camera_max, world_width),
            0
        );
        assert_eq!(
            scrollbar_position(camera_max, camera_min, camera_max, world_width),
            world_width.saturating_sub(1)
        );
    }

    #[test]
    fn soft_line_glyph_prefers_shallow_and_diagonal_strokes() {
        let shallow = (LinePoint { x: 0, y: 0 }, LinePoint { x: 64, y: -10 });
        let down_right = (LinePoint { x: 0, y: 0 }, LinePoint { x: 1, y: -4 });
        let up_right = (LinePoint { x: 0, y: 0 }, LinePoint { x: 1, y: 4 });
        let up_left = (LinePoint { x: 0, y: 0 }, LinePoint { x: -1, y: 4 });

        let glyphs: Vec<char> = [0, 4, 8]
            .into_iter()
            .map(|step| soft_line_glyph(shallow.0, shallow.1, step, 8))
            .collect();

        assert!(glyphs.contains(&'-'));
        assert!(glyphs
            .iter()
            .any(|glyph| matches!(glyph, '.' | ',' | '`' | '\'' | '_')));
        assert!(matches!(
            soft_line_glyph(down_right.0, down_right.1, 2, 8),
            '/' | '\\'
        ));
        assert!(matches!(
            soft_line_glyph(up_right.0, up_right.1, 5, 8),
            '/' | '\\'
        ));
        assert!(matches!(
            soft_line_glyph(up_left.0, up_left.1, 5, 8),
            '/' | '\\'
        ));
    }

    #[test]
    fn soft_probe_line_draws_a_visible_mark() {
        let mut grid = crate::render::compositor::Grid::new(124, 32);
        draw_soft_probe_line(
            &mut grid,
            -63,
            -17,
            32,
            crate::scene::coords::WorldPos { x: -28, y: 22 },
            crate::scene::coords::WorldPos { x: 36, y: 12 },
        );

        let visible = grid.cells.iter().any(|cell| cell.symbol != ' ');
        assert!(visible);
    }

    #[test]
    fn mirrored_soft_probe_line_draws_a_visible_mark() {
        let mut grid = crate::render::compositor::Grid::new(124, 32);
        draw_soft_probe_line(
            &mut grid,
            -63,
            -17,
            32,
            crate::scene::coords::WorldPos { x: -28, y: 12 },
            crate::scene::coords::WorldPos { x: 36, y: 22 },
        );

        let visible = grid.cells.iter().any(|cell| cell.symbol != ' ');
        assert!(visible);
    }
}
