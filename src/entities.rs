#[derive(Clone, Copy, Debug)]
pub struct Entity {
    pub proto: Option<&'static Entity>,
    pub breed: i16,
    pub level: i16,
    pub hp: i16,
    pub active: bool,
}

pub static NONE: Entity = Entity {
    proto: None,
    breed: -1,
    level: 0,
    hp: 999,
    active: false,
};

pub static MONSTERS: [Entity; 10] = [
    Entity {
        proto: None,
        breed: 0,
        level: 0,
        hp: 10,
        active: false,
    },
    Entity {
        proto: None,
        breed: 1,
        level: 1,
        hp: 1,
        active: false,
    },
    Entity {
        proto: None,
        breed: 2,
        level: 2,
        hp: 2,
        active: false,
    },
    Entity {
        proto: None,
        breed: 3,
        level: 3,
        hp: 3,
        active: false,
    },
    Entity {
        proto: None,
        breed: 4,
        level: 4,
        hp: 4,
        active: false,
    },
    Entity {
        proto: None,
        breed: 5,
        level: 5,
        hp: 5,
        active: false,
    },
    Entity {
        proto: None,
        breed: 6,
        level: 6,
        hp: 6,
        active: false,
    },
    Entity {
        proto: None,
        breed: 7,
        level: 7,
        hp: 7,
        active: false,
    },
    Entity {
        proto: None,
        breed: 8,
        level: 8,
        hp: 8,
        active: false,
    },
    Entity {
        proto: None,
        breed: 9,
        level: 9,
        hp: 9,
        active: false,
    },
];

impl Entity {
    pub fn new(proto: &'static Entity) -> Self {
        Self {
            proto: Some(proto),
            breed: proto.breed,
            level: proto.level,
            hp: proto.hp,
            active: proto.active,
        }
    }
}
