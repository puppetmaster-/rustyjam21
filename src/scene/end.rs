use crate::{MainState, FONT_COLOR, TITLE_ZOOM};
use macroquad::prelude::*;
use quad_snd::decoder;
use quad_snd::mixer::{SoundMixer, Volume, SoundId};

const MUSIC_BYTES: &[u8] = include_bytes!("../../assets/music/end.ogg");

pub struct End {
    camera: Camera2D,
    font: Font,
    text1: Vec<String>,
    start: bool,
    sound_id: Option<SoundId>,
}

impl End {
    pub async fn init() -> End {
        let camera = Camera2D {
            zoom: vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM / screen_height() * 2.0),
            target: vec2(0.0, 0.0),
            ..Default::default()
        };
        // todo write end text
        let font = load_ttf_font_from_bytes(include_bytes!("../../assets/fonts/GothicPixels.ttf"));
        let t1 = "You have found the exit!\n\nThe world with all its illusions\nis waiting for you.\n\nThanks for playing my game.\n\n";
        let text1 = t1.to_string().split('\n').map(String::from).collect();
        End { 
            camera, 
            font: font.unwrap(),
            text1, 
            start: true,
            sound_id: None
        }
    }

    pub fn run(&mut self, mixer: &mut SoundMixer) -> Option<MainState> {
        if self.start {
            let id = mixer.play(decoder::read_ogg(MUSIC_BYTES).unwrap());
            mixer.set_volume(id, Volume(0.6));
            self.start = false;
            self.sound_id = Some(id);
        }
        update_camera(self, vec2(0.0, 0.0));
        set_camera(&self.camera);
        set_default_camera();
        let tp = TextParams {
            font: self.font,
            font_size: 80,
            font_scale: 0.5,
            font_scale_aspect: 1.0,
            color: FONT_COLOR,
        };
        for (i, line) in self.text1.iter().enumerate() {
            draw_text_ex(
                line,
                (screen_width() / 2.0) - 350.0,
                (screen_height() / 2.0) - 350.0 + i as f32 * 80.0,
                tp,
            );
        }

        if get_last_key_pressed().is_some() {
            mixer.stop(self.sound_id.unwrap());
            return Some(MainState::TITLE);
        }
        None
    }
}

fn update_camera(scene: &mut End, new_target: Vec2) {
    scene.camera.target = new_target;
    scene.camera.zoom = vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM / screen_height() * 2.0);
}
