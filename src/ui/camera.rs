#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub follow_hero: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            follow_hero: true,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        if !self.follow_hero {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn center_on(&mut self, hero_x: i32, hero_y: i32) {
        if self.follow_hero {
            self.x = hero_x;
            self.y = hero_y;
        }
    }
}
