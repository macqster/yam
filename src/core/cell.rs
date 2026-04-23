#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub entity_id: Option<u32>,
    pub density: f32,
    pub age: u32,
}
