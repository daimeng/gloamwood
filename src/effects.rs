use crate::worldmap::WorldMap;

pub trait Effect {
    fn run(&mut self, world: &mut WorldMap);
}
