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
        world_to_screen(
            self.world.clock_world,
            self.hud.camera.x,
            self.hud.camera.y,
            self.hud.camera.height,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::coords::world_to_screen;
    use crate::scene::viewport::Viewport;

    #[test]
    fn clock_screen_matches_the_shared_projection_helpers() {
        let world = WorldFrame {
            hero_world: WorldPos { x: 50, y: 30 },
            hero_visual_anchor: WorldPos { x: 40, y: 20 },
            clock_world: WorldPos { x: 45, y: 25 },
        };
        let hud = HudFrame {
            viewport: Viewport {
                x: 30,
                y: 10,
                width: 124,
                height: 32,
            },
            viewport_rect: Rect::new(0, 0, 124, 32),
            camera: Camera {
                x: 30,
                y: 10,
                width: 124,
                height: 32,
                follow_hero: false,
            },
        };
        let state = RenderState { world, hud };

        let expected = world_to_screen(
            state.world.clock_world,
            state.hud.camera.x,
            state.hud.camera.y,
            state.hud.camera.height,
        );
        let viewport = state
            .hud
            .viewport
            .world_to_view(state.world.clock_world.x, state.world.clock_world.y)
            .expect("clock should be inside the viewport in this test");
        let camera = state
            .hud
            .camera
            .world_to_screen(state.world.clock_world.x, state.world.clock_world.y)
            .expect("camera should project the clock in this test");

        assert_eq!(state.clock_screen(), expected);
        assert_eq!(
            (viewport.0 as i32, viewport.1 as i32),
            (expected.x, expected.y)
        );
        assert_eq!((camera.0 as i32, camera.1 as i32), (expected.x, expected.y));
    }

    #[test]
    fn hero_and_clock_projection_helpers_agree_with_viewport_origin() {
        let world = WorldFrame {
            hero_world: WorldPos { x: 50, y: 30 },
            hero_visual_anchor: WorldPos { x: 40, y: 20 },
            clock_world: WorldPos { x: 45, y: 25 },
        };
        let hud = HudFrame {
            viewport: Viewport {
                x: 30,
                y: 10,
                width: 124,
                height: 32,
            },
            viewport_rect: Rect::new(0, 0, 124, 32),
            camera: Camera {
                x: 30,
                y: 10,
                width: 124,
                height: 32,
                follow_hero: false,
            },
        };
        let state = RenderState { world, hud };

        let hero_screen = world_to_screen(
            state.world.hero_visual_anchor,
            state.hud.camera.x,
            state.hud.camera.y,
            state.hud.camera.height,
        );
        let clock_screen = state.clock_screen();
        let hero_view = state
            .hud
            .viewport
            .world_to_view(
                state.world.hero_visual_anchor.x,
                state.world.hero_visual_anchor.y,
            )
            .expect("hero anchor should be inside the viewport in this test");
        let clock_view = state
            .hud
            .viewport
            .world_to_view(state.world.clock_world.x, state.world.clock_world.y)
            .expect("clock should be inside the viewport in this test");

        assert_eq!(
            (hero_view.0 as i32, hero_view.1 as i32),
            (hero_screen.x, hero_screen.y)
        );
        assert_eq!(
            (clock_view.0 as i32, clock_view.1 as i32),
            (clock_screen.x, clock_screen.y)
        );
        assert_eq!(
            clock_screen,
            world_to_screen(state.world.clock_world, 30, 10, 32)
        );
    }

    #[test]
    fn zero_size_viewport_rejects_screen_coordinates() {
        let world = WorldFrame {
            hero_world: WorldPos { x: 1, y: 1 },
            hero_visual_anchor: WorldPos { x: 1, y: 1 },
            clock_world: WorldPos { x: 1, y: 1 },
        };
        let hud = HudFrame {
            viewport: Viewport {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            viewport_rect: Rect::new(0, 0, 0, 0),
            camera: Camera {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                follow_hero: false,
            },
        };
        let state = RenderState { world, hud };

        assert_eq!(state.hud.viewport.world_to_view(1, 1), None);
        assert_eq!(state.hud.camera.world_to_screen(1, 1), None);
        assert_eq!(
            state.clock_screen(),
            world_to_screen(state.world.clock_world, 0, 0, 0)
        );
        assert_eq!(state.hud.viewport_rect.width, 0);
        assert_eq!(state.hud.viewport_rect.height, 0);
    }
}
