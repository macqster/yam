use super::model::WeatherVisual;
use super::render::{WeatherColorRole, WeatherSprite, WeatherSpriteLine, WeatherSpriteSpan};

const CLOUDY_SHAPE: &str = include_str!("assets/runtime/cloudy.txt");
const CLEAR_SHAPE: &str = include_str!("assets/runtime/clear.txt");
const CLEAR_NIGHT_SHAPE: &str = include_str!("assets/runtime/clear_night.txt");
const MIST_SHAPE: &str = include_str!("assets/runtime/mist.txt");
const FOG_SHAPE: &str = include_str!("assets/runtime/fog.txt");
const PARTLY_CLOUDY_SHAPE: &str = include_str!("assets/runtime/partly_cloudy.txt");
const LIGHT_SHOWERS_SHAPE: &str = include_str!("assets/runtime/light_showers.txt");
const LIGHT_RAIN_SHAPE: &str = include_str!("assets/runtime/light_rain.txt");
const HEAVY_SHOWERS_SHAPE: &str = include_str!("assets/runtime/heavy_showers.txt");
const HEAVY_RAIN_SHAPE: &str = include_str!("assets/runtime/heavy_rain.txt");
const SLEET_SHAPE: &str = include_str!("assets/runtime/sleet.txt");
const LIGHT_SLEET_SHAPE: &str = include_str!("assets/runtime/light_sleet.txt");
const LIGHT_SLEET_SHOWERS_SHAPE: &str = include_str!("assets/runtime/light_sleet_showers.txt");
const LIGHT_SNOW_SHAPE: &str = include_str!("assets/runtime/light_snow.txt");
const HEAVY_SNOW_SHAPE: &str = include_str!("assets/runtime/heavy_snow.txt");
const LIGHT_SNOW_SHOWERS_SHAPE: &str = include_str!("assets/runtime/light_snow_showers.txt");
const HEAVY_SNOW_SHOWERS_SHAPE: &str = include_str!("assets/runtime/heavy_snow_showers.txt");
const THUNDERY_SHOWERS_SHAPE: &str = include_str!("assets/runtime/thundery_showers.txt");
const THUNDERY_HEAVY_RAIN_SHAPE: &str = include_str!("assets/runtime/thundery_heavy_rain.txt");
const THUNDERY_SNOW_SHOWERS_SHAPE: &str = include_str!("assets/runtime/thundery_snow_showers.txt");
const STORM_SHAPE: &str = include_str!("assets/runtime/storm.txt");
const UNKNOWN_SHAPE: &str = include_str!("assets/runtime/unknown.txt");
const VERY_CLOUDY_SHAPE: &str = include_str!("assets/runtime/very_cloudy.txt");
const OVERCAST_SHAPE: &str = include_str!("assets/runtime/overcast.txt");

