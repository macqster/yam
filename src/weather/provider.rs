use super::model::{WeatherLocation, WeatherSnapshot, WeatherSource, WeatherVisual};
use super::wttr;
use chrono::Utc;
use std::process::Command;

pub trait WeatherProvider {
    fn snapshot(&self, location: &WeatherLocation) -> Result<WeatherSnapshot, WeatherError>;
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WeatherError {
    Unavailable,
    FetchFailed,
    Parse,
}

pub struct StaticWeatherProvider;

impl WeatherProvider for StaticWeatherProvider {
    fn snapshot(&self, location: &WeatherLocation) -> Result<WeatherSnapshot, WeatherError> {
        Ok(WeatherSnapshot {
            location_label: location.label.clone(),
            observed_at: Utc::now(),
            temperature_c: Some(14.0),
            feels_like_c: Some(12.0),
            humidity_pct: Some(81),
            wind_kph: Some(6.0),
            wind_dir: Some("NE".to_string()),
            visibility_km: Some(10.0),
            precip_mm: Some(0.2),
            condition_text: Some("Cloudy".to_string()),
            condition_code: None,
            forecast: Vec::new(),
            source: WeatherSource::StaticPrototype,
            stale: false,
            visual: WeatherVisual::Cloudy,
        })
    }
}

pub struct WttrInWeatherProvider;

impl WeatherProvider for WttrInWeatherProvider {
    fn snapshot(&self, location: &WeatherLocation) -> Result<WeatherSnapshot, WeatherError> {
        let location_label = location.label.clone();
        let location_path = location_label.replace(' ', "%20");
        let url = format!("https://wttr.in/{location_path}?format=j1");
        let output = Command::new("curl")
            .args([
                "--silent",
                "--show-error",
                "--fail",
                "--max-time",
                "4",
                &url,
            ])
            .output()
            .map_err(|_| WeatherError::FetchFailed)?;

        if !output.status.success() {
            return Err(WeatherError::FetchFailed);
        }

        let body = String::from_utf8(output.stdout).map_err(|_| WeatherError::Parse)?;
        wttr::parse_snapshot(&location_label, &body)
    }
}
