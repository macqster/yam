use crate::scene::camera::Camera;
use crate::scene::coords::WorldPos;
use crate::scene::viewport::Viewport;
use ratatui::prelude::Rect;

#[derive(Clone, Copy)]
pub struct RenderState {
    pub viewport: Viewport,
    pub viewport_rect: Rect,
    pub camera: Camera,
    pub hero_world: WorldPos,
    pub hero_visual_anchor: WorldPos,
    pub clock_screen: WorldPos,
}
