use crate::scene::layers::{
    clock_layer::ClockLayer, debug_layer::DebugLayer, field_layer::FieldLayer,
    hero_layer::HeroLayer, hotkeys_layer::HotkeysLayer, move_layer::MoveLayer,
    settings_layer::SettingsLayer, status_layer::StatusLayer,
};

pub fn build_ui_layers() -> Vec<Box<dyn crate::scene::Layer>> {
    vec![
        Box::new(FieldLayer),
        Box::new(HeroLayer),
        Box::new(ClockLayer),
        Box::new(DebugLayer),
        Box::new(HotkeysLayer),
        Box::new(MoveLayer),
        Box::new(SettingsLayer),
        Box::new(StatusLayer),
    ]
}
