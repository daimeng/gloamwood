use macroquad::{
    miniquad::date,
    rand::{ChooseRandom, RandGenerator},
};

pub struct WorldMap {
    mapw: usize,
    maph: usize,
    pub terrains: Vec<Vec<i16>>,
    pub monsters: Vec<Vec<i16>>,
    pub auras: Vec<Vec<i16>>,
    pub open: Vec<Vec<bool>>,
    pub flags: Vec<Vec<i16>>,
    search_buffer: Vec<(usize, usize)>,
    gen_pool: Vec<usize>,
    gen_i: usize,
    first: bool,
}

fn neighbors(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..3).flat_map(move |dy| {
        let yy = y + dy;
        (0..3).filter_map(move |dx| {
            if yy == 0 || yy > h {
                return None;
            }

            let xx = x + dx;
            if xx == 0 || xx > w {
                None
            } else {
                Some((xx - 1, yy - 1))
            }
        })
    })
}

impl WorldMap {
    pub fn new(mapw: usize, maph: usize) -> Self {
        Self {
            mapw,
            maph,
            terrains: vec![vec![0; mapw]; maph],
            monsters: vec![vec![0; mapw]; maph],
            auras: vec![vec![0; mapw]; maph],
            open: vec![vec![false; mapw]; maph],
            flags: vec![vec![0; mapw]; maph],
            search_buffer: vec![(0, 0); maph * mapw],
            gen_pool: (0..mapw * maph).collect(),
            gen_i: 0,
            first: true,
        }
    }

    pub fn init(&mut self) {
        let rng = RandGenerator::new();
        rng.srand(date::now() as u64);

        self.gen_pool.shuffle_with_state(&rng);
        for i in 0..99 {
            self.gen_i += 1;
            let n = self.gen_pool[i];
            let y = n / self.mapw;
            let x = n - y * self.mapw;
            self.set_monster(x, y, 1);
        }
    }

    pub fn set_monster(&mut self, x: usize, y: usize, n: i16) {
        let old_n = self.monsters[y][x];
        self.monsters[y][x] = n;

        for (xx, yy) in neighbors(x, y, self.mapw, self.maph) {
            // patch the difference for surrounding tile auras
            self.auras[yy][xx] += n - old_n;
        }
    }

    pub fn flag_tile(&mut self, x: usize, y: usize) {
        // clamp x y
        let x = if x >= self.mapw { self.mapw - 1 } else { x };
        let y = if y >= self.maph { self.maph - 1 } else { y };

        if self.open[y][x] {
            return;
        }
        self.flags[y][x] = (self.flags[y][x] + 1) % 6;
    }

    pub fn remine(&mut self, x: usize, y: usize) {
        for (xx, yy) in neighbors(x, y, self.mapw, self.maph) {
            let mon = self.monsters[yy][xx];
            self.set_monster(xx, yy, 0);

            if mon > 0 {
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

                    self.set_monster(j, i, mon);
                    break;
                }
            }
        }
    }

    pub fn open_tile(&mut self, x: usize, y: usize) -> bool {
        // clamp x y
        let x = if x >= self.mapw { self.mapw - 1 } else { x };
        let y = if y >= self.maph { self.maph - 1 } else { y };

        // move mines out of way for first click
        if self.first {
            self.remine(x, y);
        }
        self.first = false;

        let mut j = 0;

        self.search_buffer[j] = (x, y);
        j += 1;

        while j > 0 {
            // shadow original tile vars
            let (x, y) = self.search_buffer[j - 1];
            j -= 1;

            if self.open[y][x] {
                continue;
            };

            self.open[y][x] = true;
            self.flags[y][x] = 0;

            if self.auras[y][x] > 0 {
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

        self.open[y][x]
    }

    pub fn chord_tile(&mut self, x: usize, y: usize) {
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
            sum += self.flags[yy][xx];
        }

        if sum == aura {
            for (xx, yy) in neighbors(x, y, self.mapw, self.maph) {
                if self.flags[yy][xx] > 0 {
                    continue;
                }

                self.open_tile(xx, yy);
            }
        }
    }
}
