use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::core::world::WorldKind;
use crate::render::fonts::ClockFont;
use crate::render::hero::Hero;
use crate::scene::camera::Camera;
use crate::scene::entity::hero_and_clock_poses;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MetaState {
    #[serde(rename = "debug_layout")]
    pub dev_mode: bool,
    pub active_world: WorldKindSnapshot,
    pub vines_visible: bool,
    pub hotkeys_open: bool,
    pub move_mode_open: bool,
    pub settings_open: bool,
    pub pointer_probe_open: bool,
    pub world_frame_visible: bool,
    pub world_axis_visible: bool,
    pub world_datum_visible: bool,
    pub settings_tab: SettingsTab,
    pub settings_cursor: SettingsCursor,
    pub move_target: MoveTarget,
}

impl MetaState {
    pub fn new() -> Self {
        Self {
            dev_mode: false,
            active_world: WorldKindSnapshot::MainScene,
            vines_visible: true,
            hotkeys_open: false,
            move_mode_open: false,
            settings_open: false,
            pointer_probe_open: false,
            world_frame_visible: true,
            world_axis_visible: true,
            world_datum_visible: true,
            settings_tab: SettingsTab::default(),
            settings_cursor: SettingsCursor::default(),
            move_target: MoveTarget::default(),
        }
    }

    pub fn toggle_dev_mode(&mut self) {
        self.dev_mode = !self.dev_mode;
        if !self.dev_mode {
            self.hotkeys_open = false;
            self.move_mode_open = false;
            self.settings_open = false;
            self.pointer_probe_open = false;
        }
    }

    pub fn toggle_hotkeys(&mut self) {
        self.hotkeys_open = !self.hotkeys_open;
        if self.hotkeys_open {
            self.move_mode_open = false;
            self.settings_open = false;
            self.pointer_probe_open = false;
        }
    }

    pub fn toggle_move_mode(&mut self) {
        self.move_mode_open = !self.move_mode_open;
        if self.move_mode_open {
            self.hotkeys_open = false;
            self.settings_open = false;
            self.pointer_probe_open = false;
        }
    }

    pub fn toggle_settings(&mut self) {
        self.settings_open = !self.settings_open;
        if self.settings_open {
            self.hotkeys_open = false;
            self.move_mode_open = false;
            self.pointer_probe_open = false;
        }
    }

    pub fn toggle_pointer_probe(&mut self) {
        self.pointer_probe_open = !self.pointer_probe_open;
        if self.pointer_probe_open {
            self.hotkeys_open = false;
            self.move_mode_open = false;
            self.settings_open = false;
        }
    }

    pub fn select_move_target(&mut self, target: MoveTarget) {
        self.move_target = target;
    }

    pub fn toggle_vines_visible(&mut self) {
        self.vines_visible = !self.vines_visible;
    }

    pub fn active_world_kind(&self) -> WorldKind {
        match self.active_world {
            WorldKindSnapshot::MainScene => WorldKind::MainScene,
            WorldKindSnapshot::Sandbox => WorldKind::Sandbox,
        }
    }

    pub fn cycle_world_kind(&mut self) {
        self.active_world = match self.active_world {
            WorldKindSnapshot::MainScene => WorldKindSnapshot::Sandbox,
            WorldKindSnapshot::Sandbox => WorldKindSnapshot::MainScene,
        };
        if self.active_world == WorldKindSnapshot::MainScene {
            self.pointer_probe_open = false;
        }
    }

    pub fn next_settings_tab(&mut self) {
        self.settings_tab = self.settings_tab.next();
    }

    pub fn prev_settings_tab(&mut self) {
        self.settings_tab = self.settings_tab.prev();
    }

    pub fn selected_settings_row(&self) -> u16 {
        self.settings_cursor.row(self.settings_tab)
    }

    pub fn select_prev_settings_row(&mut self) {
        self.settings_cursor.set_row(
            self.settings_tab,
            self.selected_settings_row().saturating_sub(1),
        );
    }
}

