#![allow(dead_code)]

use crate::core::organism::OrganismId;
use crate::core::spatial::SpatialPoint;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenhouseRoomId(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessPathId(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GreenhouseZoneId(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FixtureId(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlantingSiteId(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnvironmentProfileId(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InspectionRefId(String);

macro_rules! impl_string_id {
    ($name:ident) => {
        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                let value = value.into();
                assert!(
                    !value.trim().is_empty(),
                    concat!(stringify!($name), " must not be empty")
                );
                Self(value)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }
    };
}

impl_string_id!(GreenhouseRoomId);
impl_string_id!(AccessPathId);
impl_string_id!(GreenhouseZoneId);
impl_string_id!(FixtureId);
impl_string_id!(PlantingSiteId);
impl_string_id!(EnvironmentProfileId);
impl_string_id!(InspectionRefId);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GreenhouseCapabilityFlags {
    pub inspection_read_only: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GreenhouseBounds {
    pub origin: SpatialPoint,
    pub width: u16,
    pub height: u16,
}

impl GreenhouseBounds {
    pub const fn new(origin: SpatialPoint, width: u16, height: u16) -> Self {
        Self {
            origin,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GreenhouseRoomRole {
    Nursery,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessPathKind {
    Walkway,
    InspectionLane,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GreenhouseZoneRole {
    PropagationBench,
    LampZone,
    InspectionMarker,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixtureKind {
    GlassFrame,
    PropagationBench,
    Tray,
    JarSlot,
    InspectionMarker,
    TrainingFrame,
    SpecimenShelf,
    SubstrateStrip,
    LampZone,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlantingSiteKind {
    Tray,
    JarSlot,
    ShelfSlot,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolicLight {
    Gentle,
    BrightIndirect,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolicHumidity {
    Balanced,
    Humid,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolicTemperature {
    Cool,
    Temperate,
    Warm,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolicWater {
    Light,
    Moderate,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolicAirflow {
    Still,
    SoftCirculation,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolicSubstrate {
    SeedStartingMix,
    WaterPropagation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnvironmentProfile {
    pub id: EnvironmentProfileId,
    pub light: SymbolicLight,
    pub humidity: SymbolicHumidity,
    pub temperature: SymbolicTemperature,
    pub water: SymbolicWater,
    pub airflow: SymbolicAirflow,
    pub substrate: SymbolicSubstrate,
    pub outside_weather_influence: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessPath {
    pub id: AccessPathId,
    pub kind: AccessPathKind,
    pub label: String,
    pub bounds: GreenhouseBounds,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GreenhouseZone {
    pub id: GreenhouseZoneId,
    pub role: GreenhouseZoneRole,
    pub label: String,
    pub bounds: GreenhouseBounds,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fixture {
    pub id: FixtureId,
    pub kind: FixtureKind,
    pub label: String,
    pub anchor: SpatialPoint,
    pub bounds: GreenhouseBounds,
    pub inspection_hint: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlantingSite {
    pub id: PlantingSiteId,
    pub kind: PlantingSiteKind,
    pub label: String,
    pub anchor: SpatialPoint,
    pub capacity: u8,
    pub fixture_id: Option<FixtureId>,
    /// A soft reference to the organism occupying this site, if any. This is
    /// a pointer only: the planting site does not own organism/plant state
    /// (that lives in `FloraState`, per the Functional-Space Contract's
    /// Minimum Data Owners table), so nothing here is invalidated if the
    /// referenced organism is ever removed elsewhere.
    pub occupant: Option<OrganismId>,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InspectionTarget {
    Room,
    Zone,
    Fixture,
    PlantingSite,
    Organism,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InspectionRef {
    pub id: InspectionRefId,
    pub target: InspectionTarget,
    pub target_id: String,
    pub label: String,
    pub short_text: String,
    pub read_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GreenhouseRoom {
    pub id: GreenhouseRoomId,
    pub display_name: String,
    pub role: GreenhouseRoomRole,
    pub bounds: GreenhouseBounds,
    pub access_paths: Vec<AccessPath>,
    pub zones: Vec<GreenhouseZone>,
    pub fixtures: Vec<Fixture>,
    pub planting_sites: Vec<PlantingSite>,
    pub environment: EnvironmentProfile,
    pub inspection_refs: Vec<InspectionRef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GreenhouseState {
    pub rooms: Vec<GreenhouseRoom>,
    pub active_room_id: GreenhouseRoomId,
    pub capabilities: GreenhouseCapabilityFlags,
}

pub const FIRST_GREENHOUSE_ROOM_ID: &str = "greenhouse_nursery";

impl GreenhouseState {
    pub fn new(
        rooms: Vec<GreenhouseRoom>,
        active_room_id: GreenhouseRoomId,
        capabilities: GreenhouseCapabilityFlags,
    ) -> Self {
        assert!(
            !rooms.is_empty(),
            "greenhouse state must contain at least one room"
        );
        assert!(
            rooms.iter().any(|room| room.id == active_room_id),
            "active greenhouse room must exist"
        );

        for room in &rooms {
            room.assert_invariants();
        }

        for (index, room) in rooms.iter().enumerate() {
            assert!(
                rooms[index + 1..].iter().all(|other| other.id != room.id),
                "greenhouse room ids must be unique"
            );
        }

        Self {
            rooms,
            active_room_id,
            capabilities,
        }
    }

    pub fn nursery() -> Self {
        let room = GreenhouseRoom::nursery();
        let active_room_id = room.id.clone();
        Self::new(
            vec![room],
            active_room_id,
            GreenhouseCapabilityFlags {
                inspection_read_only: true,
            },
        )
    }

    /// Returns the currently active room, or `None` if `active_room_id` does
    /// not resolve against `rooms`. `GreenhouseState::new` enforces this
    /// invariant at construction time, but both fields are public and can be
    /// mutated independently afterward, so this stays fallible rather than
    /// panicking (matching the no-panic-in-production pattern used elsewhere,
    /// e.g. `render/chafa.rs`'s placeholder-frame fallback).
    pub fn active_room(&self) -> Option<&GreenhouseRoom> {
        self.rooms
            .iter()
            .find(|room| room.id == self.active_room_id)
    }
}

impl GreenhouseRoom {
    pub fn nursery() -> Self {
        let bounds = GreenhouseBounds::new(SpatialPoint::new(-24, -8), 48, 16);
        let propagation_bench_bounds = GreenhouseBounds::new(SpatialPoint::new(-16, -2), 18, 4);
        let shelf_bounds = GreenhouseBounds::new(SpatialPoint::new(8, 0), 8, 3);

        Self {
            id: GreenhouseRoomId::new(FIRST_GREENHOUSE_ROOM_ID),
            display_name: "Greenhouse Nursery".to_string(),
            role: GreenhouseRoomRole::Nursery,
            bounds,
            access_paths: vec![
                AccessPath {
                    id: AccessPathId::new("center_walkway"),
                    kind: AccessPathKind::Walkway,
                    label: "Center Walkway".to_string(),
                    bounds: GreenhouseBounds::new(SpatialPoint::new(-3, -8), 6, 16),
                },
                AccessPath {
                    id: AccessPathId::new("inspection_lane"),
                    kind: AccessPathKind::InspectionLane,
                    label: "Inspection Lane".to_string(),
                    bounds: GreenhouseBounds::new(SpatialPoint::new(6, -3), 12, 6),
                },
            ],
            zones: vec![
                GreenhouseZone {
                    id: GreenhouseZoneId::new("propagation_bench"),
                    role: GreenhouseZoneRole::PropagationBench,
                    label: "Propagation Bench".to_string(),
                    bounds: propagation_bench_bounds,
                },
                GreenhouseZone {
                    id: GreenhouseZoneId::new("lamp_zone"),
                    role: GreenhouseZoneRole::LampZone,
                    label: "Lamp Zone".to_string(),
                    bounds: GreenhouseBounds::new(SpatialPoint::new(4, 2), 14, 4),
                },
                GreenhouseZone {
                    id: GreenhouseZoneId::new("inspection_marker"),
                    role: GreenhouseZoneRole::InspectionMarker,
                    label: "Inspection Marker".to_string(),
                    bounds: GreenhouseBounds::new(SpatialPoint::new(10, -4), 4, 2),
                },
            ],
            fixtures: vec![
                Fixture {
                    id: FixtureId::new("glass_frame"),
                    kind: FixtureKind::GlassFrame,
                    label: "Glass Frame".to_string(),
                    anchor: SpatialPoint::new(0, 0),
                    bounds,
                    inspection_hint: "Controlled nursery shell with clear air around supports."
                        .to_string(),
                },
                Fixture {
                    id: FixtureId::new("propagation_bench"),
                    kind: FixtureKind::PropagationBench,
                    label: "Propagation Bench".to_string(),
                    anchor: SpatialPoint::new(-7, 0),
                    bounds: propagation_bench_bounds,
                    inspection_hint: "Primary staging support for the first propagation sites."
                        .to_string(),
                },
                Fixture {
                    id: FixtureId::new("left_tray"),
                    kind: FixtureKind::Tray,
                    label: "Left Tray".to_string(),
                    anchor: SpatialPoint::new(-12, 1),
                    bounds: GreenhouseBounds::new(SpatialPoint::new(-14, 0), 5, 2),
                    inspection_hint: "Shallow tray for rooted cuttings or seedlings.".to_string(),
                },
                Fixture {
                    id: FixtureId::new("cutting_jar_slot"),
                    kind: FixtureKind::JarSlot,
                    label: "Cutting Jar Slot".to_string(),
                    anchor: SpatialPoint::new(-5, 1),
                    bounds: GreenhouseBounds::new(SpatialPoint::new(-6, 0), 3, 2),
                    inspection_hint: "Reserved slot for water propagation jars.".to_string(),
                },
                Fixture {
                    id: FixtureId::new("inspection_marker"),
                    kind: FixtureKind::InspectionMarker,
                    label: "Inspection Marker".to_string(),
                    anchor: SpatialPoint::new(12, -3),
                    bounds: GreenhouseBounds::new(SpatialPoint::new(11, -4), 2, 2),
                    inspection_hint: "Read-only inspection station for closer review.".to_string(),
                },
                Fixture {
                    id: FixtureId::new("training_frame"),
                    kind: FixtureKind::TrainingFrame,
                    label: "Training Frame".to_string(),
                    anchor: SpatialPoint::new(4, 3),
                    bounds: GreenhouseBounds::new(SpatialPoint::new(3, 2), 6, 4),
                    inspection_hint: "Reserved support for later guided growth studies."
                        .to_string(),
                },
                Fixture {
                    id: FixtureId::new("specimen_shelf"),
                    kind: FixtureKind::SpecimenShelf,
                    label: "Specimen Shelf".to_string(),
                    anchor: SpatialPoint::new(12, 1),
                    bounds: shelf_bounds,
                    inspection_hint: "Sparse holding surface for tagged nursery specimens."
                        .to_string(),
                },
                Fixture {
                    id: FixtureId::new("substrate_strip"),
                    kind: FixtureKind::SubstrateStrip,
                    label: "Substrate Strip".to_string(),
                    anchor: SpatialPoint::new(-18, -5),
                    bounds: GreenhouseBounds::new(SpatialPoint::new(-19, -6), 7, 2),
                    inspection_hint: "Narrow substrate staging area for nursery prep.".to_string(),
                },
                Fixture {
                    id: FixtureId::new("lamp_zone"),
                    kind: FixtureKind::LampZone,
                    label: "Lamp Zone".to_string(),
                    anchor: SpatialPoint::new(10, 4),
                    bounds: GreenhouseBounds::new(SpatialPoint::new(6, 3), 10, 3),
                    inspection_hint: "Symbolic bright-indirect lighting zone.".to_string(),
                },
            ],
            planting_sites: vec![
                PlantingSite {
                    id: PlantingSiteId::new("left_tray"),
                    kind: PlantingSiteKind::Tray,
                    label: "Left Tray".to_string(),
                    anchor: SpatialPoint::new(-12, 1),
                    capacity: 1,
                    fixture_id: Some(FixtureId::new("left_tray")),
                    // The first greenhouse organism: a soft reference only,
                    // not ownership -- see `PlantingSite::occupant`'s doc
                    // comment. The actual seedling data lives in
                    // `FloraState`, populated via `WorldPopulationPlan`.
                    occupant: Some(crate::core::flora::FIRST_GREENHOUSE_SEEDLING_ORGANISM_ID),
                },
                PlantingSite {
                    id: PlantingSiteId::new("cutting_jar_slot"),
                    kind: PlantingSiteKind::JarSlot,
                    label: "Cutting Jar Slot".to_string(),
                    anchor: SpatialPoint::new(-5, 1),
                    capacity: 1,
                    fixture_id: Some(FixtureId::new("cutting_jar_slot")),
                    occupant: None,
                },
                PlantingSite {
                    id: PlantingSiteId::new("specimen_shelf_slot"),
                    kind: PlantingSiteKind::ShelfSlot,
                    label: "Specimen Shelf Slot".to_string(),
                    anchor: SpatialPoint::new(12, 1),
                    capacity: 1,
                    fixture_id: Some(FixtureId::new("specimen_shelf")),
                    occupant: None,
                },
            ],
            environment: EnvironmentProfile {
                id: EnvironmentProfileId::new("nursery_symbolic_profile"),
                light: SymbolicLight::BrightIndirect,
                humidity: SymbolicHumidity::Humid,
                temperature: SymbolicTemperature::Temperate,
                water: SymbolicWater::Moderate,
                airflow: SymbolicAirflow::SoftCirculation,
                substrate: SymbolicSubstrate::SeedStartingMix,
                outside_weather_influence: false,
            },
            inspection_refs: vec![
                InspectionRef {
                    id: InspectionRefId::new("room_overview"),
                    target: InspectionTarget::Room,
                    target_id: FIRST_GREENHOUSE_ROOM_ID.to_string(),
                    label: "Room Overview".to_string(),
                    short_text: "Sparse nursery room with symbolic environment and tiny capacity."
                        .to_string(),
                    read_only: true,
                },
                InspectionRef {
                    id: InspectionRefId::new("bench_overview"),
                    target: InspectionTarget::Zone,
                    target_id: "propagation_bench".to_string(),
                    label: "Propagation Bench".to_string(),
                    short_text: "Primary preparation zone for the first planting sites."
                        .to_string(),
                    read_only: true,
                },
                InspectionRef {
                    id: InspectionRefId::new("marker_reference"),
                    target: InspectionTarget::Fixture,
                    target_id: "inspection_marker".to_string(),
                    label: "Inspection Marker".to_string(),
                    short_text: "Read-only closer-look reference point.".to_string(),
                    read_only: true,
                },
                InspectionRef {
                    id: InspectionRefId::new("left_tray_reference"),
                    target: InspectionTarget::PlantingSite,
                    target_id: "left_tray".to_string(),
                    label: "Left Tray Site".to_string(),
                    short_text: "Small tray site for one future nursery occupant.".to_string(),
                    read_only: true,
                },
            ],
        }
    }

    fn assert_invariants(&self) {
        assert!(
            (1..=3).contains(&self.planting_sites.len()),
            "greenhouse room planting-site count must stay tiny"
        );
        assert!(
            !self.environment.outside_weather_influence,
            "first greenhouse environment must ignore outside weather"
        );
        assert!(
            self.inspection_refs
                .iter()
                .all(|inspection| inspection.read_only),
            "first greenhouse inspection refs must stay read-only"
        );

        assert_unique_ids(
            self.access_paths.iter().map(|path| path.id.as_str()),
            "access path ids must be unique inside a room",
        );
        assert_unique_ids(
            self.zones.iter().map(|zone| zone.id.as_str()),
            "zone ids must be unique inside a room",
        );
        assert_unique_ids(
            self.fixtures.iter().map(|fixture| fixture.id.as_str()),
            "fixture ids must be unique inside a room",
        );
        assert_unique_ids(
            self.planting_sites.iter().map(|site| site.id.as_str()),
            "planting-site ids must be unique inside a room",
        );
        assert_unique_ids(
            self.inspection_refs
                .iter()
                .map(|inspection| inspection.id.as_str()),
            "inspection ids must be unique inside a room",
        );

        for site in &self.planting_sites {
            assert!(site.capacity > 0, "planting-site capacity must be positive");
            if let Some(fixture_id) = &site.fixture_id {
                assert!(
                    self.fixtures
                        .iter()
                        .any(|fixture| fixture.id == *fixture_id),
                    "planting-site fixture references must resolve inside the room"
                );
            }
        }

        for inspection in &self.inspection_refs {
            assert!(
                !inspection.target_id.trim().is_empty(),
                "inspection target ids must not be empty"
            );
            match inspection.target {
                InspectionTarget::Room => {
                    assert_eq!(inspection.target_id, self.id.as_str());
                }
                InspectionTarget::Zone => {
                    assert!(self
                        .zones
                        .iter()
                        .any(|zone| zone.id.as_str() == inspection.target_id));
                }
                InspectionTarget::Fixture => {
                    assert!(self
                        .fixtures
                        .iter()
                        .any(|fixture| fixture.id.as_str() == inspection.target_id));
                }
                InspectionTarget::PlantingSite => {
                    assert!(self
                        .planting_sites
                        .iter()
                        .any(|site| site.id.as_str() == inspection.target_id));
                }
                InspectionTarget::Organism => {}
            }
        }
    }
}

fn assert_unique_ids<'a>(ids: impl Iterator<Item = &'a str>, message: &str) {
    let ids: Vec<&str> = ids.collect();
    for (index, id) in ids.iter().enumerate() {
        assert!(
            ids[index + 1..].iter().all(|other| other != id),
            "{message}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nursery_state_constructs_one_active_room() {
        let greenhouse = GreenhouseState::nursery();

        assert_eq!(greenhouse.rooms.len(), 1);
        assert_eq!(greenhouse.active_room_id.as_str(), FIRST_GREENHOUSE_ROOM_ID);
        assert_eq!(
            greenhouse
                .active_room()
                .expect("active room must resolve")
                .id
                .as_str(),
            FIRST_GREENHOUSE_ROOM_ID
        );
        assert!(greenhouse.capabilities.inspection_read_only);
    }

    #[test]
    fn greenhouse_room_ids_are_stable_and_unique() {
        let greenhouse = GreenhouseState::new(
            vec![GreenhouseRoom::nursery()],
            GreenhouseRoomId::new(FIRST_GREENHOUSE_ROOM_ID),
            GreenhouseCapabilityFlags {
                inspection_read_only: true,
            },
        );

        assert_eq!(greenhouse.rooms[0].id.as_str(), FIRST_GREENHOUSE_ROOM_ID);
    }

    #[test]
    fn active_room_contains_resolvable_zones_fixtures_sites_and_inspection_refs() {
        let room = GreenhouseState::nursery()
            .active_room()
            .expect("active room must resolve")
            .clone();

        assert_eq!(room.zones.len(), 3);
        assert_eq!(room.fixtures.len(), 9);
        assert_eq!(room.planting_sites.len(), 3);
        assert_eq!(room.inspection_refs.len(), 4);
        assert!(room
            .zones
            .iter()
            .any(|zone| zone.id.as_str() == "propagation_bench"));
        assert!(room
            .fixtures
            .iter()
            .any(|fixture| fixture.id.as_str() == "inspection_marker"));
        assert!(room
            .planting_sites
            .iter()
            .any(|site| site.id.as_str() == "left_tray"));
        assert!(room
            .inspection_refs
            .iter()
            .any(|inspection| inspection.target == InspectionTarget::Fixture
                && inspection.target_id == "inspection_marker"));
    }

    #[test]
    fn nursery_environment_is_symbolic_and_weather_independent() {
        let room = GreenhouseState::nursery()
            .active_room()
            .expect("active room must resolve")
            .clone();

        assert_eq!(room.environment.id.as_str(), "nursery_symbolic_profile");
        assert_eq!(room.environment.light, SymbolicLight::BrightIndirect);
        assert_eq!(room.environment.humidity, SymbolicHumidity::Humid);
        assert_eq!(room.environment.temperature, SymbolicTemperature::Temperate);
        assert_eq!(room.environment.water, SymbolicWater::Moderate);
        assert_eq!(room.environment.airflow, SymbolicAirflow::SoftCirculation);
        assert_eq!(
            room.environment.substrate,
            SymbolicSubstrate::SeedStartingMix
        );
        assert!(!room.environment.outside_weather_influence);
    }

    #[test]
    fn inspection_refs_are_read_only_and_do_not_own_targets() {
        let room = GreenhouseState::nursery()
            .active_room()
            .expect("active room must resolve")
            .clone();

        assert!(room
            .inspection_refs
            .iter()
            .all(|inspection| inspection.read_only));
        assert!(room
            .inspection_refs
            .iter()
            .all(|inspection| !inspection.target_id.is_empty()));
    }

    #[test]
    #[should_panic(expected = "greenhouse room ids must be unique")]
    fn duplicate_room_ids_are_rejected() {
        let room = GreenhouseRoom::nursery();
        GreenhouseState::new(
            vec![room.clone(), room],
            GreenhouseRoomId::new(FIRST_GREENHOUSE_ROOM_ID),
            GreenhouseCapabilityFlags {
                inspection_read_only: true,
            },
        );
    }

    #[test]
    #[should_panic(expected = "active greenhouse room must exist")]
    fn active_room_id_must_resolve() {
        GreenhouseState::new(
            vec![GreenhouseRoom::nursery()],
            GreenhouseRoomId::new("missing_room"),
            GreenhouseCapabilityFlags {
                inspection_read_only: true,
            },
        );
    }

    #[test]
    #[should_panic(expected = "planting-site count must stay tiny")]
    fn planting_site_capacity_stays_tiny() {
        let mut room = GreenhouseRoom::nursery();
        room.planting_sites.push(PlantingSite {
            id: PlantingSiteId::new("overflow_site"),
            kind: PlantingSiteKind::ShelfSlot,
            label: "Overflow Site".to_string(),
            anchor: SpatialPoint::new(14, 1),
            capacity: 1,
            fixture_id: Some(FixtureId::new("specimen_shelf")),
            occupant: None,
        });

        GreenhouseState::new(
            vec![room],
            GreenhouseRoomId::new(FIRST_GREENHOUSE_ROOM_ID),
            GreenhouseCapabilityFlags {
                inspection_read_only: true,
            },
        );
    }
}
