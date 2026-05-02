use crate::core::guide::{Guide, GuidePoint, GuideShape, GuideState};
use crate::core::guide_line::{glyph_for_line_step, rasterize_line, LinePoint};
use crate::render::compositor::Grid;
use crate::scene::coords::{world_to_screen, WorldPos};
use crate::theme::style;
use ratatui::prelude::Color;
use ratatui::style::Style;

pub fn draw_guides(grid: &mut Grid, guides: &GuideState, camera_x: i32, camera_y: i32) {
    for guide in guides.enabled_guides().filter(|guide| guide.style.visible) {
        draw_guide(grid, guide, camera_x, camera_y);
    }
}

fn draw_guide(grid: &mut Grid, guide: &Guide, camera_x: i32, camera_y: i32) {
    match &guide.shape {
        GuideShape::Point => draw_point(grid, guide.anchor, guide.style.glyph, camera_x, camera_y),
        GuideShape::Line { end } => draw_segment(
            grid,
            guide.anchor,
            *end,
            guide.style.glyph,
            camera_x,
            camera_y,
        ),
        GuideShape::Polyline(points) => draw_polyline(
            grid,
            guide.anchor,
            points,
            guide.style.glyph,
            camera_x,
            camera_y,
            false,
        ),
        GuideShape::Polygon(points) => draw_polyline(
            grid,
            guide.anchor,
            points,
            guide.style.glyph,
            camera_x,
            camera_y,
            true,
        ),
        GuideShape::Rect { width, height } => {
            let origin = guide.anchor;
            let points = [
                GuidePoint {
                    x: origin.x,
                    y: origin.y,
                },
                GuidePoint {
                    x: origin.x + *width,
                    y: origin.y,
                },
                GuidePoint {
                    x: origin.x + *width,
                    y: origin.y + *height,
                },
                GuidePoint {
                    x: origin.x,
                    y: origin.y + *height,
                },
            ];
            draw_polyline(
                grid,
                guide.anchor,
                &points[1..],
                guide.style.glyph,
                camera_x,
                camera_y,
                true,
            );
        }
        GuideShape::Circle { radius } => {
            let radius = (*radius).max(1);
            let center = world_to_screen(
                WorldPos {
                    x: guide.anchor.x,
                    y: guide.anchor.y,
                },
                camera_x,
                camera_y,
            );
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if dx * dx + dy * dy > radius * radius {
                        continue;
                    }
                    if let Some(cell) =
                        grid.cell_mut((center.x + dx) as u16, (center.y + dy) as u16)
                    {
                        cell.symbol = guide.style.glyph;
                        cell.style = Style::default().fg(Color::DarkGray);
                    }
                }
            }
        }
    }
}

fn draw_point(grid: &mut Grid, point: GuidePoint, glyph: char, camera_x: i32, camera_y: i32) {
    let screen = world_to_screen(
        WorldPos {
            x: point.x,
            y: point.y,
        },
        camera_x,
        camera_y,
    );
    if let Some(cell) = grid.cell_mut(screen.x as u16, screen.y as u16) {
        cell.symbol = glyph;
        cell.style = style::pointer_probe();
    }
}

fn draw_segment(
    grid: &mut Grid,
    start: GuidePoint,
    end: GuidePoint,
    glyph: char,
    camera_x: i32,
    camera_y: i32,
) {
    let start = world_to_screen(
        WorldPos {
            x: start.x,
            y: start.y,
        },
        camera_x,
        camera_y,
    );
    let end = world_to_screen(WorldPos { x: end.x, y: end.y }, camera_x, camera_y);
    let start = LinePoint {
        x: start.x,
        y: start.y,
    };
    let end = LinePoint { x: end.x, y: end.y };

    let steps = rasterize_line(start, end);
    for (idx, step) in steps.iter().enumerate() {
        if step.point.x < 0 || step.point.y < 0 {
            continue;
        }
        let previous = idx
            .checked_sub(1)
            .and_then(|i| steps.get(i))
            .map(|step| step.point);
        let next = steps.get(idx + 1).map(|step| step.point);
        let ch = if glyph == ' ' {
            glyph_for_line_step(
                start, end, step.point, previous, next, step.step, step.steps,
            )
        } else {
            glyph
        };
        if let Some(cell) = grid.cell_mut(step.point.x as u16, step.point.y as u16) {
            cell.symbol = ch;
            cell.style = Style::default().fg(Color::DarkGray);
        }
    }
}

fn draw_polyline(
    grid: &mut Grid,
    anchor: GuidePoint,
    points: &[GuidePoint],
    glyph: char,
    camera_x: i32,
    camera_y: i32,
    closed: bool,
) {
    let mut last = anchor;
    for point in points {
        draw_segment(grid, last, *point, glyph, camera_x, camera_y);
        last = *point;
    }
    if closed && !points.is_empty() {
        draw_segment(grid, last, anchor, glyph, camera_x, camera_y);
    }
}
