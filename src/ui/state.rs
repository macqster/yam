use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use crate::core::world::WorldKind;
use crate::render::fonts::ClockFont;
use crate::render::hero::Hero;
use crate::scene::camera::Camera;
use crate::scene::entity::hero_scene_poses;
use crate::weather::model::{WeatherLocale, WeatherLocation, WeatherSnapshot};
use crate::weather::provider::{
    StaticWeatherProvider, WeatherError, WeatherProvider, WttrInWeatherProvider,
};
use crate::weather::render::WeatherLayout;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MetaState {
    #[serde(rename = "debug_layout")]
    pub dev_mode: bool,
    pub active_world: WorldKindSnapshot,
    pub vines_visible: bool,
    pub vines_visibility_mode: FeatureVisibilityMode,
    pub hotkeys_open: bool,
    pub move_mode_open: bool,
    pub palette_open: bool,
    pub weather_popup_open: bool,
    pub settings_open: bool,
    pub pointer_probe_open: bool,
    pub world_frame_visible: bool,
    pub world_axis_visible: bool,
    pub world_datum_visible: bool,
    pub sliders_visible: bool,
    pub debug_info_panel_visible: bool,
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
            vines_visibility_mode: FeatureVisibilityMode::On,
            hotkeys_open: false,
            move_mode_open: false,
            palette_open: false,
            weather_popup_open: false,
            settings_open: false,
            pointer_probe_open: false,
            world_frame_visible: true,
            world_axis_visible: true,
            world_datum_visible: true,
            sliders_visible: true,
            debug_info_panel_visible: true,
            settings_tab: SettingsTab::default(),
            settings_cursor: SettingsCursor::default(),
            move_target: MoveTarget::default(),
        }
    }

    fn close_dev_overlays(&mut self) {
        self.hotkeys_open = false;
        self.move_mode_open = false;
        self.palette_open = false;
        self.weather_popup_open = false;
        self.settings_open = false;
        self.pointer_probe_open = false;
    }

    pub fn toggle_dev_mode(&mut self) {
        self.dev_mode = !self.dev_mode;
        if !self.dev_mode {
            self.close_dev_overlays();
        }
    }

    pub fn toggle_hotkeys(&mut self) {
        self.hotkeys_open = !self.hotkeys_open;
        if self.hotkeys_open {
            self.close_dev_overlays();
            self.hotkeys_open = true;
        }
    }

    pub fn toggle_move_mode(&mut self) {
        self.move_mode_open = !self.move_mode_open;
        if self.move_mode_open {
            self.close_dev_overlays();
            self.move_mode_open = true;
        }
    }

    pub fn toggle_palette(&mut self) {
        self.palette_open = !self.palette_open;
        if self.palette_open {
            self.close_dev_overlays();
            self.palette_open = true;
        }
    }

    pub fn toggle_weather_popup(&mut self) {
        self.weather_popup_open = !self.weather_popup_open;
        if self.weather_popup_open {
            self.close_dev_overlays();
            self.weather_popup_open = true;
        }
    }

    pub fn toggle_settings(&mut self) {
        self.settings_open = !self.settings_open;
        if self.settings_open {
            self.close_dev_overlays();
            self.settings_open = true;
        }
    }

    pub fn toggle_pointer_probe(&mut self) {
        self.pointer_probe_open = !self.pointer_probe_open;
        if self.pointer_probe_open {
            self.close_dev_overlays();
            self.pointer_probe_open = true;
        }
    }

    pub fn select_move_target(&mut self, target: MoveTarget) {
        self.move_target = target;
    }

    pub fn toggle_vines_visible(&mut self) {
        self.vines_visible = !self.vines_visible;
    }

    pub fn cycle_vines_visibility_mode(&mut self) {
        self.vines_visibility_mode = self.vines_visibility_mode.next();
        self.vines_visible = self.vines_visibility_mode.resolve(self.vines_visible);
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
    Date,
    Calendar,
}

