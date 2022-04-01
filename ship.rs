use std::collections::HashMap;

use rand::distributions::Standard;
use rand::prelude::*;
use rand::Rng;

use crate::bullet::Bullet;
use crate::helpers::*;

enum Team {
    BLUE,
    RED,
    YELLOW,
    GREEN,
    ORANGE,
    BROWN,
    PURPLE,
    WHITE,
}

impl Distribution<Team> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Team {
        match rng.gen_range(0..8) {
            0 => Team::BLUE,
            1 => Team::RED,
            2 => Team::YELLOW,
            3 => Team::GREEN,
            4 => Team::ORANGE,
            5 => Team::BROWN,
            6 => Team::PURPLE,
            7 => Team::WHITE,
        }
    }
}

pub struct Ship {
    position: Position,
    prev_position: Position,
    destination: Position,
    alive: bool,
    move_power: i32,
    bullet_power: i32,
    team: Team,
    rng: ThreadRng,
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
            bullet_power: rand::thread_rng().gen_range(0..=10),
            team: t,
            rng: thread_rng(),
        }
    }

    fn shoot(self, entities: Entities) -> Option<Bullet> {
        if self.bulletPower != 15 {
            self.bulletPower = self.bulletPower + 1;

            return None;
        }

        self.bulletPower = 0;

        // wuss out
        if self.rand(2) == 0 {
            return None;
        }

        let ships = get_ships_from_entities(entities);

        if ships.len() == 0 {
            return None;
        }

        let seen = HashMap::new();

        loop {
            // no one to shoot at
            if seen.len() == ships.len() {
                break;
            }

            let ship = ships[self.rand(ships.len())];

            // already seen
            if seen[ship] {
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

            let x_dis = abs(self.getPosition()[0] - ship.getPosition()[0]);
            let y_dis = abs(self.getPosition()[1] - ship.getPosition()[1]);

            // no straight shot
            if x_dis != 0 && y_dis != 0 && x_dis - y_dis != 0 {
                seen[ship] = ();

                continue;
            }

            // now there must be a straight shot
            // make bullet and fire
            let x_pos = 0;
            let y_pos = 0;

            if self.getPosition()[0] > ship.getPosition()[0] {
                x_pos = -1
            } else if self.getPosition()[0] < ship.getPosition()[0] {
                x_pos = 1
            }

            if self.getPosition()[1] > ship.getPosition()[1] {
                y_pos = -1
            } else if self.getPosition()[1] < ship.getPosition()[1] {
                y_pos = 1
            }

            let pos = Position(self.position[0] + x_pos, self.position[1] + y_pos);
            let bullet = Bullet::new(pos, (x_pos, y_pos));

            return Some(&bullet);
        }

        return None;
    }

    fn move_ship(self, entities: Entities) {
        if self.movePower != 3 {
            self.movePower += 1;

            return;
        }

        self.movePower = 0;
        self.move_toward_destination();

        if self.has_reached_destination() {
            self.destination = self.get_destination(entities)
        }
    }

    fn get_destination(self, entities: Entities) -> Position {
        if self.rand(2) == 0 {
            return random_position();
        }

        for e in entities {
            let is_ship = e.type_id() == "Ship";
            if is_ship && e.team != self.team {
                return e.getPosition();
            }
        }

        return random_position();
    }

    fn has_reached_destination(self) -> bool {
        return positions_are_same(self.position, self.destination);
    }

    fn move_toward_destination(self) {
        if self.position[0] < self.destination[0] {
            self.move_right()
        } else if self.position[0] > self.destination[0] {
            self.move_left()
        }

        if self.position[1] < self.destination[1] {
            self.move_up()
        } else if self.position[1] > self.destination[1] {
            self.move_down()
        }
    }

    fn move_up(self) {
        self.prev_position[1] = self.position[1];
        self.position[1] += 1;
    }

    fn move_down(self) {
        self.prev_position[1] = self.position[1];
        self.position[1] -= 1;
    }

    fn move_right(self) {
        self.prev_position[0] = self.position[0];
        self.position[0] += 1;
    }

    fn move_left(self) {
        self.prev_position[0] = self.position[0];
        self.position[0] -= 1;
    }
}

impl Entity for Ship {
    fn avatar(self) -> String {
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

    fn get_position(self) -> Position {
        return self.position;
    }

    fn get_prev_position(self) -> Position {
        return self.prevPosition;
    }

    fn should_remove(self) -> bool {
        return !self.alive;
    }

    fn take_turn(self, entities: Entities) -> Entities {
        self.move_ship(entities);

        if let Some(bullet) = self.shoot(entities) {
            return vec![bullet];
        }

        vec![]
    }

    fn on_collide<E: Entity>(self, e: E) {
        if let ship @ Ship = e {
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

impl HasRng for Ship {}
