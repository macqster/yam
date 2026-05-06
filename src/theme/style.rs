use ratatui::style::{Modifier, Style};

use super::btas::BTAS;
use super::palette;
use crate::core::flora::VineHealth;

#[allow(dead_code)]
pub fn accent_border() -> Style {
    Style::default().fg(palette::ACCENT)
}

pub fn panel_text() -> Style {
    BTAS.panel_text()
}

pub fn modal_panel() -> Style {
    BTAS.modal_panel()
}

pub fn hero_overlay() -> Style {
    BTAS.hero_overlay()
}

pub fn camera_indicator_track() -> Style {
    BTAS.camera_indicator_track()
}

pub fn camera_indicator_thumb() -> Style {
    BTAS.camera_indicator_thumb()
}

pub fn pointer_probe() -> Style {
    BTAS.pointer_probe()
}

pub fn footer_text() -> Style {
    BTAS.footer_text()
}

pub fn debug_text() -> Style {
    BTAS.debug_text()
}

pub fn settings_selected_row() -> Style {
    Style::default()
        .fg(palette::PRIMARY_FG)
        .bg(palette::CAMERA_THUMB)
}

pub fn settings_active_field() -> Style {
    Style::default()
        .fg(palette::PANEL_BG)
        .bg(palette::MARKER)
        .add_modifier(Modifier::BOLD)
}

pub fn settings_disabled_row() -> Style {
    Style::default()
        .fg(palette::BTAS_GREY_DISABLED)
        .bg(palette::MODAL_BG)
        .add_modifier(Modifier::DIM)
}

pub fn settings_disabled_row_selected() -> Style {
    Style::default()
        .fg(palette::BTAS_GREY_DISABLED)
        .bg(palette::CAMERA_THUMB)
        .add_modifier(Modifier::DIM)
}

pub fn guide_trace() -> Style {
    BTAS.guide_trace()
}

pub fn vine_stem(health: VineHealth) -> Style {
    BTAS.vine_stem(matches!(health, VineHealth::Healthy))
}
