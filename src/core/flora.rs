use crate::core::guide::{
    Guide, GuideKind, GuidePoint, GuideSet, GuideShape, GuideState, GuideStyle,
};
use crate::core::spatial::SpatialGuideIndex;
use crate::scene::coords::WorldPos;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VineLifeState {
    Dormant,
    Growing,
    Mature,
    Senescent,
    Dead,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VineThicknessClass {
    Thread,
    Stem,
    Trunk,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VineHealth {
    Healthy,
    Stressed,
    Failing,
    Dead,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VineGrowthTipState {
    Active,
    Dormant,
    Spent,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VineOrganKind {
    Leaf,
    Flower,
    Fruit,
    ParticleSource,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VineStats {
    pub age_ticks: u64,
    pub vigor: u16,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VineSegment {
    pub start: WorldPos,
    pub end: WorldPos,
    pub thickness: VineThicknessClass,
    pub age_ticks: u64,
    pub health: VineHealth,
    pub guide_id: Option<u32>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VineAxis {
    pub id: u32,
    pub guide_set_label: Option<String>,
    pub segments: Vec<VineSegment>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VineOrgan {
    pub kind: VineOrganKind,
    pub axis_id: u32,
    pub segment_index: usize,
    pub position: WorldPos,
    pub normal: WorldPos,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VineGrowthTip {
    pub axis_id: u32,
    pub position: WorldPos,
    pub age_ticks: u64,
    pub remaining_growth_steps: u16,
    pub state: VineGrowthTipState,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VineRootAttachment {
    pub world: WorldPos,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VineInstance {
    pub id: u32,
    pub species_id: String,
    pub journal_id: String,
    pub life_state: VineLifeState,
    pub stats: VineStats,
    pub root: VineRootAttachment,
    pub axes: Vec<VineAxis>,
    pub organs: Vec<VineOrgan>,
    pub growth_tips: Vec<VineGrowthTip>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VineGuidePath {
    pub guide_id: u32,
    pub points: Vec<WorldPos>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FloraState {
    pub vines: Vec<VineInstance>,
}

pub const BORDER_VINE_SEED_ID: u32 = 1;
pub const BORDER_VINE_SEED_AXIS_ID: u32 = 1;
pub const BORDER_VINE_SPECIES_ID: &str = "yam.vine.border_v1";
pub const BORDER_VINE_JOURNAL_ID: &str = "journal.vine.border_v1.seed";
pub const BORDER_VINE_ROOT: WorldPos = WorldPos { x: 104, y: -27 };
pub const BORDER_VINE_GUIDE_SET_LABEL: &str = "main-scene-vine-frame";
pub const BORDER_VINE_MAX_SEGMENTS: usize = 12;
pub const BORDER_VINE_GROWTH_INTERVAL_TICKS: u64 = 4;
pub const BORDER_VINE_TIP_GROWTH_BUDGET: u16 = 8;
pub const BORDER_VINE_LEAF_MATURITY_TICKS: u64 = 6;

#[allow(dead_code)]
pub fn border_vine_seed() -> VineInstance {
    VineInstance {
        id: BORDER_VINE_SEED_ID,
        species_id: BORDER_VINE_SPECIES_ID.to_string(),
        journal_id: BORDER_VINE_JOURNAL_ID.to_string(),
        life_state: VineLifeState::Dormant,
        stats: VineStats {
            age_ticks: 0,
            vigor: 100,
        },
        root: VineRootAttachment {
            world: BORDER_VINE_ROOT,
        },
        axes: Vec::new(),
        organs: Vec::new(),
        growth_tips: vec![VineGrowthTip {
            axis_id: BORDER_VINE_SEED_AXIS_ID,
            position: BORDER_VINE_ROOT,
            age_ticks: 0,
            remaining_growth_steps: BORDER_VINE_TIP_GROWTH_BUDGET,
            state: VineGrowthTipState::Dormant,
        }],
    }
}

#[allow(dead_code)]
pub fn derive_vine_axis_from_guide_set(
    guide_index: SpatialGuideIndex<'_>,
    set_label: &str,
) -> Vec<VineGuidePath> {
    guide_index
        .guides_in_set(set_label)
        .into_iter()
        .filter(|guide| guide.enabled)
        .filter_map(|guide| guide_shape_to_path(guide.id, guide.anchor, &guide.shape))
        .collect()
}

#[allow(dead_code)]
pub fn derive_static_main_axis(
    vine: &VineInstance,
    guide_paths: &[VineGuidePath],
    guide_set_label: &str,
) -> Option<VineAxis> {
    if guide_paths.is_empty() {
        return None;
    }

    let mut segments = Vec::new();
    let mut current_start = vine.root.world;

    for path in guide_paths {
        for point in &path.points {
            if *point == current_start {
                continue;
            }
            segments.push(VineSegment {
                start: current_start,
                end: *point,
                thickness: VineThicknessClass::Stem,
                age_ticks: 0,
                health: VineHealth::Healthy,
                guide_id: Some(path.guide_id),
            });
            current_start = *point;
        }
    }

    if segments.is_empty() {
        return None;
    }

    Some(VineAxis {
        id: BORDER_VINE_SEED_AXIS_ID,
        guide_set_label: Some(guide_set_label.to_string()),
        segments,
    })
}

#[allow(dead_code)]
pub fn main_scene_vine_guides() -> GuideState {
    let mut guides = GuideState::new();
    guides.guides.push(Guide {
        id: 10,
        label: "border-vine-left".to_string(),
        group: Some("vine-frame".to_string()),
        kind: GuideKind::Axis,
        anchor: GuidePoint {
            x: BORDER_VINE_ROOT.x,
            y: BORDER_VINE_ROOT.y,
        },
        shape: GuideShape::Line {
            end: GuidePoint { x: 92, y: -22 },
        },
        style: GuideStyle {
            glyph: ' ',
            visible: true,
            accent: false,
        },
        enabled: true,
    });
    guides.guides.push(Guide {
        id: 11,
        label: "border-vine-arc".to_string(),
        group: Some("vine-frame".to_string()),
        kind: GuideKind::Axis,
        anchor: GuidePoint { x: 92, y: -22 },
        shape: GuideShape::Polyline(vec![
            GuidePoint { x: 80, y: -18 },
            GuidePoint { x: 68, y: -13 },
            GuidePoint { x: 58, y: -7 },
            GuidePoint { x: 50, y: 0 },
        ]),
        style: GuideStyle {
            glyph: ' ',
            visible: true,
            accent: false,
        },
        enabled: true,
    });
    guides.add_set(GuideSet::new(BORDER_VINE_GUIDE_SET_LABEL, vec![10, 11]));
    guides
}

#[allow(dead_code)]
pub fn realize_border_vine_axis(flora: &mut FloraState, guide_index: SpatialGuideIndex<'_>) {
    let guide_paths = derive_vine_axis_from_guide_set(guide_index, BORDER_VINE_GUIDE_SET_LABEL);
    let Some(vine) = flora
        .vines
        .iter_mut()
        .find(|vine| vine.id == BORDER_VINE_SEED_ID)
    else {
        return;
    };
    let Some(axis) = derive_static_main_axis(vine, &guide_paths, BORDER_VINE_GUIDE_SET_LABEL)
    else {
        return;
    };

    let tip_position = axis
        .segments
        .last()
        .map(|segment| segment.end)
        .unwrap_or(vine.root.world);
    vine.life_state = VineLifeState::Growing;
    vine.axes = vec![axis];
    if let Some(tip) = vine
        .growth_tips
        .iter_mut()
        .find(|tip| tip.axis_id == BORDER_VINE_SEED_AXIS_ID)
    {
        tip.position = tip_position;
        tip.state = VineGrowthTipState::Active;
    }
}

#[allow(dead_code)]
pub fn rebuild_leaf_organs(vine: &mut VineInstance) {
    let mut organs = Vec::new();
    for axis in &vine.axes {
        for (segment_index, segment) in axis.segments.iter().enumerate() {
            if segment.age_ticks < BORDER_VINE_LEAF_MATURITY_TICKS || segment_index % 2 == 0 {
                continue;
            }
            let dx = segment.end.x - segment.start.x;
            let dy = segment.end.y - segment.start.y;
            if dx == 0 && dy == 0 {
                continue;
            }

            let normal = if dx.abs() >= dy.abs() {
                WorldPos {
                    x: 0,
                    y: if dx >= 0 { 1 } else { -1 },
                }
            } else {
                WorldPos {
                    x: if dy >= 0 { -1 } else { 1 },
                    y: 0,
                }
            };

            organs.push(VineOrgan {
                kind: VineOrganKind::Leaf,
                axis_id: axis.id,
                segment_index,
                position: WorldPos {
                    x: (segment.start.x + segment.end.x) / 2 + normal.x,
                    y: (segment.start.y + segment.end.y) / 2 + normal.y,
                },
                normal,
            });
        }
    }
    vine.organs = organs;
}

fn guide_shape_to_path(
    guide_id: u32,
    anchor: GuidePoint,
    shape: &GuideShape,
) -> Option<VineGuidePath> {
    let points = match shape {
        GuideShape::Line { end } => vec![guide_point_to_world(anchor), guide_point_to_world(*end)],
        GuideShape::Polyline(points) => {
            let mut path = Vec::with_capacity(points.len() + 1);
            path.push(guide_point_to_world(anchor));
            path.extend(points.iter().copied().map(guide_point_to_world));
            path
        }
        _ => return None,
    };

    if points.len() < 2 {
        return None;
    }

    Some(VineGuidePath { guide_id, points })
}

fn guide_point_to_world(point: GuidePoint) -> WorldPos {
    WorldPos {
        x: point.x,
        y: point.y,
    }
}

impl FloraState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { vines: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn with_border_vine_seed() -> Self {
        Self {
            vines: vec![border_vine_seed()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::guide::{Guide, GuideKind, GuideSet, GuideState, GuideStyle};

    #[test]
    fn flora_state_starts_empty() {
        let flora = FloraState::new();

        assert!(flora.vines.is_empty());
    }

    #[test]
    fn vine_instance_shape_is_small_and_inspectable() {
        let vine = VineInstance {
            id: 7,
            species_id: "yam.vine.border_v1".to_string(),
            journal_id: "journal.vine.7".to_string(),
            life_state: VineLifeState::Dormant,
            stats: VineStats {
                age_ticks: 0,
                vigor: 100,
            },
            root: VineRootAttachment {
                world: WorldPos { x: -12, y: 8 },
            },
            axes: vec![VineAxis {
                id: 1,
                guide_set_label: Some("fixture".to_string()),
                segments: vec![VineSegment {
                    start: WorldPos { x: -12, y: 8 },
                    end: WorldPos { x: -10, y: 8 },
                    thickness: VineThicknessClass::Stem,
                    age_ticks: 0,
                    health: VineHealth::Healthy,
                    guide_id: Some(3),
                }],
            }],
            organs: vec![VineOrgan {
                kind: VineOrganKind::Leaf,
                axis_id: 1,
                segment_index: 0,
                position: WorldPos { x: -11, y: 9 },
                normal: WorldPos { x: 0, y: 1 },
            }],
            growth_tips: vec![VineGrowthTip {
                axis_id: 1,
                position: WorldPos { x: -10, y: 8 },
                age_ticks: 0,
                remaining_growth_steps: 3,
                state: VineGrowthTipState::Dormant,
            }],
        };

        assert_eq!(vine.id, 7);
        assert_eq!(vine.species_id, "yam.vine.border_v1");
        assert_eq!(vine.root.world, WorldPos { x: -12, y: 8 });
        assert_eq!(vine.axes.len(), 1);
        assert_eq!(vine.axes[0].segments.len(), 1);
        assert_eq!(vine.organs.len(), 1);
        assert_eq!(vine.growth_tips[0].state, VineGrowthTipState::Dormant);
    }

    #[test]
    fn border_vine_seed_is_deterministic_and_invisible() {
        let vine = border_vine_seed();

        assert_eq!(vine.id, BORDER_VINE_SEED_ID);
        assert_eq!(vine.species_id, BORDER_VINE_SPECIES_ID);
        assert_eq!(vine.journal_id, BORDER_VINE_JOURNAL_ID);
        assert_eq!(vine.life_state, VineLifeState::Dormant);
        assert_eq!(vine.root.world, BORDER_VINE_ROOT);
        assert!(vine.axes.is_empty());
        assert!(vine.organs.is_empty());
        assert_eq!(vine.growth_tips.len(), 1);
        assert_eq!(vine.growth_tips[0].axis_id, BORDER_VINE_SEED_AXIS_ID);
        assert_eq!(vine.growth_tips[0].position, BORDER_VINE_ROOT);
        assert_eq!(vine.growth_tips[0].age_ticks, 0);
        assert_eq!(
            vine.growth_tips[0].remaining_growth_steps,
            BORDER_VINE_TIP_GROWTH_BUDGET
        );
        assert_eq!(vine.growth_tips[0].state, VineGrowthTipState::Dormant);
    }

    #[test]
    fn seeded_flora_starts_with_exactly_one_border_vine() {
        let flora = FloraState::with_border_vine_seed();

        assert_eq!(flora.vines.len(), 1);
        assert_eq!(flora.vines[0], border_vine_seed());
    }

    #[test]
    fn derive_vine_axis_from_guide_set_returns_empty_for_missing_set() {
        let guides = GuideState::new();
        let paths = derive_vine_axis_from_guide_set(SpatialGuideIndex::new(&guides), "missing");

        assert!(paths.is_empty());
    }

    #[test]
    fn derive_vine_axis_from_guide_set_keeps_enabled_line_and_polyline_guides() {
        let mut guides = GuideState::new();
        guides.guides.push(Guide {
            id: 10,
            label: "border-left".to_string(),
            group: Some("vine-frame".to_string()),
            kind: GuideKind::Axis,
            anchor: GuidePoint { x: -92, y: 23 },
            shape: GuideShape::Line {
                end: GuidePoint { x: -88, y: 20 },
            },
            style: GuideStyle {
                glyph: ' ',
                visible: true,
                accent: false,
            },
            enabled: true,
        });
        guides.guides.push(Guide {
            id: 11,
            label: "top-arc".to_string(),
            group: Some("vine-frame".to_string()),
            kind: GuideKind::Axis,
            anchor: GuidePoint { x: -88, y: 20 },
            shape: GuideShape::Polyline(vec![
                GuidePoint { x: -82, y: 19 },
                GuidePoint { x: -76, y: 18 },
            ]),
            style: GuideStyle {
                glyph: ' ',
                visible: true,
                accent: false,
            },
            enabled: true,
        });
        guides.add_set(GuideSet::new("main-scene-vine-frame", vec![10, 11]));

        let paths = derive_vine_axis_from_guide_set(
            SpatialGuideIndex::new(&guides),
            "main-scene-vine-frame",
        );

        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0].guide_id, 10);
        assert_eq!(
            paths[0].points,
            vec![WorldPos { x: -92, y: 23 }, WorldPos { x: -88, y: 20 }]
        );
        assert_eq!(paths[1].guide_id, 11);
        assert_eq!(
            paths[1].points,
            vec![
                WorldPos { x: -88, y: 20 },
                WorldPos { x: -82, y: 19 },
                WorldPos { x: -76, y: 18 }
            ]
        );
    }

    #[test]
    fn derive_vine_axis_from_guide_set_skips_disabled_and_unsupported_guides() {
        let mut guides = GuideState::new();
        guides.guides.push(Guide {
            id: 20,
            label: "disabled-line".to_string(),
            group: Some("vine-frame".to_string()),
            kind: GuideKind::Axis,
            anchor: GuidePoint { x: 0, y: 0 },
            shape: GuideShape::Line {
                end: GuidePoint { x: 4, y: 0 },
            },
            style: GuideStyle {
                glyph: ' ',
                visible: true,
                accent: false,
            },
            enabled: false,
        });
        guides.guides.push(Guide {
            id: 21,
            label: "point-only".to_string(),
            group: Some("vine-frame".to_string()),
            kind: GuideKind::Waypoint,
            anchor: GuidePoint { x: 2, y: 2 },
            shape: GuideShape::Point,
            style: GuideStyle {
                glyph: '+',
                visible: true,
                accent: false,
            },
            enabled: true,
        });
        guides.add_set(GuideSet::new("main-scene-vine-frame", vec![20, 21]));

        let paths = derive_vine_axis_from_guide_set(
            SpatialGuideIndex::new(&guides),
            "main-scene-vine-frame",
        );

        assert!(paths.is_empty());
    }

    #[test]
    fn derive_static_main_axis_starts_from_explicit_vine_root() {
        let vine = VineInstance {
            id: BORDER_VINE_SEED_ID,
            species_id: BORDER_VINE_SPECIES_ID.to_string(),
            journal_id: BORDER_VINE_JOURNAL_ID.to_string(),
            life_state: VineLifeState::Dormant,
            stats: VineStats {
                age_ticks: 0,
                vigor: 100,
            },
            root: VineRootAttachment {
                world: WorldPos { x: -100, y: 24 },
            },
            axes: Vec::new(),
            organs: Vec::new(),
            growth_tips: vec![VineGrowthTip {
                axis_id: BORDER_VINE_SEED_AXIS_ID,
                position: WorldPos { x: -100, y: 24 },
                age_ticks: 0,
                remaining_growth_steps: 4,
                state: VineGrowthTipState::Dormant,
            }],
        };
        let guide_paths = vec![
            VineGuidePath {
                guide_id: 10,
                points: vec![WorldPos { x: -96, y: 22 }, WorldPos { x: -92, y: 21 }],
            },
            VineGuidePath {
                guide_id: 11,
                points: vec![WorldPos { x: -88, y: 20 }],
            },
        ];

        let axis =
            derive_static_main_axis(&vine, &guide_paths, "main-scene-vine-frame").expect("axis");

        assert_eq!(axis.id, BORDER_VINE_SEED_AXIS_ID);
        assert_eq!(
            axis.guide_set_label.as_deref(),
            Some("main-scene-vine-frame")
        );
        assert_eq!(axis.segments.len(), 3);
        assert_eq!(axis.segments[0].start, WorldPos { x: -100, y: 24 });
        assert_eq!(axis.segments[0].end, WorldPos { x: -96, y: 22 });
        assert_eq!(axis.segments[0].guide_id, Some(10));
        assert_eq!(axis.segments[1].start, WorldPos { x: -96, y: 22 });
        assert_eq!(axis.segments[1].end, WorldPos { x: -92, y: 21 });
        assert_eq!(axis.segments[1].guide_id, Some(10));
        assert_eq!(axis.segments[2].start, WorldPos { x: -92, y: 21 });
        assert_eq!(axis.segments[2].end, WorldPos { x: -88, y: 20 });
        assert_eq!(axis.segments[2].guide_id, Some(11));
        assert!(axis
            .segments
            .iter()
            .all(|segment| segment.health == VineHealth::Healthy));
        assert!(axis
            .segments
            .iter()
            .all(|segment| segment.thickness == VineThicknessClass::Stem));
    }

    #[test]
    fn rebuild_leaf_organs_attaches_leaves_to_mature_segments_only() {
        let mut vine = VineInstance {
            id: BORDER_VINE_SEED_ID,
            species_id: BORDER_VINE_SPECIES_ID.to_string(),
            journal_id: BORDER_VINE_JOURNAL_ID.to_string(),
            life_state: VineLifeState::Growing,
            stats: VineStats {
                age_ticks: 0,
                vigor: 100,
            },
            root: VineRootAttachment {
                world: WorldPos { x: 0, y: 0 },
            },
            axes: vec![VineAxis {
                id: BORDER_VINE_SEED_AXIS_ID,
                guide_set_label: Some(BORDER_VINE_GUIDE_SET_LABEL.to_string()),
                segments: vec![
                    VineSegment {
                        start: WorldPos { x: 0, y: 0 },
                        end: WorldPos { x: 4, y: 0 },
                        thickness: VineThicknessClass::Stem,
                        age_ticks: BORDER_VINE_LEAF_MATURITY_TICKS,
                        health: VineHealth::Healthy,
                        guide_id: Some(10),
                    },
                    VineSegment {
                        start: WorldPos { x: 4, y: 0 },
                        end: WorldPos { x: 8, y: 0 },
                        thickness: VineThicknessClass::Stem,
                        age_ticks: BORDER_VINE_LEAF_MATURITY_TICKS,
                        health: VineHealth::Healthy,
                        guide_id: Some(11),
                    },
                    VineSegment {
                        start: WorldPos { x: 8, y: 0 },
                        end: WorldPos { x: 10, y: 2 },
                        thickness: VineThicknessClass::Stem,
                        age_ticks: BORDER_VINE_LEAF_MATURITY_TICKS - 1,
                        health: VineHealth::Healthy,
                        guide_id: Some(11),
                    },
                ],
            }],
            organs: Vec::new(),
            growth_tips: Vec::new(),
        };

        rebuild_leaf_organs(&mut vine);

        assert_eq!(vine.organs.len(), 1);
        assert_eq!(vine.organs[0].kind, VineOrganKind::Leaf);
        assert_eq!(vine.organs[0].axis_id, BORDER_VINE_SEED_AXIS_ID);
        assert_eq!(vine.organs[0].segment_index, 1);
        assert_eq!(vine.organs[0].position, WorldPos { x: 6, y: 1 });
        assert_eq!(vine.organs[0].normal, WorldPos { x: 0, y: 1 });
    }

    #[test]
    fn derive_static_main_axis_returns_none_for_empty_paths() {
        let vine = border_vine_seed();

        let axis = derive_static_main_axis(&vine, &[], "main-scene-vine-frame");

        assert!(axis.is_none());
    }

    #[test]
    fn realize_border_vine_axis_populates_seeded_vine_from_named_guides() {
        let guides = main_scene_vine_guides();
        let mut flora = FloraState::with_border_vine_seed();

        realize_border_vine_axis(&mut flora, SpatialGuideIndex::new(&guides));

        assert_eq!(flora.vines.len(), 1);
        assert_eq!(flora.vines[0].axes.len(), 1);
        assert_eq!(
            flora.vines[0].axes[0].guide_set_label.as_deref(),
            Some(BORDER_VINE_GUIDE_SET_LABEL)
        );
        assert!(!flora.vines[0].axes[0].segments.is_empty());
        assert_eq!(flora.vines[0].axes[0].segments[0].start, BORDER_VINE_ROOT);
        assert_eq!(
            flora.vines[0].growth_tips[0].position,
            flora.vines[0].axes[0]
                .segments
                .last()
                .expect("derived segment")
                .end
        );
    }
}
