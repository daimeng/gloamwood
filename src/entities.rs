use crate::{
    effect::{BaseEffect, Effect, GameEffect},
    worldmap::WorldMap,
};

#[derive(Clone, Copy, Debug)]
pub struct Entity {
    pub proto: Option<&'static Entity>,
    pub breed: i16,
    pub level: i16,
    pub hp: i16,
    pub damage: i16,
    pub active: bool,
    pub effects: [Option<GameEffect>; 4],
}

pub static NONE: Entity = Entity {
    proto: None,
    breed: -1,
    level: 0,
    hp: 999,
    damage: 0,
    active: false,
    effects: [None, None, None, None],
};

pub static MONSTERS: [Entity; 10] = [
    Entity {
        proto: None,
        breed: 0,
        level: 0,
        hp: 100,
        damage: 2,
        active: false,
        effects: [None, None, None, None],
    },
    Entity {
        proto: None,
        breed: 1,
        level: 1,
        hp: 1,
        damage: 1,
        active: false,
        effects: [None, None, None, None],
    },
    Entity {
        proto: None,
        breed: 2,
        level: 2,
        hp: 2,
        damage: 1,
        active: false,
        effects: [None, None, None, None],
    },
    Entity {
        proto: None,
        breed: 3,
        level: 3,
        hp: 3,
        damage: 1,
        active: false,
        effects: [None, None, None, None],
    },
    Entity {
        proto: None,
        breed: 4,
        level: 4,
        hp: 4,
        damage: 1,
        active: false,
        effects: [None, None, None, None],
    },
    Entity {
        proto: None,
        breed: 5,
        level: 5,
        hp: 5,
        damage: 1,
        active: false,
        effects: [None, None, None, None],
    },
    Entity {
        proto: None,
        breed: 6,
        level: 6,
        hp: 6,
        damage: 1,
        active: false,
        effects: [None, None, None, None],
    },
    Entity {
        proto: None,
        breed: 7,
        level: 7,
        hp: 7,
        damage: 1,
        active: false,
        effects: [None, None, None, None],
    },
    Entity {
        proto: None,
        breed: 8,
        level: 8,
        hp: 8,
        damage: 1,
        active: false,
        effects: [None, None, None, None],
    },
    Entity {
        proto: None,
        breed: 9,
        level: 9,
        hp: 9,
        damage: 1,
        active: false,
        effects: [None, None, None, None],
    },
];

impl Entity {
    pub fn new(proto: &'static Entity) -> Self {
        Self {
            proto: Some(proto),
            breed: proto.breed,
            level: proto.level,
            hp: proto.hp,
            damage: proto.damage,
            active: proto.active,
            effects: [None, None, None, None],
        }
    }
}
