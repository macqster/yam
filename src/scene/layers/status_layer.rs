use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::scene::Layer;
use crate::ui::layout::LayoutRegions;
use crate::ui::state::UiState;
use crate::ui::viewport::Viewport;
use crate::ui::panels::status_bar::StatusBarPanel;
use crate::ui::panel::Panel;
use ratatui::prelude::*;

pub struct StatusLayer;

impl Layer for StatusLayer {
    fn z_index(&self) -> i32 {
        1000
    }

    fn render(
        &self,
        frame: &mut Frame<'_>,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        viewport: &Viewport,
        _viewport_rect: Rect,
        layout: &LayoutRegions,
    ) {
        let panel = StatusBarPanel;
        panel.render(
            frame,
            layout.bottom_bar,
            world,
            ui,
            fonts,
            viewport,
        );
    }
}
