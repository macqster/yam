use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::ui::layout::LayoutRegions;
use crate::ui::scene::build_ui_layers;
use crate::ui::state::UiState;
use crate::scene::viewport::{select_viewport_tier, viewport_rect, Viewport};
use ratatui::prelude::*;

pub mod layers;
pub mod camera;
pub mod viewport;

pub trait Layer {
    fn z_index(&self) -> i32;
    fn render(
        &self,
        frame: &mut Frame<'_>,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        viewport: &Viewport,
        viewport_rect: Rect,
        layout: &LayoutRegions,
    );
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
        let layout = crate::ui::layout::compute_layout(full);
        let hero = &ui.hero;
        let tier = select_viewport_tier(full.width, full.height);
        let (viewport_width, viewport_height) = tier.size();
        let mut camera = ui.camera;
        camera.width = viewport_width;
        camera.height = viewport_height;
        if camera.follow_hero {
            camera.center_on(hero.x, hero.y);
        }
        let viewport = Viewport::from_camera(&camera, viewport_width, viewport_height);
        let viewport_rect = viewport_rect(full, tier);

        let mut layers = self.layers.iter().collect::<Vec<_>>();
        layers.sort_by_key(|layer| layer.z_index());
        for layer in layers {
            layer.render(frame, world, ui, fonts, &viewport, viewport_rect, &layout);
        }
    }
}

pub fn render_scene(frame: &mut Frame<'_>, world: &WorldState, ui: &UiState, fonts: &FontRegistry) {
    let scene = Scene::new(build_ui_layers());
    scene.render(frame, world, ui, fonts);
}
