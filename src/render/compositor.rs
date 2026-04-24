use ratatui::{
    prelude::*,
    text::{Line, Span},
};

#[derive(Clone)]
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

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::blank(); width as usize * height as usize],
        }
    }

    fn idx(&self, x: u16, y: u16) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y as usize * self.width as usize + x as usize)
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
