use crate::core::world::WorldState;
use crate::render::compositor::Cell;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::coords::{world_to_screen, WorldPos};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;
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
        if !ui.meta.dev_mode {
            return LayerOutput { grid, mask: None };
        }

        draw_camera_scrollbars(&mut grid, width, height, ctx);

        let panel_x = 10u16;
        let panel_y = 5u16;
        let panel_width = 52u16;
        let panel_height = 10u16;

        let cam_x = ctx.hud.camera.x;
        let cam_y = ctx.hud.camera.y;
        let screen_w = width as i32;
        let screen_h = height as i32;
        let exclude_x0 = panel_x.saturating_sub(2);
        let exclude_y0 = panel_y.saturating_sub(2);
        let exclude_x1 = panel_x + panel_width + 2;
        let exclude_y1 = panel_y + panel_height + 2;
        // Datum-centered world-border probe:
        // the frame is defined in world space and projected through the current camera.
        let border = border_probe_bounds();

        let mut draw_border_cell = |wx: i32, wy: i32, ch: char| {
            let screen = world_to_screen(WorldPos { x: wx, y: wy }, cam_x, cam_y);
            if screen.x < 0 || screen.y < 0 || screen.x >= screen_w || screen.y >= screen_h {
                return;
            }
            if screen.x >= exclude_x0 as i32
                && screen.x <= exclude_x1 as i32
                && screen.y >= exclude_y0 as i32
                && screen.y <= exclude_y1 as i32
            {
                return;
            }
            if let Some(cell) = grid.cell_mut(screen.x as u16, screen.y as u16) {
                cell.symbol = ch;
                cell.style = Style::default().fg(Color::DarkGray);
            }
        };

        for wx in border.left..=border.right {
            let ch = if wx == border.left || wx == border.mid_x || wx == border.right {
                '+'
            } else {
                '-'
            };
            draw_border_cell(wx, border.top, ch);
            draw_border_cell(wx, border.mid_y, ch);
            draw_border_cell(wx, border.bottom, ch);
        }

        for wy in border.bottom + 1..border.top {
            let ch = if wy == border.mid_y { '+' } else { '|' };
            draw_border_cell(border.left, wy, ch);
            draw_border_cell(border.mid_x, wy, ch);
            draw_border_cell(border.right, wy, ch);
        }

        let hero = &ui.hero;
        let hero_anchor = ctx.world.hero_world;
        let hero_visual_anchor = ctx.world.hero_visual_anchor;
        let clock_world = ctx.world.clock_world;
        let clock_screen = ctx.clock_screen();
        let clock_visible = clock_screen.x >= 0
            && clock_screen.y >= 0
            && clock_screen.x < width as i32
            && clock_screen.y < height as i32;
        let camera_mode = if ui.camera.follow_hero {
            "Camera mode: follow-hero"
        } else {
            "Camera mode: manual pan"
        };
        let center_x = width as i32 / 2;
        let center_y = height as i32 / 2;
        let cam_dx = cam_x - center_x;
        let cam_dy = cam_y - center_y;
        let lines = [
            format!("FPS: {:.1}", ui.fps),
            format!("Hero FPS: {:.1}", ui.offsets.hero_fps),
            format!("Frame: {} / {}", hero.current_frame, hero.frames.len()),
            format!("Playing: {}", hero.playing),
            format!("Hero anchor: ({}, {})", hero_anchor.x, hero_anchor.y),
            format!(
                "Hero visual anchor: ({}, {})",
                hero_visual_anchor.x, hero_visual_anchor.y
            ),
            format!(
                "Hero offset: ({}, {})",
                ui.offsets.hero_dx, ui.offsets.hero_dy
            ),
            format!("Clock world: ({}, {})", clock_world.x, clock_world.y),
            format!("Clock screen: ({}, {})", clock_screen.x, clock_screen.y),
            format!(
                "Clock anchor: ({}, {})",
                hero_visual_anchor.x, hero_visual_anchor.y
            ),
            format!(
                "Clock offset: ({}, {})",
                ui.offsets.clock_dx, ui.offsets.clock_dy
            ),
            format!("Clock final: ({}, {})", clock_screen.x, clock_screen.y),
            format!("Clock visible: {}", clock_visible),
            camera_mode.to_string(),
            format!("Camera: ({}, {})", cam_x, cam_y),
            format!("Camera Δ: ({}, {})", cam_dx, cam_dy),
        ];
        for (row, line) in lines.iter().enumerate() {
            write_string(
                &mut grid,
                panel_x,
                panel_y + row as u16,
                line,
                Style::default().fg(Color::Green),
            );
        }
        LayerOutput { grid, mask: None }
    }
}

fn draw_camera_scrollbars(grid: &mut Grid, width: u16, height: u16, ctx: &RenderState) {
    let inset = 2;
    if width <= inset * 2 || height <= inset * 2 {
        return;
    }

    let viewport = ctx.hud.viewport;
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
    let horizontal_area = Rect::new(inset, inset, width - inset * 2, 1);
    let vertical_area = Rect::new(inset, inset, 1, height - inset * 2);

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
    use super::{border_probe_bounds, draw_camera_scrollbars, scrollbar_position};
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::coords::WorldPos;
    use crate::scene::viewport::Viewport;
    use crate::scene::{WORLD_HALF_H, WORLD_HALF_W};
    use crate::theme::palette;
    use ratatui::prelude::Rect;

    #[test]
    fn border_probe_stays_datum_centered_with_one_cell_inset() {
        let border = border_probe_bounds();

        assert_eq!(border.left, -WORLD_HALF_W);
        assert_eq!(border.right, WORLD_HALF_W - 1);
        assert_eq!(border.top, WORLD_HALF_H - 1);
        assert_eq!(border.bottom, -WORLD_HALF_H);
        assert_eq!(border.mid_x, 0);
        assert_eq!(border.mid_y, 0);
        assert_eq!(border.top - border.bottom, crate::scene::WORLD_HEIGHT - 2);
        assert_eq!(border.right - border.left, crate::scene::WORLD_WIDTH - 1);
    }

    #[test]
    fn camera_scrollbars_render_in_the_third_cell_inward() {
        let mut grid = crate::render::compositor::Grid::new(124, 32);
        let ctx = RenderState {
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

        draw_camera_scrollbars(&mut grid, 124, 32, &ctx);

        let top_row = 2;
        let left_col = 2;
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
}
