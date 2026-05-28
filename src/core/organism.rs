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
}
