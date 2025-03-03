#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEffect {
    // player only
    Dagger(i16),
    Sword(i16),
    Axe(i16),

    // common
    Missile(i16),
    Immolate, // damage self, fire resist
    Regen(i16),
    Vamp,

    // monster only
    Claw(i16),
    Howl,
    Spear(i16),
    Wail,
    Pounce,
    VampAura,
}
