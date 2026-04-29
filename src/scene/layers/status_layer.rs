use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;
use crate::ui::widgets::status::build_status_label;

pub struct StatusLayer;

impl Layer for StatusLayer {
    fn z_index(&self) -> i32 {
        1000
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        _world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let footer_y = footer_row(height);
        let left_text = if ui.meta.dev_mode {
            "q - quit   •   space - play/pause   •   . - step   •   [d]ev   •   hjkl hero   •   HJKL clock   •   { } font"
        } else {
            "q - quit   •   space - play/pause   •   . - step"
        };
        let right_text = build_status_label();
        write_string(&mut grid, 0, footer_y, left_text, theme_style::panel_text());
        let stamp_width = right_text.chars().count() as u16 + 2;
        let x = width.saturating_sub(stamp_width);
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

fn footer_row(height: u16) -> u16 {
    height.saturating_sub(1)
}

#[cfg(test)]
mod tests {
    use super::footer_row;

    #[test]
    fn footer_uses_bottom_row_for_any_height() {
        assert_eq!(footer_row(57), 56);
        assert_eq!(footer_row(36), 35);
        assert_eq!(footer_row(1), 0);
        assert_eq!(footer_row(0), 0);
    }
}
