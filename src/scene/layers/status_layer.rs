use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::{Layer, LayerOutput};
use crate::theme::style as theme_style;
use crate::ui::anchor::{resolve_anchor, Anchor};
use crate::ui::state::UiState;
use crate::ui::widgets::status::build_status_label;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

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
        _viewport: &crate::scene::viewport::Viewport,
        _viewport_rect: Rect,
    ) {
        let _ = world;
        let screen = frame.area();
        let footer_width = screen.width;
        let pos = resolve_anchor(
            Anchor::BottomLeft,
            screen.width,
            screen.height,
            footer_width,
            1,
        );
        let area = Rect::new(screen.x + pos.x, screen.y + pos.y, footer_width, 1);
        let stamp = build_status_label();
        let stamp_width = stamp.chars().count() as u16 + 2;
        let stamp_pos = resolve_anchor(
            Anchor::BottomRight,
            screen.width,
            screen.height,
            stamp_width,
            1,
        );
        let stamp_area = Rect::new(
            screen.x + stamp_pos.x,
            screen.y + stamp_pos.y,
            stamp_width.min(footer_width),
            1,
        );

        let left_text = if ui.debug_layout {
            "q - quit   •   space - play/pause   •   . - step   •   d - debug   •   hjkl hero   •   HJKL clock   •   { } font".to_string()
        } else {
            "q - quit   •   space - play/pause   •   . - step".to_string()
        };

        let left = Paragraph::new(left_text)
            .style(theme_style::panel_text())
            .alignment(Alignment::Left);
        frame.render_widget(left, area);

        let right = Paragraph::new(stamp)
            .style(theme_style::panel_text())
            .alignment(Alignment::Right);
        frame.render_widget(right, stamp_area);
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _viewport: &crate::scene::viewport::Viewport,
        _viewport_rect: Rect,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let screen_width = grid.width;
        let screen_height = grid.height;
        let footer_y = screen_height.saturating_sub(1);
        let left_text = if ui.debug_layout {
            "q - quit   •   space - play/pause   •   . - step   •   d - debug   •   hjkl hero   •   HJKL clock   •   { } font"
        } else {
            "q - quit   •   space - play/pause   •   . - step"
        };
        let right_text = build_status_label();
        write_string(&mut grid, 0, footer_y, left_text, theme_style::panel_text());
        let stamp_width = right_text.chars().count() as u16 + 2;
        let x = screen_width.saturating_sub(stamp_width);
        write_string(
            &mut grid,
            x,
            footer_y,
            &right_text,
            theme_style::panel_text(),
        );
        LayerOutput { grid, mask: None }
    }
}
