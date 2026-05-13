use ratatui::style::Style;
use ratatui::text::{Line, Span};
use unicode_width::UnicodeWidthStr;

use crate::theme::style as theme_style;

use super::atlas::compact_sprite_for;
use super::layout::lines_for_snapshot;
use super::model::{WeatherLocale, WeatherSnapshot, WeatherVisual};
use super::text::visual_label;

pub const COMPACT_WEATHER_WIDTH: usize = 30;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeatherColorRole {
    CloudEdge,
    CloudAccent,
    CloudShadow,
    Text,
    TextDim,
    Fog,
    Rain,
    RainHeavy,
    Snow,
    SunCore,
    SunRay,
    Lightning,
    Alert,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WeatherSpriteSpan {
    pub text: &'static str,
    pub role: WeatherColorRole,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeatherSpriteLine {
    pub spans: Vec<WeatherSpriteSpan>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeatherSprite {
    pub width: u16,
    pub height: u16,
    pub lines: Vec<WeatherSpriteLine>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WeatherLayout {
    #[default]
    WttrCompact,
}

pub fn compact_widget_lines(
    snapshot: &WeatherSnapshot,
    locale: WeatherLocale,
    layout: WeatherLayout,
) -> Vec<Line<'static>> {
    lines_for_snapshot(snapshot, locale, layout)
}

pub fn line_width(line: &Line<'_>) -> usize {
    line.spans
        .iter()
        .map(|span| UnicodeWidthStr::width(span.content.as_ref()))
        .sum()
}

pub fn role_style(role: WeatherColorRole) -> Style {
    match role {
        WeatherColorRole::CloudEdge => theme_style::weather_cloud_edge(),
        WeatherColorRole::CloudAccent => theme_style::weather_cloud_accent(),
        WeatherColorRole::CloudShadow => theme_style::weather_cloud_shadow(),
        WeatherColorRole::Text => theme_style::weather_text(),
        WeatherColorRole::TextDim => theme_style::weather_text_dim(),
        WeatherColorRole::Fog => theme_style::weather_fog(),
        WeatherColorRole::Rain => theme_style::weather_rain(),
        WeatherColorRole::RainHeavy => theme_style::weather_rain_heavy(),
        WeatherColorRole::Snow => theme_style::weather_snow(),
        WeatherColorRole::SunCore => theme_style::weather_sun_core(),
        WeatherColorRole::SunRay => theme_style::weather_sun_ray(),
        WeatherColorRole::Lightning => theme_style::weather_lightning(),
        WeatherColorRole::Alert => theme_style::weather_alert(),
    }
}

pub fn sprite_inspection_lines(locale: WeatherLocale) -> Vec<Line<'static>> {
    let visuals = [
        WeatherVisual::Sunny,
        WeatherVisual::ClearNight,
        WeatherVisual::PartlyCloudy,
        WeatherVisual::Cloudy,
        WeatherVisual::VeryCloudy,
        WeatherVisual::Overcast,
        WeatherVisual::Mist,
        WeatherVisual::Fog,
        WeatherVisual::LightShowers,
        WeatherVisual::LightRain,
        WeatherVisual::HeavyShowers,
        WeatherVisual::HeavyRain,
        WeatherVisual::LightSnow,
        WeatherVisual::HeavySnow,
        WeatherVisual::LightSnowShowers,
        WeatherVisual::HeavySnowShowers,
        WeatherVisual::LightSleet,
        WeatherVisual::LightSleetShowers,
        WeatherVisual::Sleet,
        WeatherVisual::ThunderyShowers,
        WeatherVisual::ThunderyHeavyRain,
        WeatherVisual::ThunderySnowShowers,
        WeatherVisual::Thunderstorm,
        WeatherVisual::Unknown,
    ];
    let block_width = 15usize;
    let block_height = 6usize;
    let columns = 6usize;
    let gutter = 2usize;
    let mut lines = Vec::new();

    lines.push(Line::from(vec![Span::styled(
        "weather sprites",
        theme_style::weather_text(),
    )]));
    lines.push(Line::from(vec![Span::styled(
        "dev popup atlas review",
        theme_style::weather_text_dim(),
    )]));
    lines.push(Line::from(vec![Span::styled("", Style::default())]));

    for row_visuals in visuals.chunks(columns) {
        for inner_row in 0..block_height {
            let mut spans = Vec::new();
            for (index, visual) in row_visuals.iter().copied().enumerate() {
                if index > 0 {
                    spans.push(Span::styled(" ".repeat(gutter), Style::default()));
                }
                spans.extend(sprite_table_block(visual, locale, inner_row, block_width));
            }
            lines.push(Line::from(spans));
        }
        lines.push(Line::from(vec![Span::styled("", Style::default())]));
    }

    lines.pop();

    lines
}

fn sprite_table_block(
    visual: WeatherVisual,
    locale: WeatherLocale,
    row: usize,
    block_width: usize,
) -> Vec<Span<'static>> {
    if row == 0 {
        return pad_line(
            Span::styled(
                truncate_label(visual_label(visual, locale), block_width),
                theme_style::weather_text(),
            ),
            block_width,
        );
    }
    let sprite = compact_sprite_for(visual);
    let sprite_row = row - 1;
    if let Some(line) = sprite.lines.get(sprite_row) {
        let mut spans = line
            .spans
            .iter()
            .map(|span| Span::styled(span.text, role_style(span.role)))
            .collect::<Vec<_>>();
        let used_width = line
            .spans
            .iter()
            .map(|span| UnicodeWidthStr::width(span.text))
            .sum::<usize>();
        if used_width < block_width {
            spans.push(Span::styled(
                " ".repeat(block_width - used_width),
                Style::default(),
            ));
        }
        return spans;
    }

    vec![Span::styled(" ".repeat(block_width), Style::default())]
}

fn pad_line(span: Span<'static>, block_width: usize) -> Vec<Span<'static>> {
    let width = UnicodeWidthStr::width(span.content.as_ref());
    let mut spans = vec![span];
    if width < block_width {
        spans.push(Span::styled(
            " ".repeat(block_width - width),
            Style::default(),
        ));
    }
    spans
}

fn truncate_label(label: &str, block_width: usize) -> String {
    let mut out = String::new();
    for ch in label.chars() {
        let next = format!("{out}{ch}");
        if UnicodeWidthStr::width(next.as_str()) > block_width {
            break;
        }
        out.push(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{compact_widget_lines, line_width, WeatherLayout, COMPACT_WEATHER_WIDTH};
    use crate::weather::model::{WeatherLocale, WeatherLocation};
    use crate::weather::provider::{StaticWeatherProvider, WeatherProvider};

    #[test]
    fn compact_weather_widget_stays_within_clock_scale_width() {
        let provider = StaticWeatherProvider;
        let snapshot = provider
            .snapshot(&WeatherLocation::named("Sulkowice"))
            .expect("static weather provider should always return a snapshot");
        let lines = compact_widget_lines(&snapshot, WeatherLocale::En, WeatherLayout::WttrCompact);

        assert_eq!(lines.len(), 5);
        assert!(lines
            .iter()
            .all(|line| line_width(line) <= COMPACT_WEATHER_WIDTH));
    }
}
