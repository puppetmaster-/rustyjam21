mod constants;
mod entity;
mod scene;
mod tilemap;
mod utils;

use macroquad::prelude::*;

use crate::scene::title::Title;
use crate::scene::game::Game;
use std::time::Duration;
use macroquad::prelude::scene::clear;
use miniquad::gl::glClearColor;

const BACKGROUND_COLOR: Color = color_u8!(202, 202, 202, 255);
const GAME_ZOOM: f32 = 6.0;
const DEBUG: bool = true;

#[macroquad::main(window_conf)]
async fn main() {
    let mut main_state = MainState::GAME;
    let mut title = Title::init().await;
    let mut game = Game::init().await;
    loop {
        clear_background(BLACK);
        match main_state {
            MainState::EXIT => break,
            MainState::TITLE => {
                if let Some(gs) = title.run() {
                    main_state = gs
                }
            }
            MainState::GAME => {
                if let Some(gs) = game.run() {
                    main_state = gs
                }
            }
            _ => {}
        }
        next_frame().await;
        std:: thread ::sleep(Duration::from_millis(10));
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "RustyJam2021".to_owned(),
        window_width: 800,
        window_height: 800,
        high_dpi: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MainState {
    TITLE,
    STORY,
    GAME,
    EXIT,
    RUN,
    END,
}