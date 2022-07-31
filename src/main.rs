use macroquad::prelude::*;

mod structs;

use self::structs::{
    mainstate::MainState,
    ship::Ship,
    asteroid::Asteroid,
    params::{ GameParams, ShipParams, AsteroidParams },
};

// abbreviations:
//     win:     window
//     conf:    config
//     pos:     position
//     vel:     vector


#[macroquad::main(win_conf)]
async fn main() {
    let params = {
        let ship = ShipParams::default();
        let asteroid = AsteroidParams::default();
        GameParams {
            ship,
            asteroid,
            asteroid_spawning_rate: 1
        }
    };
    let mut state = MainState::new(params);

    loop {
        clear_background(BLACK);
        state.handle_input();
        state.update();
        state.draw();
        next_frame().await
    }
}

fn win_conf() -> Conf {
    Conf {
        window_title: "Asteroids in Rust with macroquad".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

