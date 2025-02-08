pub struct WorldMap {
    mapw: usize,
    maph: usize,
    pub terrains: Vec<Vec<i16>>,
    pub monsters: Vec<Vec<i16>>,
    pub auras: Vec<Vec<i16>>,
    pub open: Vec<Vec<bool>>,
    search_buffer: Vec<(usize, usize)>,
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
            search_buffer: vec![(0, 0); maph * mapw],
        }
    }

    pub fn set_monster(&mut self, x: usize, y: usize, n: i16) {
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

                self.auras[yy - 1][xx - 1] += n;
            }
        }
    }

    pub fn open_tile(&mut self, x: usize, y: usize) -> bool {
        // clamp x y
        let x = if x >= self.mapw { self.mapw - 1 } else { x };
        let y = if y >= self.maph { self.maph - 1 } else { y };

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
