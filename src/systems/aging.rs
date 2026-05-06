use crate::core::flora::rebuild_leaf_organs;
use crate::core::world::WorldState;

/// Advance entity and cell age
pub fn run_aging(world: &mut WorldState) {
    for vine in &mut world.flora.vines {
        for axis in &mut vine.axes {
            for segment in &mut axis.segments {
                segment.age_ticks += 1;
            }
        }
        rebuild_leaf_organs(vine);
    }
}

#[cfg(test)]
mod tests {
    use super::run_aging;
    use crate::core::flora::{VineOrganKind, BORDER_VINE_LEAF_MATURITY_TICKS};
    use crate::core::world::WorldState;

    #[test]
    fn aging_increments_segment_age_and_rebuilds_leaf_organs() {
        let mut world = WorldState::new();
        for segment in &mut world.flora.vines[0].axes[0].segments {
            segment.age_ticks = BORDER_VINE_LEAF_MATURITY_TICKS - 1;
        }

        run_aging(&mut world);

        assert!(world.flora.vines[0].axes[0]
            .segments
            .iter()
            .all(|segment| segment.age_ticks >= BORDER_VINE_LEAF_MATURITY_TICKS));
        assert!(world.flora.vines[0]
            .organs
            .iter()
            .all(|organ| matches!(organ.kind, VineOrganKind::Leaf)));
    }
}
