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
}

pub static NONE: Entity = Entity {
    proto: None,
    breed: -1,
    level: 0,
    hp: 999,
    damage: 0,
    active: false,
};

pub static MONSTERS: [Entity; 10] = [
    Entity {
        proto: None,
        breed: 0,
        level: 0,
        hp: 100,
        damage: 2,
        active: false,
    },
    Entity {
        proto: None,
        breed: 1,
        level: 1,
        hp: 1,
        damage: 1,
        active: false,
    },
    Entity {
        proto: None,
        breed: 2,
        level: 2,
        hp: 2,
        damage: 1,
        active: false,
    },
    Entity {
        proto: None,
        breed: 3,
        level: 3,
        hp: 3,
        damage: 1,
        active: false,
    },
    Entity {
        proto: None,
        breed: 4,
        level: 4,
        hp: 4,
        damage: 1,
        active: false,
    },
    Entity {
        proto: None,
        breed: 5,
        level: 5,
        hp: 5,
        damage: 1,
        active: false,
    },
    Entity {
        proto: None,
        breed: 6,
        level: 6,
        hp: 6,
        damage: 1,
        active: false,
    },
    Entity {
        proto: None,
        breed: 7,
        level: 7,
        hp: 7,
        damage: 1,
        active: false,
    },
    Entity {
        proto: None,
        breed: 8,
        level: 8,
        hp: 8,
        damage: 1,
        active: false,
    },
    Entity {
        proto: None,
        breed: 9,
        level: 9,
        hp: 9,
        damage: 1,
        active: false,
    },
];

pub static MONSTER_EFFECTS: [[Option<GameEffect>; 4]; 10] = [
    [Some(GameEffect::Dagger(2)), None, None, None],
    [
        Some(GameEffect::Claw(1)),
        Some(GameEffect::Howl),
        None,
        None,
    ],
    [Some(GameEffect::Missile(2)), None, None, None],
    [
        Some(GameEffect::Spear(3)),
        Some(GameEffect::Regen(1)),
        None,
        None,
    ],
    [
        Some(GameEffect::Claw(4)),
        Some(GameEffect::VampAura),
        None,
        None,
    ],
    [Some(GameEffect::Claw(5)), None, None, None],
    [Some(GameEffect::Wail(6)), None, None, None],
    [
        Some(GameEffect::Claw(7)),
        Some(GameEffect::Regen(2)),
        None,
        None,
    ],
    [
        Some(GameEffect::Missile(8)),
        Some(GameEffect::Regen(3)),
        None,
        None,
    ],
    [
        Some(GameEffect::Claw(9)),
        Some(GameEffect::Immolate(9)),
        Some(GameEffect::Regen(4)),
        None,
    ],
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
        }
    }
}
