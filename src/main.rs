mod entity;
mod scene;
mod tilemap;
mod utils;

use macroquad::prelude::*;

use crate::scene::title::Title;
use crate::scene::game::Game;
use crate::scene::end::End;

use quad_snd::decoder;
use quad_snd::mixer::{Volume, SoundMixer, PlaybackStyle};

const FONT_COLOR: Color = color_u8!(202, 202, 202, 255);
const GAME_ZOOM: f32 = 6.0;
const TITLE_ZOOM: f32 = 6.0;
const DEBUG: bool = false;

const MUSIC_BYTES: &[u8] = include_bytes!("../assets/music/start.ogg");

#[macroquad::main(window_conf)]
async fn main() {
    let mut main_state = MainState::TITLE;
    let mut title = Title::init().await;
    let mut game = Game::init().await;
    let mut end = End::init().await;
    let mut mixer = SoundMixer::new();
    let mut sound = decoder::read_ogg(MUSIC_BYTES).unwrap();
    sound.playback_style = PlaybackStyle::Looped;
    let sound_id = mixer.play(sound);
    mixer.set_volume(sound_id, Volume(0.6));
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
                        mixer.stop(sound_id);
                    }
                    main_state = gs
                }
            }
            MainState::END => {
                if let Some(gs) = end.run(&mut mixer) {
                    if gs == MainState::TITLE {
                        title.reset();
                    }
                    main_state = gs
                }
            }
            _ => {}
        }
        mixer.frame();
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