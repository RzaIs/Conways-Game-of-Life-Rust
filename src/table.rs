use crate::cell::Cell;

pub struct Table {
  table: Vec<Vec<Cell>>
}

impl Table {
  pub fn new(cols : i32, rows : i32) -> Self {
    let mut table : Vec<Vec<Cell>> = Vec::new();

    for i in 0..cols {
      let mut sub_vec: Vec<Cell> = Vec::new();
      for j in 0..rows {
        let cell = Cell::new(i, j, false);
        sub_vec.push(cell);
      }
      table.push(sub_vec);
    }

    Self { table }
  }

  pub fn get(&self, col : i32, row : i32) -> &Cell {

    let c: usize = col.try_into().unwrap();
    let r: usize = row.try_into().unwrap();

    return self.table
      .get(c)
      .unwrap()
      .get(r)
      .unwrap();
  }

  pub fn set(&mut self, col : i32, row : i32, state : bool) {

    let c: usize = col.try_into().unwrap();
    let r: usize = row.try_into().unwrap();

    let cell = self.table
      .get_mut(c)
      .unwrap()
      .get_mut(r)
      .unwrap();

    cell.set_state(state);
  }

  pub fn mut_iter(&mut self) -> &mut Vec<Vec<Cell>> {
    return &mut self.table;
  }

  pub fn iter(&self) -> &Vec<Vec<Cell>> {
    return &self.table;
  }
}