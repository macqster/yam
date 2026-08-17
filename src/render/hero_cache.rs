#![allow(dead_code)]

use std::{fs, io, path::Path};

use ratatui::text::Line;
use serde::{Deserialize, Serialize};

use crate::render::cell_grid::CellGrid;

/// The disposable, runtime-local hero frame cache. This is the "user
/// cache" owner layer in docs/hero-revision.md's pipeline table: it exists
/// purely to skip GIF-decode/chafa-spawn cost on ordinary startup and is
/// never the authority for what the hero should look like. The validated,
/// versioned authority is `render::hero_package::HeroPackage`; this type
/// intentionally carries no manifest/provenance fields.
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

pub fn load_hero_frame_set(path: &Path) -> io::Result<HeroFrameSet> {
    let json = fs::read_to_string(path)?;
    serde_json::from_str(&json).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

pub fn save_hero_frame_set(path: &Path, frame_set: &HeroFrameSet) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    // Compact, not pretty: this file is machine-only and large (tens of MB for
    // a full 96x48 frame set), so indentation would be a substantial share of
    // it for no reader benefit.
    let json = serde_json::to_string(frame_set)
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
