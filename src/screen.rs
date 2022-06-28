use sdl2::{video::Window, render::Canvas, pixels::Color, rect::Rect, Sdl };
use crate::cell::Cell;

pub struct Screen {
  canvas : Canvas<Window>,
  scale : i32
}

impl Screen {
  pub fn new(cols : i32, rows : i32, scale : i32, sdl_init: &Sdl) -> Self{
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

    Self { canvas, scale }
  }

  pub fn render(&mut self, cell: &Cell) {
    let rect = Rect::new(
      cell.get_col() * self.scale, 
      cell.get_row() * self.scale,
      self.scale.try_into().unwrap(),
      self.scale.try_into().unwrap()
    );
    
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

  pub fn display(&mut self) {
    self.canvas.present();
  }
}
