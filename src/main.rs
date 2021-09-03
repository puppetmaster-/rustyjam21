mod entity;
mod scene;
mod tilemap;
mod utils;

use macroquad::{audio, prelude::*};

use crate::scene::title::Title;
use crate::scene::game::Game;
use crate::scene::end::End;
use macroquad::audio::PlaySoundParams;

const FONT_COLOR: Color = color_u8!(202, 202, 202, 255);
const GAME_ZOOM: f32 = 6.0;
const TITLE_ZOOM: f32 = 6.0;
const DEBUG: bool = false;

const MUSIC_START_BYTES: &[u8] = include_bytes!("../assets/music/start.ogg");
const MUSIC_END_BYTES: &[u8] = include_bytes!("../assets/music/end.ogg");

#[macroquad::main(window_conf)]
async fn main() {
    let mut main_state = MainState::TITLE;
    let mut title = Title::init().await;
    let mut game = Game::init().await;
    let mut end = End::init().await;
    let music_start = audio::load_sound_from_bytes(MUSIC_START_BYTES).await.unwrap();
    let music_end  = audio::load_sound_from_bytes(MUSIC_END_BYTES).await.unwrap();
    audio::play_sound(music_start,PlaySoundParams{ looped: true, volume: 0.1 });
    loop {
        clear_background(BLACK);
        match main_state {
            MainState::EXIT => break,
            MainState::TITLE => {
                if let Some(gs) = title.run() {
                    if gs == MainState::GAME {
                        game.reset();
                    }
                    main_state = gs
                }
            }
            MainState::GAME => {
                if let Some(gs) = game.run() {
                    if gs == MainState::END {
                        audio::stop_sound(music_start);
                        audio::play_sound(music_end,PlaySoundParams{ looped: true, volume: 0.1 });
                    }
                    main_state = gs
                }
            }
            MainState::END => {
                if let Some(gs) = end.run() {
                    if gs == MainState::TITLE {
                        audio::stop_sound(music_end);
                        audio::play_sound(music_start,PlaySoundParams{ looped: true, volume: 0.1 });
                        title.reset();
                    }
                    main_state = gs
                }
            }
            _ => {}
        }
        next_frame().await;
        //std:: thread ::sleep(Duration::from_millis(10));
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