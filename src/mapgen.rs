use macroquad::{miniquad::date, prelude::*};

pub fn genmap_fissure(terrain: &mut [Vec<f32>]) {
    let rng = rand::RandGenerator::new();
    rng.srand(date::now() as u64);

    let h = terrain.len();
    let w = terrain[0].len();

    let times = 1000;

    for t in 0..times {
        let p1 = (rng.gen_range(0, w) as i16, rng.gen_range(0, h) as i16);

        let mut p2 = p1.clone();
        while p1 == p2 {
            p2 = (
                rng.gen_range(0, w - 1) as i16,
                rng.gen_range(0, h - 1) as i16,
            );
        }

        let dx = p2.0 as f32 - p1.0 as f32;
        let dy = p2.1 as f32 - p1.1 as f32;

        // check if line more vertical or horizontal
        if dy.abs() > dx.abs() {
            // x = my + b
            // b = x - my
            let m = dx / dy;
            let b = p1.0 as f32 - m * p1.1 as f32;

            let mut y: usize = 0;
            let mut x = b;

            while y < h {
                let xx = x.round() as usize;
                if p1.1 > p2.1 {
                    for j in 0..xx.min(w) {
                        terrain[y][j] += 1.0;
                    }
                    for j in xx..w {
                        terrain[y][j] -= 1.0;
                    }
                } else {
                    for j in 0..xx.min(w) {
                        terrain[y][j] -= 1.0;
                    }
                    for j in xx..w {
                        terrain[y][j] += 1.0;
                    }
                }

                y += 1;
                x += m;
            }
        } else {
            // y = mx + b
            // b = y - mx
            let m = dy / dx;
            let b = p1.1 as f32 - m * p1.0 as f32;

            let mut x: usize = 0;
            let mut y = b;

            while x < w {
                let yy = y.round() as usize;
                if p1.0 > p2.0 {
                    for i in 0..yy.min(h) {
                        terrain[i][x] += 1.0;
                    }
                    for i in yy..h {
                        terrain[i][x] -= 1.0;
                    }
                } else {
                    for i in 0..yy.min(h) {
                        terrain[i][x] -= 1.0;
                    }
                    for i in yy..h {
                        terrain[i][x] += 1.0;
                    }
                }

                x += 1;
                y += m;
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            terrain[i][j] = terrain[i][j] / times as f32;
        }
    }

    println!("{:?}", terrain);
}
