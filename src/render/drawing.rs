use ratatui::style::Style;

use crate::core::guide_line::{glyph_for_line_step, rasterize_line, LinePoint, LineStep};
use crate::render::compositor::{Cell, Grid};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrokeWeight {
    Hairline,
    Stem,
    Trunk,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Brush {
    pub style: Style,
    pub weight: StrokeWeight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GlyphStamp {
    pub point: LinePoint,
    pub glyph: char,
    pub style: Style,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OccupancyMask {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

impl OccupancyMask {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    pub fn mark(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.width + x;
        self.data[idx] = self.data[idx].saturating_add(1);
    }

    #[allow(dead_code)]
    pub fn coverage(&self, x: usize, y: usize) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        self.data[y * self.width + x]
    }
}

pub fn stroke_path(
    grid: &mut Grid,
    occupancy: Option<&mut OccupancyMask>,
    path: &[LinePoint],
    brush: Brush,
) {
    if path.len() < 2 {
        if let Some(point) = path.first().copied() {
            stamp_glyph(
                grid,
                occupancy,
                GlyphStamp {
                    point,
                    glyph: match brush.weight {
                        StrokeWeight::Hairline => '.',
                        StrokeWeight::Stem => 'o',
                        StrokeWeight::Trunk => '#',
                    },
                    style: brush.style,
                },
            );
        }
        return;
    }

    let mut occupancy = occupancy;
    for pair in path.windows(2) {
        stroke_segment(grid, occupancy.as_deref_mut(), pair[0], pair[1], brush);
    }
}

pub fn stamp_glyph(grid: &mut Grid, occupancy: Option<&mut OccupancyMask>, stamp: GlyphStamp) {
    let Some((x, y)) = checked_grid_coords(stamp.point) else {
        return;
    };
    if let Some(cell) = grid.cell_mut(x, y) {
        *cell = Cell {
            symbol: stamp.glyph,
            style: stamp.style,
        };
        if let Some(occupancy) = occupancy {
            occupancy.mark(x as usize, y as usize);
        }
    }
}

pub fn stroke_segment(
    grid: &mut Grid,
    occupancy: Option<&mut OccupancyMask>,
    start: LinePoint,
    end: LinePoint,
    brush: Brush,
) {
    let steps = rasterize_line(start, end);
    let mut occupancy = occupancy;
    for (idx, step) in steps.iter().enumerate() {
        let Some((x, y)) = checked_grid_coords(step.point) else {
            continue;
        };

        let previous = idx
            .checked_sub(1)
            .and_then(|i| steps.get(i))
            .map(|candidate| candidate.point);
        let next = steps.get(idx + 1).map(|candidate| candidate.point);
        let glyph = glyph_for_weight(brush.weight, start, end, step, previous, next);

        if let Some(cell) = grid.cell_mut(x, y) {
            *cell = Cell {
                symbol: glyph,
                style: brush.style,
            };
            if let Some(occupancy) = occupancy.as_deref_mut() {
                occupancy.mark(x as usize, y as usize);
            }
        }
    }
}

fn checked_grid_coords(point: LinePoint) -> Option<(u16, u16)> {
    Some((u16::try_from(point.x).ok()?, u16::try_from(point.y).ok()?))
}

fn glyph_for_weight(
    weight: StrokeWeight,
    start: LinePoint,
    end: LinePoint,
    step: &LineStep,
    previous: Option<LinePoint>,
    next: Option<LinePoint>,
) -> char {
    match weight {
        StrokeWeight::Hairline => glyph_for_line_step(
            start, end, step.point, previous, next, step.step, step.steps,
        ),
        StrokeWeight::Stem => stem_glyph(previous, step.point, next),
        StrokeWeight::Trunk => trunk_glyph(previous, step.point, next),
    }
}

fn stem_glyph(previous: Option<LinePoint>, current: LinePoint, next: Option<LinePoint>) -> char {
    let incoming = previous.map(|point| (current.x - point.x, current.y - point.y));
    let outgoing = next.map(|point| (point.x - current.x, point.y - current.y));
    match (incoming, outgoing) {
        (Some((dx0, dy0)), Some((dx1, dy1))) => continuity_glyph((dx0, dy0), (dx1, dy1)),
        (Some((dx, dy)), None) | (None, Some((dx, dy))) => direction_glyph(dx, dy),
        (None, None) => 'o',
    }
}

fn trunk_glyph(previous: Option<LinePoint>, current: LinePoint, next: Option<LinePoint>) -> char {
    match stem_glyph(previous, current, next) {
        '-' | '/' | '\\' => '=',
        '|' | '+' => '#',
        other => other,
    }
}

fn continuity_glyph(incoming: (i32, i32), outgoing: (i32, i32)) -> char {
    let a = normalize_step(incoming);
    let b = normalize_step(outgoing);
    if a == b {
        return direction_glyph(a.0, a.1);
    }
    if a.0 == b.0 && a.0 == 0 {
        return '|';
    }
    if a.1 == b.1 && a.1 == 0 {
        return '-';
    }
    if diagonal_family(a) == diagonal_family(b) {
        return diagonal_family(a);
    }
    '+'
}

fn direction_glyph(dx: i32, dy: i32) -> char {
    let (dx, dy) = normalize_step((dx, dy));
    if dx == 0 {
        '|'
    } else if dy == 0 {
        '-'
    } else {
        diagonal_family((dx, dy))
    }
}

fn diagonal_family((dx, dy): (i32, i32)) -> char {
    if dx.signum() == dy.signum() {
        '\\'
    } else {
        '/'
    }
}

fn normalize_step((dx, dy): (i32, i32)) -> (i32, i32) {
    (dx.signum(), dy.signum())
}

#[cfg(test)]
mod tests {
    use ratatui::style::{Color, Style};

    use super::{stamp_glyph, stroke_path, Brush, GlyphStamp, OccupancyMask, StrokeWeight};
    use crate::core::guide_line::LinePoint;
    use crate::render::compositor::Grid;

    #[test]
    fn stroke_path_marks_occupancy_and_uses_structural_stem_glyphs() {
        let mut grid = Grid::new(6, 3);
        let mut occupancy = OccupancyMask::new(6, 3);
        stroke_path(
            &mut grid,
            Some(&mut occupancy),
            &[
                LinePoint { x: 1, y: 1 },
                LinePoint { x: 2, y: 1 },
                LinePoint { x: 3, y: 1 },
            ],
            Brush {
                style: Style::default().fg(Color::Green),
                weight: StrokeWeight::Stem,
            },
        );

        assert_eq!(grid.get_mut(1, 1).map(|cell| cell.symbol), Some('-'));
        assert_eq!(grid.get_mut(2, 1).map(|cell| cell.symbol), Some('-'));
        assert_eq!(occupancy.coverage(1, 1), 1);
        assert_eq!(occupancy.coverage(2, 1), 2);
        assert_eq!(occupancy.coverage(3, 1), 1);
    }

    #[test]
    fn stamp_glyph_writes_one_cell_and_marks_occupancy() {
        let mut grid = Grid::new(4, 4);
        let mut occupancy = OccupancyMask::new(4, 4);
        stamp_glyph(
            &mut grid,
            Some(&mut occupancy),
            GlyphStamp {
                point: LinePoint { x: 2, y: 1 },
                glyph: '*',
                style: Style::default().fg(Color::Yellow),
            },
        );

        assert_eq!(grid.get_mut(2, 1).map(|cell| cell.symbol), Some('*'));
        assert_eq!(occupancy.coverage(2, 1), 1);
    }

    #[test]
    fn drawing_points_outside_grid_coordinate_range_do_not_wrap() {
        let mut grid = Grid::new(4, 4);
        let far_x = u16::MAX as i32 + 1;
        stamp_glyph(
            &mut grid,
            None,
            GlyphStamp {
                point: LinePoint { x: far_x, y: 0 },
                glyph: '*',
                style: Style::default().fg(Color::Yellow),
            },
        );
        stroke_path(
            &mut grid,
            None,
            &[
                LinePoint { x: far_x, y: 1 },
                LinePoint { x: far_x + 1, y: 1 },
            ],
            Brush {
                style: Style::default().fg(Color::Green),
                weight: StrokeWeight::Stem,
            },
        );

        assert!(grid.cells.iter().all(|cell| cell.symbol == ' '));
    }
}
