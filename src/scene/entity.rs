use crate::core::spatial::{SpatialPoint as WorldPos, SpatialResolver};

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
        SpatialResolver::resolve_anchor(self.world, self.offset)
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
        SpatialResolver::resolve_anchor(self.anchor, self.offset)
    }
}

pub fn attached_pose(anchor: WorldPos, offset: WorldPos) -> AttachedEntityPose {
    AttachedEntityPose::new(anchor, offset)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HeroSceneAttachment {
    pub hero: EntityPose,
    pub clock: AttachedEntityPose,
    pub weather: AttachedEntityPose,
    pub date: AttachedEntityPose,
    pub calendar: AttachedEntityPose,
}

impl HeroSceneAttachment {
    pub fn new(
        hero_world: WorldPos,
        hero_offset: WorldPos,
        clock_offset: WorldPos,
        weather_offset: WorldPos,
        date_offset: WorldPos,
        calendar_offset: WorldPos,
    ) -> Self {
        let hero = hero_pose(hero_world, hero_offset);
        let clock = attached_pose(hero.anchor_world(), clock_offset);
        let weather = attached_pose(hero.anchor_world(), weather_offset);
        let date = attached_pose(hero.anchor_world(), date_offset);
        let calendar = attached_pose(hero.anchor_world(), calendar_offset);
        Self {
            hero,
            clock,
            weather,
            date,
            calendar,
        }
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

    pub fn weather_world(&self) -> WorldPos {
        self.weather.world()
    }

    pub fn date_world(&self) -> WorldPos {
        self.date.world()
    }

    pub fn calendar_world(&self) -> WorldPos {
        self.calendar.world()
    }
}

pub fn hero_scene_poses(
    hero_world: WorldPos,
    hero_offset: WorldPos,
    clock_offset: WorldPos,
    weather_offset: WorldPos,
    date_offset: WorldPos,
    calendar_offset: WorldPos,
) -> HeroSceneAttachment {
    HeroSceneAttachment::new(
        hero_world,
        hero_offset,
        clock_offset,
        weather_offset,
        date_offset,
        calendar_offset,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchored_scene_companions_keep_world_offsets_stable() {
        let attachment = hero_scene_poses(
            WorldPos { x: 150, y: 60 },
            WorldPos { x: -110, y: -54 },
            WorldPos { x: 96, y: 9 },
            WorldPos { x: 120, y: 14 },
            WorldPos { x: 96, y: 15 },
            WorldPos { x: 126, y: 15 },
        );

        assert_eq!(attachment.hero_world(), WorldPos { x: 150, y: 60 });
        assert_eq!(attachment.hero_visual_anchor(), WorldPos { x: 40, y: 6 });
        assert_eq!(attachment.clock_world(), WorldPos { x: 136, y: 15 });
        assert_eq!(attachment.weather_world(), WorldPos { x: 160, y: 20 });
        assert_eq!(attachment.date_world(), WorldPos { x: 136, y: 21 });
        assert_eq!(attachment.calendar_world(), WorldPos { x: 166, y: 21 });
    }
}
