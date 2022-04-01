// import (
// 	"math/rand"
// )

use rand::prelude::*;
use rand::{thread_rng, Rng};

use crate::{ship::Ship, terminal::get_size};

pub struct Position(pub i32, pub i32);

pub trait Entity {
    fn get_position(self) -> Position;
    fn get_prev_position(self) -> Position;
    fn should_remove(self) -> bool;
    fn avatar(self) -> String;
    fn take_turn(self, entities: Entities);
    fn on_collide(self);
    fn on_remove_explode(self) -> bool;
}

pub type Entities = Vec<Box<dyn Entity>>;

pub trait HasRng {
    fn rand<T>(self, limit: T) -> T {
        rand(self.rng, limit)
    }
}

fn rand<T>(rng: ThreadRng, limit: T) -> T {
    rng.gen_range(0..limit)
}

pub fn collided<E1: Entity, E2: Entity>(entity_a: E1, entity_b: E2) -> bool {
    let (posA, posB) = (entity_a.getPosition(), entity_b.getPosition());

    if positions_are_same(posA, posB) {
        return true;
    }

    let (prevPosA, prevPosB) = (entity_a.getPrevPosition(), entity_b.getPrevPosition());

    // swapped position
    if positions_are_same(posA, prevPosB) && positions_are_same(posB, prevPosA) {
        return true;
    }

    return false;
}

pub fn positions_are_same(a: Position, b: Position) -> bool {
    return a[0] == b[0] && a[1] == b[1];
}

pub fn random_position() -> (i32, i32) {
    let rng = thread_rng();

    let (width, height) = get_size();
    let x = rng.gen_range(0..width) + 1;
    let y = rng.gen_range(0..height) + 2;

    (x, y)
}

pub fn wall_position() -> i32 {
    let (maxX, maxY) = get_size();

    // switch rand.Intn(4) {
    // case 0:
    // 	// top
    // 	return [2]i32{rand.Intn(maxX), 1}
    // case 1:
    // 	// bottom
    // 	return [2]i32{rand.Intn(maxX), maxY}
    // case 2:
    // 	// left
    // 	return [2]i32{1, rand.Intn(maxY)}
    // default:
    // 	// right
    // 	return [2]i32{maxX, rand.Intn(maxY)}
    // }
}

pub fn count_ships(entities: Entities) -> i32 {
    get_ships_from_entities(entities).len()
}

pub fn get_ships_from_entities(entities: Entities) -> Vec<Ship> {
    let ships = Vec::new();

    for e in entities {
        if let Ship = e {
            ships.append(e)
        }
    }

    ships
}

pub fn abs(i: i32) -> i32 {
    if i < 0 {
        i *= -1
    }

    return i;
}
