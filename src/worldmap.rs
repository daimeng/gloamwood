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
        for yi in 0..3 {
            let yy = y + yi;
            if yy == 0 || yy > self.maph {
                continue;
            }

            for xi in 0..3 {
                let xx = x + xi;
                if xx == 0 || xx > self.mapw {
                    continue;
                }

                // patch the difference for surrounding tile auras
                self.auras[yy - 1][xx - 1] += n - old_n;
            }
        }
    }

    pub fn flag_tile(&mut self, x: usize, y: usize) {
        // clamp x y
        let x = if x >= self.mapw { self.mapw - 1 } else { x };
        let y = if y >= self.maph { self.maph - 1 } else { y };

        self.flags[y][x] = (self.flags[y][x] + 1) % 6;
    }

    pub fn remine(&mut self, x: usize, y: usize) {
        for dy in 0..3 {
            let yy = y + dy;
            if yy == 0 || yy > self.maph {
                continue;
            }

            for dx in 0..3 {
                let xx = x + dx;
                if xx == 0 || xx > self.mapw {
                    continue;
                }

                let mon = self.monsters[yy - 1][xx - 1];
                self.set_monster(xx - 1, yy - 1, 0);

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
            let (xx, yy) = self.search_buffer[j - 1];
            j -= 1;

            if self.open[yy][xx] {
                continue;
            };

            self.open[yy][xx] = true;

            if self.auras[yy][xx] > 0 {
                continue;
            }

            if yy < self.maph - 1 && !self.open[yy + 1][xx] {
                self.search_buffer[j] = (xx, yy + 1);
                j += 1;
            }
            if xx < self.mapw - 1 && !self.open[yy][xx + 1] {
                self.search_buffer[j] = (xx + 1, yy);
                j += 1;
            }
            if yy > 0 && !self.open[yy - 1][xx] {
                self.search_buffer[j] = (xx, yy - 1);
                j += 1;
            }
            if xx > 0 && !self.open[yy][xx - 1] {
                self.search_buffer[j] = (xx - 1, yy);
                j += 1;
            }
        }

        self.open[y][x]
    }
}
