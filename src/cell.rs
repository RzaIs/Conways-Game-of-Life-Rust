pub struct Cell {
  col : i32,
  row : i32,
  state : bool
}

impl Cell {
    pub fn new(col : i32, row : i32, state : bool) -> Self {
      Self { col, row, state }
    }

    pub fn get_col(&self) -> i32 {
      return self.col;
    }

    pub fn get_row(&self) -> i32 {
      return self.row;
    }

    pub fn get_state(&self) -> bool {
      return self.state;
    }

    pub fn set_state(&mut self, state : bool) {
      self.state = state;
    }
}