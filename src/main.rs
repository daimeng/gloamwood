use macroquad::prelude::*;

const SQUARES: i16 = 16;

const S: f32 = 16.;
const S2: f32 = S + 2.;

type Point = (i16, i16);

const BG_COLOR: Color = color_u8!(20, 20, 20, 0);

#[macroquad::main("Gloamwood")]
async fn main() {
    let tiles_tex = load_texture("assets/tiles.png").await.unwrap();
    tiles_tex.set_filter(FilterMode::Nearest);
    let chars_tex = load_texture("assets/chars.png").await.unwrap();
    chars_tex.set_filter(FilterMode::Nearest);

    let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
    let mut score = 0;
    let mut last_update = get_time();
    // let mut game_over = false;

    // let up = (0, -1);
    // let down = (0, 1);
    // let right = (1, 0);
    // let left = (-1, 0);

    let SCR_W: f32 = screen_width();
    let SCR_H: f32 = screen_height();

    let gamecam = Camera2D {
        zoom: vec2(1. / SCR_W * 6., 1. / SCR_H * 6.),
        target: vec2(SCR_W / 6., SCR_H / 6.),
        ..Default::default()
    };

    let mapw = 30;
    let maph = 16;

    let terrains = vec![vec![1; mapw]; maph];
    let mut monsters = vec![vec![0; mapw]; maph];
    let mut auras = vec![vec![0; mapw]; maph];
    let mut open = vec![vec![false; mapw]; maph];
    let dest_size = Some(vec2(S, S));

    let mut set_monster = |x: usize, y: usize, n: i32| {
        monsters[y][x] = n;
        for yi in 0..3 {
            let yy = y + yi;
            if yy == 0 || yy > maph {
                continue;
            }

            for xi in 0..3 {
                let xx = x + xi;
                if xx == 0 || xx > mapw {
                    continue;
                }

                auras[yy - 1][xx - 1] += n;
            }
        }
    };

    set_monster(3, 2, 1);

    loop {
        clear_background(BG_COLOR);

        set_camera(&gamecam);

        // let game_size = screen_width().min(screen_height());
        // let offset_x = (screen_width() - game_size) / 2. + 10.;
        // let offset_y = (screen_height() - game_size) / 2. + 10.;
        // let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

        // draw terrain grid
        for i in 0..maph {
            for j in 0..mapw {
                let t = terrains[i][j];
                let trow = t / 16;
                let tmod = t - trow * 16;

                draw_texture_ex(
                    &tiles_tex,
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

        // draw monsters
        for i in 0..maph {
            for j in 0..mapw {
                let t = monsters[i][j];
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

        // draw fog
        for i in 0..maph {
            for j in 0..mapw {
                let t = open[i][j];

                if t == false {
                    draw_rectangle(
                        S * j as f32,
                        S * i as f32,
                        S,
                        S,
                        Color::from_rgba(0, 0, 0, 255),
                    );
                }
            }
        }

        set_default_camera();

        // draw aura
        for i in 0..maph {
            for j in 0..mapw {
                let t = auras[i][j];

                draw_text_ex(
                    &format!("{t}"),
                    S * 3. * j as f32 + 16.,
                    S * 3. * i as f32 + 36.,
                    TextParams {
                        font_size: 42,
                        color: Color::from_rgba(255, 255, 255, 200),
                        ..Default::default()
                    },
                );
            }
        }

        next_frame().await;
    }
}
