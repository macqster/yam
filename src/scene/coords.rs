#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WorldPos {
    pub x: i32,
    pub y: i32,
}

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
    WorldPos {
        x: anchor.x + offset.x,
        y: anchor.y + offset.y,
    }
}

pub fn world_to_screen(world: WorldPos, camera_x: i32, camera_y: i32) -> WorldPos {
    WorldPos {
        x: world.x - camera_x,
        y: world.y - camera_y,
    }
}

#[allow(dead_code)]
pub fn screen_to_world(screen: WorldPos, camera_x: i32, camera_y: i32) -> WorldPos {
    WorldPos {
        x: screen.x + camera_x,
        y: screen.y + camera_y,
    }
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
}
