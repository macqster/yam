//! Shared per-cell correction/serialization contract.
//!
//! `CellGrid` is the "cell corrections" owner layer named in
//! `docs/hero-revision.md`'s pipeline table: a fixed-size grid of styled
//! terminal cells that both the disposable runtime hero cache
//! (`render::hero_cache`) and the validated compiled hero package
//! (`render::hero_package`) serialize through. Neither owner should
//! redefine this shape independently; extend it here instead.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use serde::{Deserialize, Serialize};

use crate::render::compositor::{grid_to_lines, lines_to_grid};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellGrid {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<CachedCell>,
}

impl CellGrid {
    pub fn from_lines(lines: &[Line<'static>], width: u16, height: u16) -> Self {
        let grid = lines_to_grid(lines, width, height);
        let cells = grid
            .cells
            .into_iter()
            .map(|cell| CachedCell {
                symbol: cell.symbol.to_string(),
                style: CachedStyle::from_style(cell.style),
            })
            .collect();
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn to_lines(&self) -> Vec<Line<'static>> {
        let mut grid = crate::render::compositor::Grid::new(self.width, self.height);
        for (index, cached) in self.cells.iter().enumerate() {
            if let Some(cell) = grid.cells.get_mut(index) {
                cell.symbol = cached.symbol.chars().next().unwrap_or(' ');
                cell.style = cached.style.to_style();
            }
        }
        grid_to_lines(&grid)
    }

    /// True when every cell is a blank space with no styling: the shape a
    /// freshly constructed `Grid` starts in. A package/cache frame matching
    /// this is indistinguishable from "never written to" and should not be
    /// treated as valid hero output.
    pub fn is_blank(&self) -> bool {
        self.cells
            .iter()
            .all(|cell| cell.symbol == " " && cell.style == CachedStyle::default())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CachedCell {
    pub symbol: String,
    pub style: CachedStyle,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CachedStyle {
    pub fg: Option<CachedColor>,
    pub bg: Option<CachedColor>,
    pub add_modifier: u16,
    pub sub_modifier: u16,
}

impl CachedStyle {
    fn from_style(style: Style) -> Self {
        Self {
            fg: style.fg.map(CachedColor::from_color),
            bg: style.bg.map(CachedColor::from_color),
            add_modifier: style.add_modifier.bits(),
            sub_modifier: style.sub_modifier.bits(),
        }
    }

    fn to_style(&self) -> Style {
        Style {
            fg: self.fg.as_ref().map(CachedColor::to_color),
            bg: self.bg.as_ref().map(CachedColor::to_color),
            add_modifier: Modifier::from_bits_truncate(self.add_modifier),
            sub_modifier: Modifier::from_bits_truncate(self.sub_modifier),
            ..Style::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CachedColor {
    Reset,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl CachedColor {
    fn from_color(color: Color) -> Self {
        match color {
            Color::Reset => Self::Reset,
            Color::Rgb(r, g, b) => Self::Rgb(r, g, b),
            Color::Indexed(index) => Self::Indexed(index),
            Color::Black => Self::Indexed(0),
            Color::Red => Self::Indexed(1),
            Color::Green => Self::Indexed(2),
            Color::Yellow => Self::Indexed(3),
            Color::Blue => Self::Indexed(4),
            Color::Magenta => Self::Indexed(5),
            Color::Cyan => Self::Indexed(6),
            Color::Gray => Self::Indexed(7),
            Color::DarkGray => Self::Indexed(8),
            Color::LightRed => Self::Indexed(9),
            Color::LightGreen => Self::Indexed(10),
            Color::LightYellow => Self::Indexed(11),
            Color::LightBlue => Self::Indexed(12),
            Color::LightMagenta => Self::Indexed(13),
            Color::LightCyan => Self::Indexed(14),
            Color::White => Self::Indexed(15),
        }
    }

    fn to_color(&self) -> Color {
        match self {
            Self::Reset => Color::Reset,
            Self::Rgb(r, g, b) => Color::Rgb(*r, *g, *b),
            Self::Indexed(index) => Color::Indexed(*index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CachedStyle, CellGrid};
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};

    #[test]
    fn cell_grid_round_trips_symbol_and_style() {
        let lines = vec![Line::from(vec![
            Span::styled(
                "AB",
                Style::default()
                    .fg(Color::Rgb(114, 22, 15))
                    .bg(Color::Rgb(16, 1, 0))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ", Style::default()),
        ])];

        let grid = CellGrid::from_lines(&lines, 4, 1);
        let round_trip = grid.to_lines();

        assert_eq!(round_trip[0].spans[0].content.as_ref(), "AB");
        assert_eq!(
            round_trip[0].spans[0].style.fg,
            Some(Color::Rgb(114, 22, 15))
        );
        assert_eq!(round_trip[0].spans[0].style.bg, Some(Color::Rgb(16, 1, 0)));
        assert!(round_trip[0].spans[0]
            .style
            .add_modifier
            .contains(Modifier::BOLD));
    }

    #[test]
    fn freshly_built_grid_is_blank() {
        let grid = CellGrid::from_lines(&[], 4, 2);
        assert!(grid.is_blank());
    }

    #[test]
    fn styled_grid_is_not_blank() {
        let lines = vec![Line::from(vec![Span::styled(
            "X",
            Style::default().fg(Color::Rgb(1, 2, 3)),
        )])];
        let grid = CellGrid::from_lines(&lines, 1, 1);
        assert!(!grid.is_blank());
    }

    #[test]
    fn default_cached_style_matches_blank_style() {
        // Guards the invariant `is_blank()` relies on: a cell that was never
        // written to serializes to `CachedStyle::default()`.
        assert_eq!(
            CachedStyle::from_style(Style::default()),
            CachedStyle::default()
        );
    }
}
