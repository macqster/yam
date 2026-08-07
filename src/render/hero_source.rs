//! Hero source assets.
//!
//! A `HeroSource` is the single place that knows which GIF backs a hero, what
//! shape that GIF is, and how its frames are cached. Everything else in the
//! hero path (decode, chafa conversion, cache key, geometry tests) reads those
//! facts from here instead of repeating them, so adding or swapping hero art
//! is a descriptor change rather than a hunt through constants, cache-name
//! literals, tests, and docs.
//!
//! Render dimensions stay per-source because they are a chafa *request*, not a
//! guarantee: chafa preserves source aspect, so the emitted cell footprint is
//! derived from the frames themselves (see `render::hero`), not assumed here.

/// Stable identity for a hero asset.
///
/// The `stem` and `cache_revision` form the cache-file prefix, so two sources
/// cannot share frames and older renderer output cannot mask revised behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeroSource {
    /// Stable short id, also used as the frame-cache filename prefix.
    pub stem: &'static str,
    /// Absolute path to the source GIF, resolved at compile time.
    pub path: &'static str,
    /// Logical canvas the decoder expands every subimage frame onto.
    pub canvas_width: u32,
    pub canvas_height: u32,
    /// Frame count the asset is expected to carry.
    pub frame_count: usize,
    /// Cell footprint requested from chafa.
    pub render_width: u16,
    pub render_height: u16,
    /// Bump when source-specific compilation or the serialized frame contract
    /// changes, so an older cache can never mask the new renderer behavior.
    pub cache_revision: u16,
}

/// The original BTAS/TNBA-derived Ivy hero.
pub const IVY: HeroSource = HeroSource {
    stem: "hero_gif_1",
    path: concat!(env!("CARGO_MANIFEST_DIR"), "/assets/hero_gif_1.gif"),
    canvas_width: 820,
    canvas_height: 820,
    frame_count: 64,
    render_width: 96,
    render_height: 48,
    cache_revision: 2,
};

/// Every hero source the runtime knows about.
///
/// Currently read by the geometry/uniqueness gates rather than by a runtime
/// selection surface; a second source plus its world/settings wiring is the
/// next slice, and this registry is what that wiring will iterate.
#[allow(dead_code)]
pub const ALL: &[HeroSource] = &[IVY];

/// The source used when nothing selects one explicitly.
pub const DEFAULT: HeroSource = IVY;

impl HeroSource {
    /// Frame-cache filename for this source at its render size.
    pub fn cache_file_name(&self) -> String {
        format!(
            "{}.r{}.{}x{}.frame_cache.json",
            self.stem, self.cache_revision, self.render_width, self.render_height
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{ALL, DEFAULT, IVY};

    #[test]
    fn cache_file_names_are_unique_per_source() {
        let mut names: Vec<String> = ALL.iter().map(|source| source.cache_file_name()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(
            names.len(),
            total,
            "each hero source needs its own frame cache file"
        );
    }

    #[test]
    fn source_stems_are_unique() {
        let mut stems: Vec<&str> = ALL.iter().map(|source| source.stem).collect();
        let total = stems.len();
        stems.sort();
        stems.dedup();
        assert_eq!(stems.len(), total, "hero source stems must be distinct");
    }

    #[test]
    fn default_source_is_registered() {
        assert!(ALL.contains(&DEFAULT), "default hero source must be in ALL");
    }

    #[test]
    fn ivy_cache_name_matches_the_documented_runtime_path() {
        assert_eq!(
            IVY.cache_file_name(),
            "hero_gif_1.r2.96x48.frame_cache.json"
        );
    }
}
