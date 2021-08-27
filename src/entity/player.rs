use crate::constants::FLOAT_CMP_ERROR_MARGIN;
use crate::scene::game::GameState;
use crate::tilemap::tile_animation::TileAnim;
use crate::tilemap::Tilemap;
use crate::utils::timer::Timer;
use crate::DEBUG;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use quad_snd::decoder;
use quad_snd::mixer::{Sound, SoundMixer};
use std::collections::HashMap;
use std::time::Duration;

//const EXIT: u32 = 510;

const JUMP_UP_FACTOR: f32 = 10.0;
const JUMP_DOWN_FACTOR: f32 = 8.0;
const JUMP_UP_CURVE: [f32; 12] = [8.0, 16.0, 13.0, 10.0,8.0, 7.0,6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
const JUMP_DOWN_CURVE: [f32; 5] = [1.0, 1.0, 2.0, 3.0, 5.0];

const MOVE_FACTOR: f32 = 3.0;
const MOVE_SPEED_CURVE: [f32; 8] = [1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0];
const BREAK_SPEED_CURVE: [f32; 8] = [21.0, 13.0, 8.0, 5.0, 3.0, 2.0, 1.0, 1.0];
const RAY_DOWN_COLOR: Color = PINK;
const RAY_DOWN1: (f32, f32) = (3.0, 16.0);
const RAY_DOWN2: (f32, f32) = (6.0, 16.0);
const RAY_UP_COLOR: Color = WHITE;
const RAY_UP1: (f32, f32) = (3.0, 1.0);
const RAY_UP2: (f32, f32) = (6.0, 1.0);
const RAY_RIGHT_COLOR: Color = ORANGE;
const RAY_RIGHT1: (f32, f32) = (8.0, 1.0);
const RAY_RIGHT2: (f32, f32) = (8.0, 14.0);
const RAY_LEFT_COLOR: Color = LIME;
const RAY_LEFT1: (f32, f32) = (1.0, 1.0);
const RAY_LEFT2: (f32,f32)= (1.0,14.0);
const DUCK_DISTANCE_FIX: f32 = -8.0;
const RAY_HEAD_COLOR: Color = SKYBLUE;
const RAY_HEAD: (f32, f32)= (4.0, 4.0);
const RAY_FEET_COLOR: Color = MAGENTA;
const RAY_FEET: (f32, f32)= (4.0, 12.0);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum State {
    SLIDE,
    IDLE,
    RUN,
    KILL,
    STAND
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Facing {
    LEFT,
    RIGHT,
    CAMERA,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum JumpState {
    NOT,
    JUMP,
    AIR,
    DOWN,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum AnimState {
    RUNLEFT,
    RUNRIGHT,
    STANDLEFT,
    STANDRIGHT,
    RUNDUCKLEFT,
    RUNDUCKRIGHT,
    STANDDUCKLEFT,
    STANDDUCKRIGHT,
    IDLE,
    DEAD
}

pub struct Player {
    duck_distance: f32,
    moving_timer: usize,
    break_timer: usize,
    air_timer: usize,
    jump_up_timer: usize,
    jump_down_timer: usize,
    direction: Vec2,
    pub position: Vec2,
    collide_color: Color,
    spritesheet: Texture2D,
    need_reset: bool,
    jump_timer: u32,
    state: State,
    jump_state: JumpState,
    animation_state: AnimState,
    facing: Facing,
    animations: HashMap<AnimState, TileAnim>,
    timer: Timer,
}

impl Player {
    pub fn new() -> Self {
        let spritesheet = get_player_spritesheet();
        let animations = get_animations();
        Self {
            duck_distance: 0.0,
            moving_timer: 0,
            break_timer: 0,
            air_timer: 0,
            jump_up_timer: 0,
            jump_down_timer: 0,
            direction: Vec2::ZERO,
            position: Vec2::ZERO,
            collide_color: SKYBLUE,
            spritesheet,
            need_reset: true,
            jump_timer: 0,
            state: State::STAND,
            jump_state: JumpState::NOT,
            animation_state: AnimState::STANDRIGHT,
            facing: Facing::CAMERA,
            animations,
            timer: Timer::new_sec(1),
        }
    }
    pub fn update(&mut self, tilemap: &mut Tilemap) -> Option<GameState> {
        let mut gamestate= None;

        self.animations.get_mut(&self.animation_state).unwrap().advance();

        self.collide_color = SKYBLUE;

        let delta = get_frame_time().min(1. / 30.);
        let mut new_x = self.position.x;
        let mut new_y = self.position.y;

        if self.state == State::KILL && get_last_key_pressed().is_some() {
            self.timer.restart();
            gamestate = Some(GameState::DEAD);
        }

        if self.timer.finished() {
            //wait before moving
            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                let distance = if self.jump_state == JumpState::NOT {
                    MOVE_FACTOR * MOVE_SPEED_CURVE[self.moving_timer] * delta
                }else{
                    (MOVE_FACTOR * MOVE_SPEED_CURVE[self.moving_timer] * delta) / 1.5
                };
                if can_walk_left(vec2(self.position.x - distance, self.position.y), tilemap,self.duck_distance) {
                    self.facing = Facing::LEFT;
                    self.state = State::RUN;
                    self.direction = vec2(0.0, 0.0);
                    new_x = self.position.x - distance;
                    if self.moving_timer < MOVE_SPEED_CURVE.len() - 1 {
                        self.moving_timer += 1;
                    }
                } else {
                    self.direction = vec2(1.0, 0.0);
                    self.break_timer = BREAK_SPEED_CURVE.len() - 3;
                    self.collide_color = PINK;
                }
            } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                let distance = if self.jump_state  == JumpState::NOT {
                    MOVE_FACTOR * MOVE_SPEED_CURVE[self.moving_timer] * delta
                }else{
                    (MOVE_FACTOR * MOVE_SPEED_CURVE[self.moving_timer] * delta) / 1.5
                };
                if can_walk_right(vec2(self.position.x + distance, self.position.y), tilemap,self.duck_distance) {
                    self.facing = Facing::RIGHT;
                    self.state = State::RUN;
                    self.direction = vec2(1.0, 0.0);
                    new_x = self.position.x + distance;
                    if self.moving_timer < MOVE_SPEED_CURVE.len() - 1 {
                        self.moving_timer += 1;
                    }
                } else {
                    self.direction = vec2(-1.0, 0.0);
                    self.break_timer = BREAK_SPEED_CURVE.len() - 3;
                    self.collide_color = GOLD;
                }
            } else {
                self.moving_timer = 0;
                if self.jump_state == JumpState::NOT {
                    if self.state == State::RUN {
                        self.break_timer = 0;
                        self.state = State::SLIDE
                    }
                    if self.break_timer < BREAK_SPEED_CURVE.len() - 1 {
                        let distance = (MOVE_FACTOR + 2.0) * BREAK_SPEED_CURVE[self.break_timer] * delta;
                        if self.direction.x > 0.0 {
                            // right
                            if can_walk_right(vec2(self.position.x + distance, self.position.y), tilemap,self.duck_distance) {
                                new_x = self.position.x + distance;
                            }
                        } else if can_walk_left(vec2(self.position.x - distance, self.position.y), tilemap,self.duck_distance) {
                            new_x = self.position.x - distance;
                        }
                        self.break_timer += 1;
                    } else {
                        //self.state = State::IDLE;
                        self.direction = vec2(0.0, 0.0);
                    }
                }

                match self.animation_state {
                    AnimState::RUNLEFT => {
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = false;
                        if self.animations.get_mut(&self.animation_state).unwrap().finish() {
                            self.animations.get_mut(&self.animation_state).unwrap().reset();
                            self.state = State::STAND;
                            self.facing = Facing::LEFT;
                        }
                    }
                    AnimState::RUNRIGHT => {
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = false;
                        if self.animations.get_mut(&self.animation_state).unwrap().finish() {
                            self.animations.get_mut(&self.animation_state).unwrap().reset();
                            self.state = State::STAND;
                            self.facing = Facing::RIGHT;
                        }
                    }
                    _ => {}
                }
            };
            // jump
            if (is_key_down(KeyCode::Space) || is_key_down(KeyCode::Up)) && (self.jump_state == JumpState::JUMP || self.jump_state == JumpState::NOT) {
                if self.jump_up_timer < JUMP_UP_CURVE.len() - 1 && can_jump_up(vec2(self.position.x, self.position.y), tilemap,self.duck_distance) {
                    if self.jump_state == JumpState::NOT {
                        self.jump_state = JumpState::JUMP;
                    }
                    self.jump_up_timer += 1;
                    //todo check if player can jump up
                    new_y = self.position.y - (JUMP_UP_FACTOR * JUMP_UP_CURVE[self.jump_up_timer] * delta);
                } else {
                    self.jump_state = JumpState::AIR;
                }
            }

            //stop jumping
            if (!is_key_down(KeyCode::Space) && !is_key_down(KeyCode::Up)) && self.jump_state == JumpState::JUMP {
                self.jump_state = JumpState::AIR;
                self.jump_up_timer = 0;
            }

            if self.jump_state == JumpState::AIR {
                if self.air_timer > JUMP_UP_CURVE.len() - 1 {
                    self.air_timer = 0;
                    self.jump_state = JumpState::DOWN;
                } else {
                    self.air_timer += 1;
                }
            }

            if self.jump_state == JumpState::DOWN || self.jump_state == JumpState::NOT {
                let y = JUMP_DOWN_FACTOR * JUMP_DOWN_CURVE[self.jump_down_timer] * delta;
                let x = 2.0 * self.direction.x * delta;
                if can_walk_down(vec2(new_x + x, new_y + y), tilemap) {
                    if self.jump_down_timer < JUMP_DOWN_CURVE.len() - 1 {
                        self.jump_down_timer += 1;
                    }
                    new_y += y;
                    new_x += x;
                    self.jump_state = JumpState::DOWN;
                } else {
                    self.jump_down_timer = 0;
                    self.jump_up_timer = 0;
                    self.jump_state = JumpState::NOT;
                }
            }

            if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                self.duck_distance = DUCK_DISTANCE_FIX;
            }else{
                if can_jump_up(vec2(new_x,new_y),tilemap,self.duck_distance) {
                    self.duck_distance = 0.0;
                }
            }

            self.position.x = new_x;
            self.position.y = new_y;

            let id_head = tilemap.get_id_at_position(tilemap.get_layer_id("logic"), self.position() + vec2(RAY_HEAD.0,RAY_HEAD.1-self.duck_distance));
            let id_feet = tilemap.get_id_at_position(tilemap.get_layer_id("logic"), self.position() + Vec2::from(RAY_FEET));

            // kill logic
            match id_feet {
                Some(id) => match id {
                    3 => {
                        self.state = State::KILL;
                    },
                    _ => {}
                },
                _ => {},
            }
            match id_head {
                Some(id) => match id {
                    3 => {
                        self.state = State::KILL;
                    },
                    _ => {}
                },
                _ => {},
            }

        }
        // setting animationstate
        let old_animationstate = self.animation_state;
        self.animation_state = self.get_animation_state();
        if old_animationstate !=  self.animation_state{
            self.animations.get_mut(&old_animationstate).unwrap().reset();
            self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
        }

        gamestate
    }
    pub fn position(&self) -> Vec2 {
        if self.animation_state == AnimState::STANDLEFT || self.animation_state == AnimState::STANDRIGHT || self.animation_state == AnimState::IDLE {
            return self.position.round();
        }
        self.position
    }
    pub fn draw(&self) {
        draw_texture_ex(
            self.spritesheet,
            self.position().x,
            self.position().y,
            WHITE,
            DrawTextureParams {
                source: self.animations.get(&self.get_animation_state()).unwrap().source(),
                ..Default::default()
            },
        );
        if DEBUG {
            draw_circle(self.position().x, self.position().y, 0.5, YELLOW);
            draw_rectangle_lines(self.position().x, self.position().y, 8.0, 16.0, 0.1, self.collide_color);
            draw_text(&format!("{:?}", &self.state), 400.0, 5.0, 14.0, WHITE);
            draw_circle((self.position() + Vec2::from(RAY_LEFT1)).x, (self.position() + Vec2::from(RAY_LEFT1)).y-self.duck_distance, 0.5, RAY_LEFT_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_LEFT2)).x, (self.position() + Vec2::from(RAY_LEFT2)).y, 0.5, RAY_LEFT_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_RIGHT1)).x, (self.position() + Vec2::from(RAY_RIGHT1)).y-self.duck_distance, 0.5, RAY_RIGHT_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_RIGHT2)).x, (self.position() + Vec2::from(RAY_RIGHT2)).y, 0.5, RAY_RIGHT_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_DOWN1)).x, (self.position() + Vec2::from(RAY_DOWN1)).y, 0.5, RAY_DOWN_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_DOWN2)).x, (self.position() + Vec2::from(RAY_DOWN2)).y, 0.5, RAY_DOWN_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_UP1)).x, (self.position() + Vec2::from(RAY_UP1)).y-self.duck_distance, 0.5, RAY_UP_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_UP2)).x, (self.position() + Vec2::from(RAY_UP2)).y-self.duck_distance, 0.5, RAY_UP_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_HEAD)).x, (self.position() + Vec2::from(RAY_HEAD)).y-self.duck_distance, 1.0, RAY_HEAD_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_FEET)).x, (self.position() + Vec2::from(RAY_FEET)).y, 1.0, RAY_FEET_COLOR);
        }
    }
    pub fn reset(&mut self, tilemap: &Tilemap){
        self.state = State::IDLE;
        self.jump_timer = 0;
        self.moving_timer = 0;
        self.break_timer = BREAK_SPEED_CURVE.len();
        self.need_reset = false;
        self.timer.restart();
        self.position = tilemap.get_all_position_from_id(tilemap.get_layer_id("logic"),2)[0];
        self.animation_state = AnimState::IDLE;
        self.facing = Facing::CAMERA;
        for (_, a) in self.animations.iter_mut() {
            a.reset();
        }
    }
    pub fn get_animation_state(&self) -> AnimState{
        return match self.state {
            State::SLIDE => {
                if self.facing == Facing::LEFT {
                    if self.duck_distance < 0.0 {
                        return AnimState::RUNDUCKLEFT;
                    }
                    return AnimState::RUNLEFT;
                }
                if self.duck_distance < 0.0 {
                    return AnimState::RUNDUCKRIGHT;
                }
                AnimState::RUNRIGHT
            }
            State::IDLE => {
                if self.duck_distance < 0.0 {
                    return AnimState::STANDDUCKLEFT;
                }
                AnimState::IDLE
            }
            State::RUN => {
                if self.facing == Facing::LEFT {
                    if self.duck_distance < 0.0 {
                        return AnimState::RUNDUCKLEFT;
                    }
                    return AnimState::RUNLEFT;
                }
                if self.duck_distance < 0.0 {
                    return AnimState::RUNDUCKRIGHT;
                }
                AnimState::RUNRIGHT
            }
            State::KILL => {
                AnimState::DEAD
            }
            State::STAND => {
                if self.facing == Facing::LEFT {
                    if self.duck_distance < 0.0 {
                        return AnimState::STANDDUCKLEFT;
                    }
                    return AnimState::STANDLEFT;
                }
                if self.duck_distance < 0.0 {
                    return AnimState::STANDDUCKRIGHT;
                }
                AnimState::STANDRIGHT
            }
        }
    }
}

