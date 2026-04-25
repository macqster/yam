use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::render::fonts::ClockFont;
use crate::render::hero::Hero;
use crate::scene::camera::Camera;

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
    pub debug_layout: bool,
    pub anchored_clock: bool,
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
            debug_layout: false,
            anchored_clock: false,
            clock_font: ClockFont::Gothic,
            offsets,
            camera,
            hero,
        }
    }

    pub fn load_or_new() -> Self {
        let mut state = Self::new();
        if let Ok(offsets) = Self::load_offsets() {
            state.clock_font = Self::parse_clock_font(&offsets.clock_font);
            state.offsets = offsets;
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

    pub fn toggle_debug_layout(&mut self) {
        self.debug_layout = !self.debug_layout;
    }

    pub fn toggle_clock_mode(&mut self) {
        self.anchored_clock = !self.anchored_clock;
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

    fn load_offsets() -> io::Result<UiOffsets> {
        let path = Self::state_path();
        let data = fs::read_to_string(path)?;
        serde_json::from_str(&data).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
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
        match serde_json::to_string_pretty(&self.offsets) {
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
    use super::UiState;

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
}
