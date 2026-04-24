#[derive(Copy, Clone, Debug)]
pub struct WorldPos {
    pub x: i32,
    pub y: i32,
}

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
