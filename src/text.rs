use bevy::prelude::*;

use crate::{
    components::{LifeText, ScoreText},
    PLAYER_MAX_LIVES,
};

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(score_text_spawn_system)
            .add_startup_system(lives_text_spawn_system);
    }
}

fn score_text_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add score text resource
    commands
        .spawn(
            TextBundle::from_section(
                get_score_text(0),
                TextStyle {
                    font: asset_server.load("fonts/NotoSansSC-Light.otf"),
                    font_size: 18.,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ScoreText);
}

pub fn get_score_text(num: u32) -> String {
    format!("你已经消灭了 {} 个敌人！", num)
}

fn lives_text_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add score text resource
    commands
        .spawn(
            TextBundle::from_section(
                get_lives_text(PLAYER_MAX_LIVES),
                TextStyle {
                    font: asset_server.load("fonts/NotoSansSC-Light.otf"),
                    font_size: 20.,
                    color: Color::BLUE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(LifeText);
}

pub fn get_lives_text(num: u32) -> String {
    format!("你还有 {} 条命！", num)
}
