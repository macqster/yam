use crate::core::world::WorldState;
use crate::render::compositor::{grid_to_lines, merge_grid, Grid};
use crate::render::fonts::FontRegistry;
use crate::render::mask::Mask;
use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
use crate::scene::coords::{anchor_to_world, WorldPos};
use crate::scene::viewport::Viewport;
use crate::ui::scene::build_ui_layers;
use crate::ui::state::UiState;
use ratatui::prelude::*;
use ratatui::widgets::{Clear, Paragraph};

pub mod camera;
pub mod coords;
pub mod layers;
pub mod viewport;

pub const WORLD_WIDTH: i32 = 212;
pub const WORLD_HEIGHT: i32 = 57;
pub const WORLD_HALF_W: i32 = WORLD_WIDTH / 2;
pub const WORLD_HALF_H: i32 = WORLD_HEIGHT / 2;

pub struct LayerOutput {
    pub grid: Grid,
    pub mask: Option<Mask>,
}

pub trait Layer {
    fn z_index(&self) -> i32;
    fn is_field_layer(&self) -> bool {
        false
    }

    #[allow(clippy::too_many_arguments)]
    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        render_state: &RenderState,
    ) -> LayerOutput {
        let _ = (world, ui, fonts, render_state);
        LayerOutput {
            grid: Grid::new(width, height),
            mask: None,
        }
    }
}

pub struct Scene {
    pub layers: Vec<Box<dyn Layer>>,
}

impl Scene {
    pub fn new(layers: Vec<Box<dyn Layer>>) -> Self {
        Self { layers }
    }

    pub fn render(
        &self,
        frame: &mut Frame<'_>,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
    ) {
        let full = frame.area();
        let render_state = build_render_state(full, ui);
        let mut layers = self.layers.iter().collect::<Vec<_>>();
        layers.sort_by_key(|layer| layer.z_index());
        let mut outputs = Vec::with_capacity(layers.len());
        for layer in layers.iter() {
            outputs.push(layer.render_to_grid(
                full.width,
                full.height,
                world,
                ui,
                fonts,
                &render_state,
            ));
        }

        let hero_mask: Option<Mask> = outputs.iter().find_map(|output| output.mask.clone());
        let mut final_grid = Grid::new(full.width, full.height);
        for (layer, output) in layers.into_iter().zip(outputs) {
            let mask_to_apply = if layer.is_field_layer() {
                hero_mask.as_ref()
            } else {
                None
            };
            merge_grid(&mut final_grid, &output.grid, mask_to_apply);
        }

        let lines = grid_to_lines(&final_grid);
        frame.render_widget(Clear, full);
        frame.render_widget(Paragraph::new(lines), full);
    }
}

pub fn render_scene(frame: &mut Frame<'_>, world: &WorldState, ui: &UiState, fonts: &FontRegistry) {
    let scene = Scene::new(build_ui_layers());
    scene.render(frame, world, ui, fonts);
}

pub fn build_render_state(full: Rect, ui: &UiState) -> RenderState {
    let camera = camera_for_frame(full, ui);
    let viewport = Viewport::from_camera(&camera, full.width, full.height);
    let viewport_rect = full;
    let hero_world = WorldPos {
        x: ui.hero.x,
        y: ui.hero.y,
    };
    let hero_visual_anchor = anchor_to_world(
        hero_world,
        WorldPos {
            x: ui.offsets.hero_dx,
            y: ui.offsets.hero_dy,
        },
    );
    let clock_world = anchor_to_world(
        hero_visual_anchor,
        WorldPos {
            x: ui.offsets.clock_dx as i32,
            y: ui.offsets.clock_dy as i32,
        },
    );
    RenderState {
        world: WorldFrame {
            hero_world,
            hero_visual_anchor,
            clock_world,
        },
        hud: HudFrame {
            viewport,
            viewport_rect,
            camera,
        },
    }
}

fn camera_for_frame(full: Rect, ui: &UiState) -> crate::scene::camera::Camera {
    let mut camera = ui.camera;
    camera.width = full.width;
    camera.height = full.height;
    if is_fullscreen_like(full) {
        camera.x = WORLD_HALF_W - full.width as i32 / 2;
        camera.y = WORLD_HALF_H - full.height as i32 / 2;
    }
    camera
}

fn is_fullscreen_like(full: Rect) -> bool {
    full.width as i32 >= WORLD_WIDTH && full.height as i32 >= WORLD_HEIGHT
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::state::UiState;

    #[test]
    fn render_state_world_facts_stay_stable_across_resize() {
        let ui = UiState::new();
        let fullscreen = Rect::new(0, 0, 215, 57);
        let windowed = Rect::new(0, 0, 132, 36);

        let full_state = build_render_state(fullscreen, &ui);
        let win_state = build_render_state(windowed, &ui);

        assert_eq!(full_state.world.hero_world, win_state.world.hero_world);
        assert_eq!(
            full_state.world.hero_visual_anchor,
            win_state.world.hero_visual_anchor
        );
        assert_eq!(full_state.world.clock_world, win_state.world.clock_world);
        assert_eq!(full_state.hud.viewport_rect.width, fullscreen.width);
        assert_eq!(win_state.hud.viewport_rect.width, windowed.width);
        assert_eq!(full_state.hud.viewport_rect.height, fullscreen.height);
        assert_eq!(win_state.hud.viewport_rect.height, windowed.height);
    }

    #[test]
    fn fullscreen_camera_is_locked_to_centered_crop() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = -77;
        ui.offsets.camera_y = 19;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;

        let fullscreen = Rect::new(0, 0, 215, 57);
        let state = build_render_state(fullscreen, &ui);

        assert_eq!(state.hud.camera.width, fullscreen.width);
        assert_eq!(state.hud.camera.height, fullscreen.height);
        assert_eq!(
            state.hud.camera.x,
            WORLD_HALF_W - fullscreen.width as i32 / 2
        );
        assert_eq!(
            state.hud.camera.y,
            WORLD_HALF_H - fullscreen.height as i32 / 2
        );
    }

    #[test]
    fn windowed_camera_keeps_mutable_offset() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = -77;
        ui.offsets.camera_y = 19;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;

        let windowed = Rect::new(0, 0, 132, 36);
        let state = build_render_state(windowed, &ui);

        assert_eq!(state.hud.camera.width, windowed.width);
        assert_eq!(state.hud.camera.height, windowed.height);
        assert_eq!(state.hud.camera.x, ui.camera.x);
        assert_eq!(state.hud.camera.y, ui.camera.y);
    }
}
