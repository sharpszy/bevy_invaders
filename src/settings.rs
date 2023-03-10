use bevy::prelude::*;

use crate::{components::AudioButton, consts, entity::Settings};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(settings_spawn)
            .add_system(setting_audio_system);
    }
}

fn settings_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::all(Val::Px(18.)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(32.),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor(Color::GRAY),
            z_index: ZIndex::Global(10),
            image: UiImage {
                texture: asset_server.load(consts::ICONS_VOICE_ON),
                ..default()
            },
            ..default()
        })
        .insert(AudioButton);
}

fn setting_audio_system(
    asset_server: Res<AssetServer>,
    mut settings: ResMut<Settings>,
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, (With<Button>, With<AudioButton>)),
    >,
) {
    for (interaction, mut audio_icon) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if audio_icon.texture == asset_server.load(consts::ICONS_VOICE_ON) {
                    audio_icon.texture = asset_server.load(consts::ICONS_VOICE_OFF);
                    settings.mute = true;
                } else {
                    audio_icon.texture = asset_server.load(consts::ICONS_VOICE_ON);
                    settings.mute = false;
                }
            }
            _ => {}
        }
    }
}
