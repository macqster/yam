use crate::scene::Layer;
use crate::ui::panels::field::FieldPanel;
use crate::ui::panel::Panel;
use crate::{
    core::world::WorldState, render::fonts::FontRegistry, ui::layout::LayoutRegions,
    ui::state::UiState, ui::viewport::Viewport,
};
use ratatui::prelude::*;

pub struct FieldLayer;

impl Layer for FieldLayer {
    fn z_index(&self) -> i32 {
        0
    }

    fn render(
        &self,
        frame: &mut Frame<'_>,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        viewport: &Viewport,
        viewport_rect: Rect,
        _layout: &LayoutRegions,
    ) {
        let panel = FieldPanel;
        panel.render(frame, viewport_rect, world, ui, fonts, viewport);
    }
}
