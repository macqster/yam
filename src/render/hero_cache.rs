#![allow(dead_code)]

use std::{fs, io, path::Path};

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use serde::{Deserialize, Serialize};

use crate::render::compositor::{grid_to_lines, lines_to_grid};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeroFrameSet {
    pub render_width: u16,
    pub render_height: u16,
    pub frames: Vec<CellGrid>,
}

impl HeroFrameSet {
    pub fn from_lines(
        render_width: u16,
        render_height: u16,
        frames: &[Vec<Line<'static>>],
    ) -> Self {
        let frames = frames
            .iter()
            .map(|frame| CellGrid::from_lines(frame, render_width, render_height))
            .collect();
        Self {
            render_width,
            render_height,
            frames,
        }
    }

    pub fn to_lines(&self) -> Vec<Vec<Line<'static>>> {
        self.frames.iter().map(CellGrid::to_lines).collect()
    }
}

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

pub fn load_hero_frame_set(path: &Path) -> io::Result<HeroFrameSet> {
    let json = fs::read_to_string(path)?;
    serde_json::from_str(&json).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

pub fn save_hero_frame_set(path: &Path, frame_set: &HeroFrameSet) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(frame_set)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    fs::write(path, json)
}

#[cfg(test)]
mod tests {
    use super::{load_hero_frame_set, save_hero_frame_set, HeroFrameSet};
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};
    use tempfile::tempdir;

    #[test]
    fn hero_frame_set_round_trips_lines_without_losing_shape_or_style() {
        let frames = vec![vec![
            Line::from(vec![
                Span::styled(
                    "AB",
                    Style::default()
                        .fg(Color::Rgb(114, 22, 15))
                        .bg(Color::Rgb(16, 1, 0))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  ", Style::default()),
            ]),
            Line::from(vec![Span::styled(
                "CD  ",
                Style::default().fg(Color::Indexed(14)),
            )]),
        ]];

        let frame_set = HeroFrameSet::from_lines(4, 2, &frames);
        let round_trip = frame_set.to_lines();

        assert_eq!(round_trip.len(), 1);
        assert_eq!(round_trip[0].len(), 2);
        assert_eq!(round_trip[0][0].spans[0].content.as_ref(), "AB");
        assert_eq!(
            round_trip[0][0].spans[0].style.fg,
            Some(Color::Rgb(114, 22, 15))
        );
        assert_eq!(
            round_trip[0][0].spans[0].style.bg,
            Some(Color::Rgb(16, 1, 0))
        );
        assert!(round_trip[0][0].spans[0]
            .style
            .add_modifier
            .contains(Modifier::BOLD));
        assert_eq!(round_trip[0][1].spans[0].style.fg, Some(Color::Indexed(14)));
    }

    #[test]
    fn hero_frame_set_serializes_and_loads_from_disk() {
        let frames = vec![vec![Line::from(vec![Span::styled(
            "stub",
            Style::default().fg(Color::Rgb(220, 216, 203)),
        )])]];
        let frame_set = HeroFrameSet::from_lines(4, 1, &frames);
        let dir = tempdir().expect("temp dir should exist");
        let path = dir.path().join("hero-frames.json");

        save_hero_frame_set(&path, &frame_set).expect("frame set should save");
        let loaded = load_hero_frame_set(&path).expect("frame set should load");

        assert_eq!(loaded, frame_set);
    }
}
