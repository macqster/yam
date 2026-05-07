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

pub fn loading_text() -> Style {
    Style::default().fg(palette::PRIMARY_FG)
}

pub fn loading_prompt(pulse: f32) -> Style {
    let pulse = pulse.clamp(0.0, 1.0);
    let fg = if pulse < 0.6 {
        lerp_rgb(palette::PRIMARY_FG, BTAS.pms_317, pulse / 0.6)
    } else {
        lerp_rgb(BTAS.pms_317, BTAS.pms_345, (pulse - 0.6) / 0.4)
    };
    Style::default().fg(fg)
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
        .bg(palette::CAMERA_TRACK)
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
        .bg(palette::CAMERA_TRACK)
        .add_modifier(Modifier::DIM)
}

pub fn guide_trace() -> Style {
    BTAS.guide_trace()
}

pub fn vine_stem(health: VineHealth) -> Style {
    BTAS.vine_stem(matches!(health, VineHealth::Healthy))
}

fn lerp_rgb(
    from: ratatui::style::Color,
    to: ratatui::style::Color,
    t: f32,
) -> ratatui::style::Color {
    match (from, to) {
        (ratatui::style::Color::Rgb(fr, fg, fb), ratatui::style::Color::Rgb(tr, tg, tb)) => {
            let lerp =
                |a: u8, b: u8| -> u8 { (a as f32 + (b as f32 - a as f32) * t).round() as u8 };
            ratatui::style::Color::Rgb(lerp(fr, tr), lerp(fg, tg), lerp(fb, tb))
        }
        _ => to,
    }
}
