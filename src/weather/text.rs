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

pub fn wttr_temperature_band_label(snapshot: &WeatherSnapshot) -> String {
    match (snapshot.temperature_c, snapshot.feels_like_c) {
        (Some(temp), Some(feels_like)) => format!("{temp:.0} - {feels_like:.0} C"),
        (Some(temp), None) => format!("{temp:.0} C"),
        _ => "--".to_string(),
    }
}

pub fn wttr_wind_label(snapshot: &WeatherSnapshot) -> String {
    let wind = snapshot.wind_dir.as_deref().unwrap_or("--");
    let wind_kph = match snapshot.wind_kph {
        Some(wind_kph) => format!("{wind_kph:.0} km/h"),
        None => "--".to_string(),
    };
    format!("↑ {wind} {wind_kph}")
}

pub fn wttr_visibility_label(snapshot: &WeatherSnapshot) -> String {
    match snapshot.visibility_km {
        Some(visibility_km) => format!("{visibility_km:.0} km"),
        None => "-- km".to_string(),
    }
}

pub fn wttr_precip_label(snapshot: &WeatherSnapshot) -> String {
    match snapshot.precip_mm {
        Some(precip_mm) => format!("{precip_mm:.1} mm"),
        None => "-- mm".to_string(),
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
            WeatherVisual::Sunny => "Słonecznie",
            WeatherVisual::ClearNight => "Bezchmurna noc",
            WeatherVisual::PartlyCloudy => "Lekkie zachmurzenie",
            WeatherVisual::Cloudy => "pochmurno",
            WeatherVisual::VeryCloudy => "Całkowite zachmurzenie",
            WeatherVisual::Overcast => "Całkowite zachmurzenie",
            WeatherVisual::Mist => "Zamglenie",
            WeatherVisual::Fog => "Mgła",
            WeatherVisual::LightShowers => "Przelotne lekkie opady deszczu",
            WeatherVisual::LightRain => "Lekkie opady deszczu",
            WeatherVisual::HeavyShowers => "Umiarkowane lub silne opady deszczu",
            WeatherVisual::HeavyRain => "Silne opady deszczu",
            WeatherVisual::LightSnow => "Lekkie opady śniegu",
            WeatherVisual::HeavySnow => "Silne opady śniegu",
            WeatherVisual::LightSnowShowers => "Lekkie opady śniegu",
            WeatherVisual::HeavySnowShowers => "Umiarkowane lub silne opady śniegu",
            WeatherVisual::LightSleet => "Lekki deszcz ze śniegiem",
            WeatherVisual::LightSleetShowers => "Lekki deszcz ze śniegiem",
            WeatherVisual::Sleet => "Deszcz ze śniegiem",
            WeatherVisual::ThunderyShowers => "Możliwa burza",
            WeatherVisual::ThunderyHeavyRain => {
                "Umiarkowane lub silne opady deszczu i burza z piorunami"
            }
            WeatherVisual::ThunderySnowShowers => {
                "Umiarkowane lub silne opady śniegu i burza z piorunami"
            }
            WeatherVisual::Thunderstorm => "Burza",
            WeatherVisual::Unknown => "Nieznane",
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
            113 => "Bezchmurnie",
            116 => "Lekkie zachmurzenie",
            119 => "Zachmurzenie",
            122 => "Całkowite zachmurzenie",
            143 => "Zamglenie",
            176 => "Możliwe miejscowe opady deszczu",
            179 => "Możliwe miejscowe opady śniegu",
            182 => "Możliwe miejscowe opady deszczu ze śniegiem",
            185 => "Możliwe miejscowe opady marznącego deszczu",
            200 => "Możliwa burza",
            227 => "Zawieja śnieżna",
            230 => "Śnieżyca",
            248 => "Mgła",
            260 => "Marznąca mgła",
            263 => "Przelotna mżawka",
            266 => "Mżawka",
            281 => "Marznąca mżawka",
            284 => "Marznąca mżawka",
            293 => "Przelotne lekkie opady deszczu",
            296 => "Lekkie opady deszczu",
            299 => "Przelotne umiarkowane opady deszczu",
            302 => "Umiarkowane opady deszczu",
            305 => "Przelotne silne opady deszczu",
            308 => "Silne opady deszczu",
            311 => "Lekki marznący deszcz",
            314 => "Umiarkowany lub silny marznący deszcz",
            317 => "Lekki deszcz ze śniegiem",
            320 => "Umiarkowany lub silny deszcz ze śniegiem",
            323 => "Miejscowe lekkie opady śniegu",
            326 => "Lekkie opady śniegu",
            329 => "Miejscowe umiarkowane opady śniegu",
            332 => "Umiarkowane opady śniegu",
            335 => "Miejscowe silne opady śniegu",
            338 => "Silne opady śniegu",
            350 => "Gradobicie",
            353 => "Lekkie opady deszczu",
            356 => "Umiarkowane lub silne opady deszczu",
            359 => "Oberwanie chmury",
            362 => "Lekki deszcz ze śniegiem",
            365 => "Umiarkowany lub silny deszcz ze śniegiem",
            368 => "Lekkie opady śniegu",
            371 => "Umiarkowane lub silne opady śniegu",
            386 => "Miejscowe lekkie opady deszczu i burza z piorunami",
            389 => "Umiarkowane lub silne opady deszczu i burza z piorunami",
            392 => "Miejscowe lekkie opady śniegu i burza z piorunami",
            395 => "Umiarkowane lub silne opady śniegu i burza z piorunami",
            _ => return None,
        },
    };

    Some(label)
}

#[cfg(test)]
mod tests {
    use super::condition_label;
    use crate::weather::model::{WeatherLocale, WeatherSnapshot, WeatherSource, WeatherVisual};

    #[test]
    fn polish_localization_is_available_for_visual_labels() {
        let snapshot = WeatherSnapshot {
            location_label: "Sulkowice".to_string(),
            observed_at: chrono::Utc::now(),
            temperature_c: Some(10.0),
            feels_like_c: Some(8.0),
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
}
