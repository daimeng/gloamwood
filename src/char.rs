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
}

impl Char {
    pub fn new(class: Class) -> Self {
        match class {
            Class::Hero => Self {
                class,
                level: 1,
                exprate: 2,
                exp: 0,
            },
        }
    }

    pub fn fight(&mut self, monster: i16) {
        match self.class {
            _ => {
                if monster > self.level {
                    // if monster is too tough subtract half of monster level rounded up.
                    self.level -= (monster + 1) / 2;
                }
            }
        }
    }
}
