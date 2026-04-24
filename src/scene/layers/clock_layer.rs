use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::scene::Layer;
use crate::ui::panels::clock::ClockPanel;
use crate::ui::panel::Panel;
use crate::ui::layout::LayoutRegions;
use crate::ui::state::UiState;
use crate::ui::viewport::Viewport;
use ratatui::prelude::*;

pub struct ClockLayer;

impl Layer for ClockLayer {
    fn z_index(&self) -> i32 {
        100
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
        let panel = ClockPanel;
        panel.render(
            frame,
            viewport_rect,
            world,
            ui,
            fonts,
            viewport,
        );
        draw_scrollbars(frame, viewport_rect, viewport, world);
    }
}

fn draw_scrollbars(
    frame: &mut Frame,
    viewport_rect: Rect,
    viewport: &crate::ui::viewport::Viewport,
    world: &WorldState,
) {
    if viewport_rect.width == 0 || viewport_rect.height == 0 {
        return;
    }

    let max_x = world.grid.width.saturating_sub(viewport.width).max(1) as f32;
    let max_y = world.grid.height.saturating_sub(viewport.height).max(1) as f32;
    let ratio_x = (viewport.x.max(0) as f32 / max_x).clamp(0.0, 1.0);
    let ratio_y = (viewport.y.max(0) as f32 / max_y).clamp(0.0, 1.0);

    let thumb_x =
        viewport_rect.x + ((viewport_rect.width.saturating_sub(1)) as f32 * ratio_x) as u16;
    let thumb_y =
        viewport_rect.y + ((viewport_rect.height.saturating_sub(1)) as f32 * ratio_y) as u16;
    let right_x = viewport_rect.x + viewport_rect.width.saturating_sub(1);
    let bottom_y = viewport_rect.y + viewport_rect.height.saturating_sub(1);

    if let Some(cell) = frame.buffer_mut().cell_mut((thumb_x, bottom_y)) {
        cell.set_symbol("▓").set_fg(Color::DarkGray);
    }
    if let Some(cell) = frame.buffer_mut().cell_mut((right_x, thumb_y)) {
        cell.set_symbol("▓").set_fg(Color::DarkGray);
    }
}
