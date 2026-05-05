use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::theme::style as theme_style;

#[allow(dead_code)]
pub fn draw_layout_debug(frame: &mut Frame, area: Rect) {
    let width = area.width.min(34);
    let height = area.height.min(4);
    let text = Paragraph::new("debug overlay\nlayout disabled\nscene-owned UI")
        .style(theme_style::debug_text());
    frame.render_widget(text, Rect::new(area.x, area.y, width, height));
}
