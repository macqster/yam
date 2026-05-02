use ratatui::style::Modifier;
use ratatui::style::Style;

use super::palette;

#[allow(dead_code)]
pub fn accent_border() -> Style {
    Style::default().fg(palette::ACCENT)
}

pub fn panel_text() -> Style {
    Style::default()
        .bg(palette::PANEL_BG)
        .fg(palette::PRIMARY_FG)
}

pub fn modal_panel() -> Style {
    Style::default()
        .bg(palette::MODAL_BG)
        .fg(palette::PRIMARY_FG)
}

pub fn hero_overlay() -> Style {
    Style::default()
        .fg(palette::PRIMARY_FG)
        .bg(palette::HERO_BG)
}

pub fn footer_bar() -> Style {
    Style::default()
        .bg(palette::FOOTER_BG)
        .fg(palette::FOOTER_FG)
}

pub fn camera_indicator_track() -> Style {
    Style::default()
        .fg(palette::CAMERA_TRACK)
        .add_modifier(Modifier::DIM)
}

pub fn camera_indicator_thumb() -> Style {
    Style::default()
        .fg(palette::CAMERA_THUMB)
        .add_modifier(Modifier::BOLD)
}

pub fn pointer_probe() -> Style {
    Style::default()
        .fg(palette::POINTER_PROBE)
        .add_modifier(Modifier::BOLD)
}
