use std::time::SystemTime;

use bevy::prelude::*;

use crate::consts::{self, PLAYER_MAX_LIVES};

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub palyer_laser: Handle<Image>,
    pub enemy: Handle<Image>,
    pub enemy_laser: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
    // score: Handle<>
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameLevel {
    Basic,
    Middle,
    Strong,
    Powerful,
    Invincible,
}

#[derive(Resource)]
pub struct EnemyState {
    pub count: u32,
    pub level: GameLevel,
    pub level_count: u32,
    pub velocity: f32,
}

#[derive(Resource)]
pub struct GameState {
    pub show_over: bool,
    pub is_over: bool,
}

#[derive(Resource)]
pub struct Settings {
    pub mute: bool,
}

#[derive(Resource)]
pub struct PlayerState {
    pub on: bool,
    pub last_shot: f64,
    pub born: SystemTime,
    pub invincible: bool,
    pub current_score: u32,
    pub total_score: u32,
    pub lives: u32,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.,
            born: SystemTime::now(),
            invincible: true,
            current_score: 0,
            total_score: 0,
            lives: PLAYER_MAX_LIVES,
        }
    }
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) -> u32 {
        self.on = false;
        self.last_shot = time;
        if self.lives > 0 {
            self.lives -= 1;
        }
        self.lives
    }

    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
        self.born = SystemTime::now();
        self.invincible = true;
        self.current_score = 0;
    }

    pub fn replay(&mut self) {
        self.lives = consts::PLAYER_MAX_LIVES;
        self.total_score = 0;
    }

    pub fn increase_score(&mut self) {
        self.total_score += 1;
        self.current_score += 1;
    }

    pub fn hit_to_die(&mut self) -> bool {
        if self.invincible {
            if SystemTime::now()
                .duration_since(self.born)
                .unwrap()
                .gt(&consts::PLAYER_INVINCIBLE_DURATION)
            {
                self.invincible = false;
            }
        }
        !self.invincible
    }

    pub fn get_fire_level(&self) -> GameLevel {
        Self::compute_game_level(self.current_score)
    }

    pub fn get_game_level(&self) -> GameLevel {
        Self::compute_game_level(self.total_score)
    }

    fn compute_game_level(score: u32) -> GameLevel {
        match score {
            0..=9 => GameLevel::Basic,
            10..=29 => GameLevel::Middle,
            30..=59 => GameLevel::Strong,
            60..=99 => GameLevel::Powerful,
            _ => GameLevel::Invincible,
        }
    }
}

impl Default for EnemyState {
    fn default() -> Self {
        Self {
            count: 0,
            level: GameLevel::Basic,
            level_count: 2,
            velocity: -0.5,
        }
    }
}

impl EnemyState {
    pub fn update(&mut self, level: GameLevel) -> bool {
        if self.level == level {
            return false;
        }
        match level {
            GameLevel::Basic => {
                self.level = GameLevel::Basic;
                self.level_count = 2;
                self.velocity = -0.5;
            }
            GameLevel::Middle => {
                self.level = GameLevel::Middle;
                self.level_count = 3;
                self.velocity = -0.7;
            }
            GameLevel::Strong => {
                self.level = GameLevel::Strong;
                self.level_count = 4;
                self.velocity = -0.9;
            }
            GameLevel::Powerful => {
                self.level = GameLevel::Powerful;
                self.level_count = 5;
                self.velocity = -1.1;
            }
            GameLevel::Invincible => {
                self.level = GameLevel::Invincible;
                self.level_count = 6;
                self.velocity = -1.3;
            }
        }
        true
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            show_over: false,
            is_over: false,
        }
    }
}

impl GameState {
    pub fn reset(&mut self) {
        self.show_over = false;
        self.is_over = false;
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self { mute: true }
    }
}