pub fn compact_sprite_for(visual: WeatherVisual) -> WeatherSprite {
    match visual {
        WeatherVisual::PartlyCloudy => {
            sprite_from_asset(PARTLY_CLOUDY_SHAPE, partly_cloudy_roles())
        }
        WeatherVisual::Sunny => sprite_from_asset(CLEAR_SHAPE, clear_roles(CLEAR_SHAPE)),
        WeatherVisual::ClearNight => {
            sprite_from_asset(CLEAR_NIGHT_SHAPE, clear_night_roles(CLEAR_NIGHT_SHAPE))
        }
        WeatherVisual::LightShowers => {
            sprite_from_asset(LIGHT_SHOWERS_SHAPE, rain_roles(LIGHT_SHOWERS_SHAPE))
        }
        WeatherVisual::LightRain => {
            sprite_from_asset(LIGHT_RAIN_SHAPE, rain_roles(LIGHT_RAIN_SHAPE))
        }
        WeatherVisual::HeavyShowers => {
            sprite_from_asset(HEAVY_SHOWERS_SHAPE, rain_roles(HEAVY_SHOWERS_SHAPE))
        }
        WeatherVisual::HeavyRain => {
            sprite_from_asset(HEAVY_RAIN_SHAPE, rain_roles(HEAVY_RAIN_SHAPE))
        }
        WeatherVisual::ThunderyShowers => sprite_from_asset(
            THUNDERY_SHOWERS_SHAPE,
            thunder_roles(THUNDERY_SHOWERS_SHAPE),
        ),
        WeatherVisual::ThunderyHeavyRain => sprite_from_asset(
            THUNDERY_HEAVY_RAIN_SHAPE,
            thunder_roles(THUNDERY_HEAVY_RAIN_SHAPE),
        ),
        WeatherVisual::ThunderySnowShowers => sprite_from_asset(
            THUNDERY_SNOW_SHOWERS_SHAPE,
            thunder_roles(THUNDERY_SNOW_SHOWERS_SHAPE),
        ),
        WeatherVisual::Thunderstorm => sprite_from_asset(STORM_SHAPE, thunder_roles(STORM_SHAPE)),
        WeatherVisual::Cloudy => sprite_from_asset(CLOUDY_SHAPE, cloudy_roles(CLOUDY_SHAPE)),
        WeatherVisual::VeryCloudy => {
            sprite_from_asset(VERY_CLOUDY_SHAPE, cloudy_roles(VERY_CLOUDY_SHAPE))
        }
        WeatherVisual::Overcast => sprite_from_asset(OVERCAST_SHAPE, cloudy_roles(OVERCAST_SHAPE)),
        WeatherVisual::Mist => sprite_from_asset(MIST_SHAPE, mist_roles(MIST_SHAPE)),
        WeatherVisual::Fog => sprite_from_asset(FOG_SHAPE, mist_roles(FOG_SHAPE)),
        WeatherVisual::LightSnow => {
            sprite_from_asset(LIGHT_SNOW_SHAPE, snow_roles(LIGHT_SNOW_SHAPE))
        }
        WeatherVisual::HeavySnow => {
            sprite_from_asset(HEAVY_SNOW_SHAPE, snow_roles(HEAVY_SNOW_SHAPE))
        }
        WeatherVisual::LightSnowShowers => sprite_from_asset(
            LIGHT_SNOW_SHOWERS_SHAPE,
            snow_roles(LIGHT_SNOW_SHOWERS_SHAPE),
        ),
        WeatherVisual::HeavySnowShowers => sprite_from_asset(
            HEAVY_SNOW_SHOWERS_SHAPE,
            snow_roles(HEAVY_SNOW_SHOWERS_SHAPE),
        ),
        WeatherVisual::LightSleet => {
            sprite_from_asset(LIGHT_SLEET_SHAPE, mixed_precip_roles(LIGHT_SLEET_SHAPE))
        }
        WeatherVisual::LightSleetShowers => sprite_from_asset(
            LIGHT_SLEET_SHOWERS_SHAPE,
            mixed_precip_roles(LIGHT_SLEET_SHOWERS_SHAPE),
        ),
        WeatherVisual::Sleet => sprite_from_asset(SLEET_SHAPE, mixed_precip_roles(SLEET_SHAPE)),
        _ => sprite_from_asset(UNKNOWN_SHAPE, unknown_roles(UNKNOWN_SHAPE)),
    }
}

fn sprite_from_asset(shape: &'static str, roles: Vec<Vec<SegmentRole>>) -> WeatherSprite {
    let lines = shape.lines().collect::<Vec<_>>();
    assert_eq!(
        lines.len(),
        roles.len(),
        "weather sprite assets and role maps must stay line-aligned"
    );
    let width = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as u16;
    let built = lines
        .into_iter()
        .zip(roles)
        .map(|(line, segments)| WeatherSpriteLine {
            spans: segments
                .into_iter()
                .map(|segment| WeatherSpriteSpan {
                    text: slice_chars(line, segment.start, segment.end),
                    role: segment.role,
                })
                .collect(),
        })
        .collect::<Vec<_>>();

    WeatherSprite {
        width,
        height: built.len() as u16,
        lines: built,
    }
}

#[derive(Clone, Copy)]
struct SegmentRole {
    start: usize,
    end: usize,
    role: WeatherColorRole,
}

fn cloudy_roles(shape: &'static str) -> Vec<Vec<SegmentRole>> {
    vec![
        vec![full_line(shape, 0, WeatherColorRole::CloudEdge)],
        vec![full_line(shape, 1, WeatherColorRole::CloudEdge)],
        classify_cloud_line(
            shape
                .lines()
                .nth(2)
                .expect("weather sprite line should exist"),
        ),
        vec![full_line(shape, 3, WeatherColorRole::TextDim)],
        vec![full_line(shape, 4, WeatherColorRole::TextDim)],
    ]
}

