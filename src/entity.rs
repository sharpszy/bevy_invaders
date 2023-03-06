use std::time::SystemTime;

use bevy::prelude::*;

use crate::{PLAYER_INVINCIBLE_DURATION, PLAYER_MAX_LIVES};

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

#[derive(Resource)]
pub struct EnemyCount(pub u32);

#[derive(Resource)]
struct EnemyState {
    pub count: u32,
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

pub enum GameLevel {
    Basic,
    Middle,
    Strong,
    Powerful,
    Invincible,
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
                .gt(&PLAYER_INVINCIBLE_DURATION)
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
