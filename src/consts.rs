use std::time::Duration;

// region: --- OTHER
pub(crate) const COMMON_FONT_SIZE: f32 = 18.;
pub(crate) const MIDDLE_FONT_SIZE: f32 = 24.;
pub(crate) const SIDE_MARGIN_PX: f32 = 10.;
pub(crate) const Z_COORDINATE: f32 = 10.;
// endregion: -- OTHER

// region: --- PLAYER
pub(crate) const PLAYER_SPRITE: &str = "player_a_01.png";
pub(crate) const PLAYER_SIZE: (f32, f32) = (144., 75.);
pub(crate) const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
pub(crate) const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
pub(crate) const PLAYER_RESPAWN_DELAY: f64 = 2.;
pub(crate) const PLAYER_MAX_LIVES: u32 = 5;
pub(crate) const PLAYER_INVINCIBLE_DURATION: Duration = Duration::from_secs(3);
// endregion: --- PLAYER

// region: --- ENEMY
pub(crate) const ENEMY_SPRITE: &str = "enemy_a_01.png";
pub(crate) const ENEMY_SIZE: (f32, f32) = (144., 75.);
pub(crate) const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
pub(crate) const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);
// endregion --- ENEMY

// region: --- GAME
pub(crate) const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
pub(crate) const EXPLOSION_LEN: usize = 16;
pub(crate) const SPRITE_SCALE: f32 = 0.5;
pub(crate) const TIME_STEP: f32 = 1. / 60.;
pub(crate) const BASE_SPEED: f32 = 500.;
pub(crate) const FORMATION_MEMBERS_MAX: u32 = 2;
pub(crate) const HISTORY_LEN: usize = 4;
// endregion: --- GAME

// region --- ICONS
pub(crate) const ICONS_VOICE_ON: &str = "icons/volice_on.png";
pub(crate) const ICONS_VOICE_OFF: &str = "icons/volice_off.png";
// endregion --- ICONS

// region --- AUDIOS
pub(crate) const AUDIOS_EXPLOSION: &str = "audios/explosion.wav";
pub(crate) const AUDIOS_LEVEL_UPGRADE: &str = "audios/level-upgrade.wav";
pub(crate) const AUDIOS_PLAYER_FAIL: &str = "audios/player-fail.wav";
pub(crate) const AUDIOS_SHOT_LOW: &str = "audios/shot-low.wav";
pub(crate) const AUDIOS_SHOT_MID: &str = "audios/shot-mid.wav";
pub(crate) const AUDIOS_SHOT_HIGH: &str = "audios/shot-high.wav";
// endregion --- AUDIOS
