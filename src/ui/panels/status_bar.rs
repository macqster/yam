use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::theme::style as theme_style;
use crate::ui::panel::Panel;
use crate::ui::state::UiState;
use crate::ui::viewport::Viewport;
use ratatui::{
    layout::Alignment,
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Block, Paragraph},
};

pub struct StatusBarPanel;

fn build_label() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let build_time = env!("YAM_BUILD_TIME");
    format!("yam {version}, build {build_time}")
}

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
        let footer_bg =
            Block::default().style(theme_style::panel_text());
        frame.render_widget(footer_bg, area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Length(12)])
            .split(area);

        let left = Paragraph::new(format!(
            "q - quit   •   space - play/pause   •   . - step   •   d - debug   •   {{ or }} clock font – {}",
            crate::render::fonts::FontRegistry::display_name(ui.clock_font)
        ))
        .style(theme_style::panel_text())
        .alignment(Alignment::Left);
        frame.render_widget(left, chunks[0]);

        let right = Paragraph::new(build_label())
            .style(theme_style::panel_text())
            .alignment(Alignment::Right);
        frame.render_widget(right, chunks[1]);
    }
}
