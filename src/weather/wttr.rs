use serde_json::Value;

use super::model::{WeatherSnapshot, WeatherSource, WeatherVisual};
use super::provider::WeatherError;

pub fn parse_snapshot(location_label: &str, body: &str) -> Result<WeatherSnapshot, WeatherError> {
    let root: Value = serde_json::from_str(body).map_err(|_| WeatherError::Parse)?;
    let current = root
        .get("current_condition")
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .ok_or(WeatherError::Parse)?;

    let condition_text = current
        .get("weatherDesc")
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .and_then(|entry| entry.get("value"))
        .and_then(Value::as_str)
        .map(str::to_string);

    let condition_code = parse_i32(current.get("weatherCode"));
    let visual = condition_code
        .map(visual_from_condition_code)
        .or_else(|| condition_text.as_deref().map(visual_from_condition_text))
        .unwrap_or(WeatherVisual::Unknown);
    let today = root
        .get("weather")
        .and_then(Value::as_array)
        .and_then(|items| items.first());

    Ok(WeatherSnapshot {
        location_label: location_label.to_string(),
        observed_at: chrono::Utc::now(),
        temperature_c: parse_f32(current.get("temp_C")),
        feels_like_c: parse_f32(current.get("FeelsLikeC")),
        day_max_c: parse_f32(today.and_then(|day| day.get("maxtempC"))),
        night_min_c: parse_f32(today.and_then(|day| day.get("mintempC"))),
        humidity_pct: parse_u8(current.get("humidity")),
        wind_kph: parse_f32(current.get("windspeedKmph")),
        wind_dir: current
            .get("winddir16Point")
            .and_then(Value::as_str)
            .map(str::to_string),
        visibility_km: parse_f32(current.get("visibility")),
        precip_mm: parse_f32(current.get("precipMM")),
        condition_text,
        condition_code,
        forecast: Vec::new(),
        source: WeatherSource::WttrIn,
        stale: false,
        visual,
    })
}

fn visual_from_condition_code(code: i32) -> WeatherVisual {
    match code {
        113 => WeatherVisual::Sunny,
        116 => WeatherVisual::PartlyCloudy,
        119 => WeatherVisual::Cloudy,
        122 => WeatherVisual::VeryCloudy,
        143 | 248 | 260 => WeatherVisual::Fog,
        176 | 263 | 293 | 353 => WeatherVisual::LightShowers,
        266 | 296 => WeatherVisual::LightRain,
        299 | 305 | 356 => WeatherVisual::HeavyShowers,
        302 | 308 | 359 => WeatherVisual::HeavyRain,
        179 => WeatherVisual::LightSleetShowers,
        182 | 185 | 281 | 284 | 311 | 314 | 317 | 350 | 377 => WeatherVisual::LightSleet,
        362 | 365 | 374 => WeatherVisual::LightSleetShowers,
        227 => WeatherVisual::LightSnow,
        230 | 329 | 332 | 338 => WeatherVisual::HeavySnow,
        323 | 326 | 368 => WeatherVisual::LightSnowShowers,
        335 | 371 | 395 => WeatherVisual::HeavySnowShowers,
        200 | 386 => WeatherVisual::ThunderyShowers,
        389 => WeatherVisual::ThunderyHeavyRain,
        392 => WeatherVisual::ThunderySnowShowers,
        _ => WeatherVisual::Unknown,
    }
}

fn parse_f32(value: Option<&Value>) -> Option<f32> {
    value
        .and_then(Value::as_str)
        .and_then(|value| value.parse::<f32>().ok())
}

fn parse_u8(value: Option<&Value>) -> Option<u8> {
    value
        .and_then(Value::as_str)
        .and_then(|value| value.parse::<u8>().ok())
}

fn parse_i32(value: Option<&Value>) -> Option<i32> {
    value
        .and_then(Value::as_str)
        .and_then(|value| value.parse::<i32>().ok())
}

