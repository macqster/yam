use ratatui::style::Style;
use ratatui::text::{Line, Span};
use unicode_width::UnicodeWidthStr;

use crate::theme::style as theme_style;

use super::atlas::compact_sprite_for;
use super::model::{WeatherLocale, WeatherSnapshot};
use super::render::{
    role_style, WeatherLayout, WeatherSprite, WeatherSpriteLine, COMPACT_WEATHER_WIDTH,
};
use super::text::{
    condition_label, wttr_day_night_temperature_label, wttr_precip_label, wttr_wind_label,
};

pub fn lines_for_snapshot(
    snapshot: &WeatherSnapshot,
    locale: WeatherLocale,
    layout: WeatherLayout,
) -> Vec<Line<'static>> {
    match layout {
        WeatherLayout::WttrCompact => wttr_compact_lines(snapshot, locale),
    }
}

fn wttr_compact_lines(snapshot: &WeatherSnapshot, locale: WeatherLocale) -> Vec<Line<'static>> {
    let sprite = compact_sprite_for(snapshot.visual);
    let (condition_top, condition_bottom) =
        split_condition_rows(&condition_label(snapshot, locale));
    let temperature = wttr_day_night_temperature_label(snapshot);
    let wind = wttr_wind_label(snapshot);
    let precip = wttr_precip_label(snapshot);
    let info_rows = [condition_top, condition_bottom, temperature, wind, precip];

    sprite
        .lines
        .iter()
        .zip(info_rows)
        .map(|(sprite_line, info)| compose_compact_line(&sprite, sprite_line, &info))
        .collect()
}

fn split_condition_rows(condition: &str) -> (String, String) {
    let max_width = 13usize;
    let words = condition.split_whitespace().collect::<Vec<_>>();
    if words.is_empty() {
        return (String::new(), String::new());
    }

    let mut first = String::new();
    let mut second = String::new();

    for word in words {
        let candidate = if first.is_empty() {
            word.to_string()
        } else {
            format!("{first} {word}")
        };
        if UnicodeWidthStr::width(candidate.as_str()) <= max_width {
            first = candidate;
            continue;
        }

        let candidate = if second.is_empty() {
            word.to_string()
        } else {
            format!("{second} {word}")
        };
        second = truncate_to_width(&candidate, max_width);
    }

    if first.is_empty() {
        return (truncate_to_width(condition, max_width), String::new());
    }

    (
        truncate_to_width(&first, max_width),
        truncate_to_width(&second, max_width),
    )
}

fn truncate_to_width(text: &str, max_width: usize) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        let candidate = format!("{out}{ch}");
        if UnicodeWidthStr::width(candidate.as_str()) > max_width {
            break;
        }
        out.push(ch);
    }
    out.trim_end().to_string()
}

fn compose_compact_line(
    sprite: &WeatherSprite,
    sprite_line: &WeatherSpriteLine,
    info: &str,
) -> Line<'static> {
    let mut spans = sprite_line
        .spans
        .iter()
        .map(|span| Span::styled(span.text, role_style(span.role)))
        .collect::<Vec<_>>();
    let used_sprite_width = sprite_line
        .spans
        .iter()
        .map(|span| UnicodeWidthStr::width(span.text))
        .sum::<usize>();
    let sprite_column_width = sprite.width as usize;
    let gap = "  ";
    if used_sprite_width < sprite_column_width {
        spans.push(Span::styled(
            " ".repeat(sprite_column_width - used_sprite_width),
            Style::default(),
        ));
    }
    let info_width = UnicodeWidthStr::width(info);
    let content_width = sprite_column_width + gap.len() + info_width;
    let padding = COMPACT_WEATHER_WIDTH.saturating_sub(content_width);

    spans.push(Span::styled(gap, theme_style::weather_text_dim()));
    spans.push(Span::styled(info.to_string(), theme_style::weather_text()));
    if padding > 0 {
        spans.push(Span::styled(" ".repeat(padding), Style::default()));
    }

    Line::from(spans)
}

