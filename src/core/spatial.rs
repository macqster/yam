use crate::core::guide::{Guide, GuideSet, GuideState};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpatialPoint {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpatialScreenPoint {
    pub x: u16,
    pub y: u16,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpatialAnchor {
    pub point: SpatialPoint,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpatialAttachment {
    pub anchor: SpatialAnchor,
    pub offset: SpatialPoint,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpatialProjection {
    pub camera_x: i32,
    pub camera_y: i32,
    pub width: u16,
    pub height: u16,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpatialResolver {
    pub projection: SpatialProjection,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct SpatialGuideIndex<'a> {
    pub guides: &'a GuideState,
}

#[allow(dead_code)]
impl SpatialPoint {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl SpatialAnchor {
    pub const fn new(point: SpatialPoint) -> Self {
        Self { point }
    }
}

#[allow(dead_code)]
impl SpatialAttachment {
    pub const fn new(anchor: SpatialAnchor, offset: SpatialPoint) -> Self {
        Self { anchor, offset }
    }
}

#[allow(dead_code)]
impl SpatialProjection {
    pub const fn new(camera_x: i32, camera_y: i32, width: u16, height: u16) -> Self {
        Self {
            camera_x,
            camera_y,
            width,
            height,
        }
    }
}

#[allow(dead_code)]
impl SpatialResolver {
    pub const fn new(projection: SpatialProjection) -> Self {
        Self { projection }
    }

    pub fn anchor_to_world(anchor: SpatialPoint, offset: SpatialPoint) -> SpatialPoint {
        SpatialPoint {
            x: anchor.x + offset.x,
            y: anchor.y + offset.y,
        }
    }

    pub fn resolve_attachment(attachment: SpatialAttachment) -> SpatialPoint {
        Self::anchor_to_world(attachment.anchor.point, attachment.offset)
    }

    pub fn resolve_anchor(anchor: SpatialPoint, offset: SpatialPoint) -> SpatialPoint {
        Self::anchor_to_world(anchor, offset)
    }

    pub fn resolve_anchor_or_world(
        anchor: Option<SpatialPoint>,
        fallback_world: SpatialPoint,
        offset: SpatialPoint,
    ) -> SpatialPoint {
        anchor
            .map(|anchor| Self::resolve_anchor(anchor, offset))
            .unwrap_or(fallback_world)
    }

    pub fn world_to_screen(&self, world: SpatialPoint) -> SpatialPoint {
        let top = self.projection.camera_y + self.projection.height as i32 - 1;
        SpatialPoint {
            x: world.x - self.projection.camera_x,
            y: top - world.y,
        }
    }

    pub fn world_to_screen_point(&self, world: SpatialPoint) -> SpatialScreenPoint {
        let screen = self.world_to_screen(world);
        SpatialScreenPoint {
            x: screen.x as u16,
            y: screen.y as u16,
        }
    }

    pub fn screen_to_world(&self, screen: SpatialPoint) -> SpatialPoint {
        let top = self.projection.camera_y + self.projection.height as i32 - 1;
        SpatialPoint {
            x: screen.x + self.projection.camera_x,
            y: top - screen.y,
        }
    }

    pub fn screen_to_world_point(&self, screen: SpatialScreenPoint) -> SpatialPoint {
        let top = self.projection.camera_y + self.projection.height as i32 - 1;
        SpatialPoint {
            x: screen.x as i32 + self.projection.camera_x,
            y: top - screen.y as i32,
        }
    }

    pub fn screen_point(&self, screen_x: i32, screen_y: i32) -> SpatialScreenPoint {
        SpatialScreenPoint {
            x: screen_x.max(0) as u16,
            y: screen_y.max(0) as u16,
        }
    }
}

#[allow(dead_code)]
impl<'a> SpatialGuideIndex<'a> {
    pub fn new(guides: &'a GuideState) -> Self {
        Self { guides }
    }

    pub fn guide_set(&self, label: &str) -> Option<&GuideSet> {
        self.guides.guide_set(label)
    }

    pub fn guides_in_set(&self, label: &str) -> Vec<&Guide> {
        self.guides.guides_in_set(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::guide::{Guide, GuideKind, GuidePoint, GuideShape, GuideStyle};

    #[test]
    fn attachment_resolves_through_shared_resolver() {
        let anchor = SpatialAnchor::new(SpatialPoint::new(150, 60));
        let attachment = SpatialAttachment::new(anchor, SpatialPoint::new(-110, -54));
        let point = SpatialResolver::resolve_attachment(attachment);
        assert_eq!(point, SpatialPoint::new(40, 6));
    }

    #[test]
    fn projection_round_trips_world_and_screen() {
        let resolver = SpatialResolver::new(SpatialProjection::new(30, 10, 124, 32));
        let world = SpatialPoint::new(136, 15);
        let screen = resolver.world_to_screen(world);
        assert_eq!(screen, SpatialPoint::new(106, 26));
        assert_eq!(resolver.screen_to_world(screen), world);
    }

    #[test]
    fn projection_round_trips_explicit_screen_points() {
        let resolver = SpatialResolver::new(SpatialProjection::new(30, 10, 124, 32));
        let world = SpatialPoint::new(136, 15);
        let screen = resolver.world_to_screen_point(world);
        assert_eq!(screen, SpatialScreenPoint { x: 106, y: 26 });
        assert_eq!(resolver.screen_to_world_point(screen), world);
    }

    #[test]
    fn guide_index_can_access_named_sets() {
        let mut guides = GuideState::new();
        guides.guides.push(Guide {
            id: 1,
            label: "datum".to_string(),
            group: Some("world-anchors".to_string()),
            kind: GuideKind::Datum,
            anchor: GuidePoint { x: 0, y: 0 },
            shape: GuideShape::Point,
            style: GuideStyle {
                glyph: '+',
                visible: true,
                accent: true,
            },
            enabled: true,
        });
        guides.add_set(GuideSet::new("world-anchors", vec![1]));
        let index = SpatialGuideIndex::new(&guides);
        assert!(index.guide_set("world-anchors").is_some());
    }
}
