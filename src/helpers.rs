use rand::prelude::*;

use crate::entities::{Entities, Entity, EntityBehavior};
use crate::ship::Ship;
use crate::terminal::get_size;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position(pub i32, pub i32);

pub fn collided<E1, E2>(entity_a: &E1, entity_b: &E2) -> bool
where
    E1: EntityBehavior,
    E2: EntityBehavior,
{
    let (pos_a, pos_b) = (entity_a.get_position(), entity_b.get_position());

    if positions_are_same(pos_a, pos_b) {
        return true;
    }

    let (prev_pos_a, prev_pos_b) = (entity_a.get_prev_position(), entity_b.get_prev_position());

    // swapped position
    if positions_are_same(pos_a, prev_pos_b) && positions_are_same(pos_b, prev_pos_a) {
        return true;
    }

    false
}

pub fn positions_are_same(a: Position, b: Position) -> bool {
    a.0 == b.0 && a.1 == b.1
}

pub fn random_position() -> Position {
    let mut rng = thread_rng();

    let (width, height) = get_size();
    let x = rng.gen_range(0..width) + 1;
    let y = rng.gen_range(0..height) + 2;

    Position(x.into(), y.into())
}

pub fn wall_position() -> Position {
    let mut rng = thread_rng();
    let (max_x, max_y) = get_size();

    match rng.gen_range(0..4) {
        // top
        0 => Position(rng.gen_range(0..max_x).into(), 1),
        // bottom
        1 => Position(rng.gen_range(0..max_x).into(), max_y.into()),
        // left
        2 => Position(1, rng.gen_range(0..max_y).into()),
        // right
        _ => Position(max_x.into(), rng.gen_range(0..max_y).into()),
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

/// There's probably something for this in the std library,
/// but I'm too lazy to try to find it
pub fn abs(mut i: i32) -> i32 {
    if i < 0 {
        i *= -1
    }

    i
}
