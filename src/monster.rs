#[derive(Clone, Copy)]
pub struct Monster {
    proto: Option<&'static Monster>,
    hp: i16,
}

pub static WOLF: Monster = Monster {
    proto: None,
    hp: 10,
};

impl Monster {
    pub fn new(proto: &'static Monster) -> Self {
        Self {
            proto: Some(proto),
            hp: proto.hp,
        }
    }
}
