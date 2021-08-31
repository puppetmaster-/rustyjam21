use crate::{MainState, TITLE_ZOOM};
use keyframe::functions::{EaseIn, EaseInOut, EaseOut, Linear};
use keyframe::Keyframe;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use crate::tilemap::tile_animation::TileAnim;
use std::time::Duration;
use crate::tilemap::Tilemap;
use crate::utils::tween::Tween;

pub struct Title {
    title: Texture2D,
    camera: Camera2D,
    start: bool,
    player: TileAnim,
    spritesheet: Texture2D,
    game_name: Texture2D,
    animations: Vec<Tween>,
}

impl Title {
    pub async fn init() -> Title {
        let camera = Camera2D {
            zoom: vec2(TITLE_ZOOM / screen_width() * 2.0, -TITLE_ZOOM/ screen_height() * 2.0),
            target: vec2(67.0, 67.0),
            ..Default::default()
        };
        let image = Image::from_file_with_format(include_bytes!("../../assets/images/title.png"), None);
        let title: Texture2D = Texture2D::from_image(&image);
        title.set_filter(FilterMode::Nearest);
        let player_tilemap = Tilemap::new(Rect::new(0.0, 0.0, 64.0, 128.0), 8, 16, 8, 8);
        let player = TileAnim::new(&player_tilemap, &[16, 17, 18, 19], vec![
            Duration::from_millis(100),
            Duration::from_millis(200),
            Duration::from_millis(500),
            Duration::from_millis(400)]);
        let spritesheet = get_player_spritesheet();
        let game_name = get_name_texture();
        let animations = get_tween();
        Title {
            camera,
            title,
            start: true,
            player,
            spritesheet,
            game_name,
            animations
        }
    }

    pub fn run(&mut self) -> Option<MainState> {

        self.player.advance();
        self.animations[0].update();
        self.animations[1].update();
        self.animations[2].update();

        set_camera(&self.camera);
        draw_texture_ex(self.title ,0.0, 0.0, WHITE, Default::default());
        draw_texture_ex(
            self.spritesheet,
            110.0,
            56.0,
            WHITE,
            DrawTextureParams {
                source: self.player.source(),
                ..Default::default()
            },
        );
        draw_texture_ex(self.game_name ,5.0, 10.0 + self.animations[0].value(), WHITE, Default::default());
        match process_action(self){
            None => {None}
            Some(state) => {
                //mixer.stop((self.sound_id.unwrap()));
                Some(state)
            }
        }
    }
    pub fn reset(&mut self){
        self.start = true;
    }
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

fn get_player_spritesheet() -> Texture2D {
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/player.png"), Some(ImageFormat::Png));
    let texture: Texture2D = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}
fn get_name_texture() -> Texture2D {
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/name.png"), Some(ImageFormat::Png));
    let texture: Texture2D = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}

fn get_tween() -> Vec<Tween>{
    let tween1 = Tween::from_keyframes(
        vec![Keyframe::new(0.0, 0.0, EaseOut), Keyframe::new(8.0, 0.5, EaseOut),Keyframe::new(0.0, 1.0, EaseInOut)],
        0, 3,true);
    let tween2 = Tween::from_keyframes(
        vec![Keyframe::new(0.0, 0.0, EaseOut), Keyframe::new(4.0, 0.5, EaseOut), Keyframe::new(0.0, 1.0, EaseIn)],
        0, 2, true);
    let tween3 = Tween::from_keyframes(
        vec![Keyframe::new(0.0, 0.0, Linear), Keyframe::new(6.283_185_5, 1.0, Linear)],
        0, 10, true, );
    vec![tween1, tween2, tween3]
}