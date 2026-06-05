use crate::core::{
    entity::Entity,
    flora::{main_scene_vine_guides, realize_border_vine_axis, FloraState},
    grid::Grid,
    guide::GuideState,
    scaffold::ScaffoldState,
    spatial::{SpatialAnchorLookup, SpatialGuideIndex, SpatialPoint as WorldPos},
};

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
    pub scaffold: ScaffoldState,
    pub guides: GuideState,
    pub tick: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorldKind {
    Boot,
    MainScene,
    Sandbox,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorldComposition {
    EmptyBoot,
    MainScene,
    SparseSandbox,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorldGuidePlan {
    Empty,
    MainSceneVineFrame,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorldPopulationPlan {
    Empty,
    MainSceneBorderVine,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldGridSpec {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldCameraDefaults {
    pub x: i32,
    pub y: i32,
    pub follow_hero: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldDebugSurfaces {
    pub guides: bool,
    pub flora: bool,
    pub companions: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldCapabilities {
    pub scene_companions: bool,
    pub flora_runtime: bool,
    pub guide_authoring: bool,
    pub pointer_probe: bool,
    pub debug_surfaces: WorldDebugSurfaces,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldProfile {
    pub title: &'static str,
    pub loading_label: &'static str,
    pub selectable: bool,
    pub composition: WorldComposition,
    pub grid: WorldGridSpec,
    pub camera: WorldCameraDefaults,
    pub guide_plan: WorldGuidePlan,
    pub population_plan: WorldPopulationPlan,
    pub capabilities: WorldCapabilities,
}

impl WorldKind {
    pub const SELECTABLE: [Self; 2] = [Self::MainScene, Self::Sandbox];

    pub fn profile(self) -> WorldProfile {
        match self {
            WorldKind::Boot => WorldProfile {
                title: "boot",
                loading_label: "loading...",
                selectable: false,
                composition: WorldComposition::EmptyBoot,
                grid: DEFAULT_WORLD_GRID,
                camera: DEFAULT_WORLD_CAMERA,
                guide_plan: WorldGuidePlan::Empty,
                population_plan: WorldPopulationPlan::Empty,
                capabilities: WorldCapabilities::empty(),
            },
            WorldKind::MainScene => WorldProfile {
                title: "main-scene",
                loading_label: "loading main scene...",
                selectable: true,
                composition: WorldComposition::MainScene,
                grid: DEFAULT_WORLD_GRID,
                camera: DEFAULT_WORLD_CAMERA,
                guide_plan: WorldGuidePlan::MainSceneVineFrame,
                population_plan: WorldPopulationPlan::MainSceneBorderVine,
                capabilities: WorldCapabilities {
                    scene_companions: true,
                    flora_runtime: true,
                    guide_authoring: true,
                    pointer_probe: true,
                    debug_surfaces: WorldDebugSurfaces {
                        guides: true,
                        flora: true,
                        companions: true,
                    },
                },
            },
            WorldKind::Sandbox => WorldProfile {
                title: "sandbox",
                loading_label: "loading sandbox...",
                selectable: true,
                composition: WorldComposition::SparseSandbox,
                grid: DEFAULT_WORLD_GRID,
                camera: DEFAULT_WORLD_CAMERA,
                guide_plan: WorldGuidePlan::Empty,
                population_plan: WorldPopulationPlan::Empty,
                capabilities: WorldCapabilities {
                    scene_companions: false,
                    flora_runtime: false,
                    guide_authoring: true,
                    pointer_probe: true,
                    debug_surfaces: WorldDebugSurfaces {
                        guides: true,
                        flora: false,
                        companions: false,
                    },
                },
            },
        }
    }

    pub fn title(self) -> &'static str {
        self.profile().title
    }

    pub fn loading_label(self) -> &'static str {
        self.profile().loading_label
    }

    pub fn is_selectable(self) -> bool {
        self.profile().selectable
    }

    pub fn has_main_scene_composition(self) -> bool {
        self.profile().composition == WorldComposition::MainScene
    }

    pub fn has_flora_runtime(self) -> bool {
        self.profile().capabilities.flora_runtime
    }

    pub fn selectable_or_default(self) -> Self {
        if self.is_selectable() {
            self
        } else {
            Self::MainScene
        }
    }

    pub fn next_selectable(self) -> Self {
        let current = self.selectable_or_default();
        let index = Self::SELECTABLE
            .iter()
            .position(|kind| *kind == current)
            .unwrap_or(0);
        Self::SELECTABLE[(index + 1) % Self::SELECTABLE.len()]
    }
}

pub const DEFAULT_WORLD_GRID: WorldGridSpec = WorldGridSpec {
    width: 300,
    height: 120,
};

pub const DEFAULT_WORLD_CAMERA: WorldCameraDefaults = WorldCameraDefaults {
    x: -60,
    y: -15,
    follow_hero: false,
};

impl WorldCapabilities {
    pub const fn empty() -> Self {
        Self {
            scene_companions: false,
            flora_runtime: false,
            guide_authoring: false,
            pointer_probe: false,
            debug_surfaces: WorldDebugSurfaces {
                guides: false,
                flora: false,
                companions: false,
            },
        }
    }
}

#[allow(dead_code)]
impl WorldState {
    pub fn new() -> Self {
        Self::for_kind(WorldKind::MainScene)
    }

    pub fn for_kind(kind: WorldKind) -> Self {
        let profile = kind.profile();
        let guides = guides_for_plan(profile.guide_plan);
        let mut flora = flora_for_plan(profile.population_plan);
        if profile.population_plan == WorldPopulationPlan::MainSceneBorderVine {
            realize_border_vine_axis(&mut flora, SpatialGuideIndex::new(&guides));
        }

        Self {
            kind,
            grid: Grid::new(profile.grid.width, profile.grid.height),
            entities: Vec::new(),
            fields: Fields::new(profile.grid.width, profile.grid.height),
            flora,
            scaffold: scaffold_for_kind(kind),
            guides,
            tick: 0,
        }
    }

    pub fn for_boot() -> Self {
        Self::for_kind(WorldKind::Boot)
    }

    pub fn for_main_scene() -> Self {
        Self::for_kind(WorldKind::MainScene)
    }

    pub fn for_sandbox() -> Self {
        Self::for_kind(WorldKind::Sandbox)
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

impl SpatialAnchorLookup for WorldState {
    fn anchor_world(&self, id: u32) -> Option<WorldPos> {
        self.entity_world(id)
    }
}

fn guides_for_plan(plan: WorldGuidePlan) -> GuideState {
    match plan {
        WorldGuidePlan::Empty => GuideState::new(),
        WorldGuidePlan::MainSceneVineFrame => main_scene_vine_guides(),
    }
}

fn flora_for_plan(plan: WorldPopulationPlan) -> FloraState {
    match plan {
        WorldPopulationPlan::Empty => FloraState::new(),
        WorldPopulationPlan::MainSceneBorderVine => FloraState::with_border_vine_seed(),
    }
}

fn scaffold_for_kind(kind: WorldKind) -> ScaffoldState {
    if kind.has_main_scene_composition() {
        ScaffoldState::main_scene_hero_support()
    } else {
        ScaffoldState::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        WorldComposition, WorldGuidePlan, WorldKind, WorldPopulationPlan, WorldState,
        DEFAULT_WORLD_CAMERA, DEFAULT_WORLD_GRID,
    };
    use crate::core::flora::{
        BORDER_VINE_GUIDE_SET_LABEL, BORDER_VINE_ROOT, BORDER_VINE_SEED_AXIS_ID,
        BORDER_VINE_SEED_ID,
    };
    use crate::core::scaffold::ScaffoldRole;

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
        assert_eq!(world.scaffold.segments.len(), 7);
        assert!(world
            .scaffold
            .segments
            .iter()
            .any(|segment| segment.role == ScaffoldRole::SeatCradle));
        assert!(world
            .scaffold
            .segments
            .iter()
            .any(|segment| segment.role == ScaffoldRole::NestingEdge));
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
        assert!(world.scaffold.segments.is_empty());
        assert!(world.guides.guides.is_empty());
        assert!(world.entities.is_empty());
        assert_eq!(world.tick, 0);
        assert_eq!(world.grid.width, 300);
        assert_eq!(world.grid.height, 120);
    }

    #[test]
    fn boot_world_starts_empty_and_has_no_scene_assets() {
        let world = WorldState::for_boot();

        assert_eq!(world.kind, WorldKind::Boot);
        assert!(world.flora.vines.is_empty());
        assert!(world.scaffold.segments.is_empty());
        assert!(world.guides.guides.is_empty());
        assert!(world.entities.is_empty());
        assert_eq!(world.tick, 0);
        assert_eq!(world.grid.width, 300);
        assert_eq!(world.grid.height, 120);
    }

    #[test]
    fn selectable_worlds_exclude_boot_and_cycle_explicitly() {
        assert_eq!(
            WorldKind::SELECTABLE,
            [WorldKind::MainScene, WorldKind::Sandbox]
        );
        for kind in [WorldKind::Boot, WorldKind::MainScene, WorldKind::Sandbox] {
            assert_eq!(kind.is_selectable(), WorldKind::SELECTABLE.contains(&kind));
        }
        assert_eq!(
            WorldKind::Boot.selectable_or_default(),
            WorldKind::MainScene
        );
        assert_eq!(WorldKind::MainScene.next_selectable(), WorldKind::Sandbox);
        assert_eq!(WorldKind::Sandbox.next_selectable(), WorldKind::MainScene);
    }

    #[test]
    fn world_kinds_own_transition_labels() {
        assert_eq!(WorldKind::Boot.loading_label(), "loading...");
        assert_eq!(
            WorldKind::MainScene.loading_label(),
            "loading main scene..."
        );
        assert_eq!(WorldKind::Sandbox.loading_label(), "loading sandbox...");
    }

    #[test]
    fn world_profiles_describe_current_compositions() {
        assert_eq!(
            WorldKind::Boot.profile().composition,
            WorldComposition::EmptyBoot
        );
        assert_eq!(
            WorldKind::MainScene.profile().composition,
            WorldComposition::MainScene
        );
        assert_eq!(
            WorldKind::Sandbox.profile().composition,
            WorldComposition::SparseSandbox
        );
        assert!(!WorldKind::Boot.has_main_scene_composition());
        assert!(WorldKind::MainScene.has_main_scene_composition());
        assert!(!WorldKind::Sandbox.has_main_scene_composition());
    }

    #[test]
    fn world_profiles_own_population_guides_camera_and_capabilities() {
        let boot = WorldKind::Boot.profile();
        assert_eq!(boot.grid, DEFAULT_WORLD_GRID);
        assert_eq!(boot.camera, DEFAULT_WORLD_CAMERA);
        assert_eq!(boot.guide_plan, WorldGuidePlan::Empty);
        assert_eq!(boot.population_plan, WorldPopulationPlan::Empty);
        assert!(!boot.capabilities.scene_companions);
        assert!(!boot.capabilities.pointer_probe);

        let main = WorldKind::MainScene.profile();
        assert_eq!(main.grid, DEFAULT_WORLD_GRID);
        assert_eq!(main.camera, DEFAULT_WORLD_CAMERA);
        assert_eq!(main.guide_plan, WorldGuidePlan::MainSceneVineFrame);
        assert_eq!(
            main.population_plan,
            WorldPopulationPlan::MainSceneBorderVine
        );
        assert!(main.capabilities.scene_companions);
        assert!(main.capabilities.flora_runtime);
        assert!(main.capabilities.debug_surfaces.flora);
        assert!(WorldKind::MainScene.has_flora_runtime());

        let sandbox = WorldKind::Sandbox.profile();
        assert_eq!(sandbox.grid, DEFAULT_WORLD_GRID);
        assert_eq!(sandbox.camera, DEFAULT_WORLD_CAMERA);
        assert_eq!(sandbox.guide_plan, WorldGuidePlan::Empty);
        assert_eq!(sandbox.population_plan, WorldPopulationPlan::Empty);
        assert!(!sandbox.capabilities.scene_companions);
        assert!(sandbox.capabilities.guide_authoring);
        assert!(sandbox.capabilities.pointer_probe);
        assert!(!WorldKind::Sandbox.has_flora_runtime());
    }
}