fn clear_roles(shape: &'static str) -> Vec<Vec<SegmentRole>> {
    vec![
        vec![full_line(shape, 0, WeatherColorRole::SunRay)],
        vec![full_line(shape, 1, WeatherColorRole::SunCore)],
        classify_clear_center_line(
            shape
                .lines()
                .nth(2)
                .expect("weather sprite line should exist"),
        ),
        vec![full_line(shape, 3, WeatherColorRole::SunCore)],
        vec![full_line(shape, 4, WeatherColorRole::SunRay)],
    ]
}

fn clear_night_roles(shape: &'static str) -> Vec<Vec<SegmentRole>> {
    vec![
        vec![full_line(shape, 0, WeatherColorRole::TextDim)],
        vec![full_line(shape, 1, WeatherColorRole::CloudEdge)],
        vec![full_line(shape, 2, WeatherColorRole::CloudEdge)],
        classify_clear_night_lower_line(
            shape
                .lines()
                .nth(3)
                .expect("weather sprite line should exist"),
        ),
        vec![full_line(shape, 4, WeatherColorRole::TextDim)],
    ]
}

fn partly_cloudy_roles() -> Vec<Vec<SegmentRole>> {
    vec![
        vec![
            seg(0, 2, WeatherColorRole::TextDim),
            seg(2, 7, WeatherColorRole::SunRay),
            seg(7, 12, WeatherColorRole::TextDim),
        ],
        vec![
            seg(0, 2, WeatherColorRole::SunRay),
            seg(2, 5, WeatherColorRole::SunCore),
            seg(5, 8, WeatherColorRole::CloudEdge),
            seg(8, 12, WeatherColorRole::TextDim),
        ],
        classify_partly_cloudy_upper_cloud_line(),
        classify_partly_cloudy_lower_cloud_line(),
        vec![seg(0, 12, WeatherColorRole::TextDim)],
    ]
}

fn rain_roles(shape: &'static str) -> Vec<Vec<SegmentRole>> {
    let precip_role = if matches!(shape, HEAVY_SHOWERS_SHAPE | HEAVY_RAIN_SHAPE) {
        WeatherColorRole::RainHeavy
    } else {
        WeatherColorRole::Rain
    };
    vec![
        vec![full_line(shape, 0, WeatherColorRole::CloudEdge)],
        vec![full_line(shape, 1, WeatherColorRole::CloudEdge)],
        classify_cloud_line(
            shape
                .lines()
                .nth(2)
                .expect("weather sprite line should exist"),
        ),
        vec![full_line(shape, 3, precip_role)],
        vec![full_line(shape, 4, precip_role)],
    ]
}

fn mist_roles(shape: &'static str) -> Vec<Vec<SegmentRole>> {
    whole_line_roles(
        shape,
        &[
            WeatherColorRole::Fog,
            WeatherColorRole::Fog,
            WeatherColorRole::Fog,
            WeatherColorRole::Fog,
            WeatherColorRole::Fog,
        ],
    )
}

fn snow_roles(shape: &'static str) -> Vec<Vec<SegmentRole>> {
    vec![
        vec![full_line(shape, 0, WeatherColorRole::CloudEdge)],
        vec![full_line(shape, 1, WeatherColorRole::CloudEdge)],
        classify_cloud_line(
            shape
                .lines()
                .nth(2)
                .expect("weather sprite line should exist"),
        ),
        vec![full_line(shape, 3, WeatherColorRole::Snow)],
        vec![full_line(shape, 4, WeatherColorRole::Snow)],
    ]
}

fn mixed_precip_roles(shape: &'static str) -> Vec<Vec<SegmentRole>> {
    vec![
        vec![full_line(shape, 0, WeatherColorRole::CloudEdge)],
        vec![full_line(shape, 1, WeatherColorRole::CloudEdge)],
        classify_cloud_line(
            shape
                .lines()
                .nth(2)
                .expect("weather sprite line should exist"),
        ),
        classify_precip_line(
            shape
                .lines()
                .nth(3)
                .expect("weather sprite line should exist"),
            WeatherColorRole::TextDim,
        ),
        classify_precip_line(
            shape
                .lines()
                .nth(4)
                .expect("weather sprite line should exist"),
            WeatherColorRole::TextDim,
        ),
    ]
}

