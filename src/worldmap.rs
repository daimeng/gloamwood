use macroquad::{
    miniquad::date,
    rand::{ChooseRandom, RandGenerator},
};

use crate::{
    effect::{self, GameEffect},
    entities,
};
use crate::{entities::Entity, spawns};

pub struct WorldMap {
    pub mapw: usize,
    pub maph: usize,
    pub terrains: Vec<Vec<i16>>,
    pub entities: Vec<Vec<usize>>,
    pub auras: Vec<Vec<i16>>,
    pub open: Vec<Vec<bool>>,
    pub show_terrain: Vec<Vec<bool>>,
    pub flags: Vec<Vec<i16>>,
    pub game_over: u16,
    pub effects: Vec<effect::GameEffect>,
    pub hero_pos: (usize, usize),
    pub entity_store: Vec<Entity>,
    pub effects_store: Vec<[Option<GameEffect>; 4]>,
    search_buffer: Vec<(usize, usize)>,
    gen_pool: Vec<usize>,
    gen_i: usize,
    pub initialized: bool,
    pub counts: [i16; 10],
}

pub fn neighbors(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    return neighborsn(x as i16, y as i16, w as i16, h as i16, 1);
}

#[inline(always)]
pub fn neighborsn(x: i16, y: i16, w: i16, h: i16, n: i16) -> impl Iterator<Item = (usize, usize)> {
    (-n..=n).flat_map(move |dy| {
        let yy = y + dy;
        (-n..=n).filter_map(move |dx| {
            if yy < 0 || yy >= h {
                return None;
            }

            let xx = x + dx;
            if xx < 0 || xx >= w {
                None
            } else {
                Some((xx as usize, yy as usize))
            }
        })
    })
}

impl WorldMap {
    pub fn new(mapw: usize, maph: usize) -> Self {
        let mut entity_store = Vec::with_capacity(mapw * maph);
        let mut effects_store: Vec<[Option<GameEffect>; 4]> = Vec::with_capacity(mapw * maph);

        entity_store.push(entities::NONE);
        effects_store.push([None, None, None, None]);

        entity_store.push(entities::MONSTERS[0]); // hero
        effects_store.push([Some(GameEffect::Dagger(2)), None, None, None]);

        Self {
            mapw,
            maph,
            terrains: vec![vec![0; mapw]; maph],
            entities: vec![vec![0; mapw]; maph],
            auras: vec![vec![0; mapw]; maph],
            open: vec![vec![false; mapw]; maph],
            show_terrain: vec![vec![false; mapw]; maph],
            flags: vec![vec![0; mapw]; maph],
            effects: vec![],
            entity_store,
            effects_store,
            hero_pos: (0, 0),
            game_over: 0,
            search_buffer: vec![(0, 0); maph * mapw],
            gen_pool: (0..mapw * maph).collect(),
            gen_i: 0,
            initialized: false,
            counts: [0; 10],
        }
    }

    pub fn init(&mut self, mines: usize) {
        let rng = RandGenerator::new();
        rng.srand(date::now() as u64);

        self.gen_pool.shuffle_with_state(&rng);
        let mut total = 0;

        // for i in 0..mines {
        //     self.gen_i += 1;
        //     total += 1 as i16;
        //     let n = self.gen_pool[i];
        //     let y = n / self.mapw;
        //     let x = n - y * self.mapw;
        //     self.set_monster(x, y, Entity::new(&entities::WOLF));
        //     self.counts[1] += 1;
        // }

        let mut balance = 0;
        for i in 0..mines {
            self.gen_i += 1;
            let n = self.gen_pool[i];
            let y = n / self.mapw;
            let x = n - y * self.mapw;
            let t = self.terrains[y][x];
            let spawns = spawns::SPAWNS[t as usize];
            let slen = spawns.len() as i16;

            let lvl: i16 = (rng.gen_range(-slen, slen + 1) + rng.gen_range(-slen, slen + 1)) / 2;
            let lvlabs = (lvl.abs() + balance).max(1).min(slen);
            let spawn = spawns[(lvlabs - 1) as usize];

            total += spawn;
            balance = if total > 3 * i { -1 } else { 1 };

            let next_id = self.entity_store.len();
            self.entity_store.push(entities::MONSTERS[spawn as usize]);
            self.effects_store
                .push(entities::MONSTER_EFFECTS[spawn as usize]);
            self.set_monster(x, y, next_id);
            self.counts[spawn as usize] += 1;
        }
        println!("{}", total);
    }

    pub fn set_terrain(&mut self, terrains: Vec<Vec<i16>>) {
        self.terrains = terrains;
        for i in 0..self.maph {
            for j in 0..self.mapw {
                self.show_terrain[i][j] = self.terrains[i][j] == 9;
            }
        }
    }

    pub fn set_monster(&mut self, x: usize, y: usize, eid: usize) {
        let old_idx = self.entities[y][x];
        self.entities[y][x] = eid;

        for (xx, yy) in neighbors(x, y, self.mapw, self.maph) {
            // patch the difference for surrounding tile auras
            self.auras[yy][xx] += self.entity_store[eid].level - self.entity_store[old_idx].level;
        }
    }

    pub fn flag_tile_inc(&mut self, x: usize, y: usize) {
        // clamp x y
        if x >= self.mapw || y >= self.maph {
            return;
        }

        if self.open[y][x] {
            return;
        }

        self.flags[y][x] = (self.flags[y][x] + 1) % 10;
    }

    pub fn flag_tile(&mut self, x: usize, y: usize, num: i16) {
        // clamp x y
        if x >= self.mapw || y >= self.maph {
            return;
        }

        if self.open[y][x] {
            return;
        }

        if self.flags[y][x] == num {
            self.flags[y][x] = 0;
        } else {
            self.flags[y][x] = num;
        }
    }

