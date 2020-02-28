use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum States {
  Alive,
  Dead,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
  pub state :States,
  pub next_state :States,
}

impl Cell {
  pub fn new() -> Cell {
    Cell {
      state: States::Dead,
      next_state: States::Dead}
  }
  pub fn update_state(&mut self) {
    self.state = self.next_state;
  }
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.state == States::Dead {
      write!(f, "  ")
    } else {
      write!(f, "X ")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{States, Cell};
  #[test]
  fn create_cell() {
    let mut a = Cell::new();
    let b = Cell{state: States::Alive,
      next_state: States::Alive,
    };
    a.next_state = States::Alive;
    a.update_state();
    assert_eq!(a, b);
  }
}