use macroquad::input;
use macroquad::prelude::*;

const Si: i16 = 16;
const S: f32 = Si as f32;
const S2: f32 = S + 2.;

type Point = (i16, i16);

const BG_COLOR: Color = color_u8!(15, 15, 23, 255);
const TERRAIN_TINT: Color = color_u8!(255, 255, 255, 150);

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

    let SCR_W: f32 = screen_width();
    let SCR_H: f32 = screen_height();

    let gamecam = Camera2D {
        zoom: vec2(1. / SCR_W * 4., 1. / SCR_H * 4.),
        target: vec2(SCR_W / 4., SCR_H / 4.),
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

    let mut search_buffer: Vec<(usize, usize)> = vec![(0, 0); maph * mapw];

    let mut open_tile = |x: usize, y: usize, val: Option<bool>| {
        // clamp x y
        let x = if x >= mapw { mapw - 1 } else { x };
        let y = if y >= maph { maph - 1 } else { y };

        if let Some(v) = val {
            let mut j = 0;

            search_buffer[j] = (x, y);
            j += 1;

            while j > 0 {
                let (xx, yy) = search_buffer[j - 1];
                j -= 1;

                if open[yy][xx] {
                    continue;
                };

                open[yy][xx] = true;

                if auras[yy][xx] > 0 {
                    continue;
                }

                if yy < maph - 1 && !open[yy + 1][xx] {
                    search_buffer[j] = (xx, yy + 1);
                    j += 1;
                }
                if xx < mapw - 1 && !open[yy][xx + 1] {
                    search_buffer[j] = (xx + 1, yy);
                    j += 1;
                }
                if yy > 0 && !open[yy - 1][xx] {
                    search_buffer[j] = (xx, yy - 1);
                    j += 1;
                }
                if xx > 0 && !open[yy][xx - 1] {
                    search_buffer[j] = (xx - 1, yy);
                    j += 1;
                }
            }
        }

        open[y][x]
    };

    let mut mouse_pos = input::mouse_position();

    loop {
        // PROCESS INPUT
        mouse_pos = input::mouse_position();
        let mouse_pos_world = &gamecam.screen_to_world(mouse_pos.into());
        let mouse_tile = (mouse_pos_world.x as i16 / Si, mouse_pos_world.y as i16 / Si);

        let left_click = input::is_mouse_button_pressed(MouseButton::Left);
        if left_click {
            open_tile(mouse_tile.0 as usize, mouse_tile.1 as usize, Some(true));
        }

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
                    TERRAIN_TINT,
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
        set_default_camera();

        // draw aura
        for i in 0..maph {
            for j in 0..mapw {
                let t = auras[i][j];
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

        // draw fog
        for i in 0..maph {
            for j in 0..mapw {
                let t = open_tile(j, i, None);

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
