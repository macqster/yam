use super::model::{WeatherLocale, WeatherSnapshot, WeatherVisual};

pub fn condition_label(snapshot: &WeatherSnapshot, locale: WeatherLocale) -> String {
    if let Some(code) = snapshot
        .condition_code
        .and_then(|code| wttr_code_label(code, locale))
    {
        return code.to_string();
    }
    visual_label(snapshot.visual, locale).to_string()
}

pub fn wttr_day_night_temperature_label(snapshot: &WeatherSnapshot) -> String {
    let day = snapshot
        .day_max_c
        .map(|temp| format!("{temp:.0}"))
        .unwrap_or_else(|| "--".to_string());
    let night = snapshot
        .night_min_c
        .map(|temp| format!("{temp:.0}"))
        .unwrap_or_else(|| "--".to_string());
    format!(" {night}C |  {day}C")
}

pub fn wttr_wind_label(snapshot: &WeatherSnapshot) -> String {
    let wind = snapshot.wind_dir.as_deref().unwrap_or("--");
    let wind_kph = match snapshot.wind_kph {
        Some(wind_kph) => format!("{wind_kph:.0} km/h"),
        None => "--".to_string(),
    };
    format!("{} {wind} {wind_kph}", wind_direction_arrow(wind))
}

pub fn wttr_precip_label(snapshot: &WeatherSnapshot) -> String {
    match snapshot.precip_mm {
        Some(precip_mm) if precip_mm > 0.0 => format!("{precip_mm:.1} mm"),
        _ => String::new(),
    }
}

pub fn visual_label(visual: WeatherVisual, locale: WeatherLocale) -> &'static str {
    match locale {
        WeatherLocale::En => match visual {
            WeatherVisual::Sunny => "sunny",
            WeatherVisual::ClearNight => "clear night",
            WeatherVisual::PartlyCloudy => "partly cloudy",
            WeatherVisual::Cloudy => "cloudy",
            WeatherVisual::VeryCloudy => "very cloudy",
            WeatherVisual::Overcast => "overcast",
            WeatherVisual::Mist => "mist",
            WeatherVisual::Fog => "fog",
            WeatherVisual::LightShowers => "light showers",
            WeatherVisual::LightRain => "light rain",
            WeatherVisual::HeavyShowers => "heavy showers",
            WeatherVisual::HeavyRain => "heavy rain",
            WeatherVisual::LightSnow => "light snow",
            WeatherVisual::HeavySnow => "heavy snow",
            WeatherVisual::LightSnowShowers => "light snow showers",
            WeatherVisual::HeavySnowShowers => "heavy snow showers",
            WeatherVisual::LightSleet => "light sleet",
            WeatherVisual::LightSleetShowers => "light sleet showers",
            WeatherVisual::Sleet => "sleet",
            WeatherVisual::ThunderyShowers => "thundery showers",
            WeatherVisual::ThunderyHeavyRain => "thundery heavy rain",
            WeatherVisual::ThunderySnowShowers => "thundery snow showers",
            WeatherVisual::Thunderstorm => "storm",
            WeatherVisual::Unknown => "unknown",
        },
        WeatherLocale::Pl => match visual {
            WeatherVisual::Sunny => "słonecznie",
            WeatherVisual::ClearNight => "bezchmurna noc",
            WeatherVisual::PartlyCloudy => "lekkie zachmurzenie",
            WeatherVisual::Cloudy => "zachmurzenie",
            WeatherVisual::VeryCloudy => "całkowite zachmurzenie",
            WeatherVisual::Overcast => "całkowite zachmurzenie",
            WeatherVisual::Mist => "zamglenie",
            WeatherVisual::Fog => "mgła",
            WeatherVisual::LightShowers => "przelotne lekkie opady deszczu",
            WeatherVisual::LightRain => "lekkie opady deszczu",
            WeatherVisual::HeavyShowers => "przelotne silne opady deszczu",
            WeatherVisual::HeavyRain => "silne opady deszczu",
            WeatherVisual::LightSnow => "lekkie opady śniegu",
            WeatherVisual::HeavySnow => "silne opady śniegu",
            WeatherVisual::LightSnowShowers => "lekkie opady śniegu",
            WeatherVisual::HeavySnowShowers => "umiarkowane lub silne opady śniegu",
            WeatherVisual::LightSleet => "lekki deszcz ze śniegiem",
            WeatherVisual::LightSleetShowers => "lekki deszcz ze śniegiem",
            WeatherVisual::Sleet => "deszcz ze śniegiem",
            WeatherVisual::ThunderyShowers => "możliwa burza",
            WeatherVisual::ThunderyHeavyRain => {
                "umiarkowane lub silne opady deszczu i burza z piorunami"
            }
            WeatherVisual::ThunderySnowShowers => {
                "umiarkowane lub silne opady śniegu i burza z piorunami"
            }
            WeatherVisual::Thunderstorm => "burza",
            WeatherVisual::Unknown => "nieznane",
        },
    }
}

