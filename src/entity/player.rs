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

const JUMP_UP_FACTOR: f32 = 14.0;
const JUMP_DOWN_FACTOR: f32 = 12.0;
const JUMP_UP_CURVE: [f32; 12] = [8.0, 16.0, 13.0, 10.0,8.0, 7.0,6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
const JUMP_DOWN_CURVE: [f32; 6] = [1.0, 1.0, 2.0, 3.0, 8.0, 10.0];

const MOVE_FACTOR: f32 = 3.0;
const MOVE_SPEED_CURVE: [f32; 8] = [1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0];
const BREAK_SPEED_CURVE: [f32; 8] = [21.0, 13.0, 8.0, 5.0, 3.0, 2.0, 1.0, 1.0];
const RAY_DOWN_COLOR: Color = PINK;
const RAY_DOWN1: (f32, f32) = (3.0, 16.0);
const RAY_DOWN2: (f32, f32) = (6.0, 16.0);
const RAY_UP_COLOR: Color = WHITE;
const RAY_UP1: (f32, f32) = (3.0, 3.0);
const RAY_UP2: (f32, f32) = (6.0, 3.0);
const RAY_RIGHT_COLOR: Color = ORANGE;
const RAY_RIGHT1: (f32, f32) = (8.0, 1.0);
const RAY_RIGHT2: (f32, f32) = (8.0, 14.0);
const RAY_LEFT_COLOR: Color = LIME;
const RAY_LEFT1: (f32, f32) = (1.0, 1.0);
const RAY_LEFT2: (f32,f32)= (1.0,14.0);
const DUCK_DISTANCE_FIX: f32 = 8.0;
const RAY_HEAD_COLOR: Color = SKYBLUE;
const RAY_HEAD: (f32, f32)= (4.0, 4.0);
const RAY_FEET_COLOR: Color = MAGENTA;
const RAY_FEET: (f32, f32)= (4.0, 12.0);

const JUMP_SOUND_BYTES: &[u8] = include_bytes!("../../assets/sfx/jump.wav");
const DEAD_SOUND_BYTES: &[u8] = include_bytes!("../../assets/sfx/dead.wav");

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum State {
    SLIDE,
    IDLE,
    RUN,
    AIR,
    KILL,
    WIN,
    STAND
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Facing {
    Left,
    Right,
    Camera,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum JumpState {
    BuildUp,
    Not,
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum AnimState {
    RunLeft,
    RunRight,
    StandLeft,
    StandRight,
    RunDuckLeft,
    RunDuckRight,
    StandDuckLeft,
    StandDuckRight,
    AirUp,
    AirUpDuck,
    AirDown,
    AirDownDuck,
    AirUpLeft,
    AirUpDuckLeft,
    AirUpRight,
    AirUpDuckRight,
    AirDownLeft,
    AirDownDuckLeft,
    AirDownRight,
    AirDownDuckRight,
    Idle,
    IdleDuck,
    Dead
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
    jump_sound: Sound,
    dead_sound: Sound,
    mixer: SoundMixer,

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
            jump_state: JumpState::Not,
            animation_state: AnimState::StandRight,
            facing: Facing::Camera,
            animations,
            timer: Timer::new_sec(1),
            jump_sound: decoder::read_wav(JUMP_SOUND_BYTES).unwrap(),
            dead_sound: decoder::read_wav(DEAD_SOUND_BYTES).unwrap(),
            mixer: SoundMixer::new(),
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
            gamestate = Some(GameState::DEAD);
        }
        if self.state == State::WIN {
            self.timer.restart();
            gamestate = Some(GameState::WIN);
        }

        if self.timer.finished() && self.state != State::KILL{
            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                let distance = if self.state != State::AIR {
                    MOVE_FACTOR * MOVE_SPEED_CURVE[self.moving_timer] * delta
                }else{
                    (MOVE_FACTOR * MOVE_SPEED_CURVE[self.moving_timer] * delta) / 1.5
                };
                if can_walk_left(vec2(new_x - distance, new_y), tilemap,self.duck_distance) {
                    self.facing = Facing::Left;
                    if self.state != State::AIR {
                        self.state = State::RUN;
                    }
                    self.direction = vec2(-1.0, 0.0);
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
                let distance = if self.state != State::AIR {
                    MOVE_FACTOR * MOVE_SPEED_CURVE[self.moving_timer] * delta
                }else{
                    (MOVE_FACTOR * MOVE_SPEED_CURVE[self.moving_timer] * delta) / 1.5
                };
                if can_walk_right(vec2(self.position.x + distance, self.position.y), tilemap,self.duck_distance) {
                    self.facing = Facing::Right;
                    if self.state != State::AIR{
                        self.state = State::RUN;
                    }
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
                if self.jump_state == JumpState::Not { //todo whats this
                    if self.state == State::RUN {
                        self.break_timer = 0;
                        self.state = State::SLIDE
                    }
                    if self.break_timer < BREAK_SPEED_CURVE.len() - 1 {
                        let distance = (MOVE_FACTOR + 2.0) * BREAK_SPEED_CURVE[self.break_timer] * delta;
                        if self.direction.x > 0.0 {
                            // right
                            if can_walk_right(vec2(new_x + distance, new_y), tilemap,self.duck_distance) {
                                new_x = new_x + distance;
                            }
                        } else if can_walk_left(vec2(new_x - distance, new_y), tilemap,self.duck_distance) {
                            new_x = new_x - distance;
                        }
                        self.break_timer += 1;
                    } else {
                        if self.state != State::IDLE{
                            self.state = State::STAND;
                        }
                        self.direction = vec2(0.0, 0.0);
                    }
                }

                // animation transition run-> stand
                match self.animation_state {
                    AnimState::RunLeft => {
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = false;
                        if self.animations.get_mut(&self.animation_state).unwrap().finish() {
                            self.animations.get_mut(&self.animation_state).unwrap().reset();
                            self.state = State::STAND;
                            self.facing = Facing::Left;
                        }
                    }
                    AnimState::RunRight => {
                        self.animations.get_mut(&self.animation_state).unwrap().repeating = false;
                        if self.animations.get_mut(&self.animation_state).unwrap().finish() {
                            self.animations.get_mut(&self.animation_state).unwrap().reset();
                            self.state = State::STAND;
                            self.facing = Facing::Right;
                        }
                    }
                    _ => {}
                }
            };
            // jump
            if (is_key_down(KeyCode::Space) || is_key_down(KeyCode::Up)) && (self.jump_state == JumpState::BuildUp || self.jump_state == JumpState::Not) {
                if self.jump_up_timer < JUMP_UP_CURVE.len() - 1 {
                    if self.jump_state == JumpState::Not {
                        self.mixer.play(self.jump_sound.clone());
                        self.jump_state = JumpState::BuildUp;
                        self.state = State::AIR;
                    }
                    self.jump_up_timer += 1;
                    let factor = if self.is_crouched(){JUMP_UP_FACTOR / 2.0} else {JUMP_UP_FACTOR};
                    let y = factor * JUMP_UP_CURVE[self.jump_up_timer] * delta;
                    if can_jump_up(vec2(new_x, new_y - y),tilemap,self.duck_distance){
                        new_y = new_y - y;
                    }
                } else {
                    self.state = State::AIR;
                    self.jump_state = JumpState::Up;
                }
            }

            //stop jumping
            if (!is_key_down(KeyCode::Space) && !is_key_down(KeyCode::Up)) && self.jump_state == JumpState::BuildUp {
                self.state = State::AIR;
                self.jump_state = JumpState::Up;
            }

            if self.jump_state == JumpState::Up {
                if self.air_timer > JUMP_UP_CURVE.len() - 1 {
                    self.air_timer = 0;
                    self.jump_state = JumpState::Down;
                } else {
                    self.air_timer += 1;
                }
            }

            // transition AIR
            if self.jump_state == JumpState::Down || self.jump_state == JumpState::Not {
                let y = JUMP_DOWN_FACTOR * JUMP_DOWN_CURVE[self.jump_down_timer] * delta;
                let x = 2.0 * self.direction.x * delta;
                if can_walk_down(vec2(new_x + x, new_y + y), tilemap) {
                    if self.jump_down_timer < JUMP_DOWN_CURVE.len() - 1 {
                        self.jump_down_timer += 1;
                    }
                    new_y += y;
                    new_x += x;
                    self.jump_state = JumpState::Down;
                    self.state = State::AIR
                } else {
                    self.jump_down_timer = 0;
                    self.jump_up_timer = 0;
                    if self.facing == Facing::Camera {
                        self.state = State::STAND;
                    }else{
                        self.state = State::SLIDE;
                    }
                    self.jump_state = JumpState::Not
                }
            }

            if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                self.duck_distance = DUCK_DISTANCE_FIX;
            }else{
                if can_jump_up(vec2(new_x,new_y),tilemap,0.0) {
                    self.duck_distance = 0.0;
                }
            }

            self.position.x = new_x;
            self.position.y = new_y;

            let id_head = tilemap.get_id_at_position(tilemap.get_layer_id("logic"), self.position() + vec2(RAY_HEAD.0,RAY_HEAD.1 + self.duck_distance));
            let id_feet = tilemap.get_id_at_position(tilemap.get_layer_id("logic"), self.position() + Vec2::from(RAY_FEET));

            // kill logic
            match id_feet {
                Some(id) => match id {
                    3 => {
                        self.mixer.play(self.dead_sound.clone());
                        self.state = State::KILL;
                    }
                    1 => {
                        self.state = State::WIN;
                    },
                    _ => {}
                },
                _ => {},
            }
            match id_head {
                Some(id) => match id {
                    3 => {
                        self.mixer.play(self.dead_sound.clone());
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
            self.animations.get_mut(&old_animationstate).unwrap().repeating = false;
            self.animations.get_mut(&self.animation_state).unwrap().repeating = true;
        }
        self.mixer.frame();
        gamestate
    }
    pub fn position(&self) -> Vec2 {
        if self.animation_state == AnimState::StandLeft || self.animation_state == AnimState::StandRight || self.animation_state == AnimState::Idle {
            return self.position.round();
        }
        self.position.round()
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
            draw_circle((self.position() + Vec2::from(RAY_LEFT1)).x, (self.position() + Vec2::from(RAY_LEFT1)).y + self.duck_distance, 0.5, RAY_LEFT_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_LEFT2)).x, (self.position() + Vec2::from(RAY_LEFT2)).y, 0.5, RAY_LEFT_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_RIGHT1)).x, (self.position() + Vec2::from(RAY_RIGHT1)).y + self.duck_distance, 0.5, RAY_RIGHT_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_RIGHT2)).x, (self.position() + Vec2::from(RAY_RIGHT2)).y, 0.5, RAY_RIGHT_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_DOWN1)).x, (self.position() + Vec2::from(RAY_DOWN1)).y, 0.5, RAY_DOWN_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_DOWN2)).x, (self.position() + Vec2::from(RAY_DOWN2)).y, 0.5, RAY_DOWN_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_UP1)).x, (self.position() + Vec2::from(RAY_UP1)).y + self.duck_distance, 0.5, RAY_UP_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_UP2)).x, (self.position() + Vec2::from(RAY_UP2)).y + self.duck_distance, 0.5, RAY_UP_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_HEAD)).x, (self.position() + Vec2::from(RAY_HEAD)).y + self.duck_distance, 1.0, RAY_HEAD_COLOR);
            draw_circle((self.position() + Vec2::from(RAY_FEET)).x, (self.position() + Vec2::from(RAY_FEET)).y, 1.0, RAY_FEET_COLOR);
        }
    }
    pub fn reset(&mut self, tilemap: &Tilemap){
        self.state = State::IDLE;
        self.animation_state = AnimState::Idle;
        self.jump_timer = 0;
        self.moving_timer = 0;
        self.break_timer = BREAK_SPEED_CURVE.len();
        self.need_reset = false;
        self.timer.restart();
        self.position = tilemap.get_all_position_from_id(tilemap.get_layer_id("logic"),2)[0];
        self.facing = Facing::Camera;
        for (_, a) in self.animations.iter_mut() {
            a.reset();
        }
    }
    pub fn get_animation_state(&self) -> AnimState{
        return match self.state {
            State::SLIDE => {
                if self.facing == Facing::Left {
                    if self.is_crouched() {
                        return AnimState::RunDuckLeft;
                    }
                    return AnimState::RunLeft;
                }
                if self.facing == Facing::Right {
                    if self.is_crouched() {
                        return AnimState::RunDuckRight;
                    }
                    return AnimState::RunRight;
                }
                if self.is_crouched() {
                    return AnimState::IdleDuck;
                }
                AnimState::Idle
            }
            State::IDLE => {
                if self.is_crouched() {
                    return AnimState::IdleDuck;
                }
                AnimState::Idle
            }
            State::RUN => {
                if self.facing == Facing::Left {
                    if self.is_crouched() {
                        return AnimState::RunDuckLeft;
                    }
                    return AnimState::RunLeft;
                }
                if self.is_crouched() {
                    return AnimState::RunDuckRight;
                }
                AnimState::RunRight
            }
            State::KILL => {
                AnimState::Dead
            }
            State::STAND => {
                if self.facing == Facing::Left {
                    if self.is_crouched() {
                        return AnimState::StandDuckLeft;
                    }
                    return AnimState::StandLeft;
                }
                if self.facing == Facing::Right {
                    if self.is_crouched() {
                        return AnimState::StandDuckRight;
                    }
                    return AnimState::StandRight;
                }
                if self.is_crouched() {
                    return AnimState::IdleDuck;
                }
                AnimState::Idle
            }
            State::AIR => {
                if self.facing == Facing::Left {
                    return if self.jump_state == JumpState::Up {
                        if self.is_crouched() {
                            return AnimState::AirUpDuckLeft;
                        }
                        AnimState::AirUpLeft
                    } else { // down
                        if self.is_crouched() {
                            return AnimState::AirDownDuckLeft;
                        }
                        AnimState::AirDownLeft
                    }
                }
                if self.facing == Facing::Right {
                    return if self.jump_state == JumpState::Up {
                        if self.is_crouched() {
                            return AnimState::AirUpDuckRight;
                        }
                        AnimState::AirUpRight
                    } else { // down
                        if self.is_crouched() {
                            return AnimState::AirDownDuckRight;
                        }
                        AnimState::AirDownRight
                    }
                }
                return if self.jump_state == JumpState::Up {
                    if self.is_crouched() {
                        return AnimState::AirUpDuck;
                    }
                    AnimState::AirUp
                } else { // down
                    if self.is_crouched() {
                        return AnimState::AirDownDuck;
                    }
                    AnimState::AirDown
                }
            }
            State::WIN => {
                return AnimState::Idle;
            }
        }
    }
    pub fn is_crouched(&self) -> bool{
        self.duck_distance > 0.0
    }
}

