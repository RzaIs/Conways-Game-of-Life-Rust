use game::Game;
mod cell;
mod point;
mod board;
mod table;
mod game;

fn main() {

    let mut game = Game::new(240, 120, 4);

    game.start();
}
