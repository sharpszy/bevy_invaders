use bevy::prelude::*;

use crate::{
    consts,
    entity::{GameLevel, Settings},
};

pub fn leve_upgrade(settings: &Res<Settings>, asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
    if !settings.mute {
        audio.play(asset_server.load(consts::AUDIOS_LEVEL_UPGRADE));
    }
}

pub fn explosion(
    index: usize,
    settings: &Res<Settings>,
    asset_server: &Res<AssetServer>,
    audio: &Res<Audio>,
) {
    if !settings.mute && index == 0 {
        audio.play(asset_server.load(consts::AUDIOS_EXPLOSION));
    }
}

pub fn game_over(settings: &Res<Settings>, asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
    if !settings.mute {
        audio.play(asset_server.load(consts::AUDIOS_PLAYER_FAIL));
    }
}

pub fn fire_shot(
    game_leve: GameLevel,
    settings: &Res<Settings>,
    asset_server: &Res<AssetServer>,
    audio: &Res<Audio>,
) {
    if !settings.mute {
        let music = match game_leve {
            GameLevel::Basic | GameLevel::Middle => asset_server.load(consts::AUDIOS_SHOT_LOW),
            GameLevel::Strong => asset_server.load(consts::AUDIOS_SHOT_MID),
            GameLevel::Powerful | GameLevel::Invincible => {
                asset_server.load(consts::AUDIOS_SHOT_HIGH)
            }
        };
        audio.play(music);
    }
}
