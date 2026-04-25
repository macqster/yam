/// Viewport stores the terminal-sized crop rectangle.
/// It follows the camera origin but is not the same thing as camera.
#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
}

impl Viewport {
    pub fn from_camera(camera: &crate::scene::camera::Camera, width: u16, height: u16) -> Self {
        Self {
            x: camera.x,
            y: camera.y,
            width,
            height,
        }
    }

    #[allow(dead_code)]
    pub fn world_to_view(&self, wx: i32, wy: i32) -> Option<(u16, u16)> {
        let vx = wx - self.x;
        let vy = wy - self.y;
        if vx >= 0 && vy >= 0 && vx < self.width as i32 && vy < self.height as i32 {
            Some((vx as u16, vy as u16))
        } else {
            None
        }
    }
}
