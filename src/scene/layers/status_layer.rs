use crate::core::world::WorldState;
use crate::render::fonts::FontRegistry;
use crate::theme::style as theme_style;
use crate::scene::Layer;
use crate::ui::state::UiState;
use crate::ui::widgets::status::build_status_label;
use crate::ui::viewport::Viewport;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

pub struct StatusLayer;

impl Layer for StatusLayer {
    fn z_index(&self) -> i32 {
        1000
    }

    fn render(
        &self,
        frame: &mut Frame<'_>,
        world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        viewport: &Viewport,
        _viewport_rect: Rect,
        _layout: &crate::ui::layout::LayoutRegions,
    ) {
        let _ = (world, viewport);
        let area = Rect::new(0, frame.area().bottom().saturating_sub(1), frame.area().width, 1);
        let footer_bg = Block::default().style(theme_style::panel_text());
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

        let right = Paragraph::new(build_status_label())
            .style(theme_style::panel_text())
            .alignment(Alignment::Right);
        frame.render_widget(right, chunks[1]);
    }
}