fn wttr_code_label(code: i32, locale: WeatherLocale) -> Option<&'static str> {
    let label = match locale {
        WeatherLocale::En => match code {
            113 => "Clear",
            116 => "Partly cloudy",
            119 => "Cloudy",
            122 => "Overcast",
            143 => "Mist",
            176 => "Patchy rain possible",
            179 => "Patchy snow possible",
            182 => "Patchy sleet possible",
            185 => "Patchy freezing drizzle possible",
            200 => "Thundery outbreaks possible",
            227 => "Blowing snow",
            230 => "Blizzard",
            248 => "Fog",
            260 => "Freezing fog",
            263 => "Patchy light drizzle",
            266 => "Light drizzle",
            281 => "Freezing drizzle",
            284 => "Heavy freezing drizzle",
            293 => "Patchy light rain",
            296 => "Light rain",
            299 => "Moderate rain at times",
            302 => "Moderate rain",
            305 => "Heavy rain at times",
            308 => "Heavy rain",
            311 => "Light freezing rain",
            314 => "Moderate or heavy freezing rain",
            317 => "Light sleet",
            320 => "Moderate or heavy sleet",
            323 => "Patchy light snow",
            326 => "Light snow",
            329 => "Patchy moderate snow",
            332 => "Moderate snow",
            335 => "Patchy heavy snow",
            338 => "Heavy snow",
            350 => "Ice pellets",
            353 => "Light rain shower",
            356 => "Moderate or heavy rain shower",
            359 => "Torrential rain shower",
            362 => "Light sleet showers",
            365 => "Moderate or heavy sleet showers",
            368 => "Light snow showers",
            371 => "Moderate or heavy snow showers",
            386 => "Patchy light rain with thunder",
            389 => "Moderate or heavy rain with thunder",
            392 => "Patchy light snow with thunder",
            395 => "Moderate or heavy snow with thunder",
            _ => return None,
        },
        WeatherLocale::Pl => match code {
            113 => "bezchmurnie",
            116 => "lekkie zachmurzenie",
            119 => "zachmurzenie",
            122 => "całkowite zachmurzenie",
            143 => "zamglenie",
            176 => "możliwe miejscowe opady deszczu",
            179 => "możliwe miejscowe opady śniegu",
            182 => "możliwe miejscowe opady deszczu ze śniegiem",
            185 => "możliwe miejscowe opady marznącego deszczu",
            200 => "możliwa burza",
            227 => "zawieja śnieżna",
            230 => "śnieżyca",
            248 => "mgła",
            260 => "marznąca mgła",
            263 => "przelotna mżawka",
            266 => "mżawka",
            281 => "marznąca mżawka",
            284 => "marznąca mżawka",
            293 => "przelotne lekkie opady deszczu",
            296 => "lekkie opady deszczu",
            299 => "przelotne umiarkowane opady deszczu",
            302 => "umiarkowane opady deszczu",
            305 => "przelotne silne opady deszczu",
            308 => "silne opady deszczu",
            311 => "lekki marznący deszcz",
            314 => "umiarkowany lub silny marznący deszcz",
            317 => "lekki deszcz ze śniegiem",
            320 => "umiarkowany lub silny deszcz ze śniegiem",
            323 => "miejscowe lekkie opady śniegu",
            326 => "lekkie opady śniegu",
            329 => "miejscowe umiarkowane opady śniegu",
            332 => "umiarkowane opady śniegu",
            335 => "miejscowe silne opady śniegu",
            338 => "silne opady śniegu",
            350 => "gradobicie",
            353 => "lekkie opady deszczu",
            356 => "umiarkowane lub silne opady deszczu",
            359 => "oberwanie chmury",
            362 => "lekki deszcz ze śniegiem",
            365 => "umiarkowany lub silny deszcz ze śniegiem",
            368 => "lekkie opady śniegu",
            371 => "umiarkowane lub silne opady śniegu",
            386 => "miejscowe lekkie opady deszczu i burza z piorunami",
            389 => "umiarkowane lub silne opady deszczu i burza z piorunami",
            392 => "miejscowe lekkie opady śniegu i burza z piorunami",
            395 => "umiarkowane lub silne opady śniegu i burza z piorunami",
            _ => return None,
        },
    };

    Some(label)
}

