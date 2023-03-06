use std::time::Duration;

pub(crate) const PLAYER_SPRITE: &str = "player_a_01.png";

pub(crate) const PLAYER_SIZE: (f32, f32) = (144., 75.);

pub(crate) const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";

pub(crate) const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

pub(crate) const ENEMY_SPRITE: &str = "enemy_a_01.png";

pub(crate) const ENEMY_SIZE: (f32, f32) = (144., 75.);

pub(crate) const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";

pub(crate) const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

pub(crate) const EXPLOSION_SHEET: &str = "explo_a_sheet.png";

pub(crate) const EXPLOSION_LEN: usize = 16;

pub(crate) const SPRITE_SCALE: f32 = 0.5;

pub(crate) const TIME_STEP: f32 = 1. / 60.;

pub(crate) const BASE_SPEED: f32 = 500.;

pub(crate) const PLAYER_RESPAWN_DELAY: f64 = 2.;

pub(crate) const PLAYER_MAX_LIVES: u32 = 5;

pub(crate) const PLAYER_INVINCIBLE_DURATION: Duration = Duration::from_secs(3);

pub(crate) const ENEMY_MAX: u32 = 4;

pub(crate) const FORMATION_MEMBERS_MAX: u32 = 2;