fn can_walk_left(new_position: Vec2, tilemap: &Tilemap, duck_distance: f32) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + vec2(RAY_LEFT1.0,RAY_LEFT1.1-duck_distance));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + Vec2::from(RAY_LEFT2));
    if let Some(i) = id {
        return false;
    }
    if let Some(i) = id2 {
        return false;
    }
    true
}

fn can_walk_right(new_position: Vec2, tilemap: &Tilemap, duck_distance: f32) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + vec2(RAY_RIGHT1.0,RAY_RIGHT1.1-duck_distance));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + Vec2::from(RAY_RIGHT2));
    if let Some(i) = id {
        return false;
    }

    if let Some(i) = id2 {
        return false;
    }
    true
}

fn can_jump_up(new_position: Vec2, tilemap: &Tilemap, duck_distance: f32) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + vec2(RAY_UP1.0,RAY_UP1.1-duck_distance));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + vec2(RAY_UP2.0,RAY_UP2.1-duck_distance));
    if let Some(i) = id {
        return false;
    }
    if let Some(i) = id2 {
        return false;
    }
    true
}

fn can_walk_down(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + Vec2::from(RAY_DOWN1));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + Vec2::from(RAY_DOWN2));
    if let Some(i) = id {
        return false;
    }

    if let Some(i) = id2 {
        return false;
    }
    true
}

