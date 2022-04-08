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
    new_entities: Entities,
    ship_count: u16,
    rng: ThreadRng,
}

impl Game {
    pub fn new(team_count: u16, max_wave_count: u16) -> Game {
        let num_teams = if !(1..=8).contains(&team_count) {
            2
        } else {
            team_count
        };

        let max_ships_per_wave = if !(1..=100).contains(&max_wave_count) {
            20
        } else {
            max_wave_count
        };

        Game {
            over: false,
            num_teams,
            max_ships_per_wave,
            entities: Vec::with_capacity((num_teams * max_ships_per_wave).into()),
            new_entities: Vec::new(),
            ship_count: 0,
            rng: thread_rng(),
        }
    }

    fn before_game(self) -> Game {
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
        let team = Team::from_rand(self.rng.gen_range(0..self.num_teams).into());
        let ship_count = self.rng.gen_range(0..self.max_ships_per_wave) + 1;

        for _ in 0..=ship_count {
            let ship = Entity::ship(team);
            self.entities.push(ship);

            self.ship_count += 1;
        }

        self
    }

    pub fn run_game(mut self) {
        self = self.before_game();

        while !&self.over {
            clear();

            self = self.take_turns().check_collisions().remove_entities();

            self.draw_game();

            // 60 fps
            thread::sleep(Duration::from_millis(1000 / 60));

            self.append_new_entities();

            // 0.5% chance of reinforcements
            if self.rng.gen_range(0..200) == 0 {
                self = self.reinforce();
            }
        }

        self.after_game()
    }

    fn take_turns(mut self) -> Self {
        let (entities, new_entity_options): (Vec<_>, Vec<_>) = self
            .entities
            .iter()
            .map(|entity| entity.clone().take_turn(&mut self.rng, &self.entities))
            .unzip();

        self.entities = entities;
        self.new_entities = new_entity_options.into_iter().flatten().collect::<Vec<_>>();

        self
    }

    fn check_collisions(mut self) -> Self {
        self.entities = self
            .entities
            .iter()
            .map(|entity| {
                self.entities
                    .iter()
                    .find(|other| entity != *other && collided(entity, *other))
                    .map(|other| entity.clone().on_collide(other))
                    .unwrap_or_else(|| entity.clone())
            })
            .collect();

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

    fn append_new_entities(&mut self) {
        self.entities.append(&mut self.new_entities);
    }

    fn draw_game(&self) {
        for entity in &self.entities {
            move_cursor(entity.get_position());
            draw(entity.avatar());
        }

        let (width, _) = get_size();

        let status = self.get_status();
        move_cursor(Position((width / 2 - (status.len() as u16 / 2)).into(), 0));

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

    fn after_game(self) {
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
