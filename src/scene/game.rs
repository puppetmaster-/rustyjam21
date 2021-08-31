use crate::entity::player::Player;
use crate::tilemap::Tilemap;
use crate::{MainState, GAME_ZOOM};
use macroquad::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    WIN,
    GAME,
    DEAD,
}

pub struct Game {
    player: Player,
    game_texture: Texture2D,
    game_tilemap: Tilemap,
    camera: Camera2D,
    game_state: GameState,
}

impl Game {
    pub async fn init() -> Game {
        let game_texture = get_map_texture();
        let game_tilemap = get_game_tilemap(&game_texture);
        let mut player = Player::new();
        player.reset(&game_tilemap);

        let camera = Camera2D {
            zoom: vec2(GAME_ZOOM / screen_width() * 2.0, -GAME_ZOOM / screen_height() * 2.0),
            target: Vec2::new(64.0,64.0),
            ..Default::default()
        };

        Game {
            player,
            game_texture,
            game_tilemap,
            camera,
            game_state: GameState::GAME,
        }
    }

    pub fn reset(&mut self) {
        self.game_state = GameState::GAME;
        self.player.reset(&self.game_tilemap)
    }

    pub fn run(&mut self) -> Option<MainState> {
        let mut main_state= None;
        if let Some(gs) = self.player.update(&mut self.game_tilemap) {
            match gs {
                GameState::WIN => {
                    main_state = Some(MainState::END);
                }
                GameState::DEAD => {
                    self.player.reset(&self.game_tilemap);
                }
                _ => {
                    self.game_state = gs;
                }
            }

        }
        update_camera(self, self.player.position());
        set_camera(&self.camera);
        self.game_tilemap.draw(self.game_texture, vec2(0.0, 0.0), None);
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

fn get_game_tilemap(texture2d: &Texture2D) -> Tilemap {
    let tiles_json_vec = include_bytes!("../../assets/maps/game.json").to_vec();
    let tileset_image_rect = Rect::new(0.0,0.0,64.0,64.0);
    let mut tilemap = Tilemap::from_pyxeledit(tileset_image_rect,String::from_utf8(tiles_json_vec).unwrap().as_str());
    tilemap.set_tile_rectangles_from(texture2d);
    tilemap.visibility(tilemap.get_layer_id("logic"), false);
    tilemap.visibility(tilemap.get_layer_id("collision"), false);
    tilemap
}
