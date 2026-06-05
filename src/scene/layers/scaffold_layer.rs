use crate::core::guide_line::LinePoint;
use crate::core::scaffold::{
    ScaffoldLayer as ScaffoldPlane, ScaffoldRole, ScaffoldSegment, ScaffoldState,
    ScaffoldThicknessClass,
};
use crate::core::spatial::{SpatialPoint, SpatialProjection, SpatialResolver, SpatialScreenPoint};
use crate::core::world::WorldState;
use crate::render::compositor::{merge_grid, Grid};
use crate::render::drawing::{stamp_glyph, stroke_path, Brush, GlyphStamp, StrokeWeight};
use crate::render::fonts::FontRegistry;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;

pub struct ScaffoldRearLayer;
pub struct ScaffoldForegroundLayer;

#[derive(Copy, Clone)]
struct ScaffoldProjection {
    resolver: SpatialResolver,
}

impl ScaffoldProjection {
    fn new(camera_x: i32, camera_y: i32, viewport_height: u16) -> Self {
        Self {
            resolver: SpatialResolver::new(SpatialProjection::new(
                camera_x,
                camera_y,
                0,
                viewport_height,
            )),
        }
    }

    fn project(self, point: SpatialPoint) -> SpatialScreenPoint {
        self.resolver.world_to_screen_point(point)
    }
}

impl Layer for ScaffoldRearLayer {
    fn z_index(&self) -> i32 {
        8
    }

    fn render_into_grid(
        &self,
        grid: &mut Grid,
        world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> Option<crate::render::mask::Mask> {
        if !ui.scaffold_visible_in_active_world() || !ui.supports_scaffold_prototypes() {
            return None;
        }

        let projection =
            ScaffoldProjection::new(ctx.hud.camera.x, ctx.hud.camera.y, ctx.hud.camera.height);
        let mut world_grid = Grid::new(grid.width, ctx.hud.viewport_rect.height);
        draw_scaffold_layer(
            &mut world_grid,
            &world.scaffold,
            projection,
            ScaffoldPlane::Rear,
        );
        merge_grid(grid, &world_grid, None);
        None
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let mask = self.render_into_grid(&mut grid, world, ui, fonts, ctx);
        LayerOutput { grid, mask }
    }
}

impl Layer for ScaffoldForegroundLayer {
    fn z_index(&self) -> i32 {
        15
    }

    fn render_into_grid(
        &self,
        grid: &mut Grid,
        world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> Option<crate::render::mask::Mask> {
        if !ui.scaffold_visible_in_active_world() || !ui.supports_scaffold_prototypes() {
            return None;
        }

        let projection =
            ScaffoldProjection::new(ctx.hud.camera.x, ctx.hud.camera.y, ctx.hud.camera.height);
        let mut world_grid = Grid::new(grid.width, ctx.hud.viewport_rect.height);
        draw_scaffold_layer(
            &mut world_grid,
            &world.scaffold,
            projection,
            ScaffoldPlane::Foreground,
        );
        merge_grid(grid, &world_grid, None);
        None
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        world: &WorldState,
        ui: &UiState,
        fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        let mask = self.render_into_grid(&mut grid, world, ui, fonts, ctx);
        LayerOutput { grid, mask }
    }
}

fn draw_scaffold_layer(
    grid: &mut Grid,
    scaffold: &ScaffoldState,
    projection: ScaffoldProjection,
    layer: ScaffoldPlane,
) {
    for segment in &scaffold.segments {
        if segment.layer != layer {
            continue;
        }
        draw_segment(grid, segment, projection);
        if layer == ScaffoldPlane::Rear && segment.role == ScaffoldRole::ForkMass {
            draw_fork_knot(grid, *segment, projection);
        }
    }
}

fn draw_segment(grid: &mut Grid, segment: &ScaffoldSegment, projection: ScaffoldProjection) {
    let start = projection.project(segment.start);
    let end = projection.project(segment.end);
    stroke_path(
        grid,
        None,
        &[
            LinePoint {
                x: start.x,
                y: start.y,
            },
            LinePoint { x: end.x, y: end.y },
        ],
        Brush {
            style: theme_style::scaffold_bark(segment.role),
            weight: match segment.thickness {
                ScaffoldThicknessClass::Brace => StrokeWeight::Stem,
                ScaffoldThicknessClass::Trunk => StrokeWeight::Trunk,
            },
        },
    );
}

fn draw_fork_knot(grid: &mut Grid, segment: ScaffoldSegment, projection: ScaffoldProjection) {
    let point = projection.project(segment.end);
    stamp_glyph(
        grid,
        None,
        GlyphStamp {
            point: LinePoint {
                x: point.x,
                y: point.y,
            },
            glyph: '@',
            style: theme_style::scaffold_bark(ScaffoldRole::ForkMass),
        },
    );
}

#[cfg(test)]
mod tests {
    use crate::core::scaffold::{
        ScaffoldLayer as ScaffoldPlane, ScaffoldRole, ScaffoldSegment, ScaffoldState,
        ScaffoldThicknessClass,
    };
    use crate::core::spatial::SpatialPoint as WorldPos;
    use crate::core::world::{WorldKind, WorldState};
    use crate::render::fonts::FontRegistry;
    use crate::render::render_state::{HudFrame, RenderState, WorldFrame};
    use crate::scene::camera::Camera;
    use crate::scene::viewport::Viewport;
    use crate::scene::Layer;
    use crate::ui::state::UiState;
    use ratatui::prelude::Rect;

