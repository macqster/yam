use crate::core::world::WorldState;
use crate::render::compositor::{write_string, Grid};
use crate::render::fonts::FontRegistry;
use crate::scene::layers::modal::{paint_modal_shell, ModalFooter, ModalFrame};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::ui::state::UiState;

pub struct GreenhouseInspectLayer;

const H_PADDING: u16 = 4;
const V_PADDING: u16 = 2;

impl Layer for GreenhouseInspectLayer {
    fn z_index(&self) -> i32 {
        399
    }

    fn should_render(&self, ui: &UiState) -> bool {
        ui.show_dev_surfaces() && ui.meta.greenhouse_inspect_open
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        _ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        if !self.should_render(ui) {
            return LayerOutput { grid, mask: None };
        }

        let refs = world
            .greenhouse
            .as_ref()
            .and_then(|greenhouse| greenhouse.active_room())
            .map(|room| room.inspection_refs.as_slice())
            .unwrap_or(&[]);

        let content_width = refs
            .iter()
            .flat_map(|r| [r.label.chars().count(), r.short_text.chars().count()])
            .max()
            .unwrap_or(0) as u16;
        let content_height = refs.len() as u16 * 2;
        let frame = ModalFrame::centered(
            width,
            height,
            content_width
                .saturating_add(H_PADDING * 2)
                .saturating_add(2),
            content_height
                .saturating_add(V_PADDING * 2)
                .saturating_add(2),
        );
        paint_modal_shell(
            &mut grid,
            frame,
            "[I]nspect",
            Some(ModalFooter {
                left: "read-only room survey",
                right: "? ⎋",
            }),
        );

        let body_x = frame.x + 1 + H_PADDING;
        let body_y = frame.y + 1 + V_PADDING;
        for (idx, inspection) in refs.iter().enumerate() {
            let row = idx as u16 * 2;
            write_string(
                &mut grid,
                body_x,
                body_y + row,
                &inspection.label,
                ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD),
            );
            write_string(
                &mut grid,
                body_x,
                body_y + row + 1,
                &inspection.short_text,
                ratatui::style::Style::default(),
            );
        }

        LayerOutput { grid, mask: None }
    }
}

#[cfg(test)]
mod tests {
    use super::GreenhouseInspectLayer;
    use crate::core::spatial::SpatialPoint as WorldPos;
    use crate::core::world::WorldState;
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::ui::state::UiState;
    use ratatui::prelude::Rect;

    fn render_state() -> RenderState {
        RenderState {
            world: WorldFrame {
                hero_world: WorldPos { x: 50, y: 30 },
                hero_visual_anchor: WorldPos { x: 40, y: 20 },
                clock_world: WorldPos { x: 45, y: 25 },
                weather_world: WorldPos { x: 55, y: 26 },
                date_world: WorldPos { x: 45, y: 23 },
                calendar_world: WorldPos { x: 60, y: 22 },
            },
            hud: HudFrame {
                viewport: Viewport {
                    x: 30,
                    y: 10,
                    width: 124,
                    height: 32,
                },
                viewport_rect: Rect::new(0, 0, 124, 32),
                camera: Camera {
                    x: 30,
                    y: 10,
                    width: 124,
                    height: 32,
                    follow_hero: false,
                },
            },
        }
    }

    #[test]
    fn greenhouse_inspect_requires_dev_mode_and_open_state() {
        let layer = GreenhouseInspectLayer;
        let world = WorldState::new();
        let fonts = FontRegistry::new();
        let render_state = render_state();
        let ui = UiState::new();

        let closed = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        assert_eq!(closed.grid.cells[closed.grid.index(62, 16)].symbol, ' ');
    }

    #[test]
    fn greenhouse_inspect_lists_active_room_inspection_refs() {
        let layer = GreenhouseInspectLayer;
        let world = WorldState::for_kind(crate::core::world::WorldKind::Greenhouse);
        let fonts = FontRegistry::new();
        let render_state = render_state();
        let mut ui = UiState::new();
        ui.meta.dev_mode = true;
        ui.meta.greenhouse_inspect_open = true;

        let open = layer.render_to_grid(124, 32, &world, &ui, &fonts, &render_state);
        let text: String = open.grid.cells.iter().map(|cell| cell.symbol).collect();

        assert!(text.contains("[I]nspect"));
        assert!(text.contains("? ⎋"));

        let room = world
            .greenhouse
            .as_ref()
            .and_then(|greenhouse| greenhouse.active_room())
            .expect("active room");
        for inspection in &room.inspection_refs {
            assert!(
                text.contains(inspection.label.as_str()),
                "expected label {:?} to render",
                inspection.label
            );
        }
    }
}
