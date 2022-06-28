use sdl2::{ EventPump, event::Event };
use crate::{board::Board, screen::Screen};

pub struct Game {
  board: Board,
  screen: Screen,
  event_pump : EventPump,
}

impl Game {
  pub fn new(cols : i32, rows : i32, scale : i32) -> Self{
    let sdl_init = sdl2::init().unwrap();
        
    let board = Board::new(cols, rows);

    let screen = Screen::new(cols, rows, scale, &sdl_init); 

    let event_pump = sdl_init.event_pump().unwrap();

    Self { board, screen, event_pump }
  }

  pub fn start(&mut self) {
    
    let mut running = true;

    while running {

      for sub_vec in self.board.old_table.iter() {
        for cell in sub_vec {
          self.screen.render(cell);
        }
      }

      self.screen.display();

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit {..} => {
            running = false;
          },
          _ => {}
        }
      }

      self.board.generate_next();
    }

  }
}