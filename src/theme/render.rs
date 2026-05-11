use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use super::palette;
use super::style;

const CURATED_CARD_WIDTH: usize = 28;
const CURATED_SWATCH_WIDTH: usize = 12;
const CURATED_GUTTER: &str = "   ";

const SOURCE_CARD_WIDTH: usize = 21;
const SOURCE_SWATCH_WIDTH: usize = 8;
const SOURCE_GUTTER: &str = "  ";

#[derive(Clone, Copy)]
struct PaletteEntry {
    label: &'static str,
    hex: &'static str,
    color: Color,
}

pub fn palette_popup_lines() -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    lines.extend(section_heading("curated workstation palette"));
    append_rows(
        &mut lines,
        &curated_rows(),
        CURATED_CARD_WIDTH,
        CURATED_SWATCH_WIDTH,
        CURATED_GUTTER,
    );

    lines.push(Line::from(vec![Span::raw("")]));
    lines.extend(section_heading("extracted source swatches"));
    append_rows(
        &mut lines,
        &source_rows(),
        SOURCE_CARD_WIDTH,
        SOURCE_SWATCH_WIDTH,
        SOURCE_GUTTER,
    );

    lines
}

fn curated_rows() -> Vec<Vec<PaletteEntry>> {
    vec![
        vec![
            entry("Background", "#16181A", Color::Rgb(22, 24, 26)),
            entry("Sidebar", "#181A1D", Color::Rgb(24, 26, 29)),
            entry("Panel", "#1E2124", palette::PANEL_BG),
            entry("Lifted", "#262A2E", palette::SURFACE_LIFTED),
        ],
        vec![
            entry("Divider", "#373C41", palette::DIVIDER),
            entry("Prim text", "#E6E8EB", palette::PRIMARY_FG),
            entry("Sec text", "#A0A6AD", palette::SECONDARY_FG),
            entry("Deep Green", "#1F3B2C", Color::Rgb(31, 59, 44)),
        ],
        vec![
            entry("Soft Green", "#2F5A45", palette::MODAL_BORDER),
            entry("Balanced", "#3E7357", palette::DEBUG_FG),
            entry("Root Green", "#4F8E6C", palette::VINE_HEALTHY),
            entry("Bright Grn", "#69C090", Color::Rgb(105, 192, 144)),
        ],
        vec![
            entry("Deep Blue", "#1E2F5C", Color::Rgb(30, 47, 92)),
            entry("Subtle Nvy", "#243B73", palette::CAMERA_TRACK),
            entry("Balanced", "#2C4C8A", palette::MODAL_FOOTER_SYMBOL),
            entry("Bright Blue", "#355FA8", palette::CAMERA_THUMB),
        ],
        vec![
            entry("Highlight", "#4B7BD6", palette::WEATHER_RAIN_HEAVY),
            entry("Auburn", "#8A3B2E", Color::Rgb(138, 59, 46)),
            entry("Brick Red", "#A33A32", palette::TAB_ACTIVE),
            entry("Rust", "#B24E2E", palette::TAB_INACTIVE),
        ],
        vec![
            entry("Ochre", "#C48A2C", palette::MARKER),
            entry("Dsty Amber", "#A67C34", palette::WEATHER_SUN_RAY),
        ],
    ]
}

