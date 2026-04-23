use crate::core::world::WorldState;

/// Recompute influence fields (density, attraction, avoidance)
pub fn update_fields(world: &mut WorldState) {
    let width = world.grid.width;
    let height = world.grid.height;

    for i in 0..world.fields.density.len() {
        world.fields.density[i] = 0.0;
        world.fields.attraction[i] = 0.0;
        world.fields.avoidance[i] = 0.0;
    }

    for entity in &world.entities {
        let idx = world.grid.index(entity.x, entity.y);
        world.fields.density[idx] = 1.0;
    }

    let center_x = width / 2;
    let center_y = height / 2;
    for y in 0..height {
        for x in 0..width {
            let dx = (center_x as i32 - x as i32).abs() as f32;
            let dy = (center_y as i32 - y as i32).abs() as f32;
            let dist = dx + dy;
            let idx = world.grid.index(x, y);
            world.fields.attraction[idx] = 1.0 / (1.0 + dist);
        }
    }

    for y in 0..height {
        for x in 0..width {
            let idx = world.grid.index(x, y);
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                world.fields.avoidance[idx] = 1.0;
            }
        }
    }
}
