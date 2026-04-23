use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::ui::panel::Panel;
use crate::ui::state::UiState;
use crate::ui::viewport::Viewport;
use ratatui::{
    layout::Alignment,
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::Paragraph,
};

pub struct StatusBarPanel;

impl Panel for StatusBarPanel {
    fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _viewport: &Viewport,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Length(12)])
            .split(area);

        let left = Paragraph::new(format!(
            "q - quit   •   d - debug   •   {{ or }} clock font – {}",
            crate::render::fonts::FontRegistry::display_name(ui.clock_font)
        ))
        .alignment(Alignment::Left);
        frame.render_widget(left, chunks[0]);

        let right = Paragraph::new("yam-rust 0.3").alignment(Alignment::Right);
        frame.render_widget(right, chunks[1]);
    }
}
