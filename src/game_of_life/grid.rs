use super::cell::Cell;
use super::cell::States;
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Grid {
  cells : Vec<Vec<Cell>>,
  gen : u32,
}

impl Grid {
  pub fn new(m :usize, n :usize) -> Grid {
    Grid {
      cells : vec![vec![Cell::new(); n]; m], 
      gen : 0,
    }
  }
  pub fn change_cell_state(&mut self, i :usize, j :usize) {
    if self.cells[i][j].state == States::Dead {
      self.cells[i][j].state = States::Alive;
    } else {
      self.cells[i][j].state = States::Dead;
    }
  }
  pub fn generate_new_gen(&mut self) {
    if self.cells.len() < 3 || self.cells[0].len() < 3 {
      return;
    }
    self.calculate_new_states();
    self.update();
    self.gen += 1;
  }
  fn calculate_new_states(&mut self) {
    for i in 1..self.cells.len() - 2 {
      for j in 1..self.cells[i].len() - 2 {
        self.calculate_state(i, j);
      }
    }
  }
  fn calculate_state(&mut self, i :usize, j :usize) {
    let n_neighbours = self.count_neighbours(i, j);
    if self.cells[i][j].state == States::Alive && !(n_neighbours == 2 || n_neighbours == 3) {
      self.cells[i][j].state = States::Dead;
    } else if self.cells[i][j].state == States::Dead && n_neighbours == 3 {
      self.cells[i][j].state = States::Alive;
    }
  }
  fn count_neighbours(&mut self, i :usize, j :usize) -> u32 {
    let mut count = 0;
    for k in i - 1..i + 1 {
      for t in j - 1..j + 1 {
        if k == t {
          continue;
        }
        if self.cells[k][t].state == States::Alive {
          count +=1;
        }
      }
    }
    count
  }
  fn update(&mut self) {
    for i in self.cells.iter_mut() {
      for j in i.iter_mut() {
        j.update_state()
      }
    }
  }
}

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for i in self.cells.iter() {
      for j in i.iter() {
        write!(f, "{}", j)?
      }
      write!(f, "\n")?
    }
      write!(f, "Generation {}\n", self.gen)
  }
}

#[cfg(test)]
mod tests {
  use super::Grid;
  #[test]
  fn check_update_grid() {
    let mut grid = Grid::new(5, 5); 
    grid.change_cell_state(3, 3);
    grid.change_cell_state(3, 4);
    grid.change_cell_state(4, 4);
    grid.generate_new_gen();

    //let mut grid2 = Grid::new(5, 5); 
    //grid2.change_cell_state(3, 3);
    //grid2.change_cell_state(3, 4);
    //grid2.change_cell_state(4, 4);
    //grid2.change_cell_state(3, 3);

    //assert_eq!(grid, grid2);
  }
}