use crate::core::cell::Cell;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Grid {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            cells: vec![Cell::default(); size],
        }
    }

    pub fn index(&self, x: u16, y: u16) -> usize {
        (y as usize) * (self.width as usize) + (x as usize)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.width as usize && y < self.height as usize {
            Some(&self.cells[self.index(x as u16, y as u16)])
        } else {
            None
        }
    }
}