impl Default for MetaState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum WorldKindSnapshot {
    #[default]
    MainScene,
    Sandbox,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoveTarget {
    #[default]
    Hero,
    Clock,
    Weather,
}

impl MoveTarget {
    pub fn title(self) -> &'static str {
        match self {
            MoveTarget::Hero => "hero",
            MoveTarget::Clock => "clock",
            MoveTarget::Weather => "weather",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum SettingsTab {
    #[default]
    Positions,
    #[serde(alias = "Widgets")]
    Ui,
    Gif,
    Theme,
}

impl SettingsTab {
    pub fn next(self) -> Self {
        match self {
            SettingsTab::Positions => SettingsTab::Ui,
            SettingsTab::Ui => SettingsTab::Gif,
            SettingsTab::Gif => SettingsTab::Theme,
            SettingsTab::Theme => SettingsTab::Positions,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            SettingsTab::Positions => SettingsTab::Theme,
            SettingsTab::Ui => SettingsTab::Positions,
            SettingsTab::Gif => SettingsTab::Ui,
            SettingsTab::Theme => SettingsTab::Gif,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            SettingsTab::Positions => "positions",
            SettingsTab::Ui => "ui",
            SettingsTab::Gif => "gif",
            SettingsTab::Theme => "theme",
        }
    }

    pub fn item_count(self) -> usize {
        match self {
            SettingsTab::Positions => 3,
            SettingsTab::Ui => 3,
            SettingsTab::Gif => 3,
            SettingsTab::Theme => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct SettingsCursor {
    pub positions: u16,
    pub widgets: u16,
    pub gif: u16,
    pub theme: u16,
}

impl SettingsCursor {
    pub fn row(self, tab: SettingsTab) -> u16 {
        match tab {
            SettingsTab::Positions => self.positions,
            SettingsTab::Ui => self.widgets,
            SettingsTab::Gif => self.gif,
            SettingsTab::Theme => self.theme,
        }
    }

    pub fn set_row(&mut self, tab: SettingsTab, row: u16) {
        match tab {
            SettingsTab::Positions => self.positions = row,
            SettingsTab::Ui => self.widgets = row,
            SettingsTab::Gif => self.gif = row,
            SettingsTab::Theme => self.theme = row,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SettingsAxisField {
    #[default]
    X,
    Y,
}

impl SettingsAxisField {
    pub fn other(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SettingsEditState {
    pub active: bool,
    pub row: u16,
    pub field: SettingsAxisField,
    pub x_buffer: String,
    pub y_buffer: String,
}

impl SettingsEditState {
    pub fn clear(&mut self) {
        *self = Self::default();
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
    pub camera_home_x: i32,
    pub camera_home_y: i32,
    pub pointer_x: i32,
    pub pointer_y: i32,
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
            camera_x: -63,
            camera_y: -17,
            camera_home_x: -63,
            camera_home_y: -17,
            pointer_x: 0,
            pointer_y: 0,
            hero_dx: -218,
            hero_dy: -39,
            clock_dx: 95,
            clock_dy: -10,
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
    pub pointer_blink_on: bool,
    pub settings_edit: SettingsEditState,
}

impl UiState {
    pub fn new() -> Self {
        let hero = default_hero();
        let offsets = UiOffsets::default();
        let mut camera = Camera::new();
        camera.x = offsets.camera_x;
        camera.y = offsets.camera_y;
        Self {
            fps: 0.0,
            clock_font: ClockFont::Gothic,
            meta: MetaState::new(),
            offsets,
            camera,
            hero,
            pointer_blink_on: true,
            settings_edit: SettingsEditState::default(),
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
        state.pointer_blink_on = true;
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
        if !self.meta.dev_mode {
            self.settings_edit.clear();
        }
    }

    pub fn toggle_hotkeys(&mut self) {
        self.meta.toggle_hotkeys();
    }

    pub fn toggle_move_mode(&mut self) {
        self.meta.toggle_move_mode();
    }

    pub fn toggle_settings(&mut self) {
        self.meta.toggle_settings();
        if !self.meta.settings_open {
            self.settings_edit.clear();
        }
    }

    pub fn toggle_pointer_probe(&mut self) {
        self.meta.toggle_pointer_probe();
    }

    pub fn toggle_vines_visible(&mut self) {
        self.meta.toggle_vines_visible();
    }

    pub fn active_world_kind(&self) -> WorldKind {
        self.meta.active_world_kind()
    }

    pub fn cycle_world_kind(&mut self) {
        self.meta.cycle_world_kind();
        if self.meta.active_world_kind() != WorldKind::Sandbox {
            self.pointer_blink_on = true;
        }
        self.clamp_settings_cursor_to_world();
        self.save_state();
    }

    pub fn next_settings_tab(&mut self) {
        self.meta.next_settings_tab();
        self.clamp_settings_cursor_to_world();
        self.settings_edit.clear();
        self.save_state();
    }

    pub fn prev_settings_tab(&mut self) {
        self.meta.prev_settings_tab();
        self.clamp_settings_cursor_to_world();
        self.settings_edit.clear();
        self.save_state();
    }

    pub fn select_prev_settings_row(&mut self) {
        self.meta.select_prev_settings_row();
        self.clamp_settings_cursor_to_world();
        self.settings_edit.clear();
        self.save_state();
    }

    pub fn select_next_settings_row(&mut self) {
        let max_row = self
            .settings_item_count(self.meta.settings_tab)
            .saturating_sub(1) as u16;
        let next = self
            .meta
            .selected_settings_row()
            .saturating_add(1)
            .min(max_row);
        self.meta
            .settings_cursor
            .set_row(self.meta.settings_tab, next);
        self.settings_edit.clear();
        self.save_state();
    }

    pub fn begin_settings_edit(&mut self) {
        self.begin_settings_edit_with_viewport(
            crate::scene::WORLD_WIDTH as u16,
            crate::scene::WORLD_HEIGHT as u16,
        );
    }

    pub fn begin_settings_edit_with_viewport(&mut self, viewport_width: u16, viewport_height: u16) {
        if self.meta.settings_tab != SettingsTab::Positions {
            return;
        }
        if !self.world_has_scene_companions() && self.meta.selected_settings_row() > 0 {
            return;
        }
        if self.meta.selected_settings_row() == 0
            && crate::scene::viewport_covers_full_world(viewport_width, viewport_height)
        {
            return;
        }
        let (x, y) = match self.meta.selected_settings_row() {
            0 => (self.offsets.camera_x, self.offsets.camera_y),
            1 => (self.offsets.hero_dx, self.offsets.hero_dy),
            2 => (self.offsets.clock_dx as i32, self.offsets.clock_dy as i32),
            _ => return,
        };
        self.settings_edit.active = true;
        self.settings_edit.row = self.meta.selected_settings_row();
        self.settings_edit.field = SettingsAxisField::X;
        self.settings_edit.x_buffer = x.to_string();
        self.settings_edit.y_buffer = y.to_string();
    }

    pub fn cancel_settings_edit(&mut self) {
        self.settings_edit.clear();
    }

    pub fn activate_selected_setting_with_viewport(
        &mut self,
        viewport_width: u16,
        viewport_height: u16,
    ) -> io::Result<()> {
        match self.meta.settings_tab {
            SettingsTab::Positions => {
                if self.settings_edit.active {
                    self.commit_settings_edit()
                } else {
                    self.begin_settings_edit_with_viewport(viewport_width, viewport_height);
                    Ok(())
                }
            }
            SettingsTab::Ui => {
                match self.meta.selected_settings_row() {
                    0 => self.meta.world_frame_visible = !self.meta.world_frame_visible,
                    1 => self.meta.world_axis_visible = !self.meta.world_axis_visible,
                    2 => self.meta.world_datum_visible = !self.meta.world_datum_visible,
                    _ => {}
                }
                self.save_state();
                Ok(())
            }
            SettingsTab::Gif | SettingsTab::Theme => Ok(()),
        }
    }

    pub fn toggle_settings_edit_field(&mut self) {
        if self.settings_edit.active {
            self.settings_edit.field = self.settings_edit.field.other();
        }
    }

    pub fn close_move_mode(&mut self) {
        self.meta.move_mode_open = false;
    }

    pub fn close_settings(&mut self) {
        self.meta.settings_open = false;
        self.settings_edit.clear();
    }

    pub fn settings_edit_backspace(&mut self) {
        if !self.settings_edit.active {
            return;
        }
        self.active_settings_buffer_mut().pop();
    }

    pub fn settings_edit_insert_char(&mut self, ch: char) {
        if !self.settings_edit.active || !matches!(ch, '0'..='9' | '-') {
            return;
        }
        let buffer = self.active_settings_buffer_mut();
        if ch == '-' {
            if buffer.is_empty() {
                buffer.push(ch);
            }
            return;
        }
        buffer.push(ch);
    }

    pub fn commit_settings_edit(&mut self) -> io::Result<()> {
        if !self.settings_edit.active {
            self.begin_settings_edit();
            return Ok(());
        }

        let parse_axis = |value: &str| -> Option<i32> {
            if value.is_empty() || value == "-" {
                None
            } else {
                value.parse::<i32>().ok()
            }
        };

        let Some(x) = parse_axis(&self.settings_edit.x_buffer) else {
            return Ok(());
        };
        let Some(y) = parse_axis(&self.settings_edit.y_buffer) else {
            return Ok(());
        };

        match self.settings_edit.row {
            0 => {
                self.offsets.camera_x = x;
                self.offsets.camera_y = y;
                self.camera.follow_hero = false;
                self.camera.x = x;
                self.camera.y = y;
            }
            1 => {
                self.offsets.hero_dx = x;
                self.offsets.hero_dy = y;
            }
            2 => {
                self.offsets.clock_dx = x.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                self.offsets.clock_dy = y.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            }
            _ => {}
        }

        self.settings_edit.clear();
        self.save_state();
        Ok(())
    }

    fn active_settings_buffer_mut(&mut self) -> &mut String {
        match self.settings_edit.field {
            SettingsAxisField::X => &mut self.settings_edit.x_buffer,
            SettingsAxisField::Y => &mut self.settings_edit.y_buffer,
        }
    }

    pub fn world_has_scene_companions(&self) -> bool {
        self.meta.active_world_kind() == WorldKind::MainScene
    }

    pub fn settings_item_count(&self, tab: SettingsTab) -> usize {
        match (self.meta.active_world_kind(), tab) {
            (WorldKind::Sandbox, SettingsTab::Positions) => 1,
            (WorldKind::Sandbox, SettingsTab::Gif) => 1,
            _ => tab.item_count(),
        }
    }

    fn clamp_settings_cursor_to_world(&mut self) {
        for tab in [
            SettingsTab::Positions,
            SettingsTab::Ui,
            SettingsTab::Gif,
            SettingsTab::Theme,
        ] {
            let max_row = self.settings_item_count(tab).saturating_sub(1) as u16;
            let current = self.meta.settings_cursor.row(tab);
            self.meta.settings_cursor.set_row(tab, current.min(max_row));
        }
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

    pub fn move_selected_target_left(&mut self) -> io::Result<()> {
        self.move_selected_target(-1, 0)
    }

    pub fn move_selected_target_right(&mut self) -> io::Result<()> {
        self.move_selected_target(1, 0)
    }

    pub fn move_selected_target_up(&mut self) -> io::Result<()> {
        self.move_selected_target(0, 1)
    }

    pub fn move_selected_target_down(&mut self) -> io::Result<()> {
        self.move_selected_target(0, -1)
    }

    pub fn move_selected_target(&mut self, dx: i16, dy: i16) -> io::Result<()> {
        match self.meta.move_target {
            MoveTarget::Hero => {
                self.offsets.hero_dx += dx as i32;
                self.offsets.hero_dy += dy as i32;
            }
            MoveTarget::Clock => {
                self.offsets.clock_dx = (self.offsets.clock_dx + dx).clamp(-200, 200);
                self.offsets.clock_dy = (self.offsets.clock_dy + dy).clamp(-200, 200);
            }
            MoveTarget::Weather => {}
        }
        self.save_state();
        Ok(())
    }

    pub fn move_pointer_left(&mut self) {
        self.offsets.pointer_x -= 1;
        self.save_state();
    }

    pub fn move_pointer_right(&mut self) {
        self.offsets.pointer_x += 1;
        self.save_state();
    }

    pub fn move_pointer_up(&mut self) {
        self.offsets.pointer_y += 1;
        self.save_state();
    }

    pub fn move_pointer_down(&mut self) {
        self.offsets.pointer_y -= 1;
        self.save_state();
    }

    pub fn toggle_follow_hero(&mut self) {
        self.camera.follow_hero = !self.camera.follow_hero;
    }

    pub fn store_camera_home(&mut self) {
        self.offsets.camera_home_x = self.offsets.camera_x;
        self.offsets.camera_home_y = self.offsets.camera_y;
        self.save_state();
    }

    pub fn recall_camera_home(&mut self) {
        self.camera.follow_hero = false;
        self.offsets.camera_x = self.offsets.camera_home_x;
        self.offsets.camera_y = self.offsets.camera_home_y;
        self.camera.x = self.offsets.camera_x;
        self.camera.y = self.offsets.camera_y;
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

    pub fn sync_camera_to_viewport_center(&mut self, screen_w: i32, screen_h: i32) {
        self.offsets.camera_x = -(screen_w / 2);
        self.offsets.camera_y = -(screen_h / 2);
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
        self.offsets.camera_y += 1;
        self.camera.y = self.offsets.camera_y;
        self.save_state();
    }

    pub fn move_camera_down(&mut self) {
        self.camera.follow_hero = false;
        self.offsets.camera_y -= 1;
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

#[cfg(test)]
fn default_hero() -> Hero {
    Hero::test_stub(300, 120)
}

#[cfg(not(test))]
fn default_hero() -> Hero {
    Hero::new(300, 120)
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
    use super::{
        MetaState, MoveTarget, SettingsAxisField, SettingsCursor, SettingsTab, UiOffsets, UiState,
        UiStateSnapshot, WorldKindSnapshot,
    };
    use crate::scene::coords::WorldPos;

    #[test]
    fn clamp_camera_limits_windowed_pan_to_one_cell_overscan() {
        let mut ui = UiState::new();
        ui.camera.follow_hero = false;
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
    fn follow_hero_camera_syncs_to_viewport_center_without_disabling_follow_mode() {
        let mut ui = UiState::new();
        ui.camera.follow_hero = true;

        ui.sync_camera_to_viewport_center(124, 32);

        assert!(ui.camera.follow_hero);
        assert_eq!(ui.offsets.camera_x, -62);
        assert_eq!(ui.offsets.camera_y, -16);
        assert_eq!(ui.camera.x, -62);
        assert_eq!(ui.camera.y, -16);
    }

    #[test]
    fn store_camera_home_records_the_current_camera_position() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = -18;
        ui.offsets.camera_y = 9;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;

        ui.store_camera_home();

        assert_eq!(ui.offsets.camera_home_x, -18);
        assert_eq!(ui.offsets.camera_home_y, 9);
    }

    #[test]
    fn recall_camera_home_restores_the_saved_camera_position() {
        let mut ui = UiState::new();
        ui.offsets.camera_home_x = -21;
        ui.offsets.camera_home_y = 7;
        ui.offsets.camera_x = 50;
        ui.offsets.camera_y = -12;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;
        ui.camera.follow_hero = true;

        ui.recall_camera_home();

        assert!(!ui.camera.follow_hero);
        assert_eq!(ui.offsets.camera_x, -21);
        assert_eq!(ui.offsets.camera_y, 7);
        assert_eq!(ui.camera.x, -21);
        assert_eq!(ui.camera.y, 7);
    }

    #[test]
    fn ui_state_starts_with_the_default_home_seed() {
        let ui = UiState::new();

        assert!(!ui.camera.follow_hero);
        assert_eq!(ui.offsets.camera_x, -63);
        assert_eq!(ui.offsets.camera_y, -17);
        assert_eq!(ui.offsets.camera_home_x, -63);
        assert_eq!(ui.offsets.camera_home_y, -17);
        assert_eq!(ui.camera.x, ui.offsets.camera_x);
        assert_eq!(ui.camera.y, ui.offsets.camera_y);
    }

    #[test]
    fn hotkeys_and_settings_popups_are_mutually_exclusive() {
        let mut ui = UiState::new();

        ui.toggle_hotkeys();
        assert!(ui.meta.hotkeys_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.pointer_probe_open);

        ui.toggle_settings();
        assert!(ui.meta.settings_open);
        assert!(!ui.meta.hotkeys_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.pointer_probe_open);
    }

    #[test]
    fn disabling_dev_mode_closes_all_modal_overlays() {
        let mut ui = UiState::new();

        ui.toggle_dev_mode();
        ui.toggle_hotkeys();
        ui.toggle_move_mode();
        ui.toggle_settings();
        assert!(ui.meta.dev_mode);
        assert!(ui.meta.settings_open);

        ui.toggle_dev_mode();

        assert!(!ui.meta.dev_mode);
        assert!(!ui.meta.hotkeys_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.pointer_probe_open);
    }

    #[test]
    fn move_mode_and_popups_are_mutually_exclusive() {
        let mut ui = UiState::new();

        ui.toggle_move_mode();
        assert!(ui.meta.move_mode_open);
        assert!(!ui.meta.hotkeys_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.pointer_probe_open);

        ui.toggle_hotkeys();
        assert!(ui.meta.hotkeys_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.pointer_probe_open);
    }

    #[test]
    fn pointer_probe_and_popups_are_mutually_exclusive() {
        let mut ui = UiState::new();

        ui.toggle_pointer_probe();
        assert!(ui.meta.pointer_probe_open);
        assert!(!ui.meta.hotkeys_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.settings_open);

        ui.toggle_hotkeys();
        assert!(ui.meta.hotkeys_open);
        assert!(!ui.meta.pointer_probe_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.settings_open);
    }

    #[test]
    fn hero_clock_attachment_uses_ui_offsets_as_runtime_source_of_truth() {
        let ui = UiState::new();
        let attachment = ui.hero_clock_attachment();

        assert_eq!(attachment.hero_world(), WorldPos { x: 150, y: 60 });
        assert_eq!(attachment.hero_visual_anchor(), WorldPos { x: -68, y: 21 });
        assert_eq!(attachment.clock_world(), WorldPos { x: 27, y: 11 });
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
    fn move_target_selection_changes_without_touching_attachment_facts() {
        let mut ui = UiState::new();
        let baseline = ui.hero_clock_attachment();

        ui.toggle_move_mode();
        ui.meta.select_move_target(MoveTarget::Clock);

        let after_select = ui.hero_clock_attachment();

        assert_eq!(baseline.hero_world(), after_select.hero_world());
        assert_eq!(
            baseline.hero_visual_anchor(),
            after_select.hero_visual_anchor()
        );
        assert_eq!(baseline.clock_world(), after_select.clock_world());
        assert_eq!(ui.meta.move_target, MoveTarget::Clock);
    }

    #[test]
    fn snapshot_round_trips_meta_and_offsets() {
        let snapshot = UiStateSnapshot {
            offsets: UiOffsets {
                camera_x: 12,
                camera_y: -8,
                camera_home_x: -63,
                camera_home_y: -17,
                pointer_x: 15,
                pointer_y: -2,
                hero_dx: -91,
                hero_dy: -43,
                clock_dx: 77,
                clock_dy: -5,
                clock_font: "fender".to_string(),
                hero_fps: 4.5,
            },
            meta: MetaState {
                dev_mode: true,
                active_world: WorldKindSnapshot::Sandbox,
                vines_visible: false,
                hotkeys_open: false,
                move_mode_open: true,
                settings_open: true,
                pointer_probe_open: true,
                world_frame_visible: false,
                world_axis_visible: true,
                world_datum_visible: false,
                settings_tab: SettingsTab::Theme,
                settings_cursor: SettingsCursor {
                    positions: 1,
                    widgets: 2,
                    gif: 0,
                    theme: 1,
                },
                move_target: MoveTarget::Hero,
            },
        };

        let json = serde_json::to_string(&snapshot).expect("snapshot should serialize");
        let round_trip: UiStateSnapshot =
            serde_json::from_str(&json).expect("snapshot should deserialize");

        assert_eq!(round_trip.offsets.camera_x, 12);
        assert_eq!(round_trip.offsets.camera_y, -8);
        assert_eq!(round_trip.offsets.camera_home_x, -63);
        assert_eq!(round_trip.offsets.camera_home_y, -17);
        assert_eq!(round_trip.offsets.pointer_x, 15);
        assert_eq!(round_trip.offsets.pointer_y, -2);
        assert_eq!(round_trip.offsets.hero_dx, -91);
        assert_eq!(round_trip.offsets.hero_dy, -43);
        assert_eq!(round_trip.offsets.clock_dx, 77);
        assert_eq!(round_trip.offsets.clock_dy, -5);
        assert_eq!(round_trip.offsets.clock_font, "fender");
        assert_eq!(round_trip.offsets.hero_fps, 4.5);
        assert!(round_trip.meta.dev_mode);
        assert!(!round_trip.meta.vines_visible);
        assert!(round_trip.meta.settings_open);
        assert!(round_trip.meta.move_mode_open);
        assert!(round_trip.meta.pointer_probe_open);
        assert!(!round_trip.meta.world_frame_visible);
        assert!(round_trip.meta.world_axis_visible);
        assert!(!round_trip.meta.world_datum_visible);
        assert_eq!(round_trip.meta.settings_tab, SettingsTab::Theme);
        assert_eq!(round_trip.meta.settings_cursor.positions, 1);
        assert_eq!(round_trip.meta.settings_cursor.widgets, 2);
        assert_eq!(round_trip.meta.settings_cursor.gif, 0);
        assert_eq!(round_trip.meta.settings_cursor.theme, 1);
        assert_eq!(round_trip.meta.move_target, MoveTarget::Hero);
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
        assert_eq!(snapshot.offsets.camera_home_x, -63);
        assert_eq!(snapshot.offsets.camera_home_y, -17);
        assert_eq!(snapshot.offsets.pointer_x, 0);
        assert_eq!(snapshot.offsets.pointer_y, 0);
        assert_eq!(snapshot.offsets.hero_dx, -9);
        assert_eq!(snapshot.offsets.hero_dy, -8);
        assert_eq!(snapshot.offsets.clock_dx, 7);
        assert_eq!(snapshot.offsets.clock_dy, -6);
        assert_eq!(snapshot.offsets.clock_font, "small");
        assert_eq!(snapshot.offsets.hero_fps, 1.5);
        assert!(!snapshot.meta.dev_mode);
        assert!(!snapshot.meta.move_mode_open);
        assert!(!snapshot.meta.settings_open);
        assert!(!snapshot.meta.pointer_probe_open);
        assert!(snapshot.meta.world_frame_visible);
        assert!(snapshot.meta.world_axis_visible);
        assert!(snapshot.meta.world_datum_visible);
        assert!(snapshot.meta.vines_visible);
        assert_eq!(snapshot.meta.settings_tab, SettingsTab::Positions);
        assert_eq!(snapshot.meta.settings_cursor, SettingsCursor::default());
        assert_eq!(snapshot.meta.move_target, MoveTarget::Hero);
    }

    #[test]
    fn store_and_recall_camera_home_round_trip_the_current_manual_position() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = -18;
        ui.offsets.camera_y = 9;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;
        ui.camera.follow_hero = true;

        ui.store_camera_home();
        ui.offsets.camera_x = 7;
        ui.offsets.camera_y = -12;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;
        ui.camera.follow_hero = true;

        ui.recall_camera_home();

        assert!(!ui.camera.follow_hero);
        assert_eq!(ui.offsets.camera_home_x, -18);
        assert_eq!(ui.offsets.camera_home_y, 9);
        assert_eq!(ui.offsets.camera_x, -18);
        assert_eq!(ui.offsets.camera_y, 9);
        assert_eq!(ui.camera.x, -18);
        assert_eq!(ui.camera.y, 9);
    }

    #[test]
    fn pointer_probe_moves_and_persists_as_world_coordinates() {
        let mut ui = UiState::new();
        ui.meta.active_world = WorldKindSnapshot::Sandbox;

        ui.toggle_pointer_probe();
        ui.move_pointer_right();
        ui.move_pointer_up();
        ui.move_pointer_up();

        assert!(ui.meta.pointer_probe_open);
        assert_eq!(ui.offsets.pointer_x, 1);
        assert_eq!(ui.offsets.pointer_y, 2);
    }

    #[test]
    fn vines_visibility_defaults_on_and_can_be_toggled() {
        let mut ui = UiState::new();

        assert!(ui.meta.vines_visible);
        ui.toggle_vines_visible();
        assert!(!ui.meta.vines_visible);
        ui.toggle_vines_visible();
        assert!(ui.meta.vines_visible);
    }

    #[test]
    fn world_space_up_controls_increase_y_across_targets() {
        let mut ui = UiState::new();
        ui.offsets.hero_dy = 0;
        ui.offsets.clock_dy = 0;
        ui.offsets.pointer_y = 0;
        ui.offsets.camera_y = 0;
        ui.camera.y = 0;

        ui.meta.select_move_target(MoveTarget::Hero);
        ui.move_selected_target_up()
            .expect("hero move should succeed");
        assert_eq!(ui.offsets.hero_dy, 1);
        ui.move_selected_target_down()
            .expect("hero move should succeed");
        assert_eq!(ui.offsets.hero_dy, 0);

        ui.meta.select_move_target(MoveTarget::Clock);
        ui.move_selected_target_up()
            .expect("clock move should succeed");
        assert_eq!(ui.offsets.clock_dy, 1);
        ui.move_selected_target_down()
            .expect("clock move should succeed");
        assert_eq!(ui.offsets.clock_dy, 0);

        ui.move_pointer_up();
        assert_eq!(ui.offsets.pointer_y, 1);
        ui.move_pointer_down();
        assert_eq!(ui.offsets.pointer_y, 0);

        ui.move_camera_up();
        assert_eq!(ui.offsets.camera_y, 1);
        assert_eq!(ui.camera.y, 1);
        ui.move_camera_down();
        assert_eq!(ui.offsets.camera_y, 0);
        assert_eq!(ui.camera.y, 0);
    }

    #[test]
    fn positions_settings_edit_commits_camera_values() {
        let mut ui = UiState::new();
        ui.meta.dev_mode = true;
        ui.meta.settings_open = true;
        ui.meta.settings_tab = SettingsTab::Positions;

        ui.commit_settings_edit()
            .expect("begin edit should succeed");
        assert!(ui.settings_edit.active);
        assert_eq!(ui.settings_edit.row, 0);

        ui.settings_edit.x_buffer = "-105".to_string();
        ui.settings_edit.y_buffer = "-28".to_string();
        ui.commit_settings_edit()
            .expect("commit edit should succeed");

        assert!(!ui.settings_edit.active);
        assert_eq!(ui.offsets.camera_x, -105);
        assert_eq!(ui.offsets.camera_y, -28);
        assert_eq!(ui.camera.x, -105);
        assert_eq!(ui.camera.y, -28);
    }

    #[test]
    fn positions_settings_edit_switches_active_axis_field() {
        let mut ui = UiState::new();
        ui.meta.settings_tab = SettingsTab::Positions;
        ui.begin_settings_edit();

        assert_eq!(ui.settings_edit.field, SettingsAxisField::X);
        ui.toggle_settings_edit_field();
        assert_eq!(ui.settings_edit.field, SettingsAxisField::Y);
        ui.toggle_settings_edit_field();
        assert_eq!(ui.settings_edit.field, SettingsAxisField::X);
    }

    #[test]
    fn camera_settings_edit_does_not_open_when_viewport_already_covers_the_full_world() {
        let mut ui = UiState::new();
        ui.meta.settings_tab = SettingsTab::Positions;
        ui.meta.settings_cursor.positions = 0;

        ui.begin_settings_edit_with_viewport(212, 56);

        assert!(!ui.settings_edit.active);
    }

    #[test]
    fn sandbox_settings_item_counts_hide_main_scene_only_controls() {
        let mut ui = UiState::new();
        ui.meta.active_world = WorldKindSnapshot::Sandbox;

        assert_eq!(ui.settings_item_count(SettingsTab::Positions), 1);
        assert_eq!(ui.settings_item_count(SettingsTab::Ui), 3);
        assert_eq!(ui.settings_item_count(SettingsTab::Gif), 1);
        assert_eq!(ui.settings_item_count(SettingsTab::Theme), 3);
    }

    #[test]
    fn ui_settings_toggle_world_overlay_flags() {
        let mut ui = UiState::new();
        ui.meta.settings_tab = SettingsTab::Ui;
        ui.meta.settings_cursor.widgets = 0;

        ui.activate_selected_setting_with_viewport(124, 32)
            .expect("ui toggle should succeed");
        assert!(!ui.meta.world_frame_visible);
        assert!(ui.meta.world_axis_visible);
        assert!(ui.meta.world_datum_visible);

        ui.meta.settings_cursor.widgets = 1;
        ui.activate_selected_setting_with_viewport(124, 32)
            .expect("ui toggle should succeed");
        assert!(!ui.meta.world_axis_visible);

        ui.meta.settings_cursor.widgets = 2;
        ui.activate_selected_setting_with_viewport(124, 32)
            .expect("ui toggle should succeed");
        assert!(!ui.meta.world_datum_visible);
    }
}
