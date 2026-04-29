use crate::core::world::WorldState;
use crate::render::compositor::{grid_to_lines, merge_grid, Grid};
use crate::render::fonts::FontRegistry;
use crate::render::mask::Mask;
use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
use crate::scene::viewport::Viewport;
use crate::ui::scene::build_ui_layers;
use crate::ui::state::UiState;
use ratatui::prelude::*;
use ratatui::widgets::{Clear, Paragraph};

pub mod camera;
pub mod coords;
pub mod entity;
pub mod layers;
pub mod viewport;

pub const WORLD_WIDTH: i32 = 212;
pub const WORLD_HEIGHT: i32 = 57;
pub const WORLD_HALF_W: i32 = WORLD_WIDTH / 2;
pub const WORLD_HALF_H: i32 = WORLD_HEIGHT / 2;
pub const CAMERA_OVERSCAN_CELLS: i32 = 1;

pub struct LayerOutput {
    pub grid: Grid,
    pub mask: Option<Mask>,
}

pub trait Layer {
    fn z_index(&self) -> i32;
    fn is_field_layer(&self) -> bool {
        false
    }

    #[allow(clippy::too_many_arguments)]
    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        render_state: &RenderState,
    ) -> LayerOutput {
        let _ = (world, ui, fonts, render_state);
        LayerOutput {
            grid: Grid::new(width, height),
            mask: None,
        }
    }
}

pub struct Scene {
    pub layers: Vec<Box<dyn Layer>>,
}

impl Scene {
    pub fn new(layers: Vec<Box<dyn Layer>>) -> Self {
        Self { layers }
    }

    pub fn render(
        &self,
        frame: &mut Frame<'_>,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
    ) {
        let full = frame.area();
        let render_state = build_render_state(full, ui);
        let mut layers = self.layers.iter().collect::<Vec<_>>();
        layers.sort_by_key(|layer| layer.z_index());
        let mut outputs = Vec::with_capacity(layers.len());
        for layer in layers.iter() {
            outputs.push(layer.render_to_grid(
                full.width,
                full.height,
                world,
                ui,
                fonts,
                &render_state,
            ));
        }

        let hero_mask: Option<Mask> = outputs.iter().find_map(|output| output.mask.clone());
        let mut final_grid = Grid::new(full.width, full.height);
        for (layer, output) in layers.into_iter().zip(outputs) {
            let mask_to_apply = if layer.is_field_layer() {
                hero_mask.as_ref()
            } else {
                None
            };
            merge_grid(&mut final_grid, &output.grid, mask_to_apply);
        }

        let lines = grid_to_lines(&final_grid);
        frame.render_widget(Clear, full);
        frame.render_widget(Paragraph::new(lines), full);
    }
}

pub fn render_scene(frame: &mut Frame<'_>, world: &WorldState, ui: &UiState, fonts: &FontRegistry) {
    let scene = Scene::new(build_ui_layers());
    scene.render(frame, world, ui, fonts);
}

pub fn build_render_state(full: Rect, ui: &UiState) -> RenderState {
    let camera = camera_for_frame(full, ui);
    let viewport = Viewport::from_camera(&camera, full.width, full.height);
    let viewport_rect = full;
    let attachment = ui.hero_clock_attachment();
    let hero_world = attachment.hero_world();
    let hero_visual_anchor = attachment.hero_visual_anchor();
    let clock_world = attachment.clock_world();
    RenderState {
        world: WorldFrame {
            hero_world,
            hero_visual_anchor,
            clock_world,
        },
        hud: HudFrame {
            viewport,
            viewport_rect,
            camera,
        },
    }
}

fn camera_for_frame(full: Rect, ui: &UiState) -> crate::scene::camera::Camera {
    let mut camera = ui.camera;
    camera.width = full.width;
    camera.height = full.height;
    if is_fullscreen_like(full) {
        camera.x = -(full.width as i32) / 2;
        camera.y = -(full.height as i32) / 2;
    } else {
        clamp_camera_to_world_overscan(&mut camera);
    }
    camera
}

