use crate::scene::coords::{anchor_to_world, WorldPos};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityPose {
    pub world: WorldPos,
    pub offset: WorldPos,
}

impl EntityPose {
    pub fn new(world: WorldPos, offset: WorldPos) -> Self {
        Self { world, offset }
    }

    pub fn anchor_world(&self) -> WorldPos {
        anchor_to_world(self.world, self.offset)
    }
}

pub fn hero_pose(world: WorldPos, offset: WorldPos) -> EntityPose {
    EntityPose::new(world, offset)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AttachedEntityPose {
    pub anchor: WorldPos,
    pub offset: WorldPos,
}

impl AttachedEntityPose {
    pub fn new(anchor: WorldPos, offset: WorldPos) -> Self {
        Self { anchor, offset }
    }

    pub fn world(&self) -> WorldPos {
        anchor_to_world(self.anchor, self.offset)
    }
}

pub fn attached_pose(anchor: WorldPos, offset: WorldPos) -> AttachedEntityPose {
    AttachedEntityPose::new(anchor, offset)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HeroClockAttachment {
    pub hero: EntityPose,
    pub clock: AttachedEntityPose,
}

impl HeroClockAttachment {
    pub fn new(hero_world: WorldPos, hero_offset: WorldPos, clock_offset: WorldPos) -> Self {
        let hero = hero_pose(hero_world, hero_offset);
        let clock = attached_pose(hero.anchor_world(), clock_offset);
        Self { hero, clock }
    }

    pub fn hero_world(&self) -> WorldPos {
        self.hero.world
    }

    pub fn hero_visual_anchor(&self) -> WorldPos {
        self.hero.anchor_world()
    }

    pub fn clock_world(&self) -> WorldPos {
        self.clock.world()
    }
}

pub fn hero_and_clock_poses(
    hero_world: WorldPos,
    hero_offset: WorldPos,
    clock_offset: WorldPos,
) -> HeroClockAttachment {
    HeroClockAttachment::new(hero_world, hero_offset, clock_offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchored_entities_keep_world_offsets_stable() {
        let attachment = hero_and_clock_poses(
            WorldPos { x: 150, y: 60 },
            WorldPos { x: -110, y: -54 },
            WorldPos { x: 96, y: 9 },
        );

        assert_eq!(attachment.hero_world(), WorldPos { x: 150, y: 60 });
        assert_eq!(attachment.hero_visual_anchor(), WorldPos { x: 40, y: 6 });
        assert_eq!(attachment.clock_world(), WorldPos { x: 136, y: 15 });
    }
}