fn can_walk_left(new_position: Vec2, tilemap: &Tilemap, duck_distance: f32) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + vec2(RAY_LEFT1.0,RAY_LEFT1.1 + duck_distance));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + Vec2::from(RAY_LEFT2));
    if let Some(_) = id {
        return false;
    }
    if let Some(_) = id2 {
        return false;
    }
    true
}

fn can_walk_right(new_position: Vec2, tilemap: &Tilemap, duck_distance: f32) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + vec2(RAY_RIGHT1.0,RAY_RIGHT1.1 + duck_distance));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + Vec2::from(RAY_RIGHT2));
    if let Some(_) = id {
        return false;
    }

    if let Some(_) = id2 {
        return false;
    }
    true
}

fn can_jump_up(new_position: Vec2, tilemap: &Tilemap, duck_distance: f32) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + vec2(RAY_UP1.0,RAY_UP1.1 + duck_distance));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position.round() + vec2(RAY_UP2.0,RAY_UP2.1 + duck_distance));
    if let Some(_) = id {
        return false;
    }
    if let Some(_) = id2 {
        return false;
    }
    true
}

fn can_walk_down(new_position: Vec2, tilemap: &Tilemap) -> bool {
    let id = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + Vec2::from(RAY_DOWN1));
    let id2 = tilemap.get_id_at_position(tilemap.get_layer_id("collision"), new_position + Vec2::from(RAY_DOWN2));
    if let Some(_) = id {
        return false;
    }

    if let Some(_) = id2 {
        return false;
    }
    true
}