fn thunder_roles(shape: &'static str) -> Vec<Vec<SegmentRole>> {
    let rain_role = if matches!(shape, THUNDERY_HEAVY_RAIN_SHAPE | STORM_SHAPE) {
        WeatherColorRole::RainHeavy
    } else {
        WeatherColorRole::Rain
    };
    vec![
        vec![full_line(shape, 0, WeatherColorRole::CloudEdge)],
        vec![full_line(shape, 1, WeatherColorRole::CloudEdge)],
        classify_cloud_line(
            shape
                .lines()
                .nth(2)
                .expect("weather sprite line should exist"),
        ),
        classify_thunder_line(
            shape
                .lines()
                .nth(3)
                .expect("weather sprite line should exist"),
            rain_role,
        ),
        classify_thunder_line(
            shape
                .lines()
                .nth(4)
                .expect("weather sprite line should exist"),
            rain_role,
        ),
    ]
}

fn unknown_roles(shape: &'static str) -> Vec<Vec<SegmentRole>> {
    whole_line_roles(
        shape,
        &[
            WeatherColorRole::TextDim,
            WeatherColorRole::TextDim,
            WeatherColorRole::TextDim,
            WeatherColorRole::TextDim,
            WeatherColorRole::TextDim,
        ],
    )
}

const fn seg(start: usize, end: usize, role: WeatherColorRole) -> SegmentRole {
    SegmentRole { start, end, role }
}

fn full_line(shape: &'static str, line_index: usize, role: WeatherColorRole) -> SegmentRole {
    let line = shape
        .lines()
        .nth(line_index)
        .expect("weather sprite line should exist");
    seg(0, line.chars().count(), role)
}

fn whole_line_roles(shape: &'static str, roles: &[WeatherColorRole]) -> Vec<Vec<SegmentRole>> {
    let lines = shape.lines().collect::<Vec<_>>();
    assert_eq!(
        lines.len(),
        roles.len(),
        "weather sprite assets and whole-line role maps must stay aligned"
    );

    lines
        .into_iter()
        .zip(roles.iter().copied())
        .map(|(line, role)| vec![seg(0, line.chars().count(), role)])
        .collect()
}

fn classify_precip_line(line: &'static str, default_role: WeatherColorRole) -> Vec<SegmentRole> {
    classify_line(line, |ch| match ch {
        '*' => WeatherColorRole::Snow,
        '\'' | '‘' | '’' => WeatherColorRole::Rain,
        _ => default_role,
    })
}

fn classify_cloud_line(line: &'static str) -> Vec<SegmentRole> {
    classify_line(line, |ch| match ch {
        '_' => WeatherColorRole::CloudEdge,
        ' ' => WeatherColorRole::TextDim,
        _ => WeatherColorRole::CloudEdge,
    })
}

fn classify_clear_center_line(line: &'static str) -> Vec<SegmentRole> {
    classify_line(line, |ch| match ch {
        '―' | '-' | '/' | '\\' => WeatherColorRole::SunRay,
        '(' | ')' | '.' => WeatherColorRole::SunCore,
        ' ' => WeatherColorRole::TextDim,
        _ => WeatherColorRole::SunCore,
    })
}

fn classify_clear_night_lower_line(line: &'static str) -> Vec<SegmentRole> {
    classify_line(line, |ch| match ch {
        '`' | '.' | ',' | '_' | '-' => WeatherColorRole::CloudEdge,
        ' ' => WeatherColorRole::TextDim,
        _ => WeatherColorRole::CloudEdge,
    })
}

fn classify_partly_cloudy_upper_cloud_line() -> Vec<SegmentRole> {
    let line = PARTLY_CLOUDY_SHAPE
        .lines()
        .nth(2)
        .expect("weather sprite line should exist");
    classify_line(line, |ch| match ch {
        '\\' => WeatherColorRole::SunRay,
        '_' => WeatherColorRole::CloudEdge,
        '.' => WeatherColorRole::CloudEdge,
        ' ' => WeatherColorRole::TextDim,
        _ => WeatherColorRole::CloudEdge,
    })
}

fn classify_partly_cloudy_lower_cloud_line() -> Vec<SegmentRole> {
    let line = PARTLY_CLOUDY_SHAPE
        .lines()
        .nth(3)
        .expect("weather sprite line should exist");
    classify_line(line, |ch| match ch {
        '/' => WeatherColorRole::SunRay,
        '_' => WeatherColorRole::CloudEdge,
        ' ' => WeatherColorRole::TextDim,
        _ => WeatherColorRole::CloudEdge,
    })
}

