use crate::bullet::Bullet;
use crate::explosion::Explosion;
use crate::helpers::Position;
use crate::ship::{Ship, Team};

#[derive(PartialEq, Eq, Hash)]
pub enum Entity {
    Ship(Ship),
    Bullet(Bullet),
    Explosion(Explosion),
}

impl Entity {
    pub fn ship(t: Team) -> Entity {
        Entity::Ship(Ship::new(t))
    }

    pub fn explosion(pos: Position) -> Entity {
        Entity::Explosion(Explosion::new(pos))
    }
}

pub trait EntityBehavior {
    fn get_position(&self) -> Position;
    fn get_prev_position(&self) -> Position;
    fn should_remove(&self) -> bool;
    fn avatar(&self) -> &str;
    fn take_turn(&mut self, entities: &Entities) -> Entities;
    fn on_collide(&mut self, other: &Entity);
    fn on_remove_explode(&self) -> bool;
}

#[macro_export]
macro_rules! match_entity {
    ( $entity:expr, $name:ident $( , $arg:ident )* ) => {
        match $entity {
            Entity::Ship(e) => e.$name($($arg),*),
            Entity::Bullet(e) => e.$name($($arg),*),
            Entity::Explosion(e) => e.$name($($arg),*),
        }
    };
}

impl EntityBehavior for Entity {
    fn get_position(&self) -> Position {
        match_entity!(self, get_position)
    }

    fn get_prev_position(&self) -> Position {
        match_entity!(self, get_prev_position)
    }

    fn should_remove(&self) -> bool {
        match_entity!(self, should_remove)
    }

    fn avatar(&self) -> &str {
        match_entity!(self, avatar)
    }

    fn take_turn(&mut self, entities: &Entities) -> Entities {
        match_entity!(self, take_turn, entities)
    }

    fn on_collide(&mut self, other: &Entity) {
        match_entity!(self, on_collide, other)
    }

    fn on_remove_explode(&self) -> bool {
        match_entity!(self, on_remove_explode)
    }
}

pub type Entities = Vec<Entity>;
