#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEffect {
    // player only
    Dagger(i16),
    Sword(i16),
    Axe(i16),

    // common
    Missile(i16),
    Immolate(i16),
    Regen(i16),

    // monster only
    Claw(i16),
    Howl,
    Spear(i16),
    Wail(i16),
    Raze,
    Vamp,
    VampAura,
}