fn get_animations() -> HashMap<AnimState, TileAnim> {
    let player_tilemap = Tilemap::new(Rect::new(0.0, 0.0, 64.0, 128.0), 8, 16, 8, 8);
    let mut hashmap = HashMap::new();
    hashmap.insert(AnimState::Idle, TileAnim::new(&player_tilemap, &[0, 1, 2], vec![Duration::from_millis(500), Duration::from_millis(200), Duration::from_millis(100)]));
    hashmap.insert(AnimState::IdleDuck, TileAnim::new(&player_tilemap, &[4, 5, 6], vec![Duration::from_millis(500), Duration::from_millis(200), Duration::from_millis(100)]));

    hashmap.insert(AnimState::StandRight, TileAnim::new(&player_tilemap, &[8, 9], vec![Duration::from_millis(500)]));
    hashmap.insert(AnimState::StandDuckRight, TileAnim::new(&player_tilemap, &[12, 13], vec![Duration::from_millis(500)]));

    hashmap.insert(AnimState::StandLeft, TileAnim::new(&player_tilemap, &[16, 17], vec![Duration::from_millis(500)]));
    hashmap.insert(AnimState::StandDuckLeft, TileAnim::new(&player_tilemap, &[20, 48], vec![Duration::from_millis(500)]));

    hashmap.insert(AnimState::RunRight, TileAnim::new(&player_tilemap, &[24, 25], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::RunDuckRight, TileAnim::new(&player_tilemap, &[28, 29], vec![Duration::from_millis(80)]), );

    hashmap.insert(AnimState::RunLeft, TileAnim::new(&player_tilemap, &[32, 33], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::RunDuckLeft, TileAnim::new(&player_tilemap, &[36, 37], vec![Duration::from_millis(80)]), );

    hashmap.insert(AnimState::AirUpRight, TileAnim::new(&player_tilemap, &[40, 40], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::AirUpDuckRight, TileAnim::new(&player_tilemap, &[42, 42], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::AirDownRight, TileAnim::new(&player_tilemap, &[44, 44], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::AirDownDuckRight, TileAnim::new(&player_tilemap, &[46, 46], vec![Duration::from_millis(80)]), );

    hashmap.insert(AnimState::AirUpLeft, TileAnim::new(&player_tilemap, &[48, 48], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::AirUpDuckLeft, TileAnim::new(&player_tilemap, &[50, 50], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::AirDownLeft, TileAnim::new(&player_tilemap, &[52, 52], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::AirDownDuckLeft, TileAnim::new(&player_tilemap, &[54, 54], vec![Duration::from_millis(80)]), );

    hashmap.insert(AnimState::AirUp, TileAnim::new(&player_tilemap, &[56, 56], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::AirUpDuck, TileAnim::new(&player_tilemap, &[58, 58], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::AirDown, TileAnim::new(&player_tilemap, &[60, 60], vec![Duration::from_millis(80)]), );
    hashmap.insert(AnimState::AirDownDuck, TileAnim::new(&player_tilemap, &[62, 62], vec![Duration::from_millis(80)]), );

    hashmap.insert(AnimState::Dead, TileAnim::new(&player_tilemap, &[63, 63], vec![Duration::from_millis(500)]));

    hashmap
}

fn get_player_spritesheet() -> Texture2D {
    let image = Image::from_file_with_format(include_bytes!("../../assets/images/player.png"), Some(ImageFormat::Png));
    let spritesheet: Texture2D = Texture2D::from_image(&image);
    spritesheet.set_filter(FilterMode::Nearest);
    spritesheet
}