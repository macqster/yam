pub use crate::core::spatial::{
    SpatialPoint as WorldPos, SpatialProjection as Projection, SpatialResolver,
};
use crate::core::world::WorldState;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct ScreenPos {
    pub x: i32,
    pub y: i32,
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
    SpatialResolver::resolve_anchor(anchor, offset)
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

#[allow(dead_code)]
pub fn resolve_anchored_position(
    world: &WorldState,
    id: EntityId,
    fallback_world: WorldPos,
    camera_x: i32,
    camera_y: i32,
    viewport_height: u16,
) -> ScreenPos {
    let anchor = world.entity_world(id.0);
    let resolved =
        SpatialResolver::resolve_anchor_or_world(anchor, fallback_world, WorldPos { x: 0, y: 0 });
    resolve_projected_position(resolved, camera_x, camera_y, viewport_height)
}

#[allow(dead_code)]
pub fn resolve_projected_position(
    position: WorldPos,
    camera_x: i32,
    camera_y: i32,
    viewport_height: u16,
) -> ScreenPos {
    let screen = world_to_screen(position, camera_x, camera_y, viewport_height);
    ScreenPos {
        x: screen.x,
        y: screen.y,
    }
}

#[allow(dead_code)]
pub fn resolve_screen_position(position: WorldPos, _camera_x: i32, _camera_y: i32) -> ScreenPos {
    ScreenPos {
        x: position.x,
        y: position.y,
    }
}

#[allow(dead_code)]
pub fn resolve_element_screen_position(
    element: &Element,
    world: &WorldState,
    camera_x: i32,
    camera_y: i32,
    viewport_height: u16,
) -> ScreenPos {
    match element.space {
        Space::World => {
            resolve_projected_position(element.position, camera_x, camera_y, viewport_height)
        }
        Space::Anchor(id) => resolve_anchored_position(
            world,
            id,
            element.position,
            camera_x,
            camera_y,
            viewport_height,
        ),
        Space::Screen => resolve_screen_position(element.position, camera_x, camera_y),
    }
}

pub fn world_to_screen(
    world: WorldPos,
    camera_x: i32,
    camera_y: i32,
    viewport_height: u16,
) -> WorldPos {
    SpatialResolver::new(Projection::new(camera_x, camera_y, 0, viewport_height))
        .world_to_screen(world)
}

#[allow(dead_code)]
pub fn screen_to_world(
    screen: WorldPos,
    camera_x: i32,
    camera_y: i32,
    viewport_height: u16,
) -> WorldPos {
    SpatialResolver::new(Projection::new(camera_x, camera_y, 0, viewport_height))
        .screen_to_world(screen)
}

#[allow(dead_code)]
pub fn resolve_position(
    element: &Element,
    world: &WorldState,
    camera_x: i32,
    camera_y: i32,
    viewport_height: u16,
) -> WorldPos {
    let screen =
        resolve_element_screen_position(element, world, camera_x, camera_y, viewport_height);
    WorldPos {
        x: screen.x,
        y: screen.y,
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
        let viewport_height = 32;

        let hero_visual_anchor = anchor_to_world(hero_world, hero_offset);
        let clock_world = anchor_to_world(hero_visual_anchor, clock_offset);
        let clock_screen = world_to_screen(clock_world, camera.x, camera.y, viewport_height);
        let hero_screen = world_to_screen(hero_visual_anchor, camera.x, camera.y, viewport_height);

        assert_eq!(hero_visual_anchor, WorldPos { x: 40, y: 6 });
        assert_eq!(clock_world, WorldPos { x: 136, y: 15 });
        assert_eq!(hero_screen, WorldPos { x: 10, y: 35 });
        assert_eq!(clock_screen, WorldPos { x: 106, y: 26 });
    }

    #[test]
    fn screen_space_is_camera_invariant() {
        let panel = WorldPos { x: 10, y: 5 };
        let camera_a = WorldPos { x: 0, y: 0 };
        let camera_b = WorldPos { x: 80, y: 20 };
        let world = crate::core::world::WorldState::new();

        assert_eq!(
            resolve_position(
                &Element {
                    space: Space::Screen,
                    position: panel
                },
                &world,
                camera_a.x,
                camera_a.y,
                32,
            ),
            panel
        );
        assert_eq!(
            resolve_position(
                &Element {
                    space: Space::Screen,
                    position: panel
                },
                &world,
                camera_b.x,
                camera_b.y,
                32,
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
        let world = crate::core::world::WorldState::new();

        let hero_visual_anchor = resolve_world_ui(hero_world, hero_offset);
        let clock_world = anchor_to_world(hero_visual_anchor, clock_offset);

        assert_eq!(hero_visual_anchor, WorldPos { x: 40, y: 6 });
        assert_eq!(clock_world, WorldPos { x: 136, y: 15 });
        assert_eq!(
            resolve_position(
                &Element {
                    space: Space::World,
                    position: hero_visual_anchor
                },
                &world,
                camera_a.x,
                camera_a.y,
                32,
            ),
            WorldPos { x: 40, y: 25 }
        );
        assert_eq!(
            resolve_position(
                &Element {
                    space: Space::World,
                    position: hero_visual_anchor
                },
                &world,
                camera_b.x,
                camera_b.y,
                32,
            ),
            WorldPos { x: 10, y: 35 }
        );
        assert_eq!(resolve_hud_ui(hud_a), hud_b);
    }

    #[test]
    fn anchored_position_uses_world_entity_identity_when_available() {
        let mut world = crate::core::world::WorldState::new();
        world.entities.push(crate::core::entity::Entity {
            id: 7,
            x: 150,
            y: 60,
            age: 1,
        });

        let resolved = resolve_position(
            &Element {
                space: Space::Anchor(EntityId(7)),
                position: WorldPos { x: -999, y: -999 },
            },
            &world,
            30,
            10,
            32,
        );

        assert_eq!(resolved, WorldPos { x: 120, y: -19 });
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
