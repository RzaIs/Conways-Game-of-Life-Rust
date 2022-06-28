use crate::cell::Cell;
use crate::point::Point;
use crate::table::Table;
use rand::Rng;

pub struct Board {
  pub cols : i32,
  pub rows : i32,
  pub old_table : Table,
  pub new_table : Table,
}

impl Board {
  pub fn new(cols : i32, rows : i32) -> Self {
    let mut old_table = Table::new(cols, rows);

    Self::random_fill(&mut old_table);

    Self { cols, rows, old_table, new_table: Table::new(cols, rows) }
  }

  fn random_fill(table : &mut Table) {
    let mut rng = rand::thread_rng();
    
    for sub_vec in table.mut_iter() {
      for cell in sub_vec {
        cell.set_state(rng.gen_bool(0.5));
      }
    }
  }

  pub fn cell_state_at(&self, col : i32, row : i32) -> bool {
    return self.old_table.get(col, row).get_state();
  }

  fn does_belong(&self, col : i32, row : i32) -> bool {
    return col > 0 && col < self.cols && row > 0 && row < self.rows;
  }

  fn count_alive_neighbor(&self, col : i32, row : i32) -> i32 {

    let positions : [ Point; 8] = [
      Point::new(col + 1, row + 1),
      Point::new(col + 1, row - 1),
      Point::new(col - 1, row + 1),
      Point::new(col - 1, row - 1),
      Point::new(col + 1, row),
      Point::new(col - 1, row),
      Point::new(col, row + 1),
      Point::new(col, row - 1)
    ];

    let mut count = 0;

    for point in positions {
      if self.does_belong(point.x, point.y) {
        if self.cell_state_at(point.x, point.y) {
          count += 1;
        }
      }
    }

    return count;
  }

  fn new_cell_state(&self, cell : &Cell) -> bool {
    let nb_of_alive_nbrs = self.count_alive_neighbor(
      cell.get_col(), 
      cell.get_row()
    );

    if cell.get_state() {
      if nb_of_alive_nbrs == 2 || nb_of_alive_nbrs == 3 {
        return true;
      }
      else {
        return false;
      }
    } else {
      if nb_of_alive_nbrs == 3 {
        return true;
      }
      else {
        return false;
      }
    }
  }

  fn copy_table(&mut self) {
    for sub_vec in self.new_table.mut_iter() {
      for cell in sub_vec {
        self.old_table.set(
          cell.get_col(),
          cell.get_row(),
          cell.get_state()
        );
      }
    } 
  }

  pub fn generate_next(&mut self) {

    for sub_vec in self.old_table.iter() {
      for cell in sub_vec {
        self.new_table.set(
          cell.get_col(),
          cell.get_row(),
          self.new_cell_state(cell)
        );
      }
    }
    self.copy_table();
  }
}