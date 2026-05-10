use chrono::{DateTime, Utc};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeatherSource {
    StaticPrototype,
    WttrIn,
    OpenMeteo,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeatherVisual {
    Sunny,
    ClearNight,
    PartlyCloudy,
    Cloudy,
    VeryCloudy,
    Overcast,
    Mist,
    Fog,
    LightShowers,
    LightRain,
    HeavyShowers,
    HeavyRain,
    LightSnow,
    HeavySnow,
    LightSnowShowers,
    HeavySnowShowers,
    LightSleet,
    LightSleetShowers,
    Sleet,
    ThunderyShowers,
    ThunderyHeavyRain,
    ThunderySnowShowers,
    Thunderstorm,
    Unknown,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WeatherLocale {
    #[default]
    En,
    Pl,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeatherLocation {
    pub label: String,
}

impl WeatherLocation {
    pub fn named(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForecastPoint {
    pub label: String,
    pub temperature_c: Option<f32>,
    pub visual: WeatherVisual,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WeatherSnapshot {
    pub location_label: String,
    pub observed_at: DateTime<Utc>,
    pub temperature_c: Option<f32>,
    pub feels_like_c: Option<f32>,
    pub humidity_pct: Option<u8>,
    pub wind_kph: Option<f32>,
    pub wind_dir: Option<String>,
    pub visibility_km: Option<f32>,
    pub precip_mm: Option<f32>,
    pub condition_text: Option<String>,
    pub condition_code: Option<i32>,
    pub forecast: Vec<ForecastPoint>,
    pub source: WeatherSource,
    pub stale: bool,
    pub visual: WeatherVisual,
}