impl MoveTarget {
    pub fn title(self) -> &'static str {
        match self {
            MoveTarget::Hero => "hero",
            MoveTarget::Clock => "clock",
            MoveTarget::Weather => "weather",
            MoveTarget::Date => "date",
            MoveTarget::Calendar => "calendar",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum SettingsTab {
    #[default]
    Positions,
    #[serde(alias = "Widgets")]
    Ui,
    Features,
    Gif,
    Theme,
}

impl SettingsTab {
    pub fn next(self) -> Self {
        match self {
            SettingsTab::Positions => SettingsTab::Ui,
            SettingsTab::Ui => SettingsTab::Features,
            SettingsTab::Features => SettingsTab::Gif,
            SettingsTab::Gif => SettingsTab::Theme,
            SettingsTab::Theme => SettingsTab::Positions,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            SettingsTab::Positions => SettingsTab::Theme,
            SettingsTab::Ui => SettingsTab::Positions,
            SettingsTab::Features => SettingsTab::Ui,
            SettingsTab::Gif => SettingsTab::Features,
            SettingsTab::Theme => SettingsTab::Gif,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            SettingsTab::Positions => "positions",
            SettingsTab::Ui => "ui",
            SettingsTab::Features => "features",
            SettingsTab::Gif => "gif",
            SettingsTab::Theme => "theme",
        }
    }

    pub fn item_count(self) -> usize {
        match self {
            SettingsTab::Positions => 6,
            SettingsTab::Ui => 5,
            SettingsTab::Features => 1,
            SettingsTab::Gif => 3,
            SettingsTab::Theme => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum FeatureVisibilityMode {
    #[default]
    On,
    Off,
    Last,
}

impl FeatureVisibilityMode {
    pub fn next(self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::Last,
            Self::Last => Self::On,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
            Self::Last => "last",
        }
    }

    pub fn resolve(self, last_visible: bool) -> bool {
        match self {
            Self::On => true,
            Self::Off => false,
            Self::Last => last_visible,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct SettingsCursor {
    pub positions: u16,
    pub widgets: u16,
    pub features: u16,
    pub gif: u16,
    pub theme: u16,
}

impl SettingsCursor {
    pub fn row(self, tab: SettingsTab) -> u16 {
        match tab {
            SettingsTab::Positions => self.positions,
            SettingsTab::Ui => self.widgets,
            SettingsTab::Features => self.features,
            SettingsTab::Gif => self.gif,
            SettingsTab::Theme => self.theme,
        }
    }

    pub fn set_row(&mut self, tab: SettingsTab, row: u16) {
        match tab {
            SettingsTab::Positions => self.positions = row,
            SettingsTab::Ui => self.widgets = row,
            SettingsTab::Features => self.features = row,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootLoadingPhase {
    Coalesce,
    Bar,
    AwaitStart,
    Dissolve,
    Hold,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadingMode {
    Boot(BootLoadingPhase),
    Transition,
}

#[derive(Clone, Debug)]
pub struct LoadingState {
    pub active: bool,
    pub label: String,
    pub mode: LoadingMode,
    pub started_at: Option<Instant>,
    pub duration: Duration,
}

impl Default for LoadingState {
    fn default() -> Self {
        Self {
            active: false,
            label: String::new(),
            mode: LoadingMode::Transition,
            started_at: None,
            duration: Duration::from_millis(0),
        }
    }
}

impl LoadingState {
    pub const BOOT_COALESCE: Duration = Duration::from_millis(1000);
    pub const BOOT_BAR: Duration = Duration::from_millis(3000);
    pub const BOOT_DISSOLVE: Duration = Duration::from_millis(1000);
    pub const BOOT_HOLD: Duration = Duration::from_millis(500);

    pub fn progress(&self, now: Instant) -> f32 {
        let Some(started_at) = self.started_at else {
            return 0.0;
        };
        let duration = self.duration.as_secs_f32();
        if duration <= f32::EPSILON {
            return 1.0;
        }
        (now.duration_since(started_at).as_secs_f32() / duration).clamp(0.0, 1.0)
    }

    #[cfg(test)]
    pub fn boot_phase(&self) -> Option<BootLoadingPhase> {
        match self.mode {
            LoadingMode::Boot(phase) => Some(phase),
            LoadingMode::Transition => None,
        }
    }

    pub fn effect_phase(&self) -> Option<BootLoadingPhase> {
        match self.mode {
            LoadingMode::Boot(BootLoadingPhase::Coalesce) => Some(BootLoadingPhase::Coalesce),
            LoadingMode::Boot(BootLoadingPhase::Dissolve) => Some(BootLoadingPhase::Dissolve),
            _ => None,
        }
    }

    pub fn awaiting_start_confirmation(&self) -> bool {
        self.active && matches!(self.mode, LoadingMode::Boot(BootLoadingPhase::AwaitStart))
    }

    pub fn showing_start_prompt(&self) -> bool {
        self.active
            && matches!(
                self.mode,
                LoadingMode::Boot(BootLoadingPhase::AwaitStart)
                    | LoadingMode::Boot(BootLoadingPhase::Dissolve)
            )
    }

    pub fn bar_progress(&self, now: Instant) -> f32 {
        match self.mode {
            LoadingMode::Boot(BootLoadingPhase::Coalesce) => 0.0,
            LoadingMode::Boot(BootLoadingPhase::Bar) => self.progress(now),
            LoadingMode::Boot(BootLoadingPhase::AwaitStart)
            | LoadingMode::Boot(BootLoadingPhase::Dissolve)
            | LoadingMode::Boot(BootLoadingPhase::Hold) => 1.0,
            LoadingMode::Transition => self.progress(now),
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
    pub camera_home_x: i32,
    pub camera_home_y: i32,
    pub pointer_x: i32,
    pub pointer_y: i32,
    pub hero_dx: i32,
    pub hero_dy: i32,
    pub clock_dx: i16,
    pub clock_dy: i16,
    pub weather_dx: i16,
    pub weather_dy: i16,
    pub date_dx: i16,
    pub date_dy: i16,
    pub calendar_dx: i16,
    pub calendar_dy: i16,
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
            weather_dx: 120,
            weather_dy: 14,
            date_dx: 95,
            date_dy: -4,
            calendar_dx: 126,
            calendar_dy: -4,
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
    pub loading: LoadingState,
    pub weather_location: WeatherLocation,
    pub weather_snapshot: Option<WeatherSnapshot>,
    pub weather_last_refresh: Option<Instant>,
    pub weather_refresh_interval: Duration,
    pub weather_refresh_rx: Option<Receiver<Result<WeatherSnapshot, WeatherError>>>,
    pub weather_locale: WeatherLocale,
    pub weather_layout: WeatherLayout,
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
            loading: LoadingState::default(),
            weather_location: WeatherLocation::named("Krakow, Poland"),
            weather_snapshot: None,
            weather_last_refresh: None,
            weather_refresh_interval: Duration::from_secs(15 * 60),
            weather_refresh_rx: None,
            weather_locale: WeatherLocale::Pl,
            weather_layout: WeatherLayout::WttrCompact,
        }
    }

    pub fn load_or_new() -> Self {
        let mut state = Self::new();
        if let Ok(snapshot) = Self::load_snapshot() {
            state.clock_font = ClockFont::from_name(&snapshot.offsets.clock_font);
            state.offsets = snapshot.offsets;
            state.meta = snapshot.meta;
        }
        state.camera.x = state.offsets.camera_x;
        state.camera.y = state.offsets.camera_y;
        state.pointer_blink_on = true;
        state.start_weather_refresh();
        state
    }

    pub fn reset_for_clean_launch(&mut self, world_kind: WorldKind) {
        let vines_visibility_mode = self.meta.vines_visibility_mode;
        let last_vines_visible = self.meta.vines_visible;
        let world_frame_visible = self.meta.world_frame_visible;
        let world_axis_visible = self.meta.world_axis_visible;
        let world_datum_visible = self.meta.world_datum_visible;
        let sliders_visible = self.meta.sliders_visible;
        let debug_info_panel_visible = self.meta.debug_info_panel_visible;
        self.meta = MetaState::new();
        self.meta.active_world = match world_kind {
            WorldKind::Boot => WorldKindSnapshot::MainScene,
            WorldKind::MainScene => WorldKindSnapshot::MainScene,
            WorldKind::Sandbox => WorldKindSnapshot::Sandbox,
        };
        self.meta.vines_visibility_mode = vines_visibility_mode;
        self.meta.vines_visible = vines_visibility_mode.resolve(last_vines_visible);
        self.meta.world_frame_visible = world_frame_visible;
        self.meta.world_axis_visible = world_axis_visible;
        self.meta.world_datum_visible = world_datum_visible;
        self.meta.sliders_visible = sliders_visible;
        self.meta.debug_info_panel_visible = debug_info_panel_visible;
        self.camera.follow_hero = false;
        self.settings_edit.clear();
        self.loading = LoadingState::default();
        self.pointer_blink_on = true;
    }

    pub fn refresh_weather_if_due(&mut self) {
        self.finish_weather_refresh_if_ready();
        if self.weather_refresh_rx.is_some() {
            return;
        }
        let now = Instant::now();
        if self
            .weather_last_refresh
            .is_none_or(|last| now.duration_since(last) >= self.weather_refresh_interval)
        {
            self.start_weather_refresh();
        }
    }

    #[cfg(test)]
    pub fn refresh_weather_now(&mut self) {
        self.start_weather_refresh();
        self.finish_weather_refresh_blocking();
    }

    pub fn start_weather_refresh(&mut self) {
        if self.weather_refresh_rx.is_some() {
            return;
        }
        let location = self.weather_location.clone();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let provider = WttrInWeatherProvider;
            let result = provider.snapshot(&location);
            let _ = tx.send(result);
        });
        self.weather_refresh_rx = Some(rx);
    }

    fn finish_weather_refresh_if_ready(&mut self) {
        let Some(rx) = self.weather_refresh_rx.as_ref() else {
            return;
        };
        match rx.try_recv() {
            Ok(result) => {
                self.weather_refresh_rx = None;
                self.apply_weather_refresh_result(result);
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.weather_refresh_rx = None;
                self.apply_weather_refresh_result(Err(WeatherError::Unavailable));
            }
        }
    }

    #[cfg(test)]
    fn finish_weather_refresh_blocking(&mut self) {
        let Some(rx) = self.weather_refresh_rx.take() else {
            return;
        };
        let result = rx.recv().unwrap_or(Err(WeatherError::Unavailable));
        self.apply_weather_refresh_result(result);
    }

    fn apply_weather_refresh_result(&mut self, result: Result<WeatherSnapshot, WeatherError>) {
        match result {
            Ok(snapshot) => {
                self.weather_snapshot = Some(snapshot);
                self.weather_last_refresh = Some(Instant::now());
            }
            Err(_) => {
                if let Some(snapshot) = self.weather_snapshot.as_mut() {
                    snapshot.stale = true;
                } else {
                    let provider = StaticWeatherProvider;
                    if let Ok(mut snapshot) = provider.snapshot(&self.weather_location) {
                        snapshot.stale = true;
                        self.weather_snapshot = Some(snapshot);
                    }
                }
                self.weather_last_refresh = Some(Instant::now());
            }
        }
    }

    pub fn next_font(&mut self) {
        self.clock_font = self.clock_font.next();
        self.offsets.clock_font = self.clock_font.display_name().to_string();
        self.save_state();
    }

    pub fn prev_font(&mut self) {
        self.clock_font = self.clock_font.prev();
        self.offsets.clock_font = self.clock_font.display_name().to_string();
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

    pub fn toggle_palette(&mut self) {
        self.meta.toggle_palette();
    }

    pub fn toggle_weather_popup(&mut self) {
        self.meta.toggle_weather_popup();
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
        self.save_state();
    }

    pub fn show_dev_surfaces(&self) -> bool {
        self.meta.dev_mode && !self.loading.active
    }

    pub fn active_world_kind(&self) -> WorldKind {
        self.meta.active_world_kind()
    }

    pub fn cycle_world_kind(&mut self) {
        self.meta.cycle_world_kind();
        let label = match self.meta.active_world_kind() {
            WorldKind::Boot => "loading...",
            WorldKind::MainScene => "loading main scene...",
            WorldKind::Sandbox => "loading sandbox...",
        };
        self.start_loading_transition(label, Duration::from_millis(900));
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
            3 => (
                self.offsets.weather_dx as i32,
                self.offsets.weather_dy as i32,
            ),
            4 => (self.offsets.date_dx as i32, self.offsets.date_dy as i32),
            5 => (
                self.offsets.calendar_dx as i32,
                self.offsets.calendar_dy as i32,
            ),
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
                    3 => self.meta.sliders_visible = !self.meta.sliders_visible,
                    4 => self.meta.debug_info_panel_visible = !self.meta.debug_info_panel_visible,
                    _ => {}
                }
                self.save_state();
                Ok(())
            }
            SettingsTab::Features => {
                if self.meta.selected_settings_row() == 0 {
                    self.meta.cycle_vines_visibility_mode();
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

    pub fn start_loading_boot(&mut self) {
        self.loading.active = true;
        self.loading.label = "loading...".to_string();
        self.loading.mode = LoadingMode::Boot(BootLoadingPhase::Coalesce);
        self.loading.started_at = Some(Instant::now());
        self.loading.duration = LoadingState::BOOT_COALESCE;
    }

    pub fn start_loading_transition(&mut self, label: &str, duration: Duration) {
        self.loading.active = true;
        self.loading.label = label.to_string();
        self.loading.mode = LoadingMode::Transition;
        self.loading.started_at = Some(Instant::now());
        self.loading.duration = duration;
    }

    pub fn acknowledge_loading_start(&mut self) {
        if !self.loading.awaiting_start_confirmation() {
            return;
        }
        self.loading.mode = LoadingMode::Boot(BootLoadingPhase::Dissolve);
        self.loading.started_at = Some(Instant::now());
        self.loading.duration = LoadingState::BOOT_DISSOLVE;
    }

    pub fn update_loading(&mut self) {
        if !self.loading.active {
            return;
        }
        let now = Instant::now();
        match self.loading.mode {
            LoadingMode::Transition => {
                if self.loading.progress(now) >= 1.0 {
                    self.loading = LoadingState::default();
                }
            }
            LoadingMode::Boot(BootLoadingPhase::Coalesce) => {
                if self.loading.progress(now) >= 1.0 {
                    self.loading.mode = LoadingMode::Boot(BootLoadingPhase::Bar);
                    self.loading.started_at = Some(now);
                    self.loading.duration = LoadingState::BOOT_BAR;
                }
            }
            LoadingMode::Boot(BootLoadingPhase::Bar) => {
                if self.loading.progress(now) >= 1.0 {
                    self.loading.mode = LoadingMode::Boot(BootLoadingPhase::AwaitStart);
                    self.loading.started_at = None;
                    self.loading.duration = Duration::from_millis(0);
                }
            }
            LoadingMode::Boot(BootLoadingPhase::AwaitStart) => {}
            LoadingMode::Boot(BootLoadingPhase::Dissolve) => {
                if self.loading.progress(now) >= 1.0 {
                    self.loading.mode = LoadingMode::Boot(BootLoadingPhase::Hold);
                    self.loading.started_at = Some(now);
                    self.loading.duration = LoadingState::BOOT_HOLD;
                }
            }
            LoadingMode::Boot(BootLoadingPhase::Hold) => {
                if self.loading.progress(now) >= 1.0 {
                    self.loading = LoadingState::default();
                }
            }
        }
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
            3 => {
                self.offsets.weather_dx = x.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                self.offsets.weather_dy = y.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            }
            4 => {
                self.offsets.date_dx = x.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                self.offsets.date_dy = y.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            }
            5 => {
                self.offsets.calendar_dx = x.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                self.offsets.calendar_dy = y.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
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
            SettingsTab::Features,
            SettingsTab::Gif,
            SettingsTab::Theme,
        ] {
            let max_row = self.settings_item_count(tab).saturating_sub(1) as u16;
            let current = self.meta.settings_cursor.row(tab);
            self.meta.settings_cursor.set_row(tab, current.min(max_row));
        }
    }

    pub fn hero_scene_attachment(&self) -> crate::scene::entity::HeroSceneAttachment {
        hero_scene_poses(
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
            crate::scene::coords::WorldPos {
                x: self.offsets.weather_dx as i32,
                y: self.offsets.weather_dy as i32,
            },
            crate::scene::coords::WorldPos {
                x: self.offsets.date_dx as i32,
                y: self.offsets.date_dy as i32,
            },
            crate::scene::coords::WorldPos {
                x: self.offsets.calendar_dx as i32,
                y: self.offsets.calendar_dy as i32,
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
            MoveTarget::Weather => {
                self.offsets.weather_dx = (self.offsets.weather_dx + dx).clamp(-200, 200);
                self.offsets.weather_dy = (self.offsets.weather_dy + dy).clamp(-200, 200);
            }
            MoveTarget::Date => {
                self.offsets.date_dx = (self.offsets.date_dx + dx).clamp(-200, 200);
                self.offsets.date_dy = (self.offsets.date_dy + dy).clamp(-200, 200);
            }
            MoveTarget::Calendar => {
                self.offsets.calendar_dx = (self.offsets.calendar_dx + dx).clamp(-200, 200);
                self.offsets.calendar_dy = (self.offsets.calendar_dy + dy).clamp(-200, 200);
            }
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
        BootLoadingPhase, FeatureVisibilityMode, LoadingMode, LoadingState, MetaState, MoveTarget,
        SettingsAxisField, SettingsCursor, SettingsTab, UiOffsets, UiState, UiStateSnapshot,
        WorldKindSnapshot,
    };
    use crate::core::world::WorldKind;
    use crate::scene::coords::WorldPos;
    use crate::weather::provider::WeatherError;
    use std::time::{Duration, Instant};

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
        assert!(!ui.meta.palette_open);
        assert!(!ui.meta.weather_popup_open);
        assert!(!ui.meta.pointer_probe_open);

        ui.toggle_settings();
        assert!(ui.meta.settings_open);
        assert!(!ui.meta.hotkeys_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.palette_open);
        assert!(!ui.meta.weather_popup_open);
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
        assert!(!ui.meta.palette_open);
        assert!(!ui.meta.weather_popup_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.pointer_probe_open);
    }

    #[test]
    fn move_mode_and_popups_are_mutually_exclusive() {
        let mut ui = UiState::new();

        ui.toggle_move_mode();
        assert!(ui.meta.move_mode_open);
        assert!(!ui.meta.hotkeys_open);
        assert!(!ui.meta.palette_open);
        assert!(!ui.meta.weather_popup_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.pointer_probe_open);

        ui.toggle_hotkeys();
        assert!(ui.meta.hotkeys_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.palette_open);
        assert!(!ui.meta.weather_popup_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.pointer_probe_open);
    }

    #[test]
    fn palette_and_popups_are_mutually_exclusive() {
        let mut ui = UiState::new();

        ui.toggle_palette();
        assert!(ui.meta.palette_open);
        assert!(!ui.meta.hotkeys_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.weather_popup_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.pointer_probe_open);

        ui.toggle_hotkeys();
        assert!(ui.meta.hotkeys_open);
        assert!(!ui.meta.palette_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.weather_popup_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.pointer_probe_open);
    }

    #[test]
    fn weather_popup_and_popups_are_mutually_exclusive() {
        let mut ui = UiState::new();

        ui.toggle_weather_popup();
        assert!(ui.meta.weather_popup_open);
        assert!(!ui.meta.hotkeys_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.palette_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.pointer_probe_open);

        ui.toggle_palette();
        assert!(ui.meta.palette_open);
        assert!(!ui.meta.weather_popup_open);
        assert!(!ui.meta.hotkeys_open);
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
        assert!(!ui.meta.palette_open);
        assert!(!ui.meta.weather_popup_open);
        assert!(!ui.meta.settings_open);

        ui.toggle_hotkeys();
        assert!(ui.meta.hotkeys_open);
        assert!(!ui.meta.pointer_probe_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.palette_open);
        assert!(!ui.meta.weather_popup_open);
        assert!(!ui.meta.settings_open);
    }

    #[test]
    fn hero_scene_attachment_uses_ui_offsets_as_runtime_source_of_truth() {
        let ui = UiState::new();
        let attachment = ui.hero_scene_attachment();

        assert_eq!(attachment.hero_world(), WorldPos { x: 150, y: 60 });
        assert_eq!(attachment.hero_visual_anchor(), WorldPos { x: -68, y: 21 });
        assert_eq!(attachment.clock_world(), WorldPos { x: 27, y: 11 });
        assert_eq!(attachment.weather_world(), WorldPos { x: 52, y: 35 });
        assert_eq!(attachment.date_world(), WorldPos { x: 27, y: 17 });
        assert_eq!(attachment.calendar_world(), WorldPos { x: 58, y: 17 });
    }

    #[test]
    fn hero_scene_attachment_reflects_offset_changes() {
        let mut ui = UiState::new();
        ui.offsets.hero_dx = -100;
        ui.offsets.hero_dy = -50;
        ui.offsets.clock_dx = 12;
        ui.offsets.clock_dy = -3;
        ui.offsets.weather_dx = 40;
        ui.offsets.weather_dy = 8;
        ui.offsets.date_dx = 12;
        ui.offsets.date_dy = 4;
        ui.offsets.calendar_dx = 30;
        ui.offsets.calendar_dy = 4;

        let attachment = ui.hero_scene_attachment();

        assert_eq!(attachment.hero_world(), WorldPos { x: 150, y: 60 });
        assert_eq!(attachment.hero_visual_anchor(), WorldPos { x: 50, y: 10 });
        assert_eq!(attachment.clock_world(), WorldPos { x: 62, y: 7 });
        assert_eq!(attachment.weather_world(), WorldPos { x: 90, y: 18 });
        assert_eq!(attachment.date_world(), WorldPos { x: 62, y: 14 });
        assert_eq!(attachment.calendar_world(), WorldPos { x: 80, y: 14 });
    }

    #[test]
    fn toggling_meta_does_not_change_attachment_facts() {
        let mut ui = UiState::new();
        let baseline = ui.hero_scene_attachment();

        ui.toggle_dev_mode();

        let after_toggle = ui.hero_scene_attachment();

        assert_eq!(baseline.hero_world(), after_toggle.hero_world());
        assert_eq!(
            baseline.hero_visual_anchor(),
            after_toggle.hero_visual_anchor()
        );
        assert_eq!(baseline.clock_world(), after_toggle.clock_world());
        assert_eq!(baseline.weather_world(), after_toggle.weather_world());
        assert_eq!(baseline.date_world(), after_toggle.date_world());
        assert_eq!(baseline.calendar_world(), after_toggle.calendar_world());
    }

    #[test]
    fn move_target_selection_changes_without_touching_attachment_facts() {
        let mut ui = UiState::new();
        let baseline = ui.hero_scene_attachment();

        ui.toggle_move_mode();
        ui.meta.select_move_target(MoveTarget::Clock);

        let after_select = ui.hero_scene_attachment();

        assert_eq!(baseline.hero_world(), after_select.hero_world());
        assert_eq!(
            baseline.hero_visual_anchor(),
            after_select.hero_visual_anchor()
        );
        assert_eq!(baseline.clock_world(), after_select.clock_world());
        assert_eq!(baseline.weather_world(), after_select.weather_world());
        assert_eq!(baseline.date_world(), after_select.date_world());
        assert_eq!(baseline.calendar_world(), after_select.calendar_world());
        assert_eq!(ui.meta.move_target, MoveTarget::Clock);
    }

    #[test]
    fn refresh_weather_now_populates_cached_snapshot() {
        let mut ui = UiState::new();

        assert!(ui.weather_snapshot.is_none());
        assert!(ui.weather_last_refresh.is_none());

        ui.refresh_weather_now();

        assert!(ui.weather_snapshot.is_some());
        assert!(ui.weather_last_refresh.is_some());
        assert_eq!(ui.weather_location.label, "Krakow, Poland");
    }

    #[test]
    fn failed_weather_refresh_marks_existing_snapshot_stale() {
        let mut ui = UiState::new();
        ui.refresh_weather_now();
        let fresh = ui
            .weather_snapshot
            .clone()
            .expect("refresh should populate a weather snapshot");
        assert!(!fresh.stale);

        ui.apply_weather_refresh_result(Err(WeatherError::Unavailable));

        let refreshed = ui
            .weather_snapshot
            .as_ref()
            .expect("existing weather snapshot should be preserved");
        assert_eq!(refreshed.location_label, fresh.location_label);
        assert!(refreshed.stale);
    }

    #[test]
    fn commit_settings_edit_updates_weather_offsets() {
        let mut ui = UiState::new();
        ui.meta.settings_tab = SettingsTab::Positions;
        ui.meta.settings_cursor.positions = 3;
        ui.begin_settings_edit();
        ui.settings_edit.x_buffer = "88".to_string();
        ui.settings_edit.y_buffer = "-12".to_string();

        ui.commit_settings_edit()
            .expect("weather offset edit should commit");

        assert_eq!(ui.offsets.weather_dx, 88);
        assert_eq!(ui.offsets.weather_dy, -12);
    }

    #[test]
    fn commit_settings_edit_updates_date_offsets() {
        let mut ui = UiState::new();
        ui.meta.settings_tab = SettingsTab::Positions;
        ui.meta.settings_cursor.positions = 4;
        ui.begin_settings_edit();
        ui.settings_edit.x_buffer = "77".to_string();
        ui.settings_edit.y_buffer = "-6".to_string();

        ui.commit_settings_edit()
            .expect("date offset edit should commit");

        assert_eq!(ui.offsets.date_dx, 77);
        assert_eq!(ui.offsets.date_dy, -6);
    }

    #[test]
    fn commit_settings_edit_updates_calendar_offsets() {
        let mut ui = UiState::new();
        ui.meta.settings_tab = SettingsTab::Positions;
        ui.meta.settings_cursor.positions = 5;
        ui.begin_settings_edit();
        ui.settings_edit.x_buffer = "132".to_string();
        ui.settings_edit.y_buffer = "5".to_string();

        ui.commit_settings_edit()
            .expect("calendar offset edit should commit");

        assert_eq!(ui.offsets.calendar_dx, 132);
        assert_eq!(ui.offsets.calendar_dy, 5);
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
                weather_dx: 123,
                weather_dy: 11,
                date_dx: 71,
                date_dy: 3,
                calendar_dx: 140,
                calendar_dy: 9,
                clock_font: "fender".to_string(),
                hero_fps: 4.5,
            },
            meta: MetaState {
                dev_mode: true,
                active_world: WorldKindSnapshot::Sandbox,
                vines_visible: false,
                vines_visibility_mode: FeatureVisibilityMode::Last,
                hotkeys_open: false,
                move_mode_open: true,
                palette_open: false,
                weather_popup_open: false,
                settings_open: true,
                pointer_probe_open: true,
                world_frame_visible: false,
                world_axis_visible: true,
                world_datum_visible: false,
                sliders_visible: false,
                debug_info_panel_visible: true,
                settings_tab: SettingsTab::Theme,
                settings_cursor: SettingsCursor {
                    positions: 1,
                    widgets: 2,
                    features: 0,
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
        assert_eq!(round_trip.offsets.weather_dx, 123);
        assert_eq!(round_trip.offsets.weather_dy, 11);
        assert_eq!(round_trip.offsets.date_dx, 71);
        assert_eq!(round_trip.offsets.date_dy, 3);
        assert_eq!(round_trip.offsets.calendar_dx, 140);
        assert_eq!(round_trip.offsets.calendar_dy, 9);
        assert_eq!(round_trip.offsets.clock_font, "fender");
        assert_eq!(round_trip.offsets.hero_fps, 4.5);
        assert!(round_trip.meta.dev_mode);
        assert!(!round_trip.meta.vines_visible);
        assert_eq!(
            round_trip.meta.vines_visibility_mode,
            FeatureVisibilityMode::Last
        );
        assert!(round_trip.meta.settings_open);
        assert!(round_trip.meta.move_mode_open);
        assert!(!round_trip.meta.palette_open);
        assert!(!round_trip.meta.weather_popup_open);
        assert!(round_trip.meta.pointer_probe_open);
        assert!(!round_trip.meta.world_frame_visible);
        assert!(round_trip.meta.world_axis_visible);
        assert!(!round_trip.meta.world_datum_visible);
        assert!(!round_trip.meta.sliders_visible);
        assert!(round_trip.meta.debug_info_panel_visible);
        assert_eq!(round_trip.meta.settings_tab, SettingsTab::Theme);
        assert_eq!(round_trip.meta.settings_cursor.positions, 1);
        assert_eq!(round_trip.meta.settings_cursor.widgets, 2);
        assert_eq!(round_trip.meta.settings_cursor.features, 0);
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
        assert_eq!(snapshot.offsets.date_dx, 95);
        assert_eq!(snapshot.offsets.date_dy, -4);
        assert_eq!(snapshot.offsets.calendar_dx, 126);
        assert_eq!(snapshot.offsets.calendar_dy, -4);
        assert_eq!(snapshot.offsets.clock_font, "small");
        assert_eq!(snapshot.offsets.hero_fps, 1.5);
        assert!(!snapshot.meta.dev_mode);
        assert!(!snapshot.meta.move_mode_open);
        assert!(!snapshot.meta.palette_open);
        assert!(!snapshot.meta.weather_popup_open);
        assert!(!snapshot.meta.settings_open);
        assert!(!snapshot.meta.pointer_probe_open);
        assert!(snapshot.meta.world_frame_visible);
        assert!(snapshot.meta.world_axis_visible);
        assert!(snapshot.meta.world_datum_visible);
        assert!(snapshot.meta.vines_visible);
        assert_eq!(
            snapshot.meta.vines_visibility_mode,
            FeatureVisibilityMode::On
        );
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
    fn feature_settings_cycle_vines_visibility_policy() {
        let mut ui = UiState::new();
        ui.meta.settings_tab = SettingsTab::Features;
        ui.meta.settings_cursor.features = 0;

        ui.activate_selected_setting_with_viewport(124, 32)
            .expect("feature toggle should succeed");
        assert_eq!(ui.meta.vines_visibility_mode, FeatureVisibilityMode::Off);
        assert!(!ui.meta.vines_visible);

        ui.toggle_vines_visible();
        assert!(ui.meta.vines_visible);

        ui.activate_selected_setting_with_viewport(124, 32)
            .expect("feature toggle should succeed");
        assert_eq!(ui.meta.vines_visibility_mode, FeatureVisibilityMode::Last);
        assert!(ui.meta.vines_visible);

        ui.activate_selected_setting_with_viewport(124, 32)
            .expect("feature toggle should succeed");
        assert_eq!(ui.meta.vines_visibility_mode, FeatureVisibilityMode::On);
        assert!(ui.meta.vines_visible);
    }

    #[test]
    fn world_space_up_controls_increase_y_across_targets() {
        let mut ui = UiState::new();
        ui.offsets.hero_dy = 0;
        ui.offsets.clock_dy = 0;
        ui.offsets.date_dy = 0;
        ui.offsets.calendar_dy = 0;
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

        ui.meta.select_move_target(MoveTarget::Date);
        ui.move_selected_target_up()
            .expect("date move should succeed");
        assert_eq!(ui.offsets.date_dy, 1);
        ui.move_selected_target_down()
            .expect("date move should succeed");
        assert_eq!(ui.offsets.date_dy, 0);

        ui.meta.select_move_target(MoveTarget::Calendar);
        ui.move_selected_target_up()
            .expect("calendar move should succeed");
        assert_eq!(ui.offsets.calendar_dy, 1);
        ui.move_selected_target_down()
            .expect("calendar move should succeed");
        assert_eq!(ui.offsets.calendar_dy, 0);

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
        ui.meta.settings_cursor.positions = 0;

        ui.begin_settings_edit_with_viewport(124, 32);
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
        ui.meta.dev_mode = true;
        ui.meta.settings_open = true;
        ui.meta.settings_tab = SettingsTab::Positions;
        ui.meta.settings_cursor.positions = 0;
        ui.begin_settings_edit_with_viewport(124, 32);

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
        assert_eq!(ui.settings_item_count(SettingsTab::Ui), 5);
        assert_eq!(ui.settings_item_count(SettingsTab::Features), 1);
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
        assert!(ui.meta.sliders_visible);
        assert!(ui.meta.debug_info_panel_visible);

        ui.meta.settings_cursor.widgets = 1;
        ui.activate_selected_setting_with_viewport(124, 32)
            .expect("ui toggle should succeed");
        assert!(!ui.meta.world_axis_visible);

        ui.meta.settings_cursor.widgets = 2;
        ui.activate_selected_setting_with_viewport(124, 32)
            .expect("ui toggle should succeed");
        assert!(!ui.meta.world_datum_visible);

        ui.meta.settings_cursor.widgets = 3;
        ui.activate_selected_setting_with_viewport(124, 32)
            .expect("ui toggle should succeed");
        assert!(!ui.meta.sliders_visible);

        ui.meta.settings_cursor.widgets = 4;
        ui.activate_selected_setting_with_viewport(124, 32)
            .expect("ui toggle should succeed");
        assert!(!ui.meta.debug_info_panel_visible);
    }

    #[test]
    fn clean_launch_preserves_vines_visibility_policy_and_resolves_live_state() {
        let mut ui = UiState::new();
        ui.meta.vines_visibility_mode = FeatureVisibilityMode::Last;
        ui.meta.vines_visible = false;
        ui.meta.dev_mode = true;
        ui.meta.settings_open = true;

        ui.reset_for_clean_launch(WorldKind::MainScene);

        assert_eq!(ui.meta.vines_visibility_mode, FeatureVisibilityMode::Last);
        assert!(!ui.meta.vines_visible);
        assert!(!ui.meta.dev_mode);
        assert!(!ui.meta.settings_open);

        ui.meta.vines_visibility_mode = FeatureVisibilityMode::Off;
        ui.meta.vines_visible = true;
        ui.reset_for_clean_launch(WorldKind::MainScene);
        assert!(!ui.meta.vines_visible);

        ui.meta.vines_visibility_mode = FeatureVisibilityMode::On;
        ui.meta.vines_visible = false;
        ui.reset_for_clean_launch(WorldKind::MainScene);
        assert!(ui.meta.vines_visible);
    }

    #[test]
    fn clean_launch_preserves_ui_visibility_preferences() {
        let mut ui = UiState::new();
        ui.meta.world_frame_visible = false;
        ui.meta.world_axis_visible = false;
        ui.meta.world_datum_visible = false;
        ui.meta.sliders_visible = false;
        ui.meta.debug_info_panel_visible = false;
        ui.meta.dev_mode = true;
        ui.meta.settings_open = true;

        ui.reset_for_clean_launch(WorldKind::MainScene);

        assert!(!ui.meta.world_frame_visible);
        assert!(!ui.meta.world_axis_visible);
        assert!(!ui.meta.world_datum_visible);
        assert!(!ui.meta.sliders_visible);
        assert!(!ui.meta.debug_info_panel_visible);
        assert!(!ui.meta.dev_mode);
        assert!(!ui.meta.settings_open);
    }

    #[test]
    fn cycling_world_kind_starts_a_loading_transition() {
        let mut ui = UiState::new();

        ui.cycle_world_kind();

        assert_eq!(ui.active_world_kind(), WorldKind::Sandbox);
        assert!(ui.loading.active);
        assert_eq!(ui.loading.label, "loading sandbox...");
    }

    #[test]
    fn loading_transition_clears_after_duration_elapses() {
        let mut ui = UiState::new();
        ui.start_loading_transition("loading sandbox...", Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(5));

        ui.update_loading();

        assert!(!ui.loading.active);
        assert!(ui.loading.label.is_empty());
    }

    #[test]
    fn boot_loading_waits_for_space_before_dissolve() {
        let mut ui = UiState::new();

        ui.start_loading_boot();
        ui.loading.started_at = Some(
            Instant::now()
                .checked_sub(LoadingState::BOOT_COALESCE + LoadingState::BOOT_BAR)
                .expect("boot phase start should support subtraction"),
        );
        ui.loading.mode = LoadingMode::Boot(BootLoadingPhase::Bar);
        ui.loading.duration = LoadingState::BOOT_BAR;

        ui.update_loading();

        assert!(ui.loading.awaiting_start_confirmation());
        ui.acknowledge_loading_start();
        assert_eq!(ui.loading.boot_phase(), Some(BootLoadingPhase::Dissolve));
    }

    #[test]
    fn boot_loading_holds_briefly_after_dissolve() {
        let mut ui = UiState::new();

        ui.start_loading_boot();
        ui.loading.mode = LoadingMode::Boot(BootLoadingPhase::Dissolve);
        ui.loading.started_at = Some(
            Instant::now()
                .checked_sub(LoadingState::BOOT_DISSOLVE)
                .expect("dissolve phase start should support subtraction"),
        );
        ui.loading.duration = LoadingState::BOOT_DISSOLVE;

        ui.update_loading();
        assert_eq!(ui.loading.boot_phase(), Some(BootLoadingPhase::Hold));

        ui.loading.started_at = Some(
            Instant::now()
                .checked_sub(LoadingState::BOOT_HOLD)
                .expect("hold phase start should support subtraction"),
        );
        ui.loading.duration = LoadingState::BOOT_HOLD;

        ui.update_loading();
        assert!(!ui.loading.active);
    }

    #[test]
    fn clean_launch_resets_dev_and_modal_state() {
        let mut ui = UiState::new();
        ui.meta.dev_mode = true;
        ui.meta.hotkeys_open = true;
        ui.meta.move_mode_open = true;
        ui.meta.palette_open = true;
        ui.meta.weather_popup_open = true;
        ui.meta.settings_open = true;
        ui.meta.pointer_probe_open = true;

        ui.reset_for_clean_launch(WorldKind::MainScene);

        assert!(!ui.meta.dev_mode);
        assert!(!ui.meta.hotkeys_open);
        assert!(!ui.meta.move_mode_open);
        assert!(!ui.meta.palette_open);
        assert!(!ui.meta.weather_popup_open);
        assert!(!ui.meta.settings_open);
        assert!(!ui.meta.pointer_probe_open);
        assert_eq!(ui.active_world_kind(), WorldKind::MainScene);
    }
}
