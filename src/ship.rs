use std::collections::HashMap;

use rand::prelude::*;

use crate::bullet::Bullet;
use crate::entities::{Entities, Entity, EntityBehavior};
use crate::helpers::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Team {
    BLUE,
    RED,
    YELLOW,
    GREEN,
    ORANGE,
    BROWN,
    PURPLE,
    WHITE,
}

impl Team {
    pub fn from_rand(i: u16) -> Team {
        match i {
            0 => Team::BLUE,
            1 => Team::RED,
            2 => Team::YELLOW,
            3 => Team::GREEN,
            4 => Team::ORANGE,
            5 => Team::BROWN,
            6 => Team::PURPLE,
            _ => Team::WHITE,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ship {
    position: Position,
    prev_position: Position,
    destination: Position,
    alive: bool,
    move_power: u16,
    bullet_power: u16,
    team: Team,
}

impl Ship {
    pub fn new(t: Team) -> Ship {
        let wall_pos = wall_position();

        Ship {
            position: wall_pos,
            prev_position: wall_pos,
            destination: random_position(),
            alive: true,
            move_power: 3,
            bullet_power: thread_rng().gen_range(0..=10),
            team: t,
        }
    }

    fn shoot(&mut self, rng: &mut ThreadRng, entities: &Entities) -> Option<Bullet> {
        if self.bullet_power != 15 {
            self.bullet_power = self.bullet_power + 1;

            return None;
        }

        self.bullet_power = 0;

        // wuss out
        if rng.gen_bool(0.5) {
            return None;
        }

        let ships = get_ships_from_entities(&entities);

        if ships.len() == 0 {
            return None;
        }

        let mut seen = HashMap::new();

        loop {
            // no one to shoot at
            if seen.len() == ships.len() {
                break;
            }

            let ship = ships.choose(rng).unwrap();

            // already seen
            if seen.contains_key(ship) {
                continue;
            }

            // same team
            if self.team == ship.team {
                seen.insert(ship, ());

                continue;
            }

            if positions_are_same(self.position, ship.position) {
                seen.insert(ship, ());

                continue;
            }

            let x_dis = self.get_position().0 - ship.get_position().0;
            let y_dis = self.get_position().1 - ship.get_position().1;

            // no straight shot
            if x_dis != 0 && y_dis != 0 && x_dis - y_dis != 0 {
                seen.insert(ship, ());

                continue;
            }

            // now there must be a straight shot
            // make bullet and fire
            let mut x_pos: i8 = 0;
            let mut y_pos: i8 = 0;

            if self.get_position().0 > ship.get_position().0 {
                x_pos = -1
            } else if self.get_position().0 < ship.get_position().0 {
                x_pos = 1
            }

            if self.get_position().1 > ship.get_position().1 {
                y_pos = -1
            } else if self.get_position().1 < ship.get_position().1 {
                y_pos = 1
            }

            let pos = Position(
                self.position.0 + x_pos as u16,
                self.position.1 + y_pos as u16,
            );
            let bullet = Bullet::new(pos, (x_pos, y_pos));

            return Some(bullet);
        }

        return None;
    }

    fn move_ship(&mut self, rng: &mut ThreadRng, entities: &Entities) {
        if self.move_power != 3 {
            self.move_power += 1;

            return;
        }

        self.move_power = 0;
        self.move_toward_destination();

        if self.has_reached_destination() {
            self.destination = self.get_destination(rng, entities)
        }
    }

    fn get_destination(&self, rng: &mut ThreadRng, entities: &Entities) -> Position {
        if rng.gen_bool(0.5) {
            return random_position();
        }

        for e in entities {
            match e {
                Entity::Ship(ship) if ship.team != self.team => {
                    return ship.get_position();
                }
                _ => continue,
            }
        }

        return random_position();
    }

    fn has_reached_destination(&self) -> bool {
        return positions_are_same(self.position, self.destination);
    }

    fn move_toward_destination(&mut self) {
        if self.position.0 < self.destination.0 {
            self.move_right()
        } else if self.position.0 > self.destination.0 {
            self.move_left()
        }

        if self.position.1 < self.destination.1 {
            self.move_up()
        } else if self.position.1 > self.destination.1 {
            self.move_down()
        }
    }

    fn move_up(&mut self) {
        self.prev_position.1 = self.position.1;
        self.position.1 += 1;
    }

    fn move_down(&mut self) {
        self.prev_position.1 = self.position.1;
        self.position.1 -= 1;
    }

    fn move_right(&mut self) {
        self.prev_position.0 = self.position.0;
        self.position.0 += 1;
    }

    fn move_left(&mut self) {
        self.prev_position.0 = self.position.0;
        self.position.0 -= 1;
    }
}

impl EntityBehavior for Ship {
    fn avatar(&self) -> &str {
        match &self.team {
            Team::BLUE => "🔵",
            Team::BROWN => "🟤",
            Team::GREEN => "🟢",
            Team::ORANGE => "🟠",
            Team::PURPLE => "🟣",
            Team::RED => "🔴",
            Team::WHITE => "⚪",
            Team::YELLOW => "🟡",
        }
    }

    fn get_position(&self) -> Position {
        return self.position;
    }

    fn get_prev_position(&self) -> Position {
        return self.prev_position;
    }

    fn should_remove(&self) -> bool {
        return !self.alive;
    }

    fn take_turn(mut self, rng: &mut ThreadRng, entities: &Entities) -> (Ship, Option<Entity>) {
        self.move_ship(rng, entities);

        match self.shoot(rng, entities) {
            Some(bullet) => (self, Some(Entity::Bullet(bullet))),
            None => (self, None),
        }
    }

    fn on_collide(mut self, e: &Entity) -> Ship {
        match e {
            Entity::Ship(ship) if ship.team != self.team => self.alive = false,
            Entity::Bullet(_) => self.alive = false,
            _ => (),
        }

        self
    }

    fn on_remove_explode(&self) -> bool {
        return true;
    }
}
