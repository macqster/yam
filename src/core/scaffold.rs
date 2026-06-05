use crate::core::spatial::SpatialPoint as WorldPos;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaffoldLayer {
    Rear,
    Foreground,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaffoldThicknessClass {
    Brace,
    Trunk,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaffoldRole {
    SeatCradle,
    BackBrace,
    LegBrace,
    ForkMass,
    NestingEdge,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScaffoldSegment {
    pub start: WorldPos,
    pub end: WorldPos,
    pub thickness: ScaffoldThicknessClass,
    pub role: ScaffoldRole,
    pub layer: ScaffoldLayer,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScaffoldState {
    pub segments: Vec<ScaffoldSegment>,
}

impl ScaffoldState {
    pub fn main_scene_hero_support() -> Self {
        Self {
            segments: vec![
                ScaffoldSegment {
                    start: WorldPos { x: -20, y: -30 },
                    end: WorldPos { x: -20, y: -24 },
                    thickness: ScaffoldThicknessClass::Trunk,
                    role: ScaffoldRole::SeatCradle,
                    layer: ScaffoldLayer::Rear,
                },
                ScaffoldSegment {
                    start: WorldPos { x: -20, y: -24 },
                    end: WorldPos { x: 20, y: -24 },
                    thickness: ScaffoldThicknessClass::Trunk,
                    role: ScaffoldRole::SeatCradle,
                    layer: ScaffoldLayer::Rear,
                },
                ScaffoldSegment {
                    start: WorldPos { x: 20, y: -24 },
                    end: WorldPos { x: 50, y: -10 },
                    thickness: ScaffoldThicknessClass::Trunk,
                    role: ScaffoldRole::SeatCradle,
                    layer: ScaffoldLayer::Rear,
                },
                ScaffoldSegment {
                    start: WorldPos { x: -20, y: -24 },
                    end: WorldPos { x: -67, y: 30 },
                    thickness: ScaffoldThicknessClass::Trunk,
                    role: ScaffoldRole::BackBrace,
                    layer: ScaffoldLayer::Rear,
                },
                ScaffoldSegment {
                    start: WorldPos { x: 50, y: -10 },
                    end: WorldPos { x: 25, y: 25 },
                    thickness: ScaffoldThicknessClass::Brace,
                    role: ScaffoldRole::LegBrace,
                    layer: ScaffoldLayer::Rear,
                },
                ScaffoldSegment {
                    start: WorldPos { x: -27, y: -27 },
                    end: WorldPos { x: -5, y: -20 },
                    thickness: ScaffoldThicknessClass::Trunk,
                    role: ScaffoldRole::ForkMass,
                    layer: ScaffoldLayer::Rear,
                },
                ScaffoldSegment {
                    start: WorldPos { x: 18, y: -23 },
                    end: WorldPos { x: 47, y: -12 },
                    thickness: ScaffoldThicknessClass::Brace,
                    role: ScaffoldRole::NestingEdge,
                    layer: ScaffoldLayer::Foreground,
                },
                ScaffoldSegment {
                    start: WorldPos { x: 28, y: -22 },
                    end: WorldPos { x: 41, y: -16 },
                    thickness: ScaffoldThicknessClass::Brace,
                    role: ScaffoldRole::NestingEdge,
                    layer: ScaffoldLayer::Foreground,
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ScaffoldLayer, ScaffoldRole, ScaffoldState, ScaffoldThicknessClass};

    #[test]
    fn main_scene_scaffold_keeps_rear_support_and_small_foreground_edge() {
        let scaffold = ScaffoldState::main_scene_hero_support();

        assert_eq!(scaffold.segments.len(), 8);
        assert!(scaffold
            .segments
            .iter()
            .any(|segment| segment.layer == ScaffoldLayer::Rear));
        assert!(scaffold
            .segments
            .iter()
            .any(|segment| segment.layer == ScaffoldLayer::Foreground
                && segment.role == ScaffoldRole::NestingEdge));
        assert!(scaffold
            .segments
            .iter()
            .any(|segment| segment.role == ScaffoldRole::SeatCradle
                && segment.thickness == ScaffoldThicknessClass::Trunk));
        assert!(scaffold
            .segments
            .iter()
            .any(|segment| segment.role == ScaffoldRole::BackBrace));
        assert!(scaffold
            .segments
            .iter()
            .any(|segment| segment.role == ScaffoldRole::LegBrace));
        assert!(scaffold.segments.iter().any(|segment| {
            segment.role == ScaffoldRole::SeatCradle
                && segment.start == crate::core::spatial::SpatialPoint { x: -20, y: -30 }
                && segment.end == crate::core::spatial::SpatialPoint { x: -20, y: -24 }
        }));
        assert!(scaffold.segments.iter().any(|segment| {
            segment.role == ScaffoldRole::BackBrace
                && segment.start == crate::core::spatial::SpatialPoint { x: -20, y: -24 }
                && segment.end == crate::core::spatial::SpatialPoint { x: -67, y: 30 }
        }));
        assert!(scaffold.segments.iter().any(|segment| {
            segment.role == ScaffoldRole::LegBrace
                && segment.start == crate::core::spatial::SpatialPoint { x: 50, y: -10 }
                && segment.end == crate::core::spatial::SpatialPoint { x: 25, y: 25 }
        }));
    }
}
