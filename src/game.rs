use sdl2::{video::Window, EventPump, render::Canvas, pixels::Color, rect::Rect, event::Event };
use crate::board::Board;

pub struct Game {
  canvas : Canvas<Window>,
  board : Board,
  event_pump : EventPump, 
  scale : i32
}

impl Game {
  pub fn new(cols : i32, rows : i32, scale : i32) -> Self{
    let sdl_init = sdl2::init().unwrap();
    let window: Window = sdl_init
      .video()
      .unwrap()
      .window(
        "Game of Life",
        (cols * scale).try_into().unwrap(),
        (rows * scale).try_into().unwrap(),)
      .position_centered()
      .build()
      .unwrap();

    let canvas = window.into_canvas().build().unwrap();
        
    let board = Board::new(cols, rows);

    let event_pump = sdl_init.event_pump().unwrap();

    Self { canvas, board, event_pump, scale }
  }

  pub fn start(&mut self) {
    
    let mut running = true;

    while running {
      self.canvas.set_draw_color(Color::RGB(0, 0, 0));
      self.canvas.clear();

      let mut rect = Rect::new(
        0, 0,
        self.scale.try_into().unwrap(),
        self.scale.try_into().unwrap()
      );

      for sub_vec in self.board.old_table.iter() {
        for cell in sub_vec {
          rect.set_x(cell.get_col() * self.scale);
          rect.set_y(cell.get_row() * self.scale);
          
          if cell.get_state() {
            self.canvas.set_draw_color(Color::RGB(0, 255, 0));
          } else {
            self.canvas.set_draw_color(Color::RGB(255, 0, 0));
          }
  
          let result = self.canvas.fill_rect(rect);
          if result.is_err() {
            print!("{}", sdl2::get_error());
          }
        }
      }

      self.canvas.present();

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit {..} => {
            running = false;
          },
          _ => {}
        }
      }

      self.board.generate_next();
      // sleep(Duration::from_millis(300));
    }

  }
}