fn wind_direction_arrow(direction: &str) -> &'static str {
    let normalized = direction.trim().to_ascii_uppercase();
    match normalized.as_str() {
        "N" => "↑",
        "NE" | "NNE" | "ENE" => "↗",
        "E" => "→",
        "SE" | "ESE" | "SSE" => "↘",
        "S" => "↓",
        "SW" | "SSW" | "WSW" => "↙",
        "W" => "←",
        "NW" | "WNW" | "NNW" => "↖",
        _ if normalized.starts_with('N') && normalized.ends_with('E') => "↗",
        _ if normalized.starts_with('S') && normalized.ends_with('E') => "↘",
        _ if normalized.starts_with('S') && normalized.ends_with('W') => "↙",
        _ if normalized.starts_with('N') && normalized.ends_with('W') => "↖",
        _ if normalized.starts_with('N') => "↑",
        _ if normalized.starts_with('E') => "→",
        _ if normalized.starts_with('S') => "↓",
        _ if normalized.starts_with('W') => "←",
        _ => "↑",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        condition_label, wttr_code_label, wttr_day_night_temperature_label, wttr_wind_label,
    };
    use crate::weather::model::{WeatherLocale, WeatherSnapshot, WeatherSource, WeatherVisual};

    #[test]
    fn polish_localization_is_available_for_visual_labels() {
        let snapshot = WeatherSnapshot {
            location_label: "Sulkowice".to_string(),
            observed_at: chrono::Utc::now(),
            temperature_c: Some(10.0),
            feels_like_c: Some(8.0),
            day_max_c: Some(12.0),
            night_min_c: Some(6.0),
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

        assert_eq!(condition_label(&snapshot, WeatherLocale::En), "overcast");
        assert_eq!(
            condition_label(&snapshot, WeatherLocale::Pl),
            "zachmurzenie"
        );
    }

    #[test]
    fn polish_wttr_code_labels_follow_upstream_condition_strings() {
        assert_eq!(
            wttr_code_label(389, WeatherLocale::Pl),
            Some("umiarkowane lub silne opady deszczu i burza z piorunami")
        );
        assert_eq!(
            wttr_code_label(200, WeatherLocale::Pl),
            Some("możliwa burza")
        );
        assert_eq!(wttr_code_label(113, WeatherLocale::Pl), Some("bezchmurnie"));
    }

    #[test]
    fn day_night_temperature_label_uses_compact_spacing() {
        let snapshot = WeatherSnapshot {
            location_label: "Sulkowice".to_string(),
            observed_at: chrono::Utc::now(),
            temperature_c: Some(10.0),
            feels_like_c: Some(8.0),
            day_max_c: Some(18.0),
            night_min_c: Some(5.0),
            humidity_pct: Some(76),
            wind_kph: Some(4.0),
            wind_dir: Some("ENE".to_string()),
            visibility_km: Some(10.0),
            precip_mm: Some(0.0),
            condition_text: Some("Clear".to_string()),
            condition_code: Some(113),
            forecast: Vec::new(),
            source: WeatherSource::StaticPrototype,
            stale: false,
            visual: WeatherVisual::Sunny,
        };

        assert_eq!(
            wttr_day_night_temperature_label(&snapshot),
            " 5C |  18C"
        );
    }

    #[test]
    fn wind_label_uses_directional_arrow_for_compass_variant() {
        let snapshot = WeatherSnapshot {
            location_label: "Sulkowice".to_string(),
            observed_at: chrono::Utc::now(),
            temperature_c: Some(10.0),
            feels_like_c: Some(8.0),
            day_max_c: Some(18.0),
            night_min_c: Some(5.0),
            humidity_pct: Some(76),
            wind_kph: Some(10.0),
            wind_dir: Some("ENE".to_string()),
            visibility_km: Some(10.0),
            precip_mm: Some(0.0),
            condition_text: Some("Clear".to_string()),
            condition_code: Some(113),
            forecast: Vec::new(),
            source: WeatherSource::StaticPrototype,
            stale: false,
            visual: WeatherVisual::Sunny,
        };

        assert_eq!(wttr_wind_label(&snapshot), "↗ ENE 10 km/h");
    }

    #[test]
    fn secondary_intercardinal_directions_collapse_to_single_ordinal_arrow() {
        let base_snapshot = WeatherSnapshot {
            location_label: "Sulkowice".to_string(),
            observed_at: chrono::Utc::now(),
            temperature_c: Some(10.0),
            feels_like_c: Some(8.0),
            day_max_c: Some(18.0),
            night_min_c: Some(5.0),
            humidity_pct: Some(76),
            wind_kph: Some(10.0),
            wind_dir: Some("NNE".to_string()),
            visibility_km: Some(10.0),
            precip_mm: Some(0.0),
            condition_text: Some("Clear".to_string()),
            condition_code: Some(113),
            forecast: Vec::new(),
            source: WeatherSource::StaticPrototype,
            stale: false,
            visual: WeatherVisual::Sunny,
        };

        let nne = wttr_wind_label(&base_snapshot);
        let ese = wttr_wind_label(&WeatherSnapshot {
            wind_dir: Some("ESE".to_string()),
            ..base_snapshot.clone()
        });

        assert_eq!(nne, "↗ NNE 10 km/h");
        assert_eq!(ese, "↘ ESE 10 km/h");
    }
}
