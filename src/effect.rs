use crate::{
    entities,
    worldmap::{neighbors, WorldMap},
};

#[derive(Clone, Copy, Debug)]
pub enum GameEffect {
    Dagger,
    Sword,
}

impl Effect for GameEffect {
    fn on_move(&mut self, world: &mut WorldMap, x: usize, y: usize) {
        match self {
            _ => {}
        }
    }
}

#[derive(Clone, Copy)]
pub struct BaseEffect {}

impl Effect for BaseEffect {}

pub static NONE: BaseEffect = BaseEffect {};

pub struct Sword {}
impl Effect for Sword {
    fn on_move(&mut self, world: &mut WorldMap, x: usize, y: usize) {
        let ent = world.entities[y][x];

        for (xx, yy) in neighbors(x, y, world.mapw, world.maph) {
            // check if open
            if !world.open[yy][xx] {
                continue;
            }

            // melee
            let eid = world.entities[yy][xx];
            if world.entity_store[eid].breed > 0 {
                world.entity_store[eid].hp -= world.entity_store[ent].damage;
                if world.entity_store[eid].hp < 1 {
                    world.set_monster(xx, yy, 0);
                }
            }
        }
    }
}

pub trait Effect {
    fn on_move(&mut self, world: &mut WorldMap, x: usize, y: usize) {}
    fn on_attack(&mut self, world: &mut WorldMap) {}
    fn on_ready(&mut self, world: &mut WorldMap) {}
}