    #[test]
    fn scaffold_respects_sandbox_visibility_toggle() {
        let layer = super::ScaffoldRearLayer;
        let mut world = WorldState::for_kind(WorldKind::Sandbox);
        world.scaffold = ScaffoldState {
            segments: vec![ScaffoldSegment {
                start: WorldPos { x: 0, y: 0 },
                end: WorldPos { x: 4, y: 0 },
                thickness: ScaffoldThicknessClass::Trunk,
                role: ScaffoldRole::SeatCradle,
                layer: ScaffoldPlane::Rear,
            }],
        };
        let mut ui = UiState::new();
        ui.meta.active_world = crate::ui::state::WorldKindSnapshot::Sandbox;
        ui.meta.sandbox_scaffold_visible = false;
        let fonts = FontRegistry::new();
        let ctx = render_state();

        let grid = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid;
        assert!(grid.cells.iter().all(|cell| cell.symbol == ' '));

        ui.meta.sandbox_scaffold_visible = true;
        let grid = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid;
        assert!(grid
            .cells
            .iter()
            .any(|cell| matches!(cell.symbol, '#' | '=' | '@')));
    }

    #[test]
    fn rear_scaffold_draws_support_inside_the_world_stack() {
        let layer = super::ScaffoldRearLayer;
        let world = WorldState::new();
        let ui = UiState::new();
        let fonts = FontRegistry::new();
        let ctx = render_state();

        let grid = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid;
        assert!(grid
            .cells
            .iter()
            .any(|cell| matches!(cell.symbol, '#' | '=' | '@')));
    }

    #[test]
    fn foreground_scaffold_draws_nesting_edge_above_hero() {
        let layer = super::ScaffoldForegroundLayer;
        let world = WorldState::new();
        let ui = UiState::new();
        let fonts = FontRegistry::new();
        let ctx = render_state();

        let grid = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid;
        assert!(grid
            .cells
            .iter()
            .any(|cell| matches!(cell.symbol, '-' | '/' | '\\')));
    }

    fn render_state() -> RenderState {
        RenderState {
            world: WorldFrame {
                hero_world: WorldPos { x: 150, y: 60 },
                hero_visual_anchor: WorldPos { x: -59, y: 20 },
                clock_world: WorldPos { x: 33, y: 12 },
                weather_world: WorldPos { x: 30, y: 1 },
                date_world: WorldPos { x: 32, y: 12 },
                calendar_world: WorldPos { x: 61, y: 12 },
            },
            hud: HudFrame {
                viewport: Viewport {
                    x: -61,
                    y: -15,
                    width: 124,
                    height: 32,
                },
                viewport_rect: Rect::new(0, 0, 124, 32),
                camera: Camera {
                    x: -61,
                    y: -15,
                    width: 124,
                    height: 32,
                    follow_hero: false,
                },
            },
        }
    }
}
