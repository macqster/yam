use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::render::fonts::ClockFont;
use crate::render::hero::Hero;
use crate::scene::camera::Camera;
use crate::scene::entity::hero_and_clock_poses;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MetaState {
    #[serde(rename = "debug_layout")]
    pub dev_mode: bool,
    pub settings_open: bool,
    pub settings_tab: SettingsTab,
}

impl MetaState {
    pub fn toggle_dev_mode(&mut self) {
        self.dev_mode = !self.dev_mode;
    }

    pub fn toggle_settings(&mut self) {
        self.settings_open = !self.settings_open;
    }

    pub fn next_settings_tab(&mut self) {
        self.settings_tab = self.settings_tab.next();
    }

    pub fn prev_settings_tab(&mut self) {
        self.settings_tab = self.settings_tab.prev();
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum SettingsTab {
    #[default]
    Positions,
    Widgets,
    Gif,
    Theme,
}

impl SettingsTab {
    pub fn next(self) -> Self {
        match self {
            SettingsTab::Positions => SettingsTab::Widgets,
            SettingsTab::Widgets => SettingsTab::Gif,
            SettingsTab::Gif => SettingsTab::Theme,
            SettingsTab::Theme => SettingsTab::Positions,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            SettingsTab::Positions => SettingsTab::Theme,
            SettingsTab::Widgets => SettingsTab::Positions,
            SettingsTab::Gif => SettingsTab::Widgets,
            SettingsTab::Theme => SettingsTab::Gif,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            SettingsTab::Positions => "positions",
            SettingsTab::Widgets => "widgets",
            SettingsTab::Gif => "gif",
            SettingsTab::Theme => "theme",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UiStateSnapshot {
    offsets: UiOffsets,
    meta: MetaState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct UiOffsets {
    pub camera_x: i32,
    pub camera_y: i32,
    pub hero_dx: i32,
    pub hero_dy: i32,
    pub clock_dx: i16,
    pub clock_dy: i16,
    pub clock_font: String,
    pub hero_fps: f32,
}

impl Default for UiOffsets {
    fn default() -> Self {
        Self {
            camera_x: -69,
            camera_y: -17,
            hero_dx: -110,
            hero_dy: -54,
            clock_dx: 96,
            clock_dy: 9,
            clock_font: "gothic".to_string(),
            hero_fps: 2.0,
        }
    }
}

pub struct UiState {
    pub fps: f64,
    pub clock_font: ClockFont,
    pub meta: MetaState,
    pub offsets: UiOffsets,
    pub camera: Camera,
    pub hero: Hero,
}

impl UiState {
    pub fn new() -> Self {
        let hero = Hero::new(300, 120);
        let offsets = UiOffsets::default();
        let mut camera = Camera::new();
        camera.x = offsets.camera_x;
        camera.y = offsets.camera_y;
        Self {
            fps: 0.0,
            clock_font: ClockFont::Gothic,
            meta: MetaState::default(),
            offsets,
            camera,
            hero,
        }
    }

    pub fn load_or_new() -> Self {
        let mut state = Self::new();
        if let Ok(snapshot) = Self::load_snapshot() {
            state.clock_font = Self::parse_clock_font(&snapshot.offsets.clock_font);
            state.offsets = snapshot.offsets;
            state.meta = snapshot.meta;
        }
        state.camera.x = state.offsets.camera_x;
        state.camera.y = state.offsets.camera_y;
        state
    }

    pub fn next_font(&mut self) {
        self.clock_font = match self.clock_font {
            ClockFont::Small => ClockFont::Standard,
            ClockFont::Standard => ClockFont::Fender,
            ClockFont::Fender => ClockFont::Gothic,
            ClockFont::Gothic => ClockFont::Small,
        };
        self.offsets.clock_font = Self::clock_font_name(self.clock_font).to_string();
        self.save_state();
    }

    pub fn prev_font(&mut self) {
        self.clock_font = match self.clock_font {
            ClockFont::Small => ClockFont::Gothic,
            ClockFont::Standard => ClockFont::Small,
            ClockFont::Fender => ClockFont::Standard,
            ClockFont::Gothic => ClockFont::Fender,
        };
        self.offsets.clock_font = Self::clock_font_name(self.clock_font).to_string();
        self.save_state();
    }

    pub fn toggle_dev_mode(&mut self) {
        self.meta.toggle_dev_mode();
    }

    pub fn toggle_settings(&mut self) {
        self.meta.toggle_settings();
    }

    pub fn next_settings_tab(&mut self) {
        self.meta.next_settings_tab();
    }

    pub fn prev_settings_tab(&mut self) {
        self.meta.prev_settings_tab();
    }

    pub fn hero_clock_attachment(&self) -> crate::scene::entity::HeroClockAttachment {
        hero_and_clock_poses(
            crate::scene::coords::WorldPos {
                x: self.hero.x,
                y: self.hero.y,
            },
            crate::scene::coords::WorldPos {
                x: self.offsets.hero_dx,
                y: self.offsets.hero_dy,
            },
            crate::scene::coords::WorldPos {
                x: self.offsets.clock_dx as i32,
                y: self.offsets.clock_dy as i32,
            },
        )
    }

    pub fn move_hero_offset_left(&mut self) {
        self.offsets.hero_dx -= 1;
        self.save_state();
    }

    pub fn move_hero_offset_right(&mut self) {
        self.offsets.hero_dx += 1;
        self.save_state();
    }

    pub fn move_hero_offset_up(&mut self) {
        self.offsets.hero_dy -= 1;
        self.save_state();
    }

    pub fn move_hero_offset_down(&mut self) {
        self.offsets.hero_dy += 1;
        self.save_state();
    }

    pub fn adjust_clock_offset(&mut self, dx: i16, dy: i16) -> io::Result<()> {
        self.offsets.clock_dx = (self.offsets.clock_dx + dx).clamp(-200, 200);
        self.offsets.clock_dy = (self.offsets.clock_dy + dy).clamp(-200, 200);
        self.save_state();
        Ok(())
    }

    pub fn toggle_follow_hero(&mut self) {
        self.camera.follow_hero = !self.camera.follow_hero;
    }

    pub fn center_camera(&mut self) {
        self.camera.follow_hero = false;
        self.offsets.camera_x = 0;
        self.offsets.camera_y = 0;
        self.offsets.hero_dx = -110;
        self.offsets.hero_dy = -54;
        self.offsets.clock_dx = 96;
        self.offsets.clock_dy = 9;
        self.camera.x = 0;
        self.camera.y = 0;
        self.save_state();
    }

    pub fn increase_hero_fps(&mut self) {
        self.offsets.hero_fps = (self.offsets.hero_fps + 0.5).clamp(0.5, 120.0);
        self.save_state();
    }

    pub fn decrease_hero_fps(&mut self) {
        self.offsets.hero_fps = (self.offsets.hero_fps - 0.5).clamp(0.5, 120.0);
        self.save_state();
    }

    pub fn clamp_camera(&mut self, screen_w: i32, screen_h: i32) {
        use crate::scene::{CAMERA_OVERSCAN_CELLS, WORLD_HALF_H, WORLD_HALF_W};

        let min_x = -WORLD_HALF_W - CAMERA_OVERSCAN_CELLS;
        let max_x = WORLD_HALF_W - 1 + CAMERA_OVERSCAN_CELLS - screen_w + 1;
        let min_y = -WORLD_HALF_H - CAMERA_OVERSCAN_CELLS;
        let max_y = WORLD_HALF_H - 1 + CAMERA_OVERSCAN_CELLS - screen_h + 1;
        self.offsets.camera_x = clamp_axis(self.offsets.camera_x, min_x, max_x, screen_w);
        self.offsets.camera_y = clamp_axis(self.offsets.camera_y, min_y, max_y, screen_h);
        self.camera.x = self.offsets.camera_x;
        self.camera.y = self.offsets.camera_y;
    }

    #[allow(dead_code)]
    pub fn center_camera_to_viewport(&mut self, screen_w: i32, screen_h: i32) {
        use crate::scene::{WORLD_HALF_H, WORLD_HALF_W};

        self.offsets.camera_x = WORLD_HALF_W - screen_w / 2;
        self.offsets.camera_y = WORLD_HALF_H - screen_h / 2;
        self.camera.x = self.offsets.camera_x;
        self.camera.y = self.offsets.camera_y;
    }

    pub fn move_camera_left(&mut self) {
        self.camera.follow_hero = false;
        self.offsets.camera_x -= 1;
        self.camera.x = self.offsets.camera_x;
        self.save_state();
    }

    pub fn move_camera_right(&mut self) {
        self.camera.follow_hero = false;
        self.offsets.camera_x += 1;
        self.camera.x = self.offsets.camera_x;
        self.save_state();
    }

    pub fn move_camera_up(&mut self) {
        self.camera.follow_hero = false;
        self.offsets.camera_y -= 1;
        self.camera.y = self.offsets.camera_y;
        self.save_state();
    }

    pub fn move_camera_down(&mut self) {
        self.camera.follow_hero = false;
        self.offsets.camera_y += 1;
        self.camera.y = self.offsets.camera_y;
        self.save_state();
    }

    fn state_path() -> PathBuf {
        let home = std::env::var_os("HOME").unwrap_or_default();
        Path::new(&home)
            .join(".config")
            .join("yam")
            .join("state.json")
    }

    fn load_snapshot() -> io::Result<UiStateSnapshot> {
        let path = Self::state_path();
        let data = fs::read_to_string(path)?;
        Self::snapshot_from_json(&data)
    }

    fn snapshot_from_json(data: &str) -> io::Result<UiStateSnapshot> {
        if let Ok(snapshot) = serde_json::from_str::<UiStateSnapshot>(data) {
            return Ok(snapshot);
        }
        let offsets = serde_json::from_str::<UiOffsets>(data)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        Ok(UiStateSnapshot {
            offsets,
            meta: MetaState::default(),
        })
    }

    fn parse_clock_font(name: &str) -> ClockFont {
        match name {
            "small" => ClockFont::Small,
            "standard" => ClockFont::Standard,
            "fender" => ClockFont::Fender,
            "gothic" => ClockFont::Gothic,
            _ => ClockFont::Gothic,
        }
    }

    fn clock_font_name(font: ClockFont) -> &'static str {
        match font {
            ClockFont::Small => "small",
            ClockFont::Standard => "standard",
            ClockFont::Fender => "fender",
            ClockFont::Gothic => "gothic",
        }
    }

    fn save_state(&self) {
        let path = Self::state_path();
        if let Some(parent) = path.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                eprintln!("[yam] failed to create state dir: {err}");
                return;
            }
        }
        let snapshot = UiStateSnapshot {
            offsets: self.offsets.clone(),
            meta: self.meta.clone(),
        };
        match serde_json::to_string_pretty(&snapshot) {
            Ok(json) => {
                if let Err(err) = fs::write(path, json) {
                    eprintln!("[yam] failed to write state: {err}");
                }
            }
            Err(err) => eprintln!("[yam] failed to encode state: {err}"),
        }
    }
}

fn clamp_axis(value: i32, min: i32, max: i32, viewport_len: i32) -> i32 {
    if min > max {
        -(viewport_len / 2)
    } else {
        value.clamp(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::{MetaState, SettingsTab, UiOffsets, UiState, UiStateSnapshot};
    use crate::scene::coords::WorldPos;

    #[test]
    fn clamp_camera_limits_windowed_pan_to_one_cell_overscan() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = 500;
        ui.offsets.camera_y = -500;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;

        ui.clamp_camera(124, 32);

        assert_eq!(ui.offsets.camera_x, -17);
        assert_eq!(ui.offsets.camera_y, -29);
        assert_eq!(ui.camera.x, ui.offsets.camera_x);
        assert_eq!(ui.camera.y, ui.offsets.camera_y);
    }

    #[test]
    fn hero_clock_attachment_uses_ui_offsets_as_runtime_source_of_truth() {
        let ui = UiState::new();
        let attachment = ui.hero_clock_attachment();

        assert_eq!(attachment.hero_world(), WorldPos { x: 150, y: 60 });
        assert_eq!(attachment.hero_visual_anchor(), WorldPos { x: 40, y: 6 });
        assert_eq!(attachment.clock_world(), WorldPos { x: 136, y: 15 });
    }

    #[test]
    fn hero_clock_attachment_reflects_offset_changes() {
        let mut ui = UiState::new();
        ui.offsets.hero_dx = -100;
        ui.offsets.hero_dy = -50;
        ui.offsets.clock_dx = 12;
        ui.offsets.clock_dy = -3;

        let attachment = ui.hero_clock_attachment();

        assert_eq!(attachment.hero_world(), WorldPos { x: 150, y: 60 });
        assert_eq!(attachment.hero_visual_anchor(), WorldPos { x: 50, y: 10 });
        assert_eq!(attachment.clock_world(), WorldPos { x: 62, y: 7 });
    }

    #[test]
    fn toggling_meta_does_not_change_attachment_facts() {
        let mut ui = UiState::new();
        let baseline = ui.hero_clock_attachment();

        ui.toggle_dev_mode();

        let after_toggle = ui.hero_clock_attachment();

        assert_eq!(baseline.hero_world(), after_toggle.hero_world());
        assert_eq!(
            baseline.hero_visual_anchor(),
            after_toggle.hero_visual_anchor()
        );
        assert_eq!(baseline.clock_world(), after_toggle.clock_world());
    }

    #[test]
    fn snapshot_round_trips_meta_and_offsets() {
        let snapshot = UiStateSnapshot {
            offsets: UiOffsets {
                camera_x: 12,
                camera_y: -8,
                hero_dx: -91,
                hero_dy: -43,
                clock_dx: 77,
                clock_dy: -5,
                clock_font: "fender".to_string(),
                hero_fps: 4.5,
            },
            meta: MetaState {
                dev_mode: true,
                settings_open: true,
                settings_tab: SettingsTab::Theme,
            },
        };

        let json = serde_json::to_string(&snapshot).expect("snapshot should serialize");
        let round_trip: UiStateSnapshot =
            serde_json::from_str(&json).expect("snapshot should deserialize");

        assert_eq!(round_trip.offsets.camera_x, 12);
        assert_eq!(round_trip.offsets.camera_y, -8);
        assert_eq!(round_trip.offsets.hero_dx, -91);
        assert_eq!(round_trip.offsets.hero_dy, -43);
        assert_eq!(round_trip.offsets.clock_dx, 77);
        assert_eq!(round_trip.offsets.clock_dy, -5);
        assert_eq!(round_trip.offsets.clock_font, "fender");
        assert_eq!(round_trip.offsets.hero_fps, 4.5);
        assert!(round_trip.meta.dev_mode);
        assert!(round_trip.meta.settings_open);
        assert_eq!(round_trip.meta.settings_tab, SettingsTab::Theme);
    }

    #[test]
    fn legacy_offsets_only_snapshot_defaults_meta() {
        let legacy = serde_json::json!({
            "camera_x": 3,
            "camera_y": -4,
            "hero_dx": -9,
            "hero_dy": -8,
            "clock_dx": 7,
            "clock_dy": -6,
            "clock_font": "small",
            "hero_fps": 1.5
        })
        .to_string();

        let snapshot = UiState::snapshot_from_json(&legacy).expect("legacy snapshot should load");

        assert_eq!(snapshot.offsets.camera_x, 3);
        assert_eq!(snapshot.offsets.camera_y, -4);
        assert_eq!(snapshot.offsets.hero_dx, -9);
        assert_eq!(snapshot.offsets.hero_dy, -8);
        assert_eq!(snapshot.offsets.clock_dx, 7);
        assert_eq!(snapshot.offsets.clock_dy, -6);
        assert_eq!(snapshot.offsets.clock_font, "small");
        assert_eq!(snapshot.offsets.hero_fps, 1.5);
        assert!(!snapshot.meta.dev_mode);
        assert!(!snapshot.meta.settings_open);
        assert_eq!(snapshot.meta.settings_tab, SettingsTab::Positions);
    }
}
