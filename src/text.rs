use bevy::prelude::*;

use crate::{
    components::{CurrentScoreText, GameOverText, LifeText, TotalScoreText},
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
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                flex_wrap: FlexWrap::Wrap,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(TextBundle::from_sections([TextSection::new(
                    get_current_score_text(0),
                    TextStyle {
                        font: asset_server.load("fonts/NotoSansSC-Light.otf"),
                        font_size: 18.,
                        color: Color::GREEN,
                    },
                )]))
                .insert(CurrentScoreText);
            builder
                .spawn(TextBundle::from_sections([TextSection::new(
                    get_total_score_text(0),
                    TextStyle {
                        font: asset_server.load("fonts/NotoSansSC-Light.otf"),
                        font_size: 18.,
                        color: Color::ORANGE_RED,
                    },
                )]))
                .insert(TotalScoreText);
        });
}

pub fn get_current_score_text(num: u32) -> String {
    format!("当前歼灭敌机数: {}", num)
}

pub fn get_total_score_text(num: u32) -> String {
    format!("总共歼灭敌机数: {}", num)
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
                    color: Color::GOLD,
                },
            )])
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                size: Size {
                    width: Val::Px(100.),
                    ..default()
                },
                position_type: PositionType::Absolute,
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
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

pub fn game_over_text_bundle(asset_server: Res<AssetServer>) -> (TextBundle, GameOverText) {
    (
        TextBundle::from_sections([
            TextSection::new(
                "游戏结束",
                TextStyle {
                    font: asset_server.load("fonts/NotoSansSC-Light.otf"),
                    font_size: 28.,
                    color: Color::RED,
                },
            ),
            TextSection::new(
                "按[P]继续",
                TextStyle {
                    font: asset_server.load("fonts/NotoSansSC-Light.otf"),
                    font_size: 22.,
                    color: Color::ORANGE_RED,
                },
            ),
        ])
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            size: Size {
                width: Val::Px(90.),
                ..default()
            },
            border: UiRect::new(Val::Px(10.), Val::Px(10.), Val::Px(10.), Val::Px(10.)),
            flex_direction: FlexDirection::Column,
            ..default()
        }),
        GameOverText,
    )
}
