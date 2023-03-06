use bevy::prelude::*;

use crate::{
    components::{LifeText, ScoreText},
    consts::{self},
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
            TextBundle::from_sections([
                TextSection::new(
                    get_current_score_text(0),
                    TextStyle {
                        font: asset_server.load("fonts/NotoSansSC-Light.otf"),
                        font_size: 18.,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    get_total_score_text(0),
                    TextStyle {
                        font: asset_server.load("fonts/NotoSansSC-Light.otf"),
                        font_size: 22.,
                        color: Color::ORANGE_RED,
                    },
                ),
            ])
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                flex_wrap: FlexWrap::Wrap,
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

pub fn get_current_score_text(num: u32) -> String {
    format!("本轮消灭敌人数: {}", num)
}

pub fn get_total_score_text(num: u32) -> String {
    format!("总共消灭敌人数: {}", num)
}

fn lives_text_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add score text resource
    commands
        .spawn(
            TextBundle::from_sections([TextSection::new(
                get_lives_text(consts::PLAYER_MAX_LIVES),
                TextStyle {
                    font: asset_server.load("fonts/NotoSansSC-Light.otf"),
                    font_size: 20.,
                    color: Color::BLUE,
                },
            )])
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
