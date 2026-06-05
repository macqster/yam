use crate::core::spatial::SpatialPoint as WorldPos;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaffoldLayer {
    Rear,
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
                    start: WorldPos { x: -20, y: -25 },
                    end: WorldPos { x: -34, y: -2 },
                    thickness: ScaffoldThicknessClass::Trunk,
                    role: ScaffoldRole::SeatCradle,
                    layer: ScaffoldLayer::Rear,
                },
                ScaffoldSegment {
                    start: WorldPos { x: -34, y: -2 },
                    end: WorldPos { x: -55, y: 12 },
                    thickness: ScaffoldThicknessClass::Trunk,
                    role: ScaffoldRole::SeatCradle,
                    layer: ScaffoldLayer::Rear,
                },
                ScaffoldSegment {
                    start: WorldPos { x: -55, y: 12 },
                    end: WorldPos { x: -67, y: 30 },
                    thickness: ScaffoldThicknessClass::Trunk,
                    role: ScaffoldRole::BackBrace,
                    layer: ScaffoldLayer::Rear,
                },
                ScaffoldSegment {
                    start: WorldPos { x: -50, y: 11 },
                    end: WorldPos { x: -28, y: 19 },
                    thickness: ScaffoldThicknessClass::Brace,
                    role: ScaffoldRole::LegBrace,
                    layer: ScaffoldLayer::Rear,
                },
                ScaffoldSegment {
                    start: WorldPos { x: -38, y: 3 },
                    end: WorldPos { x: -54, y: 20 },
                    thickness: ScaffoldThicknessClass::Trunk,
                    role: ScaffoldRole::ForkMass,
                    layer: ScaffoldLayer::Rear,
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ScaffoldLayer, ScaffoldRole, ScaffoldState, ScaffoldThicknessClass};

    #[test]
    fn main_scene_scaffold_stays_rear_support_first() {
        let scaffold = ScaffoldState::main_scene_hero_support();

        assert_eq!(scaffold.segments.len(), 5);
        assert!(scaffold
            .segments
            .iter()
            .all(|segment| segment.layer == ScaffoldLayer::Rear));
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
    }
}
