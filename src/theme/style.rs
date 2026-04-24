use ratatui::style::Style;

use super::palette;

pub fn accent_border() -> Style {
    Style::default().fg(palette::ACCENT)
}

pub fn panel_text() -> Style {
    Style::default().bg(palette::PANEL_BG).fg(palette::PRIMARY_FG)
}

pub fn hero_overlay() -> Style {
    Style::default().fg(palette::PRIMARY_FG).bg(palette::HERO_BG)
}
