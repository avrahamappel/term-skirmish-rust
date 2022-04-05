use crate::entities::{Entities, Entity, EntityBehavior};
use crate::helpers::*;
use crate::terminal::get_size;

#[derive(PartialEq, Eq, Hash)]
pub struct Bullet {
    position: Position,
    prev_position: Position,
    direction: (i8, i8),
    active: bool,
    bullet_power: u16,
}

impl Bullet {
    pub fn new(pos: Position, direction: (i8, i8)) -> Bullet {
        return Bullet {
            position: pos,
            prev_position: pos,
            direction,
            active: true,
            bullet_power: 1,
        };
    }

    fn move_self(&mut self) {
        self.prev_position = self.get_position();
        let x_pos = self.position.0 + self.direction.0 as u16;
        let y_pos = self.position.1 + self.direction.1 as u16;

        let (maxX, maxY) = get_size();
        if x_pos < 1 || x_pos > maxX || y_pos < 1 || y_pos > maxY {
            self.active = false;
        } else {
            self.position = Position(x_pos, y_pos);
        }
    }
}

impl EntityBehavior for Bullet {
    fn avatar(&self) -> &str {
        return "🔸";
    }

    fn get_position(&self) -> Position {
        return self.position;
    }

    fn get_prev_position(&self) -> Position {
        return self.prev_position;
    }

    fn should_remove(&self) -> bool {
        return !self.active;
    }

    fn take_turn(&mut self, _: &Entities) -> Entities {
        if self.bullet_power == 1 {
            self.move_self();
            self.bullet_power = 0;
        } else {
            self.bullet_power = 1;
        }

        vec![]
    }

    fn on_collide(&mut self, _: &Entity) {
        self.active = false
    }

    fn on_remove_explode(&self) -> bool {
        return false;
    }
}
