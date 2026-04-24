use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::render::hero::{draw_hero, draw_hero_debug};
use crate::scene::Layer;
use crate::ui::layout::LayoutRegions;
use crate::ui::state::UiState;
use crate::scene::viewport::Viewport;
use ratatui::prelude::*;

pub struct HeroLayer;

impl Layer for HeroLayer {
    fn z_index(&self) -> i32 {
        10
    }

    fn render(
        &self,
        frame: &mut Frame<'_>,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        viewport: &Viewport,
        _viewport_rect: Rect,
        _layout: &LayoutRegions,
    ) {
        let hero = &ui.hero;
        draw_hero_debug(frame, hero, viewport, ui.hero_offset_x, ui.hero_offset_y);
        draw_hero(frame, hero, viewport, ui.hero_offset_x, ui.hero_offset_y);
    }
}
