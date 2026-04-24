//! SCENE COMPOSITION
//!
//! Render order (STRICT):
//! 1. World (field)
//! 2. Hero
//! 3. UI (clock, panels)
//! 4. Debug
//!
//! DO NOT reorder without updating docs/RENDERING.md

use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::scene::layers::{
    clock_layer::ClockLayer, debug_layer::DebugLayer, field_layer::FieldLayer,
    hero_layer::HeroLayer, status_layer::StatusLayer,
};
use crate::scene::Scene;
use crate::ui::state::UiState;
use ratatui::prelude::*;

pub fn render_scene(frame: &mut Frame<'_>, world: &WorldState, ui: &UiState, fonts: &FontRegistry) {
    let scene = Scene::new(vec![
        Box::new(FieldLayer),
        Box::new(HeroLayer),
        Box::new(ClockLayer),
        Box::new(DebugLayer),
        Box::new(StatusLayer),
    ]);
    scene.render(frame, world, ui, fonts);
}
