use super::cell::Cell;

pub struct Grid {
  cells : Vec<Vec<Cell>>,
}

impl Grid {
  pub fn new(m :usize, n :usize) -> Grid {
    Grid {
      cells : vec![vec![Cell::new(); n]; m], 
    }
  }
}