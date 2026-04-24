use crate::ui::layout::LayoutRegions;
use crate::theme::style as theme_style;
use crate::ui::widgets::debug::layout_title;
use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders};

pub fn draw_layout_debug(frame: &mut Frame, layout: &LayoutRegions) {
    let style = theme_style::accent_border();
    let left = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::LightDoubleDashed)
        .title(Line::from(layout_title(
            "FIELD",
            layout.center_left.width,
            layout.center_left.height,
        )))
        .style(style);
    let right = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::LightDoubleDashed)
        .title(Line::from(layout_title(
            "CLOCK",
            layout.center_right_top.width,
            layout.center_right_top.height,
        )))
        .style(style);
    let right_bottom = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::LightDoubleDashed)
        .title(Line::from(layout_title(
            "UNUSED",
            layout.center_right_bottom.width,
            layout.center_right_bottom.height,
        )))
        .style(style);
    let bottom = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::LightDoubleDashed)
        .title(Line::from(layout_title(
            "STATUS",
            layout.bottom_bar.width,
            layout.bottom_bar.height,
        )))
        .style(style);

    frame.render_widget(left, layout.center_left);
    frame.render_widget(right, layout.center_right_top);
    frame.render_widget(right_bottom, layout.center_right_bottom);
    frame.render_widget(bottom, layout.bottom_bar);
}
