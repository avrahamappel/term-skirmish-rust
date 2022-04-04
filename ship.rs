use std::collections::HashMap;

use rand::prelude::*;
use rand::Rng;

use crate::bullet::Bullet;
use crate::entities::{Entities, Entity, EntityBehavior};
use crate::helpers::*;

#[derive(PartialEq, Eq, Hash)]
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

#[derive(PartialEq, Eq, Hash)]
pub struct Ship {
    position: Position,
    prev_position: Position,
    destination: Position,
    alive: bool,
    move_power: u16,
    bullet_power: u16,
    team: Team,
}

const rng: ThreadRng = thread_rng();

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

    fn shoot(self, entities: Entities) -> Option<Bullet> {
        if self.bullet_power != 15 {
            self.bullet_power = self.bullet_power + 1;

            return None;
        }

        self.bullet_power = 0;

        // wuss out
        if rng.gen_bool(0.5) {
            return None;
        }

        let ships = get_ships_from_entities(entities);

        if ships.len() == 0 {
            return None;
        }

        let seen: HashMap<Ship, ()> = HashMap::new();

        loop {
            // no one to shoot at
            if seen.len() == ships.len() {
                break;
            }

            let ship = ships.choose(&mut rng).unwrap();

            // already seen
            if seen.contains_key(ship) {
                continue;
            }

            // same team
            if self.team == ship.team {
                seen[ship] = ();

                continue;
            }

            if positions_are_same(self.position, ship.position) {
                seen[ship] = ();

                continue;
            }

            let x_dis = self.get_position().0 - ship.get_position().0;
            let y_dis = self.get_position().1 - ship.get_position().1;

            // no straight shot
            if x_dis != 0 && y_dis != 0 && x_dis - y_dis != 0 {
                seen[ship] = ();

                continue;
            }

            // now there must be a straight shot
            // make bullet and fire
            let x_pos: i8 = 0;
            let y_pos: i8 = 0;

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

    fn move_ship(self, entities: Entities) {
        if self.move_power != 3 {
            self.move_power += 1;

            return;
        }

        self.move_power = 0;
        self.move_toward_destination();

        if self.has_reached_destination() {
            self.destination = self.get_destination(entities)
        }
    }

    fn get_destination(self, entities: Entities) -> Position {
        if rng.gen_bool(0.5) {
            return random_position();
        }

        for e in entities {
            match e {
                Entity::Ship(ship) if ship.team != self.team => {
                    return e.get_position();
                }
                _ => continue,
            }
        }

        return random_position();
    }

    fn has_reached_destination(self) -> bool {
        return positions_are_same(self.position, self.destination);
    }

    fn move_toward_destination(self) {
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

    fn move_up(self) {
        self.prev_position.1 = self.position.1;
        self.position.1 += 1;
    }

    fn move_down(self) {
        self.prev_position.1 = self.position.1;
        self.position.1 -= 1;
    }

    fn move_right(self) {
        self.prev_position.0 = self.position.0;
        self.position.0 += 1;
    }

    fn move_left(self) {
        self.prev_position.0 = self.position.0;
        self.position.0 -= 1;
    }
}

impl EntityBehavior for Ship {
    fn avatar(&self) -> &str {
        match self.team {
            BLUE => "🔵",
            BROWN => "🟤",
            GREEN => "🟢",
            ORANGE => "🟠",
            PURPLE => "🟣",
            RED => "🔴",
            WHITE => "⚪",
            YELLOW => "🟡",
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

    fn take_turn(self, entities: Entities) -> Entities {
        self.move_ship(entities);

        if let Some(bullet) = self.shoot(entities) {
            return vec![Entity::Bullet(bullet)];
        }

        vec![]
    }

    fn on_collide(self, e: Entity) {
        if let Entity::Ship(ship) = e {
            // don't explode colliding with same team
            if ship.team != self.team {
                self.alive = false
            }
        }
    }

    fn on_remove_explode(self) -> bool {
        return true;
    }
}
