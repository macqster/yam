pub use crate::core::spatial::{
    SpatialPoint as WorldPos, SpatialProjection as Projection, SpatialResolver,
};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct ScreenPos {
    pub x: u16,
    pub y: u16,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Space {
    World,
    Anchor(EntityId),
    Screen,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct EntityId(pub u32);

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct Element {
    pub space: Space,
    pub position: WorldPos,
}

pub fn anchor_to_world(anchor: WorldPos, offset: WorldPos) -> WorldPos {
    SpatialResolver::anchor_to_world(anchor, offset)
}

/// World-ui resolves in world space and stays pinned to its world attachment.
#[allow(dead_code)]
pub fn resolve_world_ui(anchor: WorldPos, offset: WorldPos) -> WorldPos {
    anchor_to_world(anchor, offset)
}

/// Hud-ui is screen-attached and must not inherit world motion.
#[allow(dead_code)]
pub fn resolve_hud_ui(screen: WorldPos) -> WorldPos {
    screen
}

pub fn world_to_screen(world: WorldPos, camera_x: i32, camera_y: i32) -> WorldPos {
    SpatialResolver::new(Projection::new(camera_x, camera_y, 0, 0)).world_to_screen(world)
}

#[allow(dead_code)]
pub fn screen_to_world(screen: WorldPos, camera_x: i32, camera_y: i32) -> WorldPos {
    SpatialResolver::new(Projection::new(camera_x, camera_y, 0, 0)).screen_to_world(screen)
}

#[allow(dead_code)]
pub fn resolve_position(element: &Element, camera_x: i32, camera_y: i32) -> WorldPos {
    match element.space {
        Space::World => world_to_screen(element.position, camera_x, camera_y),
        Space::Anchor(_) => world_to_screen(element.position, camera_x, camera_y),
        Space::Screen => element.position,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_then_project_preserves_relative_offset() {
        let hero_world = WorldPos { x: 150, y: 60 };
        let hero_offset = WorldPos { x: -110, y: -54 };
        let clock_offset = WorldPos { x: 96, y: 9 };
        let camera = WorldPos { x: 30, y: 10 };

        let hero_visual_anchor = anchor_to_world(hero_world, hero_offset);
        let clock_world = anchor_to_world(hero_visual_anchor, clock_offset);
        let clock_screen = world_to_screen(clock_world, camera.x, camera.y);
        let hero_screen = world_to_screen(hero_visual_anchor, camera.x, camera.y);

        assert_eq!(hero_visual_anchor, WorldPos { x: 40, y: 6 });
        assert_eq!(clock_world, WorldPos { x: 136, y: 15 });
        assert_eq!(hero_screen, WorldPos { x: 10, y: -4 });
        assert_eq!(clock_screen, WorldPos { x: 106, y: 5 });
    }

    #[test]
    fn screen_space_is_camera_invariant() {
        let panel = WorldPos { x: 10, y: 5 };
        let camera_a = WorldPos { x: 0, y: 0 };
        let camera_b = WorldPos { x: 80, y: 20 };

        assert_eq!(
            resolve_position(
                &Element {
                    space: Space::Screen,
                    position: panel
                },
                camera_a.x,
                camera_a.y
            ),
            panel
        );
        assert_eq!(
            resolve_position(
                &Element {
                    space: Space::Screen,
                    position: panel
                },
                camera_b.x,
                camera_b.y
            ),
            panel
        );
    }

    #[test]
    fn world_ui_tracks_world_and_camera() {
        let hero = WorldPos { x: 150, y: 60 };
        let offset = WorldPos { x: -110, y: -54 };
        let camera_a = WorldPos { x: 0, y: 0 };
        let camera_b = WorldPos { x: 30, y: 10 };

        let hero_world_a = resolve_world_ui(hero, offset);
        let hero_world_b = resolve_world_ui(hero, offset);

        assert_eq!(hero_world_a, WorldPos { x: 40, y: 6 });
        assert_eq!(hero_world_b, WorldPos { x: 40, y: 6 });
        assert_eq!(camera_a, WorldPos { x: 0, y: 0 });
        assert_eq!(camera_b, WorldPos { x: 30, y: 10 });
    }

    #[test]
    fn hud_ui_is_screen_attached() {
        let hud = WorldPos { x: 8, y: 3 };
        let second = WorldPos { x: 8, y: 3 };
        assert_eq!(resolve_hud_ui(hud), hud);
        assert_eq!(resolve_hud_ui(second), hud);
    }

    #[test]
    fn world_pinned_clock_keeps_hero_attachment_and_hud_stays_screen_attached() {
        let hero_world = WorldPos { x: 150, y: 60 };
        let hero_offset = WorldPos { x: -110, y: -54 };
        let clock_offset = WorldPos { x: 96, y: 9 };
        let camera_a = WorldPos { x: 0, y: 0 };
        let camera_b = WorldPos { x: 30, y: 10 };
        let hud_a = WorldPos { x: 8, y: 3 };
        let hud_b = WorldPos { x: 8, y: 3 };

        let hero_visual_anchor = resolve_world_ui(hero_world, hero_offset);
        let clock_world = anchor_to_world(hero_visual_anchor, clock_offset);

        assert_eq!(hero_visual_anchor, WorldPos { x: 40, y: 6 });
        assert_eq!(clock_world, WorldPos { x: 136, y: 15 });
        assert_eq!(
            world_to_screen(hero_visual_anchor, camera_a.x, camera_a.y),
            WorldPos { x: 40, y: 6 }
        );
        assert_eq!(
            world_to_screen(hero_visual_anchor, camera_b.x, camera_b.y),
            WorldPos { x: 10, y: -4 }
        );
        assert_eq!(resolve_hud_ui(hud_a), hud_b);
    }

    #[test]
    fn world_origin_and_quadrants_are_signed_from_zero() {
        let datum = WorldPos { x: 0, y: 0 };
        let top_left = WorldPos { x: -1, y: -1 };
        let top_right = WorldPos { x: 1, y: -1 };
        let bottom_left = WorldPos { x: -1, y: 1 };
        let bottom_right = WorldPos { x: 1, y: 1 };

        assert_eq!(datum, WorldPos { x: 0, y: 0 });
        assert!(top_left.x < datum.x && top_left.y < datum.y);
        assert!(top_right.x > datum.x && top_right.y < datum.y);
        assert!(bottom_left.x < datum.x && bottom_left.y > datum.y);
        assert!(bottom_right.x > datum.x && bottom_right.y > datum.y);
    }
}
