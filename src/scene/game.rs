use crate::entity::player::Player;
use crate::tilemap::Tilemap;
use crate::utils::tween::Tween;
use crate::{MainState, GAME_ZOOM, BACKGROUND_COLOR};
use macroquad::prelude::*;
use std::collections::HashMap;

const OFFSET_CAMERA: f32 = 0.0;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    WIN,
    MAP,
    DEAD,
}

pub struct Game {
    player: Player,
    map_texture: Texture2D,
    map_tilemap: Tilemap,
    tilemaps: HashMap<GameState, Tilemap>,
    camera: Camera2D,
    game_state: GameState,
    init_sidemap: bool,
    draw_sky: bool,
}

impl Game {
    pub async fn init() -> Game {
        let map_texture = get_map_texture();
        let map_tilemap = get_map_tilemap();
        let mut player = Player::new();
        player.reset(&map_tilemap);

        let camera = Camera2D {
            zoom: vec2(GAME_ZOOM / screen_width() * 2.0, -GAME_ZOOM / screen_height() * 2.0),
            target: Vec2::new(64.0,64.0),
            ..Default::default()
        };

        Game {
            player,
            map_texture,
            map_tilemap,
            tilemaps: get_tilemaps(),
            camera,
            game_state: GameState::MAP,
            init_sidemap: true,
            draw_sky: true,
        }
    }

    pub fn reset(&mut self) {
        self.tilemaps = get_tilemaps();
        self.game_state = GameState::MAP;
        self.init_sidemap = true;
    }

    pub fn run(&mut self) -> Option<MainState> {
        let mut main_state= None;
        if let Some(gs) = self.player.update(&mut self.map_tilemap) {
            match gs {
                GameState::WIN => {
                    main_state = Some(MainState::END);
                }
                GameState::DEAD => {
                    self.player.reset(&self.map_tilemap);
                }
                _ => {
                    self.game_state = gs;
                }
            }

        }
        update_camera(self, self.player.position());
        set_camera(&self.camera);
        self.map_tilemap.draw(self.map_texture, vec2(0.0, 0.0), None);
        self.player.draw();
        main_state
    }
}

fn update_camera(game: &mut Game, new_target: Vec2) {
    game.camera.target.x = new_target.x.round();
    game.camera.target.y = new_target.y.round();
    game.camera.zoom = vec2(GAME_ZOOM / screen_width() * 2.0, -GAME_ZOOM / screen_height() * 2.0);
}

fn get_map_texture() -> Texture2D {
    let image = Image::from_file_with_format(include_bytes!("../../assets/maps/game.png"), None);
    let texture: Texture2D = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}

fn get_map_tilemap() -> Tilemap {
    let tiles_json_vec = include_bytes!("../../assets/maps/game.json").to_vec();
    let tileset_image_rect = Rect::new(0.0,0.0,64.0,64.0);
    let mut tilemap = Tilemap::from_pyxeledit(tileset_image_rect,String::from_utf8(tiles_json_vec).unwrap().as_str());
    tilemap.visibility(tilemap.get_layer_id("logic"), false);
    tilemap.visibility(tilemap.get_layer_id("collision"), false);
    tilemap
}

fn get_tilemaps() -> HashMap<GameState, Tilemap> {
    let mut tilemaps = HashMap::new();
    tilemaps
}
