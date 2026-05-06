use crate::core::guide::{Guide, GuidePoint, GuideShape};
use crate::core::guide_line::{glyph_for_line_step, rasterize_line, LinePoint};
use crate::core::spatial::SpatialGuideIndex;
use crate::render::compositor::Grid;
use crate::scene::coords::{world_to_screen, WorldPos};
use crate::theme::style;

#[derive(Copy, Clone)]
struct GuideProjection {
    camera_x: i32,
    camera_y: i32,
    viewport_height: u16,
}

pub fn draw_guides(
    grid: &mut Grid,
    guide_index: SpatialGuideIndex<'_>,
    camera_x: i32,
    camera_y: i32,
    viewport_height: u16,
) {
    let projection = GuideProjection {
        camera_x,
        camera_y,
        viewport_height,
    };
    for guide in guide_index
        .guides
        .enabled_guides()
        .filter(|guide| guide.style.visible)
    {
        draw_guide(grid, guide, projection);
    }
}

fn draw_guide(grid: &mut Grid, guide: &Guide, projection: GuideProjection) {
    match &guide.shape {
        GuideShape::Point => draw_point(grid, guide.anchor, guide.style.glyph, projection),
        GuideShape::Line { end } => {
            draw_segment(grid, guide.anchor, *end, guide.style.glyph, projection)
        }
        GuideShape::Polyline(points) => draw_polyline(
            grid,
            guide.anchor,
            points,
            guide.style.glyph,
            projection,
            false,
        ),
        GuideShape::Polygon(points) => draw_polyline(
            grid,
            guide.anchor,
            points,
            guide.style.glyph,
            projection,
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
                projection,
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
                projection.camera_x,
                projection.camera_y,
                projection.viewport_height,
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
                        cell.style = style::guide_trace();
                    }
                }
            }
        }
    }
}

fn draw_point(grid: &mut Grid, point: GuidePoint, glyph: char, projection: GuideProjection) {
    let screen = world_to_screen(
        WorldPos {
            x: point.x,
            y: point.y,
        },
        projection.camera_x,
        projection.camera_y,
        projection.viewport_height,
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
    projection: GuideProjection,
) {
    let start = world_to_screen(
        WorldPos {
            x: start.x,
            y: start.y,
        },
        projection.camera_x,
        projection.camera_y,
        projection.viewport_height,
    );
    let end = world_to_screen(
        WorldPos { x: end.x, y: end.y },
        projection.camera_x,
        projection.camera_y,
        projection.viewport_height,
    );
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
            cell.style = style::guide_trace();
        }
    }
}

fn draw_polyline(
    grid: &mut Grid,
    anchor: GuidePoint,
    points: &[GuidePoint],
    glyph: char,
    projection: GuideProjection,
    closed: bool,
) {
    let mut last = anchor;
    for point in points {
        draw_segment(grid, last, *point, glyph, projection);
        last = *point;
    }
    if closed && !points.is_empty() {
        draw_segment(grid, last, anchor, glyph, projection);
    }
}
