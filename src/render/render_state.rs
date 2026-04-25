use crate::scene::camera::Camera;
use crate::scene::coords::{world_to_screen, WorldPos};
use crate::scene::viewport::Viewport;
use ratatui::prelude::Rect;

#[derive(Clone, Copy, Debug)]
pub struct WorldFrame {
    pub hero_world: WorldPos,
    pub hero_visual_anchor: WorldPos,
    pub clock_world: WorldPos,
}

#[derive(Clone, Copy, Debug)]
pub struct HudFrame {
    pub viewport: Viewport,
    pub viewport_rect: Rect,
    pub camera: Camera,
}

#[derive(Clone, Copy, Debug)]
pub struct RenderState {
    pub world: WorldFrame,
    pub hud: HudFrame,
}

impl RenderState {
    pub fn clock_screen(&self) -> WorldPos {
        world_to_screen(self.world.clock_world, self.hud.camera.x, self.hud.camera.y)
    }
}