fn source_rows() -> Vec<Vec<PaletteEntry>> {
    vec![
        vec![
            entry("White", "#FEFEFC", Color::Rgb(254, 254, 252)),
            entry("PMS 113", "#FDD309", Color::Rgb(253, 211, 9)),
            entry("PMS 142", "#FCAC11", Color::Rgb(252, 172, 17)),
            entry("PMS 131", "#E78919", Color::Rgb(231, 137, 25)),
        ],
        vec![
            entry("PMS 179", "#DB1817", Color::Rgb(219, 24, 23)),
            entry("PMS 485", "#D6081B", palette::WEATHER_ALERT),
            entry("PMS 200", "#B00D19", Color::Rgb(176, 13, 25)),
            entry("PMS 197", "#F17986", Color::Rgb(241, 121, 134)),
            entry("PMS 228", "#7B1637", Color::Rgb(123, 22, 55)),
        ],
        vec![
            entry("PMS 272", "#665C9D", Color::Rgb(102, 92, 157)),
            entry("PMS 293", "#194B8F", Color::Rgb(25, 75, 143)),
            entry("PMS 292", "#3386B7", palette::WEATHER_RAIN),
            entry("PMS 551", "#76AFB8", Color::Rgb(118, 175, 184)),
        ],
        vec![
            entry("Proc Cyan", "#02A1CC", Color::Rgb(2, 161, 204)),
            entry("PMS 319", "#5BBC8C", Color::Rgb(91, 188, 140)),
            entry("PMS 317", "#C3DDC3", Color::Rgb(195, 221, 195)),
            entry("PMS 345", "#A5CB61", Color::Rgb(165, 203, 97)),
            entry("PMS 340", "#086419", palette::VINE_HEALTHY),
        ],
        vec![
            entry("PMS 156", "#FCAF48", Color::Rgb(252, 175, 72)),
            entry("PMS 480", "#D79E64", Color::Rgb(215, 158, 100)),
            entry("PMS 465", "#CD8B2D", palette::VINE_AGED),
            entry("PMS 160", "#912017", Color::Rgb(145, 32, 23)),
            entry("PMS 161", "#4E1B23", Color::Rgb(78, 27, 35)),
        ],
        vec![
            entry("Black", "#080509", Color::Rgb(8, 5, 9)),
            entry("PMS 432", "#212B31", Color::Rgb(33, 43, 49)),
            entry("PMS 430", "#727C76", Color::Rgb(114, 124, 118)),
            entry("PMS 427", "#DCD8CB", Color::Rgb(220, 216, 203)),
            entry("PMS 441", "#C9CDB1", Color::Rgb(201, 205, 177)),
        ],
    ]
}

fn section_heading(title: &'static str) -> [Line<'static>; 1] {
    [Line::from(vec![Span::styled(title, style::weather_text())])]
}

fn append_rows(
    lines: &mut Vec<Line<'static>>,
    rows: &[Vec<PaletteEntry>],
    card_width: usize,
    swatch_width: usize,
    gutter: &'static str,
) {
    for row in rows {
        lines.push(card_swatch_line(row, card_width, swatch_width, gutter));
        lines.push(card_label_line(row, card_width, gutter));
    }
}

const fn entry(label: &'static str, hex: &'static str, color: Color) -> PaletteEntry {
    PaletteEntry { label, hex, color }
}

fn card_swatch_line(
    entries: &[PaletteEntry],
    card_width: usize,
    swatch_width: usize,
    gutter: &'static str,
) -> Line<'static> {
    let mut spans = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            spans.push(Span::raw(gutter));
        }
        spans.extend(card_swatches(entry, card_width, swatch_width));
    }
    Line::from(spans)
}

fn card_label_line(
    entries: &[PaletteEntry],
    card_width: usize,
    gutter: &'static str,
) -> Line<'static> {
    let mut spans = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            spans.push(Span::raw(gutter));
        }
        let text = format!("{:<9} {}", entry.label, entry.hex);
        spans.push(Span::styled(
            format!("{text:<card_width$}"),
            style::weather_text(),
        ));
    }
    Line::from(spans)
}

fn card_swatches(
    entry: &PaletteEntry,
    card_width: usize,
    swatch_width: usize,
) -> Vec<Span<'static>> {
    let mut spans = vec![
        Span::styled("[", style::weather_text_dim()),
        Span::styled(" ".repeat(swatch_width), Style::default().bg(entry.color)),
        Span::styled("]", style::weather_text_dim()),
    ];
    let used_width = 2 + swatch_width;
    if used_width < card_width {
        spans.push(Span::raw(" ".repeat(card_width - used_width)));
    }
    spans
}
