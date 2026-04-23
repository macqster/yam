use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::ui::state::UiState;
use crate::ui::viewport::Viewport;
use ratatui::prelude::*;

pub trait Panel {
    fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        viewport: &Viewport,
    );
}
