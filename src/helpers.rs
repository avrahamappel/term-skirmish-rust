use rand::prelude::*;

use crate::entities::{Entities, Entity, EntityBehavior};
use crate::ship::Ship;
use crate::terminal::get_size;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position(pub u16, pub u16);

pub fn collided<E1, E2>(entity_a: &E1, entity_b: &E2) -> bool
where
    E1: EntityBehavior,
    E2: EntityBehavior,
{
    let (posA, posB) = (entity_a.get_position(), entity_b.get_position());

    if positions_are_same(posA, posB) {
        return true;
    }

    let (prevPosA, prevPosB) = (entity_a.get_prev_position(), entity_b.get_prev_position());

    // swapped position
    if positions_are_same(posA, prevPosB) && positions_are_same(posB, prevPosA) {
        return true;
    }

    return false;
}

pub fn positions_are_same(a: Position, b: Position) -> bool {
    return a.0 == b.0 && a.1 == b.1;
}

pub fn random_position() -> Position {
    let mut rng = thread_rng();

    let (width, height) = get_size();
    let x = rng.gen_range(0..width) + 1;
    let y = rng.gen_range(0..height) + 2;

    Position(x, y)
}

pub fn wall_position() -> Position {
    let mut rng = thread_rng();
    let (maxX, maxY) = get_size();

    match rng.gen_range(0..4) {
        // top
        0 => Position(rng.gen_range(0..maxX.into()), 1),
        // bottom
        1 => Position(rng.gen_range(0..maxX.into()), maxY.into()),
        // left
        2 => Position(1, rng.gen_range(0..maxY.into())),
        // right
        _ => Position(maxX.into(), rng.gen_range(0..maxY.into())),
    }
}

pub fn count_ships(entities: &Entities) -> u16 {
    get_ships_from_entities(entities).len() as u16
}

pub fn get_ships_from_entities(entities: &Entities) -> Vec<&Ship> {
    entities
        .iter()
        .filter_map(|e| match e {
            Entity::Ship(ship) => Some(ship),
            _ => None,
        })
        .collect()
}

pub fn abs(i: i8) -> u16 {
    if i < 0 {
        i *= -1
    }

    return i as u16;
}
