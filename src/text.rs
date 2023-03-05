use bevy::prelude::*;

use crate::{components::ScoreText, get_score_text};

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(score_text_spawn_system);
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
                    font_size: 24.,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ScoreText);
}
