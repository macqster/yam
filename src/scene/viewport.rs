use ratatui::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ViewportTier {
    Small,
    Large,
}

impl ViewportTier {
    pub fn size(&self) -> (u16, u16) {
        match self {
            ViewportTier::Small => (72, 36),
            ViewportTier::Large => (96, 48),
        }
    }
}

pub fn select_viewport_tier(term_width: u16, term_height: u16) -> ViewportTier {
    if term_width >= 100 && term_height >= 50 {
        ViewportTier::Large
    } else {
        ViewportTier::Small
    }
}

#[derive(Clone, Copy)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
}

impl Viewport {
    pub fn from_camera(camera: &crate::scene::camera::Camera, width: u16, height: u16) -> Self {
        Self {
            x: camera.x - (width as i32 / 2),
            y: camera.y - (height as i32 / 2),
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

pub fn viewport_rect(area: Rect, tier: ViewportTier) -> Rect {
    let (width, height) = tier.size();
    let offset_x = (area.width.saturating_sub(width)) / 2;
    let offset_y = (area.height.saturating_sub(height)) / 2;
    Rect {
        x: area.x + offset_x,
        y: area.y + offset_y,
        width,
        height,
    }
}
