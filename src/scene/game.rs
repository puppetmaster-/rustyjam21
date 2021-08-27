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
        let main_state= None;
        if let Some(gs) = self.player.update(&mut self.map_tilemap) {
            if gs == GameState::DEAD {
                self.player.reset(&self.map_tilemap);
            } else{
                self.game_state = gs;
            }
        }
        update_camera(self, self.player.position());
        set_camera(&self.camera);
        self.map_tilemap.draw(self.map_texture, vec2(0.0, 0.0), None);
        self.player.draw();
        main_state
    }
}

fn update_camera(game: &mut Game, _new_target: Vec2) {
}

fn get_map_texture() -> Texture2D {
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/game.png"), None);
    let texture: Texture2D = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}

fn get_map_tilemap() -> Tilemap {
    let tiles_json_vec = include_bytes!("../../assets/maps/map.json").to_vec();
    let mut tilemap = Tilemap::from_pyxeledit(Rect::new(0.0, 0.0, 64.0, 64.0), String::from_utf8(tiles_json_vec).unwrap().as_str());
    tilemap.visibility(tilemap.get_layer_id("logic"), false);
    tilemap.visibility(tilemap.get_layer_id("collision"), false);
    tilemap
}

fn get_tilemaps() -> HashMap<GameState, Tilemap> {
    let mut tilemaps = HashMap::new();
    //tilemaps.insert(GameState::CEMETERY, get_side_tilemap(include_bytes!("../../assets/maps/cemetery.json").to_vec())); //Haar
    //tilemaps.insert(GameState::FOREST, get_side_tilemap(include_bytes!("../../assets/maps/green.json").to_vec())); //blume
    //tilemaps.insert(GameState::ICE, get_side_tilemap(include_bytes!("../../assets/maps/ice.json").to_vec())); //Stein
    //tilemaps.insert(GameState::SWAMP, get_side_tilemap(include_bytes!("../../assets/maps/swamp.json").to_vec())); // Frucht
    //tilemaps.insert(GameState::ZELDA1, get_side_tilemap(include_bytes!("../../assets/maps/zelda.json").to_vec())); // Zelda
    //tilemaps.insert(GameState::ZELDA2, get_side_tilemap(include_bytes!("../../assets/maps/zelda.json").to_vec())); // Zelda
    //tilemaps.insert(GameState::ZELDA3, get_side_tilemap(include_bytes!("../../assets/maps/tree.json").to_vec())); // Zelda
    tilemaps
}