fn visual_from_condition_text(condition: &str) -> WeatherVisual {
    let normalized = condition.to_ascii_lowercase();
    if normalized.contains("snow") && normalized.contains("thunder") {
        WeatherVisual::ThunderySnowShowers
    } else if normalized.contains("thunder") && normalized.contains("rain") {
        WeatherVisual::ThunderyHeavyRain
    } else if normalized.contains("thunder") || normalized.contains("storm") {
        WeatherVisual::ThunderyShowers
    } else if normalized.contains("sleet shower") {
        WeatherVisual::LightSleetShowers
    } else if normalized.contains("sleet") || normalized.contains("freezing drizzle") {
        WeatherVisual::LightSleet
    } else if normalized.contains("heavy snow shower") {
        WeatherVisual::HeavySnowShowers
    } else if normalized.contains("light snow shower") || normalized.contains("snow shower") {
        WeatherVisual::LightSnowShowers
    } else if normalized.contains("heavy snow") || normalized.contains("blizzard") {
        WeatherVisual::HeavySnow
    } else if normalized.contains("snow") {
        WeatherVisual::LightSnow
    } else if normalized.contains("heavy shower") {
        WeatherVisual::HeavyShowers
    } else if normalized.contains("heavy rain") || normalized.contains("torrential") {
        WeatherVisual::HeavyRain
    } else if normalized.contains("shower") {
        WeatherVisual::LightShowers
    } else if normalized.contains("rain") || normalized.contains("drizzle") {
        WeatherVisual::LightRain
    } else if normalized.contains("freezing fog") || normalized.contains("fog") {
        WeatherVisual::Fog
    } else if normalized.contains("mist") || normalized.contains("haze") {
        WeatherVisual::Mist
    } else if normalized.contains("very cloudy") || normalized.contains("completely overcast") {
        WeatherVisual::VeryCloudy
    } else if normalized.contains("overcast") {
        WeatherVisual::Overcast
    } else if normalized.contains("partly") {
        WeatherVisual::PartlyCloudy
    } else if normalized.contains("cloud") {
        WeatherVisual::Cloudy
    } else if normalized.contains("clear night") {
        WeatherVisual::ClearNight
    } else if normalized.contains("sun") || normalized.contains("clear") {
        WeatherVisual::Sunny
    } else {
        WeatherVisual::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_snapshot, visual_from_condition_code};
    use crate::weather::model::{WeatherSource, WeatherVisual};

    #[test]
    fn wttr_code_mapping_tracks_upstream_condition_families() {
        assert_eq!(visual_from_condition_code(122), WeatherVisual::VeryCloudy);
        assert_eq!(
            visual_from_condition_code(179),
            WeatherVisual::LightSleetShowers
        );
        assert_eq!(visual_from_condition_code(317), WeatherVisual::LightSleet);
        assert_eq!(
            visual_from_condition_code(326),
            WeatherVisual::LightSnowShowers
        );
        assert_eq!(visual_from_condition_code(338), WeatherVisual::HeavySnow);
        assert_eq!(
            visual_from_condition_code(389),
            WeatherVisual::ThunderyHeavyRain
        );
        assert_eq!(
            visual_from_condition_code(392),
            WeatherVisual::ThunderySnowShowers
        );
    }

    #[test]
    fn wttr_parser_normalizes_compact_current_conditions() {
        let body = r#"{
          "weather": [{
            "maxtempC": "18",
            "mintempC": "9"
          }],
          "current_condition": [{
            "temp_C": "14",
            "FeelsLikeC": "12",
            "humidity": "81",
            "windspeedKmph": "6",
            "winddir16Point": "NE",
            "visibility": "10",
            "precipMM": "0.2",
            "weatherCode": "119",
            "weatherDesc": [{"value": "Cloudy"}]
          }]
        }"#;

        let snapshot = parse_snapshot("Sulkowice", body).expect("wttr payload should parse");

        assert_eq!(snapshot.location_label, "Sulkowice");
        assert_eq!(snapshot.temperature_c, Some(14.0));
        assert_eq!(snapshot.feels_like_c, Some(12.0));
        assert_eq!(snapshot.day_max_c, Some(18.0));
        assert_eq!(snapshot.night_min_c, Some(9.0));
        assert_eq!(snapshot.humidity_pct, Some(81));
        assert_eq!(snapshot.wind_kph, Some(6.0));
        assert_eq!(snapshot.wind_dir.as_deref(), Some("NE"));
        assert_eq!(snapshot.visibility_km, Some(10.0));
        assert_eq!(snapshot.precip_mm, Some(0.2));
        assert_eq!(snapshot.condition_text.as_deref(), Some("Cloudy"));
        assert_eq!(snapshot.condition_code, Some(119));
        assert_eq!(snapshot.source, WeatherSource::WttrIn);
        assert_eq!(snapshot.visual, WeatherVisual::Cloudy);
        assert!(!snapshot.stale);
    }
}
