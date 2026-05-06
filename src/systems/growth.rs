use crate::core::flora::{
    VineGrowthTipState, VineHealth, VineLifeState, VineSegment, BORDER_VINE_GROWTH_INTERVAL_TICKS,
    BORDER_VINE_MAX_SEGMENTS, BORDER_VINE_SEED_ID,
};
use crate::core::world::WorldState;

/// Spawn new entities based on fields
pub fn run_growth(world: &mut WorldState) {
    if !(world.tick + 1).is_multiple_of(BORDER_VINE_GROWTH_INTERVAL_TICKS) {
        return;
    }

    let Some(vine) = world
        .flora
        .vines
        .iter_mut()
        .find(|vine| vine.id == BORDER_VINE_SEED_ID)
    else {
        return;
    };

    let Some(axis) = vine.axes.first_mut() else {
        return;
    };
    if axis.segments.len() >= BORDER_VINE_MAX_SEGMENTS {
        if let Some(tip) = vine.growth_tips.first_mut() {
            tip.state = VineGrowthTipState::Spent;
        }
        vine.life_state = VineLifeState::Mature;
        return;
    }

    let Some(last_segment) = axis.segments.last().cloned() else {
        return;
    };
    let Some(tip) = vine.growth_tips.first_mut() else {
        return;
    };
    if !matches!(tip.state, VineGrowthTipState::Active) {
        return;
    }
    if tip.remaining_growth_steps == 0 {
        tip.state = VineGrowthTipState::Spent;
        vine.life_state = VineLifeState::Mature;
        return;
    }

    let dx = last_segment.end.x - last_segment.start.x;
    let dy = last_segment.end.y - last_segment.start.y;
    if dx == 0 && dy == 0 {
        return;
    }

    let next_end = crate::scene::coords::WorldPos {
        x: tip.position.x + dx,
        y: tip.position.y + dy,
    };
    axis.segments.push(VineSegment {
        start: tip.position,
        end: next_end,
        thickness: last_segment.thickness,
        age_ticks: 0,
        health: VineHealth::Healthy,
        guide_id: last_segment.guide_id,
    });
    tip.position = next_end;
    tip.age_ticks += BORDER_VINE_GROWTH_INTERVAL_TICKS;
    tip.remaining_growth_steps = tip.remaining_growth_steps.saturating_sub(1);
    if tip.remaining_growth_steps == 0 {
        tip.state = VineGrowthTipState::Spent;
        vine.life_state = VineLifeState::Mature;
    } else {
        vine.life_state = VineLifeState::Growing;
    }
    vine.stats.age_ticks += 1;
}

#[cfg(test)]
mod tests {
    use super::run_growth;
    use crate::core::flora::{
        VineGrowthTipState, VineLifeState, BORDER_VINE_GROWTH_INTERVAL_TICKS,
        BORDER_VINE_MAX_SEGMENTS, BORDER_VINE_TIP_GROWTH_BUDGET,
    };
    use crate::core::world::WorldState;

    #[test]
    fn growth_waits_for_fixed_tick_cadence() {
        let mut world = WorldState::new();
        let initial_segments = world.flora.vines[0].axes[0].segments.len();

        for tick in 0..(BORDER_VINE_GROWTH_INTERVAL_TICKS - 1) {
            world.tick = tick;
            run_growth(&mut world);
        }

        assert_eq!(
            world.flora.vines[0].axes[0].segments.len(),
            initial_segments
        );

        world.tick = BORDER_VINE_GROWTH_INTERVAL_TICKS - 1;
        run_growth(&mut world);

        assert_eq!(
            world.flora.vines[0].axes[0].segments.len(),
            initial_segments + 1
        );
        assert_eq!(world.flora.vines[0].stats.age_ticks, 1);
        assert_eq!(
            world.flora.vines[0].growth_tips[0].age_ticks,
            BORDER_VINE_GROWTH_INTERVAL_TICKS
        );
        assert_eq!(
            world.flora.vines[0].growth_tips[0].remaining_growth_steps,
            BORDER_VINE_TIP_GROWTH_BUDGET - 1
        );
    }

    #[test]
    fn growth_extends_from_the_current_tip_position() {
        let mut world = WorldState::new();
        let last_before = world.flora.vines[0].axes[0]
            .segments
            .last()
            .expect("segment")
            .clone();
        let tip_before = world.flora.vines[0].growth_tips[0].position;

        world.tick = BORDER_VINE_GROWTH_INTERVAL_TICKS - 1;
        run_growth(&mut world);

        let last_after = world.flora.vines[0].axes[0]
            .segments
            .last()
            .expect("segment")
            .clone();
        assert_eq!(last_after.start, tip_before);
        assert_eq!(
            last_after.end.x - last_after.start.x,
            last_before.end.x - last_before.start.x
        );
        assert_eq!(
            last_after.end.y - last_after.start.y,
            last_before.end.y - last_before.start.y
        );
        assert_eq!(world.flora.vines[0].growth_tips[0].position, last_after.end);
        assert_eq!(world.flora.vines[0].life_state, VineLifeState::Growing);
    }

    #[test]
    fn growth_stops_when_the_local_tip_budget_is_spent() {
        let mut world = WorldState::new();
        while world.flora.vines[0].growth_tips[0].remaining_growth_steps > 0 {
            world.tick += BORDER_VINE_GROWTH_INTERVAL_TICKS - 1;
            run_growth(&mut world);
            world.tick += 1;
        }

        let segment_count = world.flora.vines[0].axes[0].segments.len();
        assert_eq!(
            world.flora.vines[0].growth_tips[0].state,
            VineGrowthTipState::Spent
        );
        assert_eq!(world.flora.vines[0].life_state, VineLifeState::Mature);

        world.tick += BORDER_VINE_GROWTH_INTERVAL_TICKS - 1;
        run_growth(&mut world);

        assert_eq!(world.flora.vines[0].axes[0].segments.len(), segment_count);
    }

    #[test]
    fn growth_stops_at_the_max_segment_cap() {
        let mut world = WorldState::new();
        world.flora.vines[0].growth_tips[0].remaining_growth_steps =
            BORDER_VINE_MAX_SEGMENTS as u16;
        while world.flora.vines[0].axes[0].segments.len() < BORDER_VINE_MAX_SEGMENTS {
            world.tick += BORDER_VINE_GROWTH_INTERVAL_TICKS - 1;
            run_growth(&mut world);
            world.tick += 1;
        }

        let segment_count = world.flora.vines[0].axes[0].segments.len();
        world.tick += BORDER_VINE_GROWTH_INTERVAL_TICKS - 1;
        run_growth(&mut world);

        assert_eq!(world.flora.vines[0].axes[0].segments.len(), segment_count);
        assert_eq!(world.flora.vines[0].life_state, VineLifeState::Mature);
        assert_eq!(
            world.flora.vines[0].growth_tips[0].state,
            VineGrowthTipState::Spent
        );
    }
}
