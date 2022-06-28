use game::Game;
mod cell;
mod point;
mod board;
mod table;
mod screen;
mod game;

fn main() {

    let mut game = Game::new(640, 360, 2);

    game.start();
}
