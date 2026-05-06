/// Camera stores the world-space origin of the visible crop.
/// It is not the viewport itself.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
    pub follow_hero: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            follow_hero: false,
        }
    }

    #[allow(dead_code)]
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        if !self.follow_hero {
            self.x += dx;
            self.y += dy;
        }
    }

    #[allow(dead_code)]
    pub fn center_on(&mut self, hero_x: i32, hero_y: i32) {
        if self.follow_hero {
            self.x = hero_x;
            self.y = hero_y;
        }
    }

    #[allow(dead_code)]
    pub fn world_to_screen(&self, wx: i32, wy: i32) -> Option<(u16, u16)> {
        let sx = wx - self.x;
        let top = self.y + self.height as i32 - 1;
        let sy = top - wy;
        if sx >= 0 && sy >= 0 && sx < self.width as i32 && sy < self.height as i32 {
            Some((sx as u16, sy as u16))
        } else {
            None
        }
    }
}
