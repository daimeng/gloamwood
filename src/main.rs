use macroquad::input;
use macroquad::prelude::*;
mod mapgen;

const Si: i16 = 16;
const S: f32 = Si as f32;
const S2: f32 = S + 2.;

type Point = (i16, i16);

const BG_COLOR: Color = color_u8!(15, 15, 23, 255);
const TERRAIN_TINT: Color = color_u8!(255, 255, 255, 150);

struct WorldMap {
    mapw: usize,
    maph: usize,
    terrains: Vec<Vec<i16>>,
    monsters: Vec<Vec<i16>>,
    auras: Vec<Vec<i16>>,
    open: Vec<Vec<bool>>,
    search_buffer: Vec<(usize, usize)>,
}

impl WorldMap {
    fn new(mapw: usize, maph: usize) -> Self {
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

    fn set_monster(&mut self, x: usize, y: usize, n: i16) {
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

    fn open_tile(&mut self, x: usize, y: usize, val: Option<bool>) -> bool {
        // clamp x y
        let x = if x >= self.mapw { self.mapw - 1 } else { x };
        let y = if y >= self.maph { self.maph - 1 } else { y };

        if let Some(v) = val {
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
        }

        self.open[y][x]
    }
}

#[macroquad::main("Gloamwood")]
async fn main() {
    let tiles_tex = load_texture("assets/tiles.png").await.unwrap();
    tiles_tex.set_filter(FilterMode::Nearest);
    let chars_tex = load_texture("assets/chars.png").await.unwrap();
    chars_tex.set_filter(FilterMode::Nearest);

    let mut score = 0;
    let mut last_update = get_time();
    // let mut game_over = false;

    // let up = (0, -1);
    // let down = (0, 1);
    // let right = (1, 0);
    // let left = (-1, 0);

    let mapw = 30;
    let maph = 16;
    let scale = 2.;

    request_new_screen_size(mapw as f32 * S * scale, maph as f32 * S * scale);

    let mut gamecam = Camera2D {
        zoom: vec2(1. / screen_width() * 4., 1. / screen_height() * 4.),
        target: vec2(screen_width() / 4., screen_height() / 4.),
        ..Default::default()
    };

    let mut genterrains = vec![vec![0f32; mapw]; maph];
    mapgen::genmap_fissure(&mut genterrains);
    // println!("{:?}", &genterrains);

    let terrains: Vec<Vec<i16>> = genterrains
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| (c.max(0.) * 100.).round() as i16)
                .collect()
        })
        .collect();

    let dest_size = Some(vec2(S, S));
    let mut world = WorldMap::new(mapw, maph);
    world.terrains = terrains;

    // ██╗███╗   ██╗██╗████████╗
    // ██║████╗  ██║██║╚══██╔══╝
    // ██║██╔██╗ ██║██║   ██║
    // ██║██║╚██╗██║██║   ██║
    // ██║██║ ╚████║██║   ██║
    // ╚═╝╚═╝  ╚═══╝╚═╝   ╚═╝
    //
    world.set_monster(3, 2, 1);
    let mut mouse_pos = input::mouse_position();

    loop {
        // adjust camera in case of screen size changes
        gamecam.zoom.x = 1. / screen_width() * 4.;
        gamecam.zoom.y = 1. / screen_height() * 4.;
        gamecam.target.x = screen_width() / 4.;
        gamecam.target.y = screen_height() / 4.;

        // ██╗███╗   ██╗██████╗ ██╗   ██╗████████╗
        // ██║████╗  ██║██╔══██╗██║   ██║╚══██╔══╝
        // ██║██╔██╗ ██║██████╔╝██║   ██║   ██║
        // ██║██║╚██╗██║██╔═══╝ ██║   ██║   ██║
        // ██║██║ ╚████║██║     ╚██████╔╝   ██║
        // ╚═╝╚═╝  ╚═══╝╚═╝      ╚═════╝    ╚═╝
        //
        mouse_pos = input::mouse_position();
        let mouse_pos_world = &gamecam.screen_to_world(mouse_pos.into());
        let mouse_tile = (mouse_pos_world.x as i16 / Si, mouse_pos_world.y as i16 / Si);

        let left_click = input::is_mouse_button_pressed(MouseButton::Left);
        if left_click {
            world.open_tile(mouse_tile.0 as usize, mouse_tile.1 as usize, Some(true));
        }

        clear_background(BG_COLOR);

        set_camera(&gamecam);

        // let game_size = screen_width().min(screen_height());
        // let offset_x = (screen_width() - game_size) / 2. + 10.;
        // let offset_y = (screen_height() - game_size) / 2. + 10.;
        // let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

        // ██████╗ ██████╗  █████╗ ██╗    ██╗    ████████╗███████╗██████╗ ██████╗  █████╗ ██╗███╗   ██╗
        // ██╔══██╗██╔══██╗██╔══██╗██║    ██║    ╚══██╔══╝██╔════╝██╔══██╗██╔══██╗██╔══██╗██║████╗  ██║
        // ██║  ██║██████╔╝███████║██║ █╗ ██║       ██║   █████╗  ██████╔╝██████╔╝███████║██║██╔██╗ ██║
        // ██║  ██║██╔══██╗██╔══██║██║███╗██║       ██║   ██╔══╝  ██╔══██╗██╔══██╗██╔══██║██║██║╚██╗██║
        // ██████╔╝██║  ██║██║  ██║╚███╔███╔╝       ██║   ███████╗██║  ██║██║  ██║██║  ██║██║██║ ╚████║
        // ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝        ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝
        //
        for i in 0..maph {
            for j in 0..mapw {
                let t = world.terrains[i][j];
                let trow = t / 16;
                let tmod = t - trow * 16;

                draw_texture_ex(
                    &tiles_tex,
                    S * j as f32,
                    S * i as f32,
                    TERRAIN_TINT,
                    DrawTextureParams {
                        dest_size,
                        source: Some(Rect::new(S * tmod as f32, S * trow as f32, S, S)),
                        ..Default::default()
                    },
                );
            }
        }

        // ██████╗ ██████╗  █████╗ ██╗    ██╗    ███╗   ███╗ ██████╗ ███╗   ██╗███████╗████████╗███████╗██████╗ ███████╗
        // ██╔══██╗██╔══██╗██╔══██╗██║    ██║    ████╗ ████║██╔═══██╗████╗  ██║██╔════╝╚══██╔══╝██╔════╝██╔══██╗██╔════╝
        // ██║  ██║██████╔╝███████║██║ █╗ ██║    ██╔████╔██║██║   ██║██╔██╗ ██║███████╗   ██║   █████╗  ██████╔╝███████╗
        // ██║  ██║██╔══██╗██╔══██║██║███╗██║    ██║╚██╔╝██║██║   ██║██║╚██╗██║╚════██║   ██║   ██╔══╝  ██╔══██╗╚════██║
        // ██████╔╝██║  ██║██║  ██║╚███╔███╔╝    ██║ ╚═╝ ██║╚██████╔╝██║ ╚████║███████║   ██║   ███████╗██║  ██║███████║
        // ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝     ╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚══════╝
        //
        for i in 0..maph {
            for j in 0..mapw {
                let t = world.monsters[i][j];
                let trow = t / 16;
                let tmod = t - trow * 16;

                draw_texture_ex(
                    &chars_tex,
                    S * j as f32,
                    S * i as f32,
                    WHITE,
                    DrawTextureParams {
                        dest_size,
                        source: Some(Rect::new(S * tmod as f32, S * trow as f32, S, S)),
                        ..Default::default()
                    },
                );
            }
        }
        set_default_camera();

        // ██████╗ ██████╗  █████╗ ██╗    ██╗     █████╗ ██╗   ██╗██████╗  █████╗
        // ██╔══██╗██╔══██╗██╔══██╗██║    ██║    ██╔══██╗██║   ██║██╔══██╗██╔══██╗
        // ██║  ██║██████╔╝███████║██║ █╗ ██║    ███████║██║   ██║██████╔╝███████║
        // ██║  ██║██╔══██╗██╔══██║██║███╗██║    ██╔══██║██║   ██║██╔══██╗██╔══██║
        // ██████╔╝██║  ██║██║  ██║╚███╔███╔╝    ██║  ██║╚██████╔╝██║  ██║██║  ██║
        // ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
        //
        for i in 0..maph {
            for j in 0..mapw {
                let t = world.auras[i][j];
                if t == 0 {
                    continue;
                }

                draw_text_ex(
                    &format!("{t}"),
                    S * 2. * j as f32 + 12.,
                    S * 2. * i as f32 + 24.,
                    TextParams {
                        font_size: 24,
                        color: Color::from_rgba(255, 255, 255, 200),
                        ..Default::default()
                    },
                );
            }
        }

        // ██████╗ ██████╗  █████╗ ██╗    ██╗    ███████╗ ██████╗  ██████╗
        // ██╔══██╗██╔══██╗██╔══██╗██║    ██║    ██╔════╝██╔═══██╗██╔════╝
        // ██║  ██║██████╔╝███████║██║ █╗ ██║    █████╗  ██║   ██║██║  ███╗
        // ██║  ██║██╔══██╗██╔══██║██║███╗██║    ██╔══╝  ██║   ██║██║   ██║
        // ██████╔╝██║  ██║██║  ██║╚███╔███╔╝    ██║     ╚██████╔╝╚██████╔╝
        // ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝     ╚═╝      ╚═════╝  ╚═════╝
        //
        for i in 0..maph {
            for j in 0..mapw {
                let t = world.open[i][j];

                if t == false {
                    draw_rectangle(
                        S * 2. * j as f32,
                        S * 2. * i as f32,
                        S * 2.,
                        S * 2.,
                        Color::from_rgba(0, 0, 0, 255),
                    );

                    draw_rectangle_lines(
                        S * 2. * j as f32,
                        S * 2. * i as f32,
                        S * 2.,
                        S * 2.,
                        2.,
                        Color::from_rgba(50, 50, 50, 255),
                    );
                }
            }
        }
        next_frame().await;
    }
}
