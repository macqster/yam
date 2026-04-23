use super::{
    aging::run_aging, constraints::resolve_constraints, density::run_density,
    fields::update_fields, growth::run_growth,
};
use crate::core::world::WorldState;

/// Advances simulation by one deterministic step
pub fn tick(world: &mut WorldState) {
    update_fields(world);
    run_growth(world);
    run_aging(world);
    run_density(world);
    resolve_constraints(world);
    world.tick += 1;
}
