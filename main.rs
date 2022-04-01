mod bullet;
mod explosion;
mod game;
mod helpers;
mod ship;
mod terminal;

use game::Game;

fn main() {
    run()
}

fn run() {
    let g = Game::new();
    g.before_game();
    g.run_game();
    g.after_game();
}
