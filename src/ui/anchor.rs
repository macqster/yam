use crate::scene::coords::ScreenPos;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Anchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

#[allow(dead_code)]
pub fn resolve_anchor(
    anchor: Anchor,
    width: u16,
    height: u16,
    element_w: u16,
    element_h: u16,
) -> ScreenPos {
    let x = match anchor {
        Anchor::TopLeft | Anchor::BottomLeft => 0,
        Anchor::TopRight | Anchor::BottomRight => width.saturating_sub(element_w),
        Anchor::Center => width.saturating_sub(element_w) / 2,
    };
    let y = match anchor {
        Anchor::TopLeft | Anchor::TopRight => 0,
        Anchor::BottomLeft | Anchor::BottomRight => height.saturating_sub(element_h),
        Anchor::Center => height.saturating_sub(element_h) / 2,
    };
    ScreenPos { x, y }
}
