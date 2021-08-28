use crate::{MainState, GAME_ZOOM};
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use quad_snd::decoder;
use quad_snd::mixer::{Volume, SoundMixer};

pub struct Title {
    title: Texture2D,
    camera: Camera2D,
    start: bool,
}

const MUSIC_BYTES: &[u8] = include_bytes!("../../assets/music/start.ogg");

impl Title {
    pub async fn init() -> Title {
        let camera = Camera2D {
            zoom: vec2(GAME_ZOOM / screen_width() * 2.0, -GAME_ZOOM / screen_height() * 2.0),
            target: vec2(0.0, 0.0),
            ..Default::default()
        };
        let image = Image::from_file_with_format(include_bytes!("../../assets/images/vollmond.png"), None);
        let title: Texture2D = Texture2D::from_image(&image);
        title.set_filter(FilterMode::Nearest);

        Title {
            camera,
            title,
            start: true,
        }
    }

    pub fn run(&mut self,mixer: &mut SoundMixer) -> Option<MainState> {
        if self.start {
            let id = mixer.play(decoder::read_ogg(MUSIC_BYTES).unwrap());
            mixer.set_volume(id, Volume(0.6));
            self.start = false;
        }
        update_camera(self, vec2(0.0, 0.0));
        set_camera(&self.camera);
        draw_texture_ex(self.title ,0.0, 0.0, WHITE, Default::default());
        set_default_camera();
        process_action(self)
    }
}

fn update_camera(scene: &mut Title, new_target: Vec2) {
    scene.camera.target = new_target;
    scene.camera.zoom = vec2(GAME_ZOOM, GAME_ZOOM);
}

fn process_action(_title: &mut Title) -> Option<MainState> {
    if get_last_key_pressed().is_some() {
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            #[cfg(not(target_arch = "wasm32"))]
            return Some(MainState::EXIT);
        } else {
            return Some(MainState::GAME);
        }
    }
    None
}
