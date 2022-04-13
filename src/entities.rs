use rand::prelude::ThreadRng;

use crate::bullet::Bullet;
use crate::explosion::Explosion;
use crate::helpers::Position;
use crate::ship::{Ship, Team};

#[derive(Clone, PartialEq, Eq, Hash)]
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
    fn take_turn(self, rng: &mut ThreadRng, entities: &Entities) -> (Self, Option<Entity>)
    where
        Self: Sized;
    fn on_collide(self, other: &Entity) -> Self;
    fn on_remove_explode(&self) -> bool;
}

// I would have liked to use the `enum_dispatch` crate here, but
// it doesn't support customizing the implementation methods
macro_rules! delegate {
    ($entity:expr, $name:ident) => {
       delegate!($entity; Ship, Bullet, Explosion; $name)
    };
    ($entity:expr; $($member:ident),+; $name:ident) => {
        match $entity {
            $(Self::$member(e) => e.$name(),)+
        }
    };
}

impl EntityBehavior for Entity {
    fn get_position(&self) -> Position {
        delegate!(&self, get_position)
    }

    fn get_prev_position(&self) -> Position {
        delegate!(self, get_prev_position)
    }

    fn should_remove(&self) -> bool {
        delegate!(self, should_remove)
    }

    fn avatar(&self) -> &str {
        delegate!(self, avatar)
    }

    fn take_turn(self, rng: &mut ThreadRng, entities: &Entities) -> (Self, Option<Entity>) {
        match self {
            Self::Ship(e) => {
                let (e, other) = e.take_turn(rng, entities);
                (Self::Ship(e), other)
            }
            Self::Bullet(e) => {
                let (e, other) = e.take_turn(rng, entities);
                (Self::Bullet(e), other)
            }
            Self::Explosion(e) => {
                let (e, other) = e.take_turn(rng, entities);
                (Self::Explosion(e), other)
            }
        }
    }

    fn on_collide(self, other: &Entity) -> Self {
        match self {
            Self::Ship(e) => Self::Ship(e.on_collide(other)),
            Self::Bullet(e) => Self::Bullet(e.on_collide(other)),
            Self::Explosion(e) => Self::Explosion(e.on_collide(other)),
        }
    }

    fn on_remove_explode(&self) -> bool {
        delegate!(self, on_remove_explode)
    }
}

pub type Entities = Vec<Entity>;
