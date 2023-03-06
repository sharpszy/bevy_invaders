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
pub struct PlayerState {
    pub on: bool,
    pub last_shot: f64,
    pub born: SystemTime,
    pub is_invincible: bool,
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
            is_invincible: true,
            current_score: 0,
            total_score: 0,
            lives: PLAYER_MAX_LIVES,
        }
    }
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
        if self.lives > 0 {
            self.lives -= 1;
        }
    }

    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
        self.born = SystemTime::now();
        self.is_invincible = true;
        self.current_score = 0;
    }

    pub fn increase_score(&mut self) {
        self.total_score += 1;
        self.current_score += 1;
    }

    pub fn hit_to_die(&mut self) -> bool {
        if self.is_invincible {
            if SystemTime::now()
                .duration_since(self.born)
                .unwrap()
                .gt(&consts::PLAYER_INVINCIBLE_DURATION)
            {
                self.is_invincible = false;
            }
        }
        !self.is_invincible
    }

    pub fn get_fire_level(&self) -> GameLevel {
        if self.current_score < 10 {
            GameLevel::Basic
        } else if self.current_score >= 10 && self.current_score < 30 {
            GameLevel::Middle
        } else if self.current_score >= 30 && self.current_score < 60 {
            GameLevel::Strong
        } else if self.current_score >= 60 && self.current_score < 100 {
            GameLevel::Powerful
        } else {
            GameLevel::Invincible
        }
    }

    pub fn get_game_level(&self) -> GameLevel {
        if self.total_score < 10 {
            GameLevel::Basic
        } else if self.total_score >= 10 && self.total_score < 30 {
            GameLevel::Middle
        } else if self.total_score >= 30 && self.total_score < 60 {
            GameLevel::Strong
        } else if self.total_score >= 60 && self.total_score < 100 {
            GameLevel::Powerful
        } else {
            GameLevel::Invincible
        }
    }

    pub fn game_over(&self) -> bool {
        self.lives <= 0
    }
}

impl Default for EnemyState {
    fn default() -> Self {
        Self {
            count: 0,
            level: GameLevel::Basic,
            level_count: 2,
            velocity: -0.6,
        }
    }
}

impl EnemyState {
    pub fn update(&mut self, level: GameLevel) {
        if self.level == level {
            return;
        }
        match level {
            GameLevel::Basic => {
                self.level = GameLevel::Basic;
                self.level_count = 2;
                self.velocity = -0.6;
            }
            GameLevel::Middle => {
                self.level = GameLevel::Middle;
                self.level_count = 3;
                self.velocity = -0.7;
            }
            GameLevel::Strong => {
                self.level = GameLevel::Strong;
                self.level_count = 4;
                self.velocity = -0.8;
            }
            GameLevel::Powerful => {
                self.level = GameLevel::Powerful;
                self.level_count = 5;
                self.velocity = -1.;
            }
            GameLevel::Invincible => {
                self.level = GameLevel::Invincible;
                self.level_count = 6;
                self.velocity = -1.2;
            }
        }
    }
}
