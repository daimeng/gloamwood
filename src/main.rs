use macroquad::input;
use macroquad::prelude::*;
use macroquad::ui::hash;
use macroquad::ui::root_ui;
mod mapgen;
mod worldmap;

const Si: i16 = 16;
const S: f32 = Si as f32;

const BG_COLOR: Color = color_u8!(15, 15, 23, 255);
const TERRAIN_TINT: Color = color_u8!(255, 255, 255, 150);

#[macroquad::main("Gloamwood")]
async fn main() {
    let tiles_tex = load_texture("assets/tiles.png").await.unwrap();
    tiles_tex.set_filter(FilterMode::Nearest);
    let chars_tex = load_texture("assets/chars.png").await.unwrap();
    chars_tex.set_filter(FilterMode::Nearest);
    let interface_tex = load_texture("assets/interface.png").await.unwrap();
    interface_tex.set_filter(FilterMode::Nearest);

    let mapw = 30;
    let maph = 16;
    let mines = 99;
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
                    .map(|c| (c.max(0.) * 100.).round() as i16)
                    .collect()
            })
            .collect();

        let mut w = worldmap::WorldMap::new(mapw, maph);
        w.terrains = terrains;
        w.init(mines);
        w
    };

    let mut world = init(mapw, maph, mines);
    let mut mouse_pos;
    let mut menu_open = false;

    loop {
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
        //
        mouse_pos = input::mouse_position();
        let mouse_pos_world = &gamecam.screen_to_world(mouse_pos.into());
        let mouse_tile = (mouse_pos_world.x as i16 / Si, mouse_pos_world.y as i16 / Si);

        let mut left_click = input::is_mouse_button_pressed(MouseButton::Left);
        let right_click = input::is_mouse_button_pressed(MouseButton::Right);
        let mid_click = input::is_mouse_button_pressed(MouseButton::Middle);

        if menu_open {
            root_ui().window(hash!(), vec2(0., 0.), vec2(200., 400.), |ui| {
                if is_mouse_button_pressed(MouseButton::Left) {
                    left_click = false;
                }
                ui.button(vec2(0., 0.), "testing");
            });
        }

        if !world.game_over {
            // OPEN tile
            if left_click {
                world.open_tile(mouse_tile.0 as usize, mouse_tile.1 as usize);
            }
            // FLAG tile
            if right_click {
                world.flag_tile(mouse_tile.0 as usize, mouse_tile.1 as usize);
            }
            // CHORD tile
            if mid_click {
                world.chord_tile(mouse_tile.0 as usize, mouse_tile.1 as usize);
            }
        }

        // Restart
        let r_pressed = input::is_key_pressed(KeyCode::R);
        if r_pressed {
            world = init(mapw, maph, mines);
        }

        clear_background(BG_COLOR);

        set_camera(&gamecam);

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
                    S * 2. * i as f32 + 24. + 50.,
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
                        S * 2. * i as f32 + 50.,
                        S * 2.,
                        S * 2.,
                        Color::from_rgba(0, 0, 0, 255),
                    );

                    draw_rectangle_lines(
                        S * 2. * j as f32,
                        S * 2. * i as f32 + 50.,
                        S * 2.,
                        S * 2.,
                        2.,
                        Color::from_rgba(50, 50, 50, 255),
                    );
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

        if world.game_over {
            let center = get_text_center("Game Over", Option::None, 48, 1.0, 0.);
            draw_text(
                "Game Over",
                screen_width() / 2. - center.x,
                screen_height() / 2. - center.y,
                48.,
                WHITE,
            );
        }

        next_frame().await;
    }
}
