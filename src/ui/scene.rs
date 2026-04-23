use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::render::hero::{draw_hero, draw_hero_debug, Hero};
use crate::ui::debug::draw_layout_debug;
use crate::ui::layout::compute_layout;
use crate::ui::panel::Panel;
use crate::ui::panels::clock::ClockPanel;
use crate::ui::panels::field::FieldPanel;
use crate::ui::panels::status_bar::StatusBarPanel;
use crate::ui::state::UiState;
use crate::ui::viewport::{select_viewport_tier, viewport_rect, Viewport};
use ratatui::prelude::*;

pub fn render_scene(frame: &mut Frame, world: &WorldState, ui: &UiState, fonts: &FontRegistry) {
    let full = frame.area();
    let layout = compute_layout(full);
    let hero = Hero::dummy(world.grid.width as usize, world.grid.height as usize);
    let tier = select_viewport_tier(full.width, full.height);
    let (viewport_width, viewport_height) = tier.size();
    let mut camera = ui.camera;
    if camera.follow_hero {
        camera.center_on(hero.x, hero.y);
    }
    let viewport = Viewport::from_camera(&camera, viewport_width, viewport_height);
    let viewport_rect = viewport_rect(full, tier);
    let clock = ClockPanel;
    let field = FieldPanel;
    let status = StatusBarPanel;

    field.render(frame, viewport_rect, world, ui, fonts, &viewport);
    draw_hero_debug(
        frame,
        &hero,
        &viewport,
        viewport_rect,
        ui.hero_offset_x,
        ui.hero_offset_y,
    );
    draw_hero(
        frame,
        &hero,
        &viewport,
        viewport_rect,
        ui.hero_offset_x,
        ui.hero_offset_y,
    );
    clock.render(frame, viewport_rect, world, ui, fonts, &viewport);
    status.render(frame, layout.bottom_bar, world, ui, fonts, &viewport);
    draw_scrollbars(frame, viewport_rect, &viewport, world);

    if ui.debug_layout {
        draw_layout_debug(frame, &layout);
    }
}

fn draw_scrollbars(frame: &mut Frame, viewport_rect: Rect, viewport: &Viewport, world: &WorldState) {
    if viewport_rect.width == 0 || viewport_rect.height == 0 {
        return;
    }

    let max_x = world.grid.width.saturating_sub(viewport.width).max(1) as f32;
    let max_y = world.grid.height.saturating_sub(viewport.height).max(1) as f32;
    let ratio_x = (viewport.x.max(0) as f32 / max_x).clamp(0.0, 1.0);
    let ratio_y = (viewport.y.max(0) as f32 / max_y).clamp(0.0, 1.0);

    let thumb_x = viewport_rect.x + ((viewport_rect.width.saturating_sub(1)) as f32 * ratio_x) as u16;
    let thumb_y = viewport_rect.y + ((viewport_rect.height.saturating_sub(1)) as f32 * ratio_y) as u16;
    let right_x = viewport_rect.x + viewport_rect.width.saturating_sub(1);
    let bottom_y = viewport_rect.y + viewport_rect.height.saturating_sub(1);

    if let Some(cell) = frame.buffer_mut().cell_mut((thumb_x, bottom_y)) {
        cell.set_symbol("▓").set_fg(Color::DarkGray);
    }
    if let Some(cell) = frame.buffer_mut().cell_mut((right_x, thumb_y)) {
        cell.set_symbol("▓").set_fg(Color::DarkGray);
    }
}
