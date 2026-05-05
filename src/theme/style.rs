use ratatui::style::Style;

use super::btas::BTAS;
use super::palette;

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

pub fn guide_trace() -> Style {
    BTAS.guide_trace()
}
