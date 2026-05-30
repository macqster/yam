use crate::core::guide::{Guide, GuidePoint, GuideShape};
use crate::core::guide_line::{glyph_for_line_step, rasterize_line, LinePoint};
use crate::core::spatial::{
    SpatialGuideIndex, SpatialPoint, SpatialProjection, SpatialResolver, SpatialScreenPoint,
};
use crate::render::compositor::{Cell, Grid};
use crate::theme::style;

#[derive(Copy, Clone)]
struct GuideProjection {
    resolver: SpatialResolver,
}

impl GuideProjection {
    fn new(camera_x: i32, camera_y: i32, viewport_height: u16) -> Self {
        Self {
            resolver: SpatialResolver::new(SpatialProjection::new(
                camera_x,
                camera_y,
                0,
                viewport_height,
            )),
        }
    }

    fn project(self, point: GuidePoint) -> SpatialScreenPoint {
        self.resolver.world_to_screen_point(SpatialPoint {
            x: point.x,
            y: point.y,
        })
    }
}

pub fn draw_guides(
    grid: &mut Grid,
    guide_index: SpatialGuideIndex<'_>,
    camera_x: i32,
    camera_y: i32,
    viewport_height: u16,
) {
    let projection = GuideProjection::new(camera_x, camera_y, viewport_height);
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
            let center = projection.project(guide.anchor);
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if dx * dx + dy * dy > radius * radius {
                        continue;
                    }
                    if let Some(cell) = cell_mut_screen(grid, center.x + dx, center.y + dy) {
                        cell.symbol = guide.style.glyph;
                        cell.style = style::guide_trace();
                    }
                }
            }
        }
    }
}

fn draw_point(grid: &mut Grid, point: GuidePoint, glyph: char, projection: GuideProjection) {
    let screen = projection.project(point);
    if let Some(cell) = cell_mut_screen(grid, screen.x, screen.y) {
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
    let start = projection.project(start);
    let end = projection.project(end);
    let start = LinePoint {
        x: start.x,
        y: start.y,
    };
    let end = LinePoint { x: end.x, y: end.y };

    let steps = rasterize_line(start, end);
    for (idx, step) in steps.iter().enumerate() {
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
        if let Some(cell) = cell_mut_screen(grid, step.point.x, step.point.y) {
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

fn cell_mut_screen(grid: &mut Grid, x: i32, y: i32) -> Option<&mut Cell> {
    let x = u16::try_from(x).ok()?;
    let y = u16::try_from(y).ok()?;
    grid.cell_mut(x, y)
}

#[cfg(test)]
mod tests {
    use super::draw_guides;
    use crate::core::guide::{Guide, GuideKind, GuidePoint, GuideShape, GuideState, GuideStyle};
    use crate::core::spatial::SpatialGuideIndex;
    use crate::render::compositor::Grid;

    #[test]
    fn guide_points_project_through_signed_screen_coordinates() {
        let mut guides = GuideState::new();
        guides.guides.push(Guide {
            id: 1,
            label: "probe".to_string(),
            group: None,
            kind: GuideKind::Datum,
            anchor: GuidePoint { x: 30, y: 10 },
            shape: GuideShape::Point,
            style: GuideStyle {
                glyph: '+',
                visible: true,
                accent: true,
            },
            enabled: true,
        });
        let mut grid = Grid::new(4, 32);

        draw_guides(&mut grid, SpatialGuideIndex::new(&guides), 30, 10, 32);

        assert_eq!(grid.get_mut(0, 31).map(|cell| cell.symbol), Some('+'));
    }

    #[test]
    fn offscreen_guide_points_do_not_wrap_into_grid() {
        let mut guides = GuideState::new();
        guides.guides.push(Guide {
            id: 1,
            label: "offscreen".to_string(),
            group: None,
            kind: GuideKind::Datum,
            anchor: GuidePoint { x: -1, y: 0 },
            shape: GuideShape::Point,
            style: GuideStyle {
                glyph: '+',
                visible: true,
                accent: true,
            },
            enabled: true,
        });
        let mut grid = Grid::new(4, 4);

        draw_guides(&mut grid, SpatialGuideIndex::new(&guides), 0, 0, 4);

        assert!(grid.cells.iter().all(|cell| cell.symbol == ' '));
    }
}
