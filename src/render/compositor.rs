use ratatui::{
    prelude::*,
    text::{Line, Span},
};

use crate::render::mask::Mask;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Cell {
    pub symbol: char,
    pub style: Style,
}

impl Cell {
    pub fn blank() -> Self {
        Self {
            symbol: ' ',
            style: Style::default(),
        }
    }
}

pub struct Grid {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
}

#[allow(dead_code)]
pub enum MaskMode<'a> {
    None,
    Apply(&'a Mask),
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::blank(); width as usize * height as usize],
        }
    }

    pub fn index(&self, x: u16, y: u16) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn idx(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(self.index(x, y))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.cells[idx] = cell;
        }
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            Some(&mut self.cells[idx])
        } else {
            None
        }
    }

    pub fn cell_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        self.idx(x, y).map(move |idx| &mut self.cells[idx])
    }

    pub fn merge_at(&mut self, x: u16, y: u16, top: &Cell) {
        if let Some(base) = self.cell_mut(x, y) {
            merge_cell(base, top);
        }
    }
}

pub fn write_string(grid: &mut Grid, x: u16, y: u16, text: &str, style: Style) {
    for (i, ch) in text.chars().enumerate() {
        let px = x + i as u16;
        if px >= grid.width || y >= grid.height {
            break;
        }
        let idx = grid.index(px, y);
        let cell = &mut grid.cells[idx];
        if ch != ' ' {
            cell.symbol = ch;
        }
        if let Some(fg) = style.fg {
            cell.style.fg = Some(fg);
        }
        if let Some(bg) = style.bg {
            cell.style.bg = Some(bg);
        }
        cell.style.add_modifier |= style.add_modifier;
        cell.style.sub_modifier |= style.sub_modifier;
    }
}

pub fn merge_cell(base: &mut Cell, top: &Cell) {
    if top.symbol != ' ' {
        base.symbol = top.symbol;
    }

    if top.style.fg.is_some() {
        base.style.fg = top.style.fg;
    }
    if top.style.bg.is_some() {
        base.style.bg = top.style.bg;
    }

    base.style.add_modifier |= top.style.add_modifier;
    base.style.sub_modifier |= top.style.sub_modifier;
}

#[allow(dead_code)]
pub fn merge_grid(base: &mut Grid, top: &Grid, mask: Option<&Mask>) {
    let width = base.width.min(top.width);
    let height = base.height.min(top.height);

    for y in 0..height {
        for x in 0..width {
            if let Some(mask) = mask {
                if mask.width == 0 || mask.height == 0 {
                    continue;
                }
                if x as usize >= mask.width
                    || y as usize >= mask.height
                    || !mask.get(x as usize, y as usize)
                {
                    continue;
                }
            }

            let idx = base.index(x, y);
            let top_idx = top.index(x, y);
            if let (Some(base_cell), Some(top_cell)) =
                (base.cells.get_mut(idx), top.cells.get(top_idx))
            {
                merge_cell(base_cell, top_cell);
            }
        }
    }
}

#[allow(dead_code)]
pub fn merge_grid_legacy(base: &mut Grid, top: &Grid, mask: MaskMode<'_>) {
    match mask {
        MaskMode::None => merge_grid(base, top, None),
        MaskMode::Apply(mask) => merge_grid(base, top, Some(mask)),
    }
}

pub fn lines_to_grid(lines: &[Line<'_>], width: u16, height: u16) -> Grid {
    let mut grid = Grid::new(width, height);

    for (y, line) in lines.iter().take(height as usize).enumerate() {
        let mut x = 0u16;
        for span in &line.spans {
            for grapheme in span.styled_graphemes(Style::default()) {
                let width = grapheme.symbol.chars().count() as u16;
                if width == 0 {
                    continue;
                }
                if x >= grid.width {
                    break;
                }

                let styled = Cell {
                    symbol: grapheme.symbol.chars().next().unwrap_or(' '),
                    style: grapheme.style,
                };
                grid.merge_at(x, y as u16, &styled);
                x = x.saturating_add(width);
            }
        }
    }

    grid
}

pub fn grid_to_lines(grid: &Grid) -> Vec<Line<'static>> {
    let mut lines = Vec::with_capacity(grid.height as usize);

    for y in 0..grid.height {
        let mut spans = Vec::new();
        let mut current_style = None;
        let mut current_text = String::new();

        for x in 0..grid.width {
            let cell = &grid.cells[(y as usize * grid.width as usize) + x as usize];
            if current_style == Some(cell.style) {
                current_text.push(cell.symbol);
            } else {
                if let Some(style) = current_style.take() {
                    spans.push(Span::styled(std::mem::take(&mut current_text), style));
                }
                current_style = Some(cell.style);
                current_text.push(cell.symbol);
            }
        }

        if let Some(style) = current_style.take() {
            spans.push(Span::styled(current_text, style));
        }

        lines.push(Line::from(spans));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_can_protect_cells_during_merge() {
        let mut base = Grid::new(2, 2);
        let mut top = Grid::new(2, 2);
        top.merge_at(
            0,
            0,
            &Cell {
                symbol: 'X',
                style: Style::default().fg(Color::Red),
            },
        );

        let mut mask = Mask::new(2, 2);
        mask.set(0, 0, false);

        merge_grid(&mut base, &top, Some(&mask));
        assert_eq!(base.cells[0].symbol, ' ');

        merge_grid(&mut base, &top, None);
        assert_eq!(base.cells[0].symbol, 'X');
    }
}
