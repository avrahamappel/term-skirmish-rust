use crate::helpers::*;
use crate::terminal::get_size;

pub struct Bullet {
    position: Position,
    prev_position: Position,
    direction: (i32, i32),
    active: bool,
    bullet_power: i32,
}

impl Bullet {
    pub fn new(pos: Position, direction: (i32, i32)) -> Bullet {
        return Bullet {
            position: pos,
            prev_position: pos,
            direction,
            active: true,
            bullet_power: 1,
        };
    }

    fn move_self(self) {
        self.prev_position = self.getPosition();
        let x_pos = self.position[0] + self.direction[0];
        let y_pos = self.position[1] + self.direction[1];

        let (maxX, maxY) = get_size();
        if x_pos < 1 || x_pos > maxX || y_pos < 1 || y_pos > maxY {
            self.active = false;

            return;
        }

        self.position = Position(x_pos, y_pos)
    }
}

impl Entity for Bullet {
    fn avatar(self) -> String {
        return "🔸";
    }

    fn get_position(self) -> Position {
        return self.position;
    }

    fn get_prev_position(self) -> Position {
        return self.prev_position;
    }

    fn should_remove(self) -> bool {
        return !self.active;
    }

    fn take_turn(self, entities: Entities) -> Entities {
        if self.bullet_power == 1 {
            self.moveSelf();
            self.bullet_power = 0;

            return vec![];
        }

        self.bullet_power = 1;

        return vec![];
    }

    fn on_collide<E: Entity>(self, e: E) {
        self.active = false
    }

    fn on_remove_explode(self) -> bool {
        return false;
    }
}
