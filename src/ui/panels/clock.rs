use crate::core::world::WorldState;
use crate::render::clock::{draw_clock, draw_clock_at};
use crate::render::fonts::FontRegistry;
use crate::ui::anchor::Anchor;
use crate::ui::panel::Panel;
use crate::ui::state::UiState;
use crate::ui::viewport::Viewport;
use ratatui::prelude::*;

pub struct ClockPanel;

impl Panel for ClockPanel {
    fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        _viewport: &Viewport,
    ) {
        if ui.anchored_clock {
            let anchor = Anchor {
                x: 0.75,
                y: 0.33,
                offset_x: 0,
                offset_y: 0,
            };
            let (ax, ay) = anchor.resolve(area);
            draw_clock_at(frame, world, area.x + ax, area.y + ay, ui, fonts);
        } else {
            draw_clock(frame, world, area, ui, fonts);
        }
    }
}