fn get_animations() -> HashMap<AnimState, TileAnim> {
    let player_tilemap = Tilemap::new(Rect::new(0.0, 0.0, 64.0, 128.0), 8, 16, 8, 8);
    let mut hashmap = HashMap::new();
    hashmap.insert(AnimState::RUNRIGHT, TileAnim::new(&player_tilemap, &[24, 25], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::RUNLEFT, TileAnim::new(&player_tilemap, &[32, 33], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::STANDRIGHT, TileAnim::new(&player_tilemap, &[8, 9], vec![Duration::from_millis(500)]));
    hashmap.insert(AnimState::STANDLEFT, TileAnim::new(&player_tilemap, &[16, 17], vec![Duration::from_millis(500)]));
    hashmap.insert(AnimState::RUNDUCKRIGHT, TileAnim::new(&player_tilemap, &[49, 49], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::RUNDUCKLEFT, TileAnim::new(&player_tilemap, &[50, 50], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::STANDDUCKRIGHT, TileAnim::new(&player_tilemap, &[48, 48], vec![Duration::from_millis(500)]));
    hashmap.insert(AnimState::STANDDUCKLEFT, TileAnim::new(&player_tilemap, &[48, 48], vec![Duration::from_millis(500)]));
    hashmap.insert(AnimState::DEAD, TileAnim::new(&player_tilemap, &[56, 56], vec![Duration::from_millis(500)]));
    hashmap.insert(AnimState::IDLE, TileAnim::new(&player_tilemap, &[0, 1, 2], vec![Duration::from_millis(500),Duration::from_millis(200),Duration::from_millis(100)]));
    hashmap
}

fn get_player_spritesheet() -> Texture2D {
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/player.png"), Some(ImageFormat::Png));
    let spritesheet: Texture2D = Texture2D::from_image(&image);
    spritesheet.set_filter(FilterMode::Nearest);
    spritesheet
}