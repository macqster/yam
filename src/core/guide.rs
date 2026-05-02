#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct GuidePoint {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GuideShape {
    Point,
    Line { end: GuidePoint },
    Polyline(Vec<GuidePoint>),
    Polygon(Vec<GuidePoint>),
    Rect { width: i32, height: i32 },
    Circle { radius: i32 },
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuideKind {
    Datum,
    Attractor,
    Avoidance,
    Boundary,
    Waypoint,
    Axis,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GuideStyle {
    pub glyph: char,
    pub visible: bool,
    pub accent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Guide {
    pub id: u32,
    pub label: String,
    pub group: Option<String>,
    pub kind: GuideKind,
    pub anchor: GuidePoint,
    pub shape: GuideShape,
    pub style: GuideStyle,
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GuideSet {
    pub label: String,
    pub guides: Vec<u32>,
}

impl GuideSet {
    #[allow(dead_code)]
    pub fn new(label: impl Into<String>, guides: impl Into<Vec<u32>>) -> Self {
        Self {
            label: label.into(),
            guides: guides.into(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GuideState {
    pub guides: Vec<Guide>,
    pub sets: Vec<GuideSet>,
}

impl GuideState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            guides: Vec::new(),
            sets: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn enabled_guides(&self) -> impl Iterator<Item = &Guide> {
        self.guides.iter().filter(|guide| guide.enabled)
    }

    #[allow(dead_code)]
    pub fn guide_by_id(&self, id: u32) -> Option<&Guide> {
        self.guides.iter().find(|guide| guide.id == id)
    }

    #[allow(dead_code)]
    pub fn guide_by_label(&self, label: &str) -> Option<&Guide> {
        self.guides.iter().find(|guide| guide.label == label)
    }

    #[allow(dead_code)]
    pub fn guides_in_group(&self, group: &str) -> Vec<&Guide> {
        self.guides
            .iter()
            .filter(move |guide| guide.group.as_deref() == Some(group))
            .collect()
    }

    #[allow(dead_code)]
    pub fn guide_set(&self, label: &str) -> Option<&GuideSet> {
        self.sets.iter().find(|set| set.label == label)
    }

    #[allow(dead_code)]
    pub fn add_set(&mut self, set: GuideSet) {
        self.sets.push(set);
    }

    #[allow(dead_code)]
    pub fn guides_in_set(&self, label: &str) -> Vec<&Guide> {
        let Some(set) = self.guide_set(label) else {
            return Vec::new();
        };
        set.guides
            .iter()
            .filter_map(|id| self.guide_by_id(*id))
            .collect()
    }

    #[allow(dead_code)]
    pub fn set_contains(&self, set_label: &str, guide_id: u32) -> bool {
        self.guide_set(set_label)
            .map(|set| set.guides.contains(&guide_id))
            .unwrap_or(false)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuideHit {
    pub guide_id: u32,
    pub point: GuidePoint,
    pub distance_sq: i64,
}

#[allow(dead_code)]
pub trait GuideField {
    fn nearest_point(&self, point: GuidePoint) -> Option<GuideHit>;
}

impl GuideField for GuideState {
    fn nearest_point(&self, point: GuidePoint) -> Option<GuideHit> {
        self.enabled_guides()
            .filter(|guide| guide.style.visible)
            .map(|guide| {
                let dx = i64::from(guide.anchor.x - point.x);
                let dy = i64::from(guide.anchor.y - point.y);
                GuideHit {
                    guide_id: guide.id,
                    point: guide.anchor,
                    distance_sq: dx * dx + dy * dy,
                }
            })
            .min_by_key(|hit| hit.distance_sq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestField {
        guides: GuideState,
    }

    impl GuideField for TestField {
        fn nearest_point(&self, point: GuidePoint) -> Option<GuideHit> {
            self.guides
                .enabled_guides()
                .filter(|guide| guide.style.visible)
                .map(|guide| {
                    let dx = i64::from(guide.anchor.x - point.x);
                    let dy = i64::from(guide.anchor.y - point.y);
                    GuideHit {
                        guide_id: guide.id,
                        point: guide.anchor,
                        distance_sq: dx * dx + dy * dy,
                    }
                })
                .min_by_key(|hit| hit.distance_sq)
        }
    }

    #[test]
    fn guide_state_filters_disabled_guides() {
        let mut state = GuideState::new();
        state.guides.push(Guide {
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
        state.guides.push(Guide {
            id: 2,
            label: "waypoint".to_string(),
            group: Some("world-anchors".to_string()),
            kind: GuideKind::Waypoint,
            anchor: GuidePoint { x: 10, y: 0 },
            shape: GuideShape::Point,
            style: GuideStyle {
                glyph: '•',
                visible: true,
                accent: false,
            },
            enabled: false,
        });

        let ids: Vec<u32> = state.enabled_guides().map(|guide| guide.id).collect();
        assert_eq!(ids, vec![1]);
        assert!(state.guide_by_id(1).is_some());
        assert!(state.guide_by_id(2).is_some());
        state.add_set(GuideSet::new("world-anchors", vec![1, 2]));
        assert!(state.guide_set("world-anchors").is_some());
        assert_eq!(state.guides_in_set("world-anchors").len(), 2);
        assert!(state.set_contains("world-anchors", 1));
    }

    #[test]
    fn guide_field_returns_closest_enabled_anchor() {
        let mut state = GuideState::new();
        state.guides.push(Guide {
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
        state.guides.push(Guide {
            id: 2,
            label: "waypoint".to_string(),
            group: Some("world-anchors".to_string()),
            kind: GuideKind::Waypoint,
            anchor: GuidePoint { x: 12, y: 0 },
            shape: GuideShape::Point,
            style: GuideStyle {
                glyph: '•',
                visible: true,
                accent: false,
            },
            enabled: true,
        });
        let field = TestField { guides: state };

        let hit = field
            .nearest_point(GuidePoint { x: 9, y: 0 })
            .expect("nearest guide");

        assert_eq!(hit.guide_id, 2);
        assert_eq!(hit.point, GuidePoint { x: 12, y: 0 });
        assert_eq!(hit.distance_sq, 9);
    }
}
