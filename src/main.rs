use effect::GameEffect;
use macroquad::input;
use macroquad::prelude::*;
use macroquad::time;
use macroquad::ui::hash;
use macroquad::ui::root_ui;
mod effect;
mod entities;
mod mapgen;
mod spawns;
mod worldmap;

const Si: i16 = 16;
const S: f32 = Si as f32;

const OUTER_BG_COLOR: Color = color_u8!(10, 10, 15, 255);
const BG_COLOR: Color = color_u8!(25, 25, 37, 255);
const TERRAIN_TINT: Color = color_u8!(255, 255, 255, 220);

const FOG_LINE: f32 = 1.;

#[macroquad::main("Gloamwood")]
async fn main() {
    set_default_filter_mode(FilterMode::Nearest);
    let font = load_ttf_font("assets/SyneMono-Regular.ttf").await.unwrap();
    let tiles_tex = load_texture("assets/tiles.png").await.unwrap();
    let chars_tex = load_texture("assets/chars.png").await.unwrap();
    let interface_tex = load_texture("assets/interface.png").await.unwrap();

    let mapw = 30;
    let maph = 16;
    let mines = 120;
    // let mapw = 50;
    // let maph = 25;
    // let mines = 300;
    let scale = 2.;
    let scalex2 = scale * 2.;

    request_new_screen_size(mapw as f32 * S * scale, maph as f32 * S * scale + 100.);

    let mut gamecam = Camera2D {
        zoom: vec2(
            1. / screen_width() * scalex2,
            1. / screen_height() * scalex2,
        ),
        target: vec2(screen_width() / scalex2, screen_height() / scalex2 - 50.),
        ..Default::default()
    };

    let dest_size = Some(vec2(S, S));
    let dest_size2 = Some(vec2(S * 2., S * 2.));

    // ██╗███╗   ██╗██╗████████╗
    // ██║████╗  ██║██║╚══██╔══╝
    // ██║██╔██╗ ██║██║   ██║
    // ██║██║╚██╗██║██║   ██║
    // ██║██║ ╚████║██║   ██║
    // ╚═╝╚═╝  ╚═══╝╚═╝   ╚═╝
    //
    let init = |mapw: usize, maph: usize, mines: usize| {
        let mut genterrains = vec![vec![0f32; mapw]; maph];
        mapgen::genmap_fissure(&mut genterrains);
        // println!("{:?}", &genterrains);

        let terrains: Vec<Vec<i16>> = genterrains
            .iter()
            .map(|row| {
                row.iter()
                    // .map(|c| (c.abs() * 90.).round() as i16)
                    .map(|c| ((c.max(-0.06) + 0.06) * 60.).round().min(10.) as i16)
                    .collect()
            })
            .collect();

        let mut w = worldmap::WorldMap::new(mapw, maph);
        w.set_terrain(terrains);
        w.init(mines);
        w
    };

    let animating = false;
    let mut world = init(mapw, maph, mines);

    let mut mouse_pos;
    let mut menu_open = false;

    let mut last_game_time = time::get_time();
    let mut left_click = false;
    let mut right_click = false;
    let mut right_click_t = last_game_time;
    let mut mid_click = false;
    let mut right_down = false;
    let mut flagged_t = last_game_time;
    let min_flag_cd = 0.02;
    let mut flag_cd = min_flag_cd;
    loop {
        let t = time::get_time();
        let dt = last_game_time - t;

        // adjust camera in case of screen size changes
        gamecam.zoom.x = 1. / screen_width() * scalex2;
        gamecam.zoom.y = 1. / screen_height() * scalex2;
        gamecam.target.x = screen_width() / scalex2;
        gamecam.target.y = screen_height() / scalex2 - 25.;

        // ██╗███╗   ██╗██████╗ ██╗   ██╗████████╗
        // ██║████╗  ██║██╔══██╗██║   ██║╚══██╔══╝
        // ██║██╔██╗ ██║██████╔╝██║   ██║   ██║
        // ██║██║╚██╗██║██╔═══╝ ██║   ██║   ██║
        // ██║██║ ╚████║██║     ╚██████╔╝   ██║
        // ╚═╝╚═╝  ╚═══╝╚═╝      ╚═════╝    ╚═╝
        // INPUT
        mouse_pos = input::mouse_position();
        let mouse_pos_world = &gamecam.screen_to_world(mouse_pos.into());
        let mouse_tile = (
            (mouse_pos_world.x / S).floor() as i16,
            (mouse_pos_world.y / S).floor() as i16,
        );

        // Exit on escape key
        if input::is_key_pressed(KeyCode::Escape) {
            break;
        }

        left_click = input::is_mouse_button_pressed(MouseButton::Left);
        right_click = input::is_mouse_button_pressed(MouseButton::Right);
        mid_click = input::is_mouse_button_pressed(MouseButton::Middle);
        right_down = input::is_mouse_button_down(MouseButton::Right);
        if input::is_mouse_button_released(MouseButton::Right) {
            flag_cd = min_flag_cd;
        }
        if left_click {
            println!("Clicked: {:?} {:?}", mouse_pos_world, mouse_tile);
        }

        // open menu if clicked
        if root_ui().button(vec2(0., 0.), "Menu") {
            menu_open = true;
        };

        if menu_open {
            root_ui().window(
                hash!(),
                vec2(screen_width() / 2. - 100., screen_width() / 2. - 400.),
                vec2(200., 400.),
                |ui| {
                    // capture mouse clicks
                    let lclick = left_click;
                    let rclick = right_click;
                    let mclick = mid_click;
                    left_click = false;
                    right_click = false;
                    mid_click = false;

                    if ui.button(vec2(40., 350.), "Close") {
                        menu_open = false;
                    }
                },
            );
        }

        if world.game_over == 0 {
            if mouse_tile.0 >= 0 && mouse_tile.1 >= 0 {
                let x = mouse_tile.0 as usize;
                let y = mouse_tile.1 as usize;
                // OPEN tile
                if left_click {
                    // guard against accidental click
                    if world.flags[y][x] == 0 {
                        world.open_tile(x, y);
                    }
                }

                // CHORD tile
                if mid_click {
                    world.chord_tile(x, y);
                }

                // FLAG tile
                if right_click {
                    world.flag_tile_inc(x, y);
                    flagged_t = t;
                } else if t - right_click_t > 0.2 && right_down {
                    if t - flagged_t > flag_cd {
                        world.flag_tile_inc(x, y);
                        flagged_t = t;
                        // increase cd each time
                        flag_cd = flag_cd * 1.5 + 0.01;
                    }
                }

                // Number key flagging
                if input::is_key_pressed(KeyCode::Key0)
                    || input::is_key_pressed(KeyCode::Apostrophe)
                {
                    world.flag_tile(x, y, 0);
                } else if input::is_key_pressed(KeyCode::Key1) {
                    world.flag_tile(x, y, 1);
                } else if input::is_key_pressed(KeyCode::Key2) {
                    world.flag_tile(x, y, 2);
                } else if input::is_key_pressed(KeyCode::Key3) {
                    world.flag_tile(x, y, 3);
                } else if input::is_key_pressed(KeyCode::Key4) {
                    world.flag_tile(x, y, 4);
                } else if input::is_key_pressed(KeyCode::Key5) {
                    world.flag_tile(x, y, 5);
                } else if input::is_key_pressed(KeyCode::Key6) {
                    world.flag_tile(x, y, 6);
                } else if input::is_key_pressed(KeyCode::Key7) {
                    world.flag_tile(x, y, 7);
                } else if input::is_key_pressed(KeyCode::Key8) {
                    world.flag_tile(x, y, 8);
                } else if input::is_key_pressed(KeyCode::Key9) {
                    world.flag_tile(x, y, 9);
                }
            }
        }

        // upate last time trackers
        if right_click {
            right_click_t = t
        }
        last_game_time = t;

        // Restart
        let r_pressed = input::is_key_pressed(KeyCode::R);
        if r_pressed {
            world = init(mapw, maph, mines);
        }

        clear_background(OUTER_BG_COLOR);

        set_camera(&gamecam);

        draw_rectangle(0., 0., S * mapw as f32, S * maph as f32, BG_COLOR);

        // ██████╗ ██████╗  █████╗ ██╗    ██╗    ████████╗███████╗██████╗ ██████╗  █████╗ ██╗███╗   ██╗
        // ██╔══██╗██╔══██╗██╔══██╗██║    ██║    ╚══██╔══╝██╔════╝██╔══██╗██╔══██╗██╔══██╗██║████╗  ██║
        // ██║  ██║██████╔╝███████║██║ █╗ ██║       ██║   █████╗  ██████╔╝██████╔╝███████║██║██╔██╗ ██║
        // ██║  ██║██╔══██╗██╔══██║██║███╗██║       ██║   ██╔══╝  ██╔══██╗██╔══██╗██╔══██║██║██║╚██╗██║
        // ██████╔╝██║  ██║██║  ██║╚███╔███╔╝       ██║   ███████╗██║  ██║██║  ██║██║  ██║██║██║ ╚████║
        // ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝        ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝
        // TERRAIN
        for i in 0..maph {
            for j in 0..mapw {
                let terrain = world.terrains[i][j];
                let trow = terrain / 16;
                let tmod = terrain - trow * 16;
                let mut wavex = 0.;
                let mut wavey = 0.;
                if terrain == 0 {
                    wavey = (4. * (t as f32 + i as f32 / 20. + j as f32 / 3.)).sin();
                }
                if terrain == 1 {
                    wavex = (2. * (t as f32 + j as f32 / 20.)).sin() * 0.4;
                }

                draw_texture_ex(
                    &tiles_tex,
                    S * j as f32 + wavex,
                    S * i as f32 + wavey,
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
        // MONSTERS
        for i in 0..maph {
            for j in 0..mapw {
                let eid = world.entities[i][j];
                let ent = world.entity(j, i);
                if ent.breed == -1 {
                    continue;
                }
                let t = ent.level;
                let trow = t / 16;
                let tmod = t - trow * 16;

                if (&world.effects_store)[eid].contains(&Some(GameEffect::Vamp)) {
                    draw_circle(S * j as f32 + 8., S * i as f32 + 8., 6., RED);
                }

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

                if ent.breed < 1 {
                    continue;
                }

                draw_rectangle(S * j as f32, S * i as f32, ent.hp as f32, 1., RED);
            }
        }
        set_default_camera();

        // ██████╗ ██████╗  █████╗ ██╗    ██╗     █████╗ ██╗   ██╗██████╗  █████╗
        // ██╔══██╗██╔══██╗██╔══██╗██║    ██║    ██╔══██╗██║   ██║██╔══██╗██╔══██╗
        // ██║  ██║██████╔╝███████║██║ █╗ ██║    ███████║██║   ██║██████╔╝███████║
        // ██║  ██║██╔══██╗██╔══██║██║███╗██║    ██╔══██║██║   ██║██╔══██╗██╔══██║
        // ██████╔╝██║  ██║██║  ██║╚███╔███╔╝    ██║  ██║╚██████╔╝██║  ██║██║  ██║
        // ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
        // AURA
        for i in 0..maph {
            for j in 0..mapw {
                let t = world.auras[i][j];
                if t == 0 {
                    continue;
                }
                if world.entity(j, i).level > 0 {
                    continue;
                }

                if world.terrains[i][j] == 8 {
                    draw_text_ex(
                        "?",
                        S * 2. * j as f32 + 9.,
                        S * 2. * i as f32 + 23. + 52.,
                        TextParams {
                            font: Some(&font),
                            font_size: 24,
                            color: WHITE,
                            ..Default::default()
                        },
                    );
                    continue;
                }

                if t < 10 {
                    draw_text_ex(
                        &format!("{t}"),
                        S * 2. * j as f32 + 9.,
                        S * 2. * i as f32 + 23. + 52.,
                        TextParams {
                            font: Some(&font),
                            font_size: 24,
                            color: Color::from_rgba(255, 255, 255, 255),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_text_ex(
                        &format!("{t}"),
                        S * 2. * j as f32 + 3.,
                        S * 2. * i as f32 + 23. + 52.,
                        TextParams {
                            font: Some(&font),
                            font_size: 22,
                            color: Color::from_rgba(255, 255, 255, 255),
                            ..Default::default()
                        },
                    );
                }
            }
        }

        // ██████╗ ██████╗  █████╗ ██╗    ██╗    ███████╗ ██████╗  ██████╗
        // ██╔══██╗██╔══██╗██╔══██╗██║    ██║    ██╔════╝██╔═══██╗██╔════╝
        // ██║  ██║██████╔╝███████║██║ █╗ ██║    █████╗  ██║   ██║██║  ███╗
        // ██║  ██║██╔══██╗██╔══██║██║███╗██║    ██╔══╝  ██║   ██║██║   ██║
        // ██████╔╝██║  ██║██║  ██║╚███╔███╔╝    ██║     ╚██████╔╝╚██████╔╝
        // ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝     ╚═╝      ╚═════╝  ╚═════╝
        // FOG
        #[cfg(not(feature = "nofog"))]
        {
            for i in 0..maph {
                for j in 0..mapw {
                    let t = world.open[i][j];

                    if t == false {
                        draw_rectangle(
                            S * 2. * j as f32,
                            S * 2. * i as f32 + 50.,
                            S * 2.,
                            S * 2.,
                            BG_COLOR,
                        );

                        let terrain = world.terrains[i][j];
                        draw_texture_ex(
                            &tiles_tex,
                            S * 2. * j as f32,
                            S * 2. * i as f32 + 50.,
                            Color::from_rgba(0, 0, 0, 255),
                            DrawTextureParams {
                                dest_size: dest_size2,
                                source: Some(Rect::new(0., S, S * 2., S * 2.)),
                                ..Default::default()
                            },
                        );

                        if world.show_terrain[i][j] {
                            draw_texture_ex(
                                &tiles_tex,
                                S * 2. * j as f32,
                                S * 2. * i as f32 + 50.,
                                Color::from_rgba(255, 255, 255, 80),
                                DrawTextureParams {
                                    dest_size: dest_size2,
                                    source: Some(Rect::new(terrain as f32 * S, 0., S, S)),
                                    ..Default::default()
                                },
                            );
                        }

                        if i > 0 && world.open[i - 1][j] {
                            draw_line(
                                S * 2. * (j as i16) as f32,
                                S * 2. * (i as i16) as f32 + 50.,
                                S * 2. * (j as i16 + 1) as f32,
                                S * 2. * (i as i16) as f32 + 50.,
                                FOG_LINE,
                                WHITE,
                            )
                        }

                        if i < world.maph - 1 && world.open[i + 1][j] {
                            draw_line(
                                S * 2. * (j as i16) as f32,
                                S * 2. * (i as i16 + 1) as f32 + 50.,
                                S * 2. * (j as i16 + 1) as f32,
                                S * 2. * (i as i16 + 1) as f32 + 50.,
                                FOG_LINE,
                                WHITE,
                            )
                        }

                        if j < world.mapw - 1 && world.open[i][j + 1] {
                            draw_line(
                                S * 2. * (j as i16 + 1) as f32,
                                S * 2. * (i as i16) as f32 + 50.,
                                S * 2. * (j as i16 + 1) as f32,
                                S * 2. * (i as i16 + 1) as f32 + 50.,
                                FOG_LINE,
                                WHITE,
                            )
                        }

                        if j > 0 && world.open[i][j - 1] {
                            draw_line(
                                S * 2. * (j as i16) as f32,
                                S * 2. * (i as i16) as f32 + 50.,
                                S * 2. * (j as i16) as f32,
                                S * 2. * (i as i16 + 1) as f32 + 50.,
                                FOG_LINE,
                                WHITE,
                            )
                        }
                    }
                }
            }
        }

        // ██████╗ ██████╗  █████╗ ██╗    ██╗    ███████╗██╗      █████╗  ██████╗ ███████╗
        // ██╔══██╗██╔══██╗██╔══██╗██║    ██║    ██╔════╝██║     ██╔══██╗██╔════╝ ██╔════╝
        // ██║  ██║██████╔╝███████║██║ █╗ ██║    █████╗  ██║     ███████║██║  ███╗███████╗
        // ██║  ██║██╔══██╗██╔══██║██║███╗██║    ██╔══╝  ██║     ██╔══██║██║   ██║╚════██║
        // ██████╔╝██║  ██║██║  ██║╚███╔███╔╝    ██║     ███████╗██║  ██║╚██████╔╝███████║
        // ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝     ╚═╝     ╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝
        //
        for i in 0..maph {
            for j in 0..mapw {
                let t = world.flags[i][j];
                let trow = t / 16;
                let tmod = t - trow * 16;

                draw_texture_ex(
                    &interface_tex,
                    S * 2. * j as f32,
                    S * 2. * i as f32 + 50.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: dest_size2,
                        source: Some(Rect::new(S * tmod as f32, S * trow as f32, S, S)),
                        ..Default::default()
                    },
                );
            }
        }

        for i in 1..=9 {
            draw_text(
                &format!("{:02}x", world.counts[i]),
                100. * (i - 1) as f32 + 10.,
                screen_height() - 10.,
                32.,
                WHITE,
            );

            draw_texture_ex(
                &chars_tex,
                100. * (i - 1) as f32 + if world.counts[i] > 99 { 64. } else { 48. },
                screen_height() - 10. - 24.,
                WHITE,
                DrawTextureParams {
                    dest_size: dest_size2,
                    source: Some(Rect::new(S * i as f32, S * 0 as f32, S, S)),
                    ..Default::default()
                },
            );
        }

        if world.initialized {
            let (herox, heroy) = world.hero_pos;
            let heroid = world.entities[heroy][herox];

            draw_text(
                &format!("HP: {}", world.hero().hp),
                screen_width() / 2.,
                20.,
                36.,
                Color::new(1., 1., 1., 1.),
            );

            for (i, effect) in world.effects_store[heroid].iter().enumerate() {
                draw_rectangle_lines(
                    100. + 50. * i as f32,
                    5.,
                    36.,
                    36.,
                    2.,
                    Color::new(1., 1., 1., 1.),
                );

                match effect {
                    Some(effect) => {}
                    None => {}
                }
            }
        }

        if world.game_over == 2 {
            let center = get_text_center("Game Over", Option::None, 48, 1.0, 0.);
            draw_text(
                "Game Over",
                screen_width() / 2. - center.x,
                screen_height() / 2. - center.y,
                48.,
                Color::new(1., 1., 1., 0.8),
            );
        }

        next_frame().await;
    }
}
