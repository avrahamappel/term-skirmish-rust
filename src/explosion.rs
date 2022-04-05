use crate::entities::{Entities, Entity, EntityBehavior};
use crate::helpers::*;

#[derive(PartialEq, Eq, Hash)]
pub struct Explosion {
    position: Position,
    health: u16,
}

impl Explosion {
    pub fn new(pos: Position) -> Explosion {
        return Explosion {
            position: pos,
            health: 10,
        };
    }
}

impl EntityBehavior for Explosion {
    fn get_position(&self) -> Position {
        return self.position;
    }

    fn get_prev_position(&self) -> Position {
        return self.position;
    }

    fn should_remove(&self) -> bool {
        return self.health == 0;
    }

    fn avatar(&self) -> &str {
        return "💥";
    }

    fn take_turn(&mut self, _: &Entities) -> Entities {
        if self.health > 0 {
            self.health -= 1;
        }

        vec![]
    }

    fn on_collide(&mut self, other_entity: &Entity) {
        match other_entity {
            Entity::Explosion(other_explosion) => {
                if self.health > other_explosion.health {
                    self.health += 100;
                } else {
                    self.health = 0;
                }
            }
            _ => self.health += 100,
        }
    }

    fn on_remove_explode(&self) -> bool {
        return false;
    }
}
