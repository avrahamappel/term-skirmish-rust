use rand::prelude::ThreadRng;

use crate::entities::{Entities, Entity, EntityBehavior};
use crate::helpers::*;
use crate::terminal::get_size;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Bullet {
    position: Position,
    prev_position: Position,
    direction: (i8, i8),
    active: bool,
    bullet_power: u16,
}

impl Bullet {
    pub fn new(pos: Position, direction: (i8, i8)) -> Bullet {
        Bullet {
            position: pos,
            prev_position: pos,
            direction,
            active: true,
            bullet_power: 1,
        }
    }

    fn move_self(&mut self) {
        self.prev_position = self.get_position();
        let x_pos = self.position.0 + self.direction.0 as u16;
        let y_pos = self.position.1 + self.direction.1 as u16;

        let (max_x, max_y) = get_size();
        if x_pos < 1 || x_pos > max_x || y_pos < 1 || y_pos > max_y {
            self.active = false;
        } else {
            self.position = Position(x_pos, y_pos);
        }
    }
}

impl EntityBehavior for Bullet {
    fn avatar(&self) -> &str {
        "🔸"
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_prev_position(&self) -> Position {
        self.prev_position
    }

    fn should_remove(&self) -> bool {
        !self.active
    }

    fn take_turn(mut self, _: &mut ThreadRng, _: &Entities) -> (Bullet, Option<Entity>) {
        if self.bullet_power == 1 {
            self.move_self();
            self.bullet_power = 0;
        } else {
            self.bullet_power = 1;
        }

        (self, None)
    }

    fn on_collide(mut self, _: &Entity) -> Bullet {
        self.active = false;
        self
    }

    fn on_remove_explode(&self) -> bool {
        false
    }
}
