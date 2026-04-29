use crate::scene::layers::{
    clock_layer::ClockLayer, debug_layer::DebugLayer, field_layer::FieldLayer,
    hero_layer::HeroLayer, settings_layer::SettingsLayer, status_layer::StatusLayer,
};

pub fn build_ui_layers() -> Vec<Box<dyn crate::scene::Layer>> {
    vec![
        Box::new(FieldLayer),
        Box::new(HeroLayer),
        Box::new(ClockLayer),
        Box::new(DebugLayer),
        Box::new(SettingsLayer),
        Box::new(StatusLayer),
    ]
}
