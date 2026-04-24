use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::scene::Layer;
use crate::ui::layout::LayoutRegions;
use crate::ui::state::UiState;
use crate::ui::viewport::Viewport;
use crate::ui::debug::draw_layout_debug;
use ratatui::prelude::*;

pub struct DebugLayer;

impl Layer for DebugLayer {
    fn z_index(&self) -> i32 {
        300
    }

    fn render(
        &self,
        frame: &mut Frame<'_>,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _viewport: &Viewport,
        _viewport_rect: Rect,
        layout: &LayoutRegions,
    ) {
        if ui.debug_layout {
            draw_layout_debug(frame, layout);
        }
    }
}
