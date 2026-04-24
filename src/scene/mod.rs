use crate::core::world::WorldState;
use crate::render::compositor::{grid_to_lines, merge_grid, Grid};
use crate::render::fonts::FontRegistry;
use crate::render::mask::Mask;
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

#[derive(Clone, Copy)]
pub struct FrameContext {
    pub viewport: Viewport,
    pub viewport_rect: Rect,
    pub camera: crate::scene::camera::Camera,
    pub hero_world: WorldPos,
    pub hero_visual_anchor: WorldPos,
    pub clock_screen: WorldPos,
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
        ctx: &FrameContext,
    ) -> LayerOutput {
        let _ = (world, ui, fonts, ctx);
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
        let mut camera = ui.camera;
        camera.width = full.width;
        camera.height = full.height;
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
        let clock_screen = anchor_to_world(
            hero_visual_anchor,
            WorldPos {
                x: ui.offsets.clock_dx as i32,
                y: ui.offsets.clock_dy as i32,
            },
        );
        let ctx = FrameContext {
            viewport,
            viewport_rect,
            camera,
            hero_world,
            hero_visual_anchor,
            clock_screen,
        };
        let mut layers = self.layers.iter().collect::<Vec<_>>();
        layers.sort_by_key(|layer| layer.z_index());
        let mut outputs = Vec::with_capacity(layers.len());
        for layer in layers.iter() {
            outputs.push(layer.render_to_grid(full.width, full.height, world, ui, fonts, &ctx));
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
