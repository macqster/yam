use crate::render::fonts::ClockFont;
use crate::ui::camera::Camera;

pub struct UiState {
    pub clock_font: ClockFont,
    pub debug_layout: bool,
    pub anchored_clock: bool,
    pub hero_offset_x: i32,
    pub hero_offset_y: i32,
    pub camera: Camera,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            clock_font: ClockFont::Small,
            debug_layout: false,
            anchored_clock: false,
            hero_offset_x: 0,
            hero_offset_y: 0,
            camera: Camera::new(),
        }
    }

    pub fn next_font(&mut self) {
        self.clock_font = match self.clock_font {
            ClockFont::Small => ClockFont::Standard,
            ClockFont::Standard => ClockFont::Fender,
            ClockFont::Fender => ClockFont::Gothic,
            ClockFont::Gothic => ClockFont::Small,
        };
    }

    pub fn prev_font(&mut self) {
        self.clock_font = match self.clock_font {
            ClockFont::Small => ClockFont::Gothic,
            ClockFont::Standard => ClockFont::Small,
            ClockFont::Fender => ClockFont::Standard,
            ClockFont::Gothic => ClockFont::Fender,
        };
    }

    pub fn toggle_debug_layout(&mut self) {
        self.debug_layout = !self.debug_layout;
    }

    pub fn toggle_clock_mode(&mut self) {
        self.anchored_clock = !self.anchored_clock;
    }

    pub fn move_hero_offset_left(&mut self) {
        self.hero_offset_x -= 1;
    }

    pub fn move_hero_offset_right(&mut self) {
        self.hero_offset_x += 1;
    }

    pub fn move_hero_offset_up(&mut self) {
        self.hero_offset_y -= 1;
    }

    pub fn move_hero_offset_down(&mut self) {
        self.hero_offset_y += 1;
    }

    pub fn toggle_follow_hero(&mut self) {
        self.camera.follow_hero = !self.camera.follow_hero;
    }
}
