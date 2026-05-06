use crate::core::{
    entity::Entity,
    flora::{main_scene_vine_guides, realize_border_vine_axis, FloraState},
    grid::Grid,
    guide::GuideState,
    spatial::SpatialGuideIndex,
};
use crate::scene::coords::WorldPos;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Fields {
    pub density: Vec<f32>,
    pub attraction: Vec<f32>,
    pub avoidance: Vec<f32>,
}

impl Fields {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            density: vec![0.0; size],
            attraction: vec![0.0; size],
            avoidance: vec![0.0; size],
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WorldState {
    pub kind: WorldKind,
    pub grid: Grid,
    pub entities: Vec<Entity>,
    pub fields: Fields,
    pub flora: FloraState,
    pub guides: GuideState,
    pub tick: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorldKind {
    MainScene,
    Sandbox,
}

impl WorldKind {
    pub fn title(self) -> &'static str {
        match self {
            WorldKind::MainScene => "main-scene",
            WorldKind::Sandbox => "sandbox",
        }
    }
}

#[allow(dead_code)]
impl WorldState {
    pub fn new() -> Self {
        Self::for_kind(WorldKind::MainScene)
    }

    pub fn for_kind(kind: WorldKind) -> Self {
        match kind {
            WorldKind::MainScene => Self::for_main_scene(),
            WorldKind::Sandbox => Self::for_sandbox(),
        }
    }

    pub fn for_main_scene() -> Self {
        let width = 300;
        let height = 120;
        let guides = main_scene_vine_guides();
        let mut flora = FloraState::with_border_vine_seed();
        realize_border_vine_axis(&mut flora, SpatialGuideIndex::new(&guides));
        Self {
            kind: WorldKind::MainScene,
            grid: Grid::new(width, height),
            entities: Vec::new(),
            fields: Fields::new(width, height),
            flora,
            guides,
            tick: 0,
        }
    }

    pub fn for_sandbox() -> Self {
        let width = 300;
        let height = 120;
        Self {
            kind: WorldKind::Sandbox,
            grid: Grid::new(width, height),
            entities: Vec::new(),
            fields: Fields::new(width, height),
            flora: FloraState::new(),
            guides: GuideState::new(),
            tick: 0,
        }
    }

    pub fn entity_world(&self, id: u32) -> Option<WorldPos> {
        self.entities
            .iter()
            .find(|entity| entity.id == id)
            .map(|entity| WorldPos {
                x: entity.x as i32,
                y: entity.y as i32,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{WorldKind, WorldState};
    use crate::core::flora::{
        BORDER_VINE_GUIDE_SET_LABEL, BORDER_VINE_ROOT, BORDER_VINE_SEED_AXIS_ID,
        BORDER_VINE_SEED_ID,
    };

    #[test]
    fn world_state_initializes_seeded_flora_without_affecting_other_state() {
        let world = WorldState::new();

        assert_eq!(world.kind, WorldKind::MainScene);
        assert_eq!(world.flora.vines.len(), 1);
        assert_eq!(world.flora.vines[0].id, BORDER_VINE_SEED_ID);
        assert_eq!(world.flora.vines[0].growth_tips.len(), 1);
        assert_eq!(
            world.flora.vines[0].growth_tips[0].axis_id,
            BORDER_VINE_SEED_AXIS_ID
        );
        assert_eq!(world.flora.vines[0].root.world, BORDER_VINE_ROOT);
        assert_eq!(world.flora.vines[0].axes.len(), 1);
        assert_eq!(
            world.flora.vines[0].axes[0].guide_set_label.as_deref(),
            Some(BORDER_VINE_GUIDE_SET_LABEL)
        );
        assert!(!world.flora.vines[0].axes[0].segments.is_empty());
        assert!(world.entities.is_empty());
        assert!(world
            .guides
            .guide_set(BORDER_VINE_GUIDE_SET_LABEL)
            .is_some());
        assert_eq!(world.tick, 0);
        assert_eq!(world.grid.width, 300);
        assert_eq!(world.grid.height, 120);
    }

    #[test]
    fn sandbox_world_starts_empty_and_isolated_from_main_scene_bootstrap() {
        let world = WorldState::for_sandbox();

        assert_eq!(world.kind, WorldKind::Sandbox);
        assert!(world.flora.vines.is_empty());
        assert!(world.guides.guides.is_empty());
        assert!(world.entities.is_empty());
        assert_eq!(world.tick, 0);
        assert_eq!(world.grid.width, 300);
        assert_eq!(world.grid.height, 120);
    }
}
