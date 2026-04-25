use crate::scene::camera::Camera;
use crate::scene::coords::WorldPos;
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