#[cfg(test)]
mod tests {
    use super::lines_for_snapshot;
    use crate::weather::model::{WeatherLocale, WeatherSnapshot, WeatherSource, WeatherVisual};
    use crate::weather::render::{line_width, WeatherLayout, COMPACT_WEATHER_WIDTH};
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn compact_layout_supports_polish_and_stays_within_width() {
        let snapshot = WeatherSnapshot {
            location_label: "Sulkowice".to_string(),
            observed_at: chrono::Utc::now(),
            temperature_c: Some(10.0),
            feels_like_c: Some(8.0),
            day_max_c: Some(18.0),
            night_min_c: Some(9.0),
            humidity_pct: Some(76),
            wind_kph: Some(4.0),
            wind_dir: Some("E".to_string()),
            visibility_km: Some(10.0),
            precip_mm: Some(0.0),
            condition_text: Some("Overcast".to_string()),
            condition_code: None,
            forecast: Vec::new(),
            source: WeatherSource::StaticPrototype,
            stale: false,
            visual: WeatherVisual::Overcast,
        };

        let lines = lines_for_snapshot(&snapshot, WeatherLocale::Pl, WeatherLayout::WttrCompact);

        assert_eq!(lines.len(), 5);
        assert!(lines
            .iter()
            .all(|line| line_width(line) <= COMPACT_WEATHER_WIDTH));
    }

    #[test]
    fn compact_layout_keeps_weather_facts_in_a_fixed_second_column() {
        let snapshot = WeatherSnapshot {
            location_label: "Sulkowice".to_string(),
            observed_at: chrono::Utc::now(),
            temperature_c: Some(10.0),
            feels_like_c: Some(10.0),
            day_max_c: Some(18.0),
            night_min_c: Some(9.0),
            humidity_pct: Some(76),
            wind_kph: Some(4.0),
            wind_dir: Some("ESE".to_string()),
            visibility_km: Some(10.0),
            precip_mm: Some(0.0),
            condition_text: Some("Overcast".to_string()),
            condition_code: None,
            forecast: Vec::new(),
            source: WeatherSource::StaticPrototype,
            stale: false,
            visual: WeatherVisual::Overcast,
        };

        let lines = lines_for_snapshot(&snapshot, WeatherLocale::En, WeatherLayout::WttrCompact);
        let info_start_columns = lines
            .iter()
            .map(|line| {
                let gap_index = line
                    .spans
                    .iter()
                    .position(|span| span.content.as_ref() == "  ")
                    .expect("compact weather rows should include a column gap");
                line.spans[..gap_index]
                    .iter()
                    .map(|span| UnicodeWidthStr::width(span.content.as_ref()))
                    .sum::<usize>()
                    + 2
            })
            .collect::<Vec<_>>();

        assert!(info_start_columns.windows(2).all(|pair| pair[0] == pair[1]));
    }

    #[test]
    fn compact_layout_wraps_long_polish_condition_over_two_rows() {
        let snapshot = WeatherSnapshot {
            location_label: "Sulkowice".to_string(),
            observed_at: chrono::Utc::now(),
            temperature_c: Some(10.0),
            feels_like_c: Some(8.0),
            day_max_c: Some(18.0),
            night_min_c: Some(9.0),
            humidity_pct: Some(76),
            wind_kph: Some(4.0),
            wind_dir: Some("E".to_string()),
            visibility_km: Some(10.0),
            precip_mm: Some(0.0),
            condition_text: Some("Thunder".to_string()),
            condition_code: Some(389),
            forecast: Vec::new(),
            source: WeatherSource::WttrIn,
            stale: false,
            visual: WeatherVisual::ThunderyHeavyRain,
        };

        let lines = lines_for_snapshot(&snapshot, WeatherLocale::Pl, WeatherLayout::WttrCompact);
        let row_one = lines[0]
            .spans
            .last()
            .expect("line should include info text")
            .content
            .to_string();
        let row_two = lines[1]
            .spans
            .last()
            .expect("line should include info text")
            .content
            .to_string();

        assert!(!row_one.trim().is_empty());
        assert!(!row_two.trim().is_empty());
        assert!(UnicodeWidthStr::width(row_one.trim()) <= 13);
        assert!(UnicodeWidthStr::width(row_two.trim()) <= 13);
    }
}
