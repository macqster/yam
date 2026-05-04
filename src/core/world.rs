use crate::core::{entity::Entity, grid::Grid, guide::GuideState};
use crate::scene::coords::WorldPos;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Fields {
    pub density: Vec<f32>,
    pub attraction: Vec<f32>,
    pub avoidance: Vec<f32>,
}

impl Fields {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            density: vec![0.0; size],
            attraction: vec![0.0; size],
            avoidance: vec![0.0; size],
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WorldState {
    pub grid: Grid,
    pub entities: Vec<Entity>,
    pub fields: Fields,
    pub guides: GuideState,
    pub tick: u64,
}

#[allow(dead_code)]
impl WorldState {
    pub fn new() -> Self {
        let width = 300;
        let height = 120;
        Self {
            grid: Grid::new(width, height),
            entities: Vec::new(),
            fields: Fields::new(width, height),
            guides: GuideState::new(),
            tick: 0,
        }
    }

    pub fn entity_world(&self, id: u32) -> Option<WorldPos> {
        self.entities
            .iter()
            .find(|entity| entity.id == id)
            .map(|entity| WorldPos {
                x: entity.x as i32,
                y: entity.y as i32,
            })
    }
}
