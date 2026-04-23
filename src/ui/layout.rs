use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct LayoutRegions {
    pub center_left: Rect,
    pub center_right_top: Rect,
    pub center_right_bottom: Rect,
    pub bottom_bar: Rect,
}

pub fn compute_layout(area: Rect) -> LayoutRegions {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let main = vertical[0];
    let bottom_bar = vertical[1];

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main);

    let right_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(horizontal[1]);

    LayoutRegions {
        center_left: horizontal[0],
        center_right_top: right_vertical[0],
        center_right_bottom: right_vertical[1],
        bottom_bar,
    }
}
