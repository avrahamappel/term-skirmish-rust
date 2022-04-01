// import (
// 	"flag"
// 	"os"
// 	"os/signal"
// )

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use rand::{prelude::*, thread_rng};

use crate::explosion::Explosion;
use crate::helpers::*;
use crate::ship::Ship;
use crate::terminal::*;

pub struct Game {
    over: bool,
    num_teams: i32,
    max_ships_per_wave: i32,
    entities: Entities,
    ship_count: i32,
    rng: ThreadRng,
}

impl HasRng for Game {}

impl Game {
    pub fn new() -> Game {
        let default_team_count = 2;
        let default_max_wave_count = 8;

        let num_teams = 0;
        let max_ships_per_wave = 0;

        // flag.IntVar(&numTeams, "teams", defaultTeamCount, "number of teams (1-8)")
        // flag.IntVar(
        // 	&maxShipsPerWave,
        // 	"wave",
        // 	defaultMaxWaveCount,
        // 	"maximum number of ships in each reinforcement wave (1-100)",
        // )

        // flag.Parse()

        if num_teams < 1 || num_teams > 8 {
            num_teams = 2
        }

        if max_ships_per_wave < 1 || max_ships_per_wave > 100 {
            max_ships_per_wave = 20
        }

        Game {
            over: false,
            num_teams,
            max_ships_per_wave,
            entities: Vec::new(),
            ship_count: 0,
            rng: thread_rng(),
        }
    }

    pub fn before_game(self) {
        // // rand.Seed(time.Now().UnixNano())

        hide_cursor();

        // c := make(chan os.Signal, 1)
        // signal.Notify(c, os.Interrupt)

        // go fn() {
        // 	for range c {
        // 		self.over = true
        // 	}
        // }()

        // initial wave
        self.reinforce()
    }

    fn reinforce(self) {
        let team = self.rand(self.numTeams);
        let ship_count = self.rand(self.maxShipsPerWave) + 1;

        for _ in 0..=ship_count {
            let ship = Ship::new(team);
            self.entities.append(&ship);

            self.ship_count = self.ship_count + 1;
        }
    }

    pub fn run_game(self) {
        while !self.over {
            clear();

            let new_entities = self.take_turns();

            self.check_collisions();
            self.remove_entities();
            self.draw_game();

            // 60 fps
            thread::sleep(Duration::new(1, 0).as_millis() / 60);

            self.entities.append(&mut new_entities);

            // 0.5% chance of reinforcements
            if self.rand(200) == 0 {
                self.reinforce();
            }
        }
    }

    fn take_turns(self) -> Vec<impl Entity> {
        let new_entities = Vec::new();

        for entity in self.entities {
            let es = entity.takeTurn(self.entities);
            new_entities.append(es);
        }

        new_entities
    }

    fn check_collisions(self) {
        let collided_entities = HashMap::new();

        for entity in self.entities {
            if collided_entities.contains_key(entity) && entity {
                continue;
            }

            for otherEntity in self.entities {
                if entity == otherEntity {
                    continue;
                }

                if collided(entity, otherEntity) {
                    entity.onCollide(otherEntity);

                    collided_entities[entity] = ();
                }
            }
        }
    }

    fn remove_entities(self) {
        let remaining_entities = Vec::new();

        for entity in self.entities {
            if !entity.shouldRemove() {
                remaining_entities.append(entity);

                continue;
            }

            if entity.onRemoveExplode() {
                let explosion = Explosion::new(entity.getPosition());
                remaining_entities.append(&explosion)
            }
        }

        self.entities = remaining_entities
    }

    fn draw_game(self) {
        for entity in self.entities {
            move_cursor(entity.getPosition());
            draw(entity.avatar());
        }

        let (width, _) = get_size();

        let status = self.get_status();
        move_cursor(Position(width / 2 - (status.len() / 2), 0));

        draw(status);

        render();
    }

    fn get_status(self) -> String {
        let current_ship_count = count_ships(self.entities);

        let message = format!(
            "current ship count: {}     destroyed count: {}",
            count_ships(self.entities),
            self.shipCount - current_ship_count,
        );

        message
    }

    pub fn after_game(self) {
        clear();
        show_cursor();

        move_cursor(Position(1, 1));
        draw(" 🔵  🔸 🔸 🔥");

        move_cursor(Position(0, 3));

        draw("See you again soon! 🦀");

        move_cursor(Position(0, 5));

        render();
        // os.Exit(0)
    }
}
