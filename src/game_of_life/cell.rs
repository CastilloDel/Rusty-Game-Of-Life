#[derive(Clone)]
pub enum States {
  Alive,
  Dead,
}

#[derive(Clone)]
pub struct Cell {
  state :States,
}

impl Cell {
  pub fn new() -> Cell {
    Cell {state: States::Dead}
  }
}