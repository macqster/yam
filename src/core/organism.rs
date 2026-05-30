use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OrganismId(u32);

impl OrganismId {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[allow(dead_code)]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Display for OrganismId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u32> for OrganismId {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpeciesId(String);

impl SpeciesId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for SpeciesId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for SpeciesId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for SpeciesId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq<&str> for SpeciesId {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct JournalId(String);

impl JournalId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for JournalId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for JournalId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for JournalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq<&str> for JournalId {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrganismIdentity {
    pub id: OrganismId,
    pub species_id: SpeciesId,
    pub journal_id: JournalId,
}

impl OrganismIdentity {
    pub fn new(id: OrganismId, species_id: SpeciesId, journal_id: JournalId) -> Self {
        Self {
            id,
            species_id,
            journal_id,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganismFamily {
    Vine,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrganismLifeState {
    Dormant,
    Growing,
    Mature,
    Senescent,
    Dead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrganismStats {
    pub age_ticks: u64,
    pub vigor: u16,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpeciesProfile {
    pub species_id: SpeciesId,
    pub display_name: String,
    pub family: OrganismFamily,
    pub habit: String,
    pub anatomy_defaults: String,
    pub growth_rule: String,
    pub lifecycle_tuning: String,
    pub allowed_organs: Vec<String>,
    pub debug_label: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpeciesRegistry {
    profiles: Vec<SpeciesProfile>,
}

#[allow(dead_code)]
impl SpeciesRegistry {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
        }
    }

    pub fn with_profiles(profiles: Vec<SpeciesProfile>) -> Self {
        Self { profiles }
    }

    pub fn register(&mut self, profile: SpeciesProfile) {
        self.profiles.push(profile);
    }

    pub fn profile(&self, species_id: &SpeciesId) -> Option<&SpeciesProfile> {
        self.profiles
            .iter()
            .find(|profile| profile.species_id.as_str() == species_id.as_str())
    }

    pub fn profiles(&self) -> &[SpeciesProfile] {
        &self.profiles
    }

    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    pub fn len(&self) -> usize {
        self.profiles.len()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JournalEventKind {
    Created,
    LifeStateChanged,
    GrowthStep,
    OrganChanged,
    EnvironmentInfluence,
    DebugNote,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JournalEvent {
    pub tick: u64,
    pub kind: JournalEventKind,
    pub message: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrganismJournal {
    pub journal_id: JournalId,
    events: Vec<JournalEvent>,
}

#[allow(dead_code)]
impl OrganismJournal {
    pub fn new(journal_id: JournalId) -> Self {
        Self {
            journal_id,
            events: Vec::new(),
        }
    }

    pub fn record(&mut self, tick: u64, kind: JournalEventKind, message: impl Into<String>) {
        self.events.push(JournalEvent {
            tick,
            kind,
            message: message.into(),
        });
    }

    pub fn events(&self) -> &[JournalEvent] {
        &self.events
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn organism_identity_keeps_runtime_and_registry_ids_together() {
        let identity = OrganismIdentity::new(
            OrganismId::new(7),
            SpeciesId::new("yam.vine.fixture"),
            JournalId::new("journal.vine.fixture.7"),
        );

        assert_eq!(identity.id, OrganismId::new(7));
        assert_eq!(identity.species_id, "yam.vine.fixture");
        assert_eq!(identity.journal_id, "journal.vine.fixture.7");
    }

    fn fixture_profile(id: &str, display_name: &str) -> SpeciesProfile {
        SpeciesProfile {
            species_id: SpeciesId::new(id),
            display_name: display_name.to_string(),
            family: OrganismFamily::Vine,
            habit: "fixture habit".to_string(),
            anatomy_defaults: "fixture anatomy".to_string(),
            growth_rule: "fixture growth".to_string(),
            lifecycle_tuning: "fixture lifecycle".to_string(),
            allowed_organs: vec!["leaf".to_string()],
            debug_label: display_name.to_string(),
        }
    }

    #[test]
    fn species_registry_stores_reusable_profiles_without_history() {
        let mut registry = SpeciesRegistry::new();
        assert!(registry.is_empty());

        registry.register(fixture_profile("yam.vine.fixture", "fixture vine"));

        let profile = registry
            .profile(&SpeciesId::new("yam.vine.fixture"))
            .expect("registered profile");
        assert_eq!(profile.display_name, "fixture vine");
        assert_eq!(registry.len(), 1);
        assert_eq!(registry.profiles().len(), 1);
    }

    #[test]
    fn organism_journal_records_per_instance_events() {
        let mut journal = OrganismJournal::new(JournalId::new("journal.vine.fixture.7"));
        assert!(journal.is_empty());

        journal.record(4, JournalEventKind::Created, "seeded fixture vine");
        journal.record(8, JournalEventKind::GrowthStep, "extended primary axis");

        assert_eq!(journal.journal_id, "journal.vine.fixture.7");
        assert_eq!(journal.events().len(), 2);
        assert_eq!(journal.events()[0].tick, 4);
        assert_eq!(journal.events()[0].kind, JournalEventKind::Created);
        assert_eq!(journal.events()[1].message, "extended primary axis");
    }
}
