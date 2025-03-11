use std::collections::HashSet;

use macroquad::{
    miniquad::date,
    rand::{ChooseRandom, RandGenerator},
};

use crate::{
    entities,
    items::{EFFECTIVE, INEFFECTIVE},
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
    pub hero_pos: (usize, usize),
    pub entity_store: Vec<Entity>,
    pub item: usize,
    pub maxhp: i16,
    search_buffer: Vec<(usize, usize)>,
    search_visited: HashSet<(usize, usize)>,
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
        entity_store.push(entities::NONE);
        entity_store.push(entities::MONSTERS[0]); // hero

        let search_visited = HashSet::with_capacity(maph * mapw);

        Self {
            mapw,
            maph,
            terrains: vec![vec![0; mapw]; maph],
            entities: vec![vec![0; mapw]; maph],
            auras: vec![vec![0; mapw]; maph],
            open: vec![vec![false; mapw]; maph],
            show_terrain: vec![vec![false; mapw]; maph],
            flags: vec![vec![0; mapw]; maph],
            entity_store,
            hero_pos: (0, 0),
            item: 1,
            game_over: 0,
            maxhp: 10,
            search_buffer: vec![(0, 0); maph * mapw],
            search_visited,
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
            self.set_monster(x, y, next_id);
            self.counts[spawn as usize] += 1;
        }

        println!("{}/{}", self.evil_count().1, total);
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

        self.search_visited.clear();
        let mut j = 0;
        self.search_buffer[j] = (x, y);
        self.search_visited.insert((x, y));

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

            for n in neighbors(x, y, self.mapw, self.maph) {
                let (xx, yy) = n;
                // skip self
                if xx == x && yy == y {
                    continue;
                }
                if self.open[yy][xx] {
                    continue;
                }
                if self.search_visited.contains(&n) {
                    continue;
                }
                self.search_visited.insert(n);

                self.search_buffer[j] = n;
                j += 1;
            }
        }

        opened
    }

    pub fn open_tile(&mut self, x: usize, y: usize) -> bool {
        // possible for tile to be open based on another effect
        if self.open[y][x] {
            if self.entities[y][x] > 1 {
                self.loot(x, y);
                self.step(x, y);
            }

            false
        } else {
            let opened = self.open_tile_(x, y);

            // step forward game if monster opened
            if self.entities[y][x] > 1 {
                self.attack(x, y);
                self.step(x, y);
            }

            opened > 0
        }
    }

    fn loot(&mut self, x: usize, y: usize) {
        let eid = self.entities[y][x];
        let target = self.entity_store[eid];
        let heroid = self.entities[self.hero_pos.1][self.hero_pos.0];

        if target.hp == 0 && target.breed >= 0 {
            if target.breed == 1 {
                if self.entity_store[heroid].hp < self.maxhp {
                    self.entity_store[heroid].hp = self.maxhp.min(self.entity_store[heroid].hp + 2);
                } else {
                    self.entity_store[heroid].hp += 1;
                    self.maxhp += 1;
                }
            } else {
                self.item = target.breed as usize;
            }
            self.set_monster(x, y, 0);

            self.counts[target.breed as usize] -= 1;
        }
    }

    fn attack(&mut self, x: usize, y: usize) {
        let eid = self.entities[y][x];
        let target = self.entity_store[eid];
        let heroid = self.entities[self.hero_pos.1][self.hero_pos.0];

        if eid > 1 {
            match self.item {
                0..=9 => {
                    let ineff = INEFFECTIVE[self.item];
                    let eff = EFFECTIVE[self.item];

                    if ineff.contains(&target.breed) {
                        self.entity_store[heroid].hp -= 2 * self.entity_store[eid].breed;
                    } else if eff.contains(&target.breed) {
                        if self.entity_store[heroid].hp < self.maxhp {
                            self.entity_store[heroid].hp += 1;
                        }
                    } else {
                        self.entity_store[heroid].hp -= self.entity_store[eid].breed;
                    }

                    // kill off monster
                    self.entity_store[eid].hp = 0;
                }
                _ => {}
            }
        }
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
                if self.open[yy][xx] {
                    continue;
                }

                self.open_tile(xx, yy);
            }
        }
    }

    #[inline(always)]
    pub fn evil_count(&self) -> (i16, i16) {
        let mut evil_count = 0;
        let mut evil_sum = 0;
        for (i, &value) in self.counts.iter().enumerate() {
            if i % 2 == 0 {
                evil_count += value;
                evil_sum += i as i16 * value;
            }
        }
        return (evil_count, evil_sum);
    }

    pub fn step(&mut self, x: usize, y: usize) {
        let heroid = self.entities[self.hero_pos.1][self.hero_pos.0];

        let (evil_count, _) = self.evil_count();

        if self.entity_store[heroid].hp < 1 {
            self.end_game(2);
        }

        if evil_count == 0 {
            self.end_game(1);
        }
    }
}