fn classify_thunder_line(line: &'static str, rain_role: WeatherColorRole) -> Vec<SegmentRole> {
    classify_line(line, |ch| match ch {
        '*' => WeatherColorRole::Snow,
        '\'' | '‘' | '’' => rain_role,
        '/' | '\\' => WeatherColorRole::Lightning,
        '_' => WeatherColorRole::Alert,
        ',' => WeatherColorRole::CloudShadow,
        ' ' => WeatherColorRole::TextDim,
        _ => rain_role,
    })
}

fn classify_line(
    line: &'static str,
    role_for_char: impl Fn(char) -> WeatherColorRole,
) -> Vec<SegmentRole> {
    let chars = line.chars().collect::<Vec<_>>();
    if chars.is_empty() {
        return vec![seg(0, 0, WeatherColorRole::TextDim)];
    }

    let mut segments = Vec::new();
    let mut start = 0usize;
    let mut current_role = role_for_char(chars[0]);

    for (index, ch) in chars.iter().copied().enumerate().skip(1) {
        let role = role_for_char(ch);
        if role != current_role {
            segments.push(seg(start, index, current_role));
            start = index;
            current_role = role;
        }
    }

    segments.push(seg(start, chars.len(), current_role));
    segments
}

fn slice_chars(line: &'static str, start: usize, end: usize) -> &'static str {
    let char_count = line.chars().count();
    assert!(
        start <= end && end <= char_count,
        "weather sprite segment must stay within character boundaries"
    );
    let start_byte = if start == char_count {
        line.len()
    } else {
        line.char_indices()
            .nth(start)
            .map(|(idx, _)| idx)
            .expect("sprite segment start should exist")
    };
    let end_byte = if end == char_count {
        line.len()
    } else {
        line.char_indices()
            .nth(end)
            .map(|(idx, _)| idx)
            .expect("sprite segment end should exist")
    };

    &line[start_byte..end_byte]
}

#[cfg(test)]
mod tests {
    use super::compact_sprite_for;
    use crate::weather::model::WeatherVisual;
    use crate::weather::render::WeatherColorRole;

    #[test]
    fn compact_sprite_assets_survive_unicode_character_boundaries() {
        let clear = compact_sprite_for(WeatherVisual::Sunny);
        let rainy = compact_sprite_for(WeatherVisual::LightRain);

        assert_eq!(clear.lines.len(), 5);
        assert_eq!(rainy.lines.len(), 5);
        assert!(clear
            .lines
            .iter()
            .flat_map(|line| line.spans.iter())
            .all(|span| !span.text.is_empty()));
        assert!(rainy
            .lines
            .iter()
            .flat_map(|line| line.spans.iter())
            .all(|span| !span.text.is_empty()));
    }

    #[test]
    fn mist_snow_and_sleet_have_dedicated_non_unknown_sprites() {
        for visual in [
            WeatherVisual::Mist,
            WeatherVisual::LightSnow,
            WeatherVisual::HeavySnow,
            WeatherVisual::Sleet,
            WeatherVisual::LightSleet,
        ] {
            let sprite = compact_sprite_for(visual);
            let flattened = sprite
                .lines
                .iter()
                .flat_map(|line| line.spans.iter())
                .map(|span| span.text)
                .collect::<String>();

            assert!(!flattened.contains('?'));
        }
    }

    #[test]
    fn heavy_precipitation_variants_use_the_heavier_rain_role() {
        for visual in [
            WeatherVisual::HeavyRain,
            WeatherVisual::HeavyShowers,
            WeatherVisual::ThunderyHeavyRain,
            WeatherVisual::Thunderstorm,
        ] {
            let sprite = compact_sprite_for(visual);
            assert!(sprite
                .lines
                .iter()
                .flat_map(|line| line.spans.iter())
                .any(|span| span.role == WeatherColorRole::RainHeavy));
        }
    }

    #[test]
    fn thunder_variants_expose_lightning_roles_without_reusing_sun_rays() {
        for visual in [
            WeatherVisual::ThunderyShowers,
            WeatherVisual::ThunderyHeavyRain,
            WeatherVisual::ThunderySnowShowers,
            WeatherVisual::Thunderstorm,
        ] {
            let sprite = compact_sprite_for(visual);
            assert!(sprite
                .lines
                .iter()
                .flat_map(|line| line.spans.iter())
                .any(|span| span.role == WeatherColorRole::Lightning));
        }
    }
}
