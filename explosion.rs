use crate::helpers::*;

pub struct Explosion {
    position: Position,
    health: i32,
}

impl Explosion {
    pub fn new(pos: Position) -> Explosion {
        return Explosion {
            position: pos,
            health: 10,
        };
    }
}

impl Entity for Explosion {
    fn get_position(self) -> Position {
        return self.position;
    }

    fn get_prev_position(self) -> Position {
        return self.position;
    }

    fn should_remove(self) -> bool {
        return self.health == 0;
    }

    fn avatar(self) -> String {
        return "💥";
    }

    fn take_turn(self, entities: Entities) -> Entities {
        if self.health > 0 {
            self.health -= 1;
        }

        entities
    }

    fn on_collide(self, other_entity: dyn Entity) {
        match other_entity {
            other_explosion @ Explosion => {
                if self.health > other_explosion.health {
                    self.health += 100;
                    other_explosion.health = 0;
                } else {
                    other_explosion.health += 100;
                    self.health = 0;
                }
            }
            _ => self.health += 100,
        }
    }

    fn on_remove_explode(self) -> bool {
        return false;
    }
}
