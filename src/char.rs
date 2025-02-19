#[derive(Clone, Copy)]
pub enum Class {
    Hero,
}

#[derive(Clone, Copy)]
pub struct Char {
    pub class: Class,
    pub level: i16,
    pub exprate: i16,
    exp: i16,
    pub max_hp: i16,
    pub hp: i16,
}

impl Char {
    pub fn new(class: Class) -> Self {
        match class {
            Class::Hero => Self {
                class,
                level: 1,
                exprate: 2,
                exp: 0,
                max_hp: 100,
                hp: 100,
            },
        }
    }

    pub fn open(&mut self) {}

    pub fn fight(&mut self, monster: i16) {
        if monster > self.level {
            self.level = 0;
        } else {
            self.exp += 1;
        }
    }
}
