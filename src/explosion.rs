use rand::prelude::ThreadRng;

use crate::entities::{Entities, Entity, EntityBehavior};
use crate::helpers::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Explosion {
    position: Position,
    health: u16,
}

impl Explosion {
    pub fn new(pos: Position) -> Explosion {
        Explosion {
            position: pos,
            health: 10,
        }
    }
}

impl EntityBehavior for Explosion {
    fn get_position(&self) -> Position {
        self.position
    }

    fn get_prev_position(&self) -> Position {
        self.position
    }

    fn should_remove(&self) -> bool {
        self.health == 0
    }

    fn avatar(&self) -> &str {
        "\x1B[0;91m*\x1B[0m"
    }

    fn take_turn(mut self, _: &mut ThreadRng, _: &Entities) -> (Explosion, Option<Entity>) {
        if self.health > 0 {
            self.health -= 1;
        }

        (self, None)
    }

    fn on_collide(mut self, other_entity: &Entity) -> Explosion {
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

        self
    }

    fn on_remove_explode(&self) -> bool {
        false
    }
}
