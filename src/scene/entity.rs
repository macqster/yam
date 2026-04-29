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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchored_entities_keep_world_offsets_stable() {
        let hero = hero_pose(WorldPos { x: 150, y: 60 }, WorldPos { x: -110, y: -54 });
        let clock = attached_pose(hero.anchor_world(), WorldPos { x: 96, y: 9 });

        assert_eq!(hero.anchor_world(), WorldPos { x: 40, y: 6 });
        assert_eq!(clock.world(), WorldPos { x: 136, y: 15 });
    }
}
