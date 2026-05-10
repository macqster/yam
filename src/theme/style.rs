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

pub fn modal_border() -> Style {
    Style::default()
        .bg(palette::MODAL_BG)
        .fg(palette::MODAL_BORDER)
}

pub fn modal_footer_symbol() -> Style {
    Style::default()
        .bg(palette::MODAL_BG)
        .fg(palette::MODAL_FOOTER_SYMBOL)
}

pub fn settings_tab_inactive() -> Style {
    Style::default()
        .bg(palette::MODAL_BG)
        .fg(palette::TAB_INACTIVE)
}

pub fn settings_tab_active() -> Style {
    Style::default()
        .bg(palette::MODAL_BG)
        .fg(palette::TAB_ACTIVE)
}

pub fn loading_text() -> Style {
    Style::default().fg(palette::PRIMARY_FG)
}

pub fn clock_text() -> Style {
    Style::default().fg(palette::PRIMARY_FG)
}

pub fn weather_text() -> Style {
    Style::default().fg(palette::PRIMARY_FG)
}

pub fn weather_text_dim() -> Style {
    Style::default().fg(palette::SECONDARY_FG)
}

pub fn weather_cloud_edge() -> Style {
    Style::default().fg(palette::SECONDARY_FG)
}

pub fn weather_cloud_accent() -> Style {
    Style::default().fg(palette::MODAL_FOOTER_SYMBOL)
}

pub fn weather_cloud_shadow() -> Style {
    Style::default().fg(palette::DIVIDER)
}

pub fn weather_fog() -> Style {
    Style::default().fg(palette::DIVIDER)
}

pub fn weather_rain() -> Style {
    Style::default().fg(palette::WEATHER_RAIN)
}

pub fn weather_rain_heavy() -> Style {
    Style::default().fg(palette::WEATHER_RAIN_HEAVY)
}

pub fn weather_snow() -> Style {
    Style::default().fg(palette::PRIMARY_FG)
}

pub fn weather_sun_core() -> Style {
    Style::default().fg(palette::WEATHER_SUN_CORE)
}

pub fn weather_sun_ray() -> Style {
    Style::default().fg(palette::WEATHER_SUN_RAY)
}

pub fn weather_lightning() -> Style {
    Style::default().fg(palette::WEATHER_LIGHTNING)
}

pub fn weather_alert() -> Style {
    Style::default().fg(palette::WEATHER_ALERT)
}

pub fn loading_prompt(pulse: f32) -> Style {
    let pulse = pulse.clamp(0.0, 1.0);
    let fg = if pulse < 0.6 {
        lerp_rgb(palette::PRIMARY_FG, palette::SECONDARY_FG, pulse / 0.6)
    } else {
        lerp_rgb(
            palette::SECONDARY_FG,
            palette::VINE_HEALTHY,
            (pulse - 0.6) / 0.4,
        )
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
    Style::default().fg(palette::SECONDARY_FG)
}

pub fn debug_text() -> Style {
    Style::default().fg(palette::DEBUG_FG)
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
    Style::default().fg(palette::SECONDARY_FG)
}

pub fn vine_stem(health: VineHealth) -> Style {
    if matches!(health, VineHealth::Healthy) {
        Style::default().fg(palette::VINE_HEALTHY)
    } else {
        Style::default().fg(palette::VINE_AGED)
    }
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