    pub fn remine(&mut self, x: usize, y: usize) {
        for (xx, yy) in neighbors(x, y, self.mapw, self.maph) {
            let monidx = self.entities[yy][xx];
            self.set_monster(xx, yy, 0);
            let mon = self.entity_store[monidx];

            if mon.level > 0 {
                // don't take new values that are also adjacent
                while self.gen_i < self.gen_pool.len() {
                    let n = self.gen_pool[self.gen_i];
                    let i = n / self.mapw;
                    let j = n - i * self.mapw;
                    self.gen_i += 1;

                    if (i as i16 - y as i16).abs() < 2 {
                        continue;
                    }
                    if (j as i16 - x as i16).abs() < 2 {
                        continue;
                    }

                    self.set_monster(j, i, monidx);
                    break;
                }
            }
        }
    }

    #[inline(always)]
    pub fn hero(&self) -> &Entity {
        let (x, y) = self.hero_pos;
        self.entity(x, y)
    }

    #[inline(always)]
    pub fn entity(&self, x: usize, y: usize) -> &Entity {
        let idx = self.entities[y][x];
        &self.entity_store[idx]
    }

    pub fn end_game(&mut self, win: u16) {
        self.game_over = win;

        for i in 0..self.maph {
            for j in 0..self.mapw {
                let idx = self.entities[i][j];
                if self.entity_store[idx].level > 0 {
                    self.open[i][j] = true;
                }
            }
        }
    }

    fn open_tile_(&mut self, x: usize, y: usize) -> i32 {
        // clamp x y
        if x >= self.mapw || y >= self.maph {
            return 0;
        }

        // move mines out of way for first click
        if !self.initialized {
            self.remine(x, y);
            self.set_monster(x, y, 1);
            self.hero_pos = (x, y);
        }
        self.initialized = true;

        let mut j = 0;

        self.search_buffer[j] = (x, y);
        j += 1;

        let mut opened = 0;

        while j > 0 {
            // shadow original tile vars
            let (x, y) = self.search_buffer[j - 1];
            j -= 1;

            if self.open[y][x] {
                continue;
            };

            self.open[y][x] = true;
            let eid = self.entities[y][x];
            self.entity_store[eid].active = true;
            opened += 1;
            self.flags[y][x] = 0;

            // hero.fight(self.monsters[y][x]);

            if self.auras[y][x] > 0 {
                for (xx, yy) in neighbors(x, y, self.mapw, self.maph) {
                    self.show_terrain[yy][xx] = true;
                }

                continue;
            }

            for (xx, yy) in neighbors(x, y, self.mapw, self.maph) {
                // skip self
                if xx == x && yy == y {
                    continue;
                }
                if self.open[yy][xx] {
                    continue;
                }

                self.search_buffer[j] = (xx, yy);
                j += 1;
            }
        }

        opened
    }

    pub fn open_tile(&mut self, x: usize, y: usize) -> bool {
        let opened = self.open_tile_(x, y);

        if opened == 0 {
            let eid = self.entities[y][x];

            if self.entity_store[eid].breed == -1 {
                self.set_monster(x, y, 1);
                self.set_monster(self.hero_pos.0, self.hero_pos.1, 0);
                self.hero_pos = (x, y);
            }

            self.step();
        } else {
            // on open triggers
            let eid = self.entities[y][x];
            if eid > 1 {
                for effect in self.effects_store[eid] {
                    match effect {
                        Some(GameEffect::Wail) => {
                            for (xx, yy) in neighborsn(
                                x as i16,
                                y as i16,
                                self.mapw as i16,
                                self.maph as i16,
                                2,
                            ) {
                                if xx == x && yy == y {
                                    continue;
                                }

                                let neighbor_id = self.entities[yy][xx];
                                let neighbor = &mut self.entity_store[neighbor_id];

                                if neighbor.breed == 0 || neighbor.level % 2 == 1 {
                                    neighbor.hp -= 6;
                                }
                            }
                        }
                        Some(GameEffect::Pounce) => {}
                        _ => {}
                    }
                }
            }
        }

        return opened > 0;
    }

    pub fn chord_tile(&mut self, x: usize, y: usize) {
        // prevent chording on cloud tiles
        if self.terrains[y][x] == 8 {
            return;
        }

        // only chord open tiles
        if !self.open[y][x] {
            return;
        }

        // tile must have some enemy nearby
        let aura = self.auras[y][x];
        if aura < 1 {
            return;
        }

        // sum current values
        let mut sum = 0;
        for (xx, yy) in neighbors(x, y, self.mapw, self.maph) {
            // handle open tile cases
            let eid = self.entities[yy][xx];
            if self.open[yy][xx] {
                sum += self.entity_store[eid].level;
            } else {
                sum += self.flags[yy][xx];
            }
        }

        if sum == aura {
            for (xx, yy) in neighbors(x, y, self.mapw, self.maph) {
                if self.flags[yy][xx] > 0 {
                    continue;
                }

                self.open_tile_(xx, yy);
            }
        }
    }

    pub fn step(&mut self) {
        let heroid = self.entities[self.hero_pos.1][self.hero_pos.0];

        for i in 0..self.maph {
            for j in 0..self.mapw {
                let idx = self.entities[i][j];
                let ent = self.entity_store[idx];
                if self.entity_store[idx].level > 0 && ent.active {
                    let hero = &mut self.entity_store[heroid];
                    hero.hp -= 1;
                }
            }
        }

        if self.entity_store[heroid].hp < 1 {
            self.end_game(2);
        }
    }
}
