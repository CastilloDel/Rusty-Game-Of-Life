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

#[cfg(test)]
mod tests {
  use super::Grid;
  #[test]
  fn check_update_grid() {
    let grid = Grid::new(5, 5); 
    grid.change_cell_state(3, 3);
    grid.change_cell_state(3, 4);
    grid.change_cell_state(4, 4);
    grid.update();

    let grid2 = Grid::new(5, 5); 
    grid2.change_cell_state(3, 3);
    grid2.change_cell_state(3, 4);
    grid2.change_cell_state(4, 4);
    grid2.change_cell_state(3, 3);

    assert_eq!(grid, grid2);
  }
}