fn is_fullscreen_like(full: Rect) -> bool {
    full.width as i32 >= WORLD_WIDTH && full.height as i32 >= WORLD_HEIGHT
}

fn clamp_camera_to_world_overscan(camera: &mut crate::scene::camera::Camera) {
    camera.x = clamp_axis_to_world_overscan(
        camera.x,
        -WORLD_HALF_W,
        WORLD_HALF_W - 1,
        camera.width as i32,
    );
    camera.y = clamp_axis_to_world_overscan(
        camera.y,
        -WORLD_HALF_H,
        WORLD_HALF_H - 1,
        camera.height as i32,
    );
}

fn clamp_axis_to_world_overscan(
    camera_origin: i32,
    world_min: i32,
    world_max: i32,
    viewport_len: i32,
) -> i32 {
    let min_origin = world_min - CAMERA_OVERSCAN_CELLS;
    let max_origin = world_max + CAMERA_OVERSCAN_CELLS - viewport_len + 1;
    if min_origin > max_origin {
        -(viewport_len / 2)
    } else {
        camera_origin.clamp(min_origin, max_origin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::world::WorldState;
    use crate::render::compositor::Grid;
    use crate::render::fonts::FontRegistry;
    use crate::render::mask::Mask;
    use crate::scene::coords::world_to_screen;
    use crate::ui::state::UiState;
    use ratatui::backend::TestBackend;
    use ratatui::prelude::Rect;
    use ratatui::Terminal;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // invariant: docs/scene-model.md#projection
    fn buffer_hash(buffer: &ratatui::buffer::Buffer) -> u64 {
        let mut hasher = DefaultHasher::new();
        buffer.area.hash(&mut hasher);
        for cell in &buffer.content {
            cell.symbol().hash(&mut hasher);
            cell.style().hash(&mut hasher);
        }
        hasher.finish()
    }

    #[test]
    fn render_state_world_facts_stay_stable_across_resize() {
        let ui = UiState::new();
        let fullscreen = Rect::new(0, 0, 215, 57);
        let windowed = Rect::new(0, 0, 132, 36);

        let full_state = build_render_state(fullscreen, &ui);
        let win_state = build_render_state(windowed, &ui);

        assert_eq!(full_state.world.hero_world, win_state.world.hero_world);
        assert_eq!(
            full_state.world.hero_visual_anchor,
            win_state.world.hero_visual_anchor
        );
        assert_eq!(full_state.world.clock_world, win_state.world.clock_world);
        assert_eq!(full_state.hud.viewport_rect.width, fullscreen.width);
        assert_eq!(win_state.hud.viewport_rect.width, windowed.width);
        assert_eq!(full_state.hud.viewport_rect.height, fullscreen.height);
        assert_eq!(win_state.hud.viewport_rect.height, windowed.height);
    }

    #[test]
    fn fullscreen_camera_is_centered_on_datum() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = -77;
        ui.offsets.camera_y = 19;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;

        let fullscreen = Rect::new(0, 0, 215, 57);
        let state = build_render_state(fullscreen, &ui);

        assert_eq!(state.hud.camera.width, fullscreen.width);
        assert_eq!(state.hud.camera.height, fullscreen.height);
        assert_eq!(state.hud.camera.x, -(fullscreen.width as i32) / 2);
        assert_eq!(state.hud.camera.y, -(fullscreen.height as i32) / 2);
    }

    #[test]
    fn default_camera_matches_124x32_starting_baseline() {
        let ui = UiState::new();
        let windowed = Rect::new(0, 0, 124, 32);
        let state = build_render_state(windowed, &ui);

        assert_eq!(state.hud.camera.width, windowed.width);
        assert_eq!(state.hud.camera.height, windowed.height);
        assert_eq!(state.hud.camera.x, -69);
        assert_eq!(state.hud.camera.y, -17);
    }

    #[test]
    fn windowed_camera_clamps_to_one_cell_world_overscan() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = -200;
        ui.offsets.camera_y = 200;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;

        let windowed = Rect::new(0, 0, 124, 32);
        let state = build_render_state(windowed, &ui);

        assert_eq!(state.hud.camera.width, windowed.width);
        assert_eq!(state.hud.camera.height, windowed.height);
        assert_eq!(state.hud.camera.x, -107);
        assert_eq!(state.hud.camera.y, -3);
    }

    #[test]
    fn resize_round_trip_preserves_world_facts_and_windowed_camera_rules() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = -200;
        ui.offsets.camera_y = 200;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;

        let windowed = Rect::new(0, 0, 124, 32);
        let fullscreen = Rect::new(0, 0, 215, 57);

        let windowed_state = build_render_state(windowed, &ui);
        let fullscreen_state = build_render_state(fullscreen, &ui);
        let round_tripped_state = build_render_state(windowed, &ui);

        assert_eq!(
            windowed_state.world.hero_world,
            fullscreen_state.world.hero_world
        );
        assert_eq!(
            windowed_state.world.hero_visual_anchor,
            fullscreen_state.world.hero_visual_anchor
        );
        assert_eq!(
            windowed_state.world.clock_world,
            fullscreen_state.world.clock_world
        );
        assert_eq!(windowed_state.hud.camera.x, -107);
        assert_eq!(windowed_state.hud.camera.y, -3);
        assert_eq!(
            fullscreen_state.hud.camera.x,
            -(fullscreen.width as i32) / 2
        );
        assert_eq!(
            fullscreen_state.hud.camera.y,
            -(fullscreen.height as i32) / 2
        );
        assert_eq!(
            round_tripped_state.hud.camera.x,
            windowed_state.hud.camera.x
        );
        assert_eq!(
            round_tripped_state.hud.camera.y,
            windowed_state.hud.camera.y
        );
    }

    #[test]
    fn windowed_camera_keeps_mutable_offset_inside_bounds() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = -77;
        ui.offsets.camera_y = -17;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;

        let windowed = Rect::new(0, 0, 124, 32);
        let state = build_render_state(windowed, &ui);

        assert_eq!(state.hud.camera.x, ui.camera.x);
        assert_eq!(state.hud.camera.y, ui.camera.y);
    }

    #[test]
    fn active_layer_z_indices_remain_sorted_by_presentation_order() {
        let layers = build_ui_layers();
        let z_indices: Vec<i32> = layers.iter().map(|layer| layer.z_index()).collect();

        assert_eq!(z_indices, vec![0, 10, 100, 300, 1000]);
    }

    struct MaskedFieldLayer;
    impl Layer for MaskedFieldLayer {
        fn z_index(&self) -> i32 {
            10
        }

        fn is_field_layer(&self) -> bool {
            true
        }

        fn render_to_grid(
            &self,
            width: u16,
            height: u16,
            _world: &WorldState,
            _ui: &UiState,
            _fonts: &FontRegistry,
            _render_state: &RenderState,
        ) -> LayerOutput {
            let mut grid = Grid::new(width, height);
            if let Some(cell) = grid.cell_mut(0, 0) {
                cell.symbol = 'F';
            }
            LayerOutput { grid, mask: None }
        }
    }

    struct MaskLayer;
    impl Layer for MaskLayer {
        fn z_index(&self) -> i32 {
            0
        }

        fn render_to_grid(
            &self,
            width: u16,
            height: u16,
            _world: &WorldState,
            _ui: &UiState,
            _fonts: &FontRegistry,
            _render_state: &RenderState,
        ) -> LayerOutput {
            let grid = Grid::new(width, height);
            let mut mask = Mask::new(width as usize, height as usize);
            if width > 0 && height > 0 {
                mask.set(0, 0, false);
            }
            LayerOutput {
                grid,
                mask: Some(mask),
            }
        }
    }

    #[test]
    fn hero_mask_only_blocks_the_field_layer() {
        let layers: Vec<Box<dyn Layer>> = vec![Box::new(MaskLayer), Box::new(MaskedFieldLayer)];
        let scene = Scene::new(layers);
        let mut terminal = Terminal::new(TestBackend::new(2, 1)).expect("terminal should init");
        let world = WorldState::new();
        let ui = UiState::new();
        let fonts = FontRegistry::new();

        terminal
            .draw(|frame| scene.render(frame, &world, &ui, &fonts))
            .expect("scene render should succeed");

        let buffer = terminal.backend().buffer();
        assert_eq!(buffer.content[0].symbol(), " ");
    }

    struct HudOverlayLayer;
    impl Layer for HudOverlayLayer {
        fn z_index(&self) -> i32 {
            100
        }

        fn render_to_grid(
            &self,
            width: u16,
            height: u16,
            _world: &WorldState,
            _ui: &UiState,
            _fonts: &FontRegistry,
            _render_state: &RenderState,
        ) -> LayerOutput {
            let mut grid = Grid::new(width, height);
            if let Some(cell) = grid.cell_mut(0, 0) {
                cell.symbol = 'U';
            }
            LayerOutput { grid, mask: None }
        }
    }

    #[test]
    fn hero_mask_does_not_block_non_field_layers() {
        let layers: Vec<Box<dyn Layer>> = vec![
            Box::new(MaskLayer),
            Box::new(MaskedFieldLayer),
            Box::new(HudOverlayLayer),
        ];
        let scene = Scene::new(layers);
        let mut terminal = Terminal::new(TestBackend::new(2, 1)).expect("terminal should init");
        let world = WorldState::new();
        let ui = UiState::new();
        let fonts = FontRegistry::new();

        terminal
            .draw(|frame| scene.render(frame, &world, &ui, &fonts))
            .expect("scene render should succeed");

        let buffer = terminal.backend().buffer();
        assert_eq!(buffer.content[0].symbol(), "U");
    }

    #[test]
    fn full_frame_render_is_deterministic_for_identical_inputs() {
        let backend = TestBackend::new(132, 36);
        let mut terminal = Terminal::new(backend).expect("terminal should initialize");
        let world = crate::core::world::WorldState::new();
        let ui = UiState::new();
        let fonts = crate::render::fonts::FontRegistry::new();

        terminal
            .draw(|frame| render_scene(frame, &world, &ui, &fonts))
            .expect("first frame should render");
        let first = buffer_hash(terminal.backend().buffer());

        terminal
            .draw(|frame| render_scene(frame, &world, &ui, &fonts))
            .expect("second frame should render");
        let second = buffer_hash(terminal.backend().buffer());

        assert_eq!(terminal.backend().buffer().area.width, 132);
        assert_eq!(terminal.backend().buffer().area.height, 36);
        assert_eq!(first, second);
    }

    #[test]
    fn clock_screen_uses_active_render_state_projection() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = 30;
        ui.offsets.camera_y = 10;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;

        let windowed = Rect::new(0, 0, 132, 36);
        let state = build_render_state(windowed, &ui);
        let expected = world_to_screen(
            state.world.clock_world,
            state.hud.camera.x,
            state.hud.camera.y,
        );

        assert_eq!(state.clock_screen(), expected);
        assert_ne!(state.clock_screen(), state.world.clock_world);
    }

    #[test]
    fn fullscreen_clock_screen_ignores_stored_camera_motion() {
        let mut ui = UiState::new();
        ui.offsets.camera_x = -77;
        ui.offsets.camera_y = 19;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;

        let fullscreen = Rect::new(0, 0, 215, 57);
        let before = build_render_state(fullscreen, &ui).clock_screen();

        ui.offsets.camera_x += 1;
        ui.offsets.camera_y += 1;
        ui.camera.x = ui.offsets.camera_x;
        ui.camera.y = ui.offsets.camera_y;
        let after = build_render_state(fullscreen, &ui).clock_screen();

        assert_eq!(before, after);
    }
}
