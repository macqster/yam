use crate::ui::layout::LayoutRegions;
use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders};

pub fn draw_layout_debug(frame: &mut Frame, layout: &LayoutRegions) {
    let style = Style::default().fg(Color::Cyan);
    let left = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::LightDoubleDashed)
        .title(Line::from(format!(
            " FIELD {}x{} ",
            layout.center_left.width, layout.center_left.height
        )))
        .style(style);
    let right = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::LightDoubleDashed)
        .title(Line::from(format!(
            " CLOCK {}x{} ",
            layout.center_right_top.width, layout.center_right_top.height
        )))
        .style(style);
    let right_bottom = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::LightDoubleDashed)
        .title(Line::from(format!(
            " UNUSED {}x{} ",
            layout.center_right_bottom.width, layout.center_right_bottom.height
        )))
        .style(style);
    let bottom = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::LightDoubleDashed)
        .title(Line::from(format!(
            " STATUS {}x{} ",
            layout.bottom_bar.width, layout.bottom_bar.height
        )))
        .style(style);

    frame.render_widget(left, layout.center_left);
    frame.render_widget(right, layout.center_right_top);
    frame.render_widget(right_bottom, layout.center_right_bottom);
    frame.render_widget(bottom, layout.bottom_bar);
}
