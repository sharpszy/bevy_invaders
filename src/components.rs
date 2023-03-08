use bevy::prelude::*;

use crate::text::{get_current_score_text, get_history_text, get_lives_text, get_total_score_text};

// region: --- Common Components

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct TotalScoreText;

impl TotalScoreText {
    pub fn update(mut query: Query<&mut Text, With<TotalScoreText>>, score: u32) {
        for mut text in &mut query {
            text.sections[0].value = get_total_score_text(score);
        }
    }
}

#[derive(Component)]
pub struct CurrentScoreText;
impl CurrentScoreText {
    pub fn update(mut query: Query<&mut Text, With<CurrentScoreText>>, score: u32) {
        for mut text in &mut query {
            text.sections[0].value = get_current_score_text(score);
        }
    }
}

#[derive(Component)]
pub struct HistoryScoreText;
impl HistoryScoreText {
    pub fn update(mut query: Query<&mut Text, With<HistoryScoreText>>, score: u32) {
        for mut text in &mut query {
            text.sections[0].value = get_history_text(score);
        }
    }
}

#[derive(Component)]
pub struct LifeText;

impl LifeText {
    pub fn update(mut query: Query<&mut Text, With<LifeText>>, lives: u32) {
        for mut text in &mut query {
            text.sections[0].value = get_lives_text(lives);
        }
    }
}

#[derive(Component)]
pub struct GameOverText;

// endregion: --- Common Components

// region: --- Player Components
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;
// endregion: --- Player Components

// region: --- Enemy Components
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;
// endregion: --- Enemy Components

// region: --- Explosion Components
#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, bevy::time::TimerMode::Repeating))
    }
}
// endregion: --- Explosion Components
