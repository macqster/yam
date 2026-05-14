use ratatui::style::Color;

use super::btas::BTAS;

#[allow(dead_code)]
pub const PANEL_BG: Color = BTAS.pms_432;
#[allow(dead_code)]
pub const PRIMARY_FG: Color = BTAS.pms_427;
pub const PRIMARY_FG_WARM: Color = Color::Rgb(220, 216, 203);
pub const SECONDARY_FG: Color = BTAS.pms_430;
pub const NEUTRAL_SLATE: Color = Color::Rgb(114, 124, 118);
pub const LOADING_TEXT: Color = Color::Rgb(160, 166, 173);
pub const GREEN_MID: Color = Color::Rgb(62, 115, 87);
pub const GREEN_BAR: Color = Color::Rgb(47, 90, 69);
pub const GREEN_DEEP: Color = Color::Rgb(8, 100, 25);
pub const BLUE_DECO: Color = Color::Rgb(25, 75, 143);
#[allow(dead_code)]
pub const SURFACE_LIFTED: Color = Color::Rgb(38, 42, 46);
#[allow(dead_code)]
pub const DIVIDER: Color = Color::Rgb(55, 60, 65);
pub const WEATHER_RAIN: Color = BTAS.pms_292;
pub const WEATHER_RAIN_HEAVY: Color = Color::Rgb(75, 123, 214);
pub const WEATHER_SUN_CORE: Color = BTAS.pms_142;
pub const WEATHER_SUN_RAY: Color = BTAS.pms_131;
pub const WEATHER_LIGHTNING: Color = BTAS.pms_131;
pub const WEATHER_ALERT: Color = BTAS.pms_485;
pub const MODAL_BORDER: Color = Color::Rgb(47, 90, 69);
pub const BTAS_GREY_DISABLED: Color = BTAS.pms_430;
#[allow(dead_code)]
pub const ACCENT: Color = BTAS.process_cyan;
pub const TAB_INACTIVE: Color = Color::Rgb(178, 78, 46);
pub const TAB_ACTIVE: Color = Color::Rgb(163, 58, 50);
pub const MARKER: Color = BTAS.pms_142;
#[allow(dead_code)]
pub const HERO_BG: Color = BTAS.black;
#[allow(dead_code)]
pub const MODAL_BG: Color = BTAS.pms_432;
#[allow(dead_code)]
pub const CAMERA_TRACK: Color = Color::Rgb(36, 59, 115);
#[allow(dead_code)]
pub const CAMERA_THUMB: Color = BTAS.pms_292;
pub const MODAL_FOOTER_SYMBOL: Color = Color::Rgb(44, 76, 138);
#[allow(dead_code)]
pub const POINTER_PROBE: Color = BTAS.pms_156;
pub const DEBUG_FG: Color = BTAS.pms_340;
pub const VINE_HEALTHY: Color = BTAS.pms_345;
pub const VINE_AGED: Color = BTAS.pms_465;
