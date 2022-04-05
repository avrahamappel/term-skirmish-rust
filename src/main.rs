mod bullet;
mod entities;
mod explosion;
mod game;
mod helpers;
mod ship;
mod terminal;

use clap::Parser;
use game::Game;

/// A skirmish in your terminal
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of teams (1-8)
    #[clap(short, long, default_value_t = 2)]
    teams: u16,

    /// Maximum number of ships in each reinforcement wave (1-100)
    #[clap(short, long, default_value_t = 8)]
    wave: u16,
}

fn main() {
    run(Args::parse())
}

fn run(args: Args) {
    Game::new(args.teams, args.wave)
        .before_game()
        .run_game()
        .after_game();
}
