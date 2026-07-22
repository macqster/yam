use crate::core::greenhouse::GreenhouseBounds;
use crate::core::guide_line::LinePoint;
use crate::core::spatial::{SpatialPoint, SpatialProjection, SpatialResolver, SpatialScreenPoint};
use crate::core::world::{WorldKind, WorldState};
use crate::render::compositor::{merge_grid, Grid};
use crate::render::drawing::{stamp_glyph, stroke_path, Brush, GlyphStamp, StrokeWeight};
use crate::render::fonts::FontRegistry;
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::{glyphs, style as theme_style};
use crate::ui::state::UiState;

/// Minimal, read-only visualization of the active greenhouse room: a bounds
/// outline plus a marker at each fixture anchor. No labels, no per-fixture
/// glyphs, no interaction — deliberately smaller than a finished room
/// sketch, matching `docs/greenhouse-roadmap.md`'s "no screenshot or golden
/// art lock should lead the design" constraint. Only ever draws anything
/// when `world.kind == WorldKind::Greenhouse`.
pub struct GreenhouseLayer;

#[derive(Copy, Clone)]
struct GreenhouseProjection {
    resolver: SpatialResolver,
}

impl GreenhouseProjection {
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

impl Layer for GreenhouseLayer {
    fn z_index(&self) -> i32 {
        10
    }

    fn render_into_grid(
        &self,
        grid: &mut Grid,
        world: &WorldState,
        _ui: &UiState,
        _fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> Option<crate::render::mask::Mask> {
        if world.kind != WorldKind::Greenhouse {
            return None;
        }
        let greenhouse = world.greenhouse.as_ref()?;
        let room = greenhouse.active_room()?;

        let projection =
            GreenhouseProjection::new(ctx.hud.camera.x, ctx.hud.camera.y, ctx.hud.camera.height);
        let mut world_grid = Grid::new(grid.width, ctx.hud.viewport_rect.height);
        draw_room_bounds(&mut world_grid, room.bounds, projection);
        for fixture in &room.fixtures {
            draw_fixture_marker(&mut world_grid, fixture.anchor, projection);
        }
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

fn draw_room_bounds(grid: &mut Grid, bounds: GreenhouseBounds, projection: GreenhouseProjection) {
    let left = bounds.origin.x;
    let bottom = bounds.origin.y;
    let right = bounds.origin.x + bounds.width as i32;
    let top = bounds.origin.y + bounds.height as i32;

    let corners = [
        SpatialPoint::new(left, bottom),
        SpatialPoint::new(right, bottom),
        SpatialPoint::new(right, top),
        SpatialPoint::new(left, top),
        SpatialPoint::new(left, bottom),
    ];
    let path: Vec<LinePoint> = corners
        .iter()
        .map(|point| {
            let screen = projection.project(*point);
            LinePoint {
                x: screen.x,
                y: screen.y,
            }
        })
        .collect();

    stroke_path(
        grid,
        None,
        &path,
        Brush {
            style: theme_style::accent_border(),
            weight: StrokeWeight::Hairline,
        },
    );
}

fn draw_fixture_marker(grid: &mut Grid, anchor: SpatialPoint, projection: GreenhouseProjection) {
    let screen = projection.project(anchor);
    stamp_glyph(
        grid,
        None,
        GlyphStamp {
            point: LinePoint {
                x: screen.x,
                y: screen.y,
            },
            glyph: glyphs::GREENHOUSE_FIXTURE_MARKER,
            style: theme_style::greenhouse_fixture_marker(),
        },
    );
}

#[cfg(test)]
mod tests {
    use crate::core::spatial::SpatialPoint as WorldPos;
    use crate::core::world::{WorldKind, WorldState};
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

    #[test]
    fn greenhouse_layer_draws_bounds_and_fixture_markers_for_the_greenhouse_world() {
        let layer = super::GreenhouseLayer;
        let world = WorldState::for_greenhouse();
        let ui = UiState::new();
        let fonts = FontRegistry::new();
        let ctx = render_state();

        let grid = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid;

        assert!(grid
            .cells
            .iter()
            .any(|cell| matches!(cell.symbol, '+' | '-' | '|')));
        assert!(grid
            .cells
            .iter()
            .any(|cell| cell.symbol == crate::theme::glyphs::GREENHOUSE_FIXTURE_MARKER));
    }

    #[test]
    fn greenhouse_layer_draws_nothing_outside_the_greenhouse_world() {
        let layer = super::GreenhouseLayer;
        let world = WorldState::new();
        assert_eq!(world.kind, WorldKind::MainScene);
        let ui = UiState::new();
        let fonts = FontRegistry::new();
        let ctx = render_state();

        let grid = layer
            .render_to_grid(124, 32, &world, &ui, &fonts, &ctx)
            .grid;

        assert!(grid.cells.iter().all(|cell| cell.symbol == ' '));
    }
}
