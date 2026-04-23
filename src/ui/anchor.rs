use ratatui::layout::Rect;

#[derive(Clone, Copy, Debug)]
pub struct Anchor {
    pub x: f32,
    pub y: f32,
    pub offset_x: i16,
    pub offset_y: i16,
}

impl Anchor {
    pub fn resolve(&self, area: Rect) -> (u16, u16) {
        let px = (area.width as f32 * self.x) as i16 + self.offset_x;
        let py = (area.height as f32 * self.y) as i16 + self.offset_y;
        let px = px.clamp(0, area.width as i16 - 1) as u16;
        let py = py.clamp(0, area.height as i16 - 1) as u16;
        (px, py)
    }
}
