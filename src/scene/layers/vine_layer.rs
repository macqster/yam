use crate::core::flora::{VineHealth, VineOrgan, VineOrganKind, VineThicknessClass};
use crate::core::guide_line::LinePoint;
use crate::core::world::WorldState;
use crate::render::compositor::Grid;
use crate::render::drawing::{stamp_glyph, stroke_path, Brush, GlyphStamp, StrokeWeight};
use crate::render::fonts::FontRegistry;
use crate::scene::coords::{world_to_screen, WorldPos};
use crate::scene::{Layer, LayerOutput, RenderState};
use crate::theme::style as theme_style;
use crate::ui::state::UiState;

pub struct VineLayer;

#[derive(Copy, Clone)]
struct VineProjection {
    camera_x: i32,
    camera_y: i32,
    viewport_height: u16,
}

impl Layer for VineLayer {
    fn z_index(&self) -> i32 {
        20
    }

    fn render_to_grid(
        &self,
        width: u16,
        height: u16,
        world: &WorldState,
        ui: &UiState,
        _fonts: &FontRegistry,
        ctx: &RenderState,
    ) -> LayerOutput {
        let mut grid = Grid::new(width, height);
        if !ui.meta.vines_visible {
            return LayerOutput { grid, mask: None };
        }
        let projection = VineProjection {
            camera_x: ctx.hud.camera.x,
            camera_y: ctx.hud.camera.y,
            viewport_height: ctx.hud.camera.height,
        };
        for vine in &world.flora.vines {
            for axis in &vine.axes {
                for segment in &axis.segments {
                    draw_segment(
                        &mut grid,
                        segment.start,
                        segment.end,
                        segment.thickness,
                        segment.health,
                        projection,
                    );
                }
            }
            for organ in &vine.organs {
                draw_organ(&mut grid, organ, projection);
            }
        }
        LayerOutput { grid, mask: None }
    }
}

fn draw_segment(
    grid: &mut Grid,
    start: WorldPos,
    end: WorldPos,
    thickness: VineThicknessClass,
    health: VineHealth,
    projection: VineProjection,
) {
    let start = world_to_screen(
        start,
        projection.camera_x,
        projection.camera_y,
        projection.viewport_height,
    );
    let end = world_to_screen(
        end,
        projection.camera_x,
        projection.camera_y,
        projection.viewport_height,
    );
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
            style: theme_style::vine_stem(health),
            weight: match thickness {
                VineThicknessClass::Thread => StrokeWeight::Hairline,
                VineThicknessClass::Stem => StrokeWeight::Stem,
                VineThicknessClass::Trunk => StrokeWeight::Trunk,
            },
        },
    );
}

fn draw_organ(grid: &mut Grid, organ: &VineOrgan, projection: VineProjection) {
    let screen = world_to_screen(
        organ.position,
        projection.camera_x,
        projection.camera_y,
        projection.viewport_height,
    );
    stamp_glyph(
        grid,
        None,
        GlyphStamp {
            point: LinePoint {
                x: screen.x,
                y: screen.y,
            },
            glyph: match organ.kind {
                VineOrganKind::Leaf => {
                    if organ.normal.x != 0 {
                        '-'
                    } else if organ.normal.y > 0 {
                        '\''
                    } else {
                        ','
                    }
                }
                VineOrganKind::Flower => '*',
                VineOrganKind::Fruit => 'o',
                VineOrganKind::ParticleSource => '.',
            },
            style: theme_style::vine_stem(VineHealth::Healthy),
        },
    );
}

#[cfg(test)]
mod tests {
    use ratatui::style::Style;

    use crate::core::guide_line::LinePoint;
    use crate::render::compositor::Grid;
    use crate::render::drawing::{stroke_path, Brush, StrokeWeight};

    #[test]
    fn vine_stroke_path_prefers_continuous_structural_strokes() {
        let mut grid = Grid::new(5, 5);
        stroke_path(
            &mut grid,
            None,
            &[
                LinePoint { x: 0, y: 0 },
                LinePoint { x: 1, y: 1 },
                LinePoint { x: 2, y: 2 },
            ],
            Brush {
                style: Style::default(),
                weight: StrokeWeight::Stem,
            },
        );
        assert_eq!(grid.get_mut(0, 0).map(|cell| cell.symbol), Some('\\'));
        assert_eq!(grid.get_mut(1, 1).map(|cell| cell.symbol), Some('\\'));
        assert_eq!(grid.get_mut(2, 2).map(|cell| cell.symbol), Some('\\'));
    }

    #[test]
    fn vine_trunk_path_heavies_the_same_language() {
        let mut grid = Grid::new(4, 2);
        stroke_path(
            &mut grid,
            None,
            &[LinePoint { x: 0, y: 0 }, LinePoint { x: 3, y: 0 }],
            Brush {
                style: Style::default(),
                weight: StrokeWeight::Trunk,
            },
        );
        assert_eq!(grid.get_mut(0, 0).map(|cell| cell.symbol), Some('='));
        assert_eq!(grid.get_mut(1, 0).map(|cell| cell.symbol), Some('='));
    }
}
