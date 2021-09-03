use crate::{MainState, FONT_COLOR, TITLE_ZOOM};
use macroquad::prelude::*;

pub struct End {
    camera: Camera2D,
    font: Font,
    text1: Vec<String>,
}

impl End {
    pub async fn init() -> End {
        let camera = Camera2D {
            zoom: vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM / screen_height() * 2.0),
            target: vec2(0.0, 0.0),
            ..Default::default()
        };
        let font = load_ttf_font_from_bytes(include_bytes!("../../assets/fonts/Born2bSportyAkan.ttf"));
        let t1 = "You have found the exit!\n\nThe world with all its illusions\nis waiting for you.\n\nThanks for playing.\n\n";
        let text1 = t1.to_string().split('\n').map(String::from).collect();
        End { 
            camera, 
            font: font.unwrap(),
            text1,
        }
    }

    pub fn run(&mut self) -> Option<MainState> {
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
                (screen_width() / 2.0) - 220.0,
                (screen_height() / 2.0) - 220.0 + i as f32 * 80.0,
                tp,
            );
        }

        if get_last_key_pressed().is_some() {
            return Some(MainState::TITLE);
        }
        None
    }
}

fn update_camera(scene: &mut End, new_target: Vec2) {
    scene.camera.target = new_target;
    scene.camera.zoom = vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM / screen_height() * 2.0);
}
