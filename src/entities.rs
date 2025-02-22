use crate::worldmap::WorldMap;

#[derive(Clone, Copy)]
pub struct Entity {
    pub proto: Option<&'static Entity>,
    pub breed: i16,
    pub level: i16,
    pub hp: i16,
    pub damage: i16,
}

pub static NONE: Entity = Entity {
    proto: None,
    breed: -1,
    level: 0,
    hp: 999,
    damage: 0,
};

pub static HERO: Entity = Entity {
    proto: None,
    breed: 0,
    level: 0,
    hp: 100,
    damage: 1,
};

pub static WOLF: Entity = Entity {
    proto: None,
    breed: 1,
    level: 1,
    hp: 10,
    damage: 1,
};

impl Entity {
    pub fn new(proto: &'static Entity) -> Self {
        Self {
            proto: Some(proto),
            breed: proto.breed,
            level: proto.level,
            hp: proto.hp,
            damage: proto.damage,
        }
    }
}
