// import (
// 	"flag"
// 	"os"
// 	"os/signal"
// )

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use rand::{prelude::*, thread_rng};

use crate::entities::{Entities, Entity, EntityBehavior};
use crate::helpers::*;
use crate::ship::Team;
use crate::terminal::*;

pub struct Game {
    over: bool,
    num_teams: u16,
    max_ships_per_wave: u16,
    entities: Entities,
    ship_count: u16,
    rng: ThreadRng,
}

impl Game {
    pub fn new(team_count: u16, max_wave_count: u16) -> Game {
        let num_teams = if team_count < 1 || team_count > 8 {
            2
        } else {
            team_count
        };

        let max_ships_per_wave = if max_wave_count < 1 || max_wave_count > 100 {
            20
        } else {
            max_wave_count
        };

        Game {
            over: false,
            num_teams,
            max_ships_per_wave,
            entities: Vec::new(),
            ship_count: 0,
            rng: thread_rng(),
        }
    }

    pub fn before_game(self) -> Game {
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

    fn reinforce(mut self) -> Game {
        let team = Team::from_rand(self.rng.gen_range(0..self.num_teams));
        let ship_count = self.rng.gen_range(0..self.max_ships_per_wave) + 1;

        for _ in 0..=ship_count {
            let ship = Entity::ship(team);
            self.entities.push(ship);

            self.ship_count += 1;
        }

        self
    }

    pub fn run_game(mut self) -> Game {
        while !&self.over {
            clear();

            let mut new_entities = self.take_turns();

            self = self.check_collisions();
            self = self.remove_entities();
            self.draw_game();

            // 60 fps
            thread::sleep(Duration::from_millis(1000 / 60));

            self.entities.append(&mut new_entities);

            // 0.5% chance of reinforcements
            if self.rng.gen_range(0..200) == 0 {
                self = self.reinforce();
            }
        }

        self
    }

    fn take_turns(&mut self) -> Entities {
        self.entities
            .iter_mut()
            .flat_map(|&mut entity| entity.take_turn(&self.entities))
            .collect()
    }

    fn check_collisions(mut self) -> Game {
        let mut collided_entities: HashMap<&Entity, ()> = HashMap::new();

        for entity in &mut self.entities {
            if collided_entities.contains_key(entity) {
                continue;
            }

            for otherEntity in &self.entities {
                if entity == otherEntity {
                    continue;
                }

                if collided(entity, otherEntity) {
                    entity.on_collide(otherEntity);

                    collided_entities.insert(entity, ());
                }
            }
        }

        self
    }

    fn remove_entities(mut self) -> Game {
        let mut remaining_entities = Vec::new();

        for entity in self.entities {
            if !entity.should_remove() {
                remaining_entities.push(entity);

                continue;
            }

            if entity.on_remove_explode() {
                let explosion = Entity::explosion(entity.get_position());
                remaining_entities.push(explosion)
            }
        }

        self.entities = remaining_entities;

        self
    }

    fn draw_game(&self) {
        for entity in &self.entities {
            move_cursor(entity.get_position());
            draw(entity.avatar());
        }

        let (width, _) = get_size();

        let status = self.get_status();
        move_cursor(Position(width / 2 - (status.len() as u16 / 2), 0));

        draw(&status);

        render();
    }

    fn get_status(&self) -> String {
        let current_ship_count = count_ships(&self.entities);

        let message = format!(
            "current ship count: {}     destroyed count: {}",
            count_ships(&self.entities),
            self.ship_count - current_ship_count,
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
