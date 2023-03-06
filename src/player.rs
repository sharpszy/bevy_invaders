use bevy::{ecs::schedule::ShouldRun, prelude::*, time::FixedTimestep};
use rand::{thread_rng, Rng};

use crate::{
    components::{FromPlayer, Laser, Movable, Player, ScoreText, SpriteSize, Velocity},
    consts::{self, PLAYER_RESPAWN_DELAY},
    entity::GameLevel,
    text::get_current_score_text,
    GameTextures, PlayerState, WinSize,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(player_spawn_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(player_fire_criteria)
                    .with_system(player_fire_system),
            )
            .add_system(player_keyboard_event_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
) {
    if player_state.game_over() {
        return;
    }

    let now = time.elapsed_seconds_f64();
    let last_shot = player_state.last_shot;

    if !player_state.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_DELAY) {
        // add player
        let bottom = -win_size.h / 2.;
        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        0.,
                        bottom + consts::PLAYER_SIZE.1 / 2. * consts::SPRITE_SCALE + 5.,
                        10.,
                    ),
                    scale: Vec3::new(
                        consts::SPRITE_SCALE,
                        consts::SPRITE_SCALE,
                        consts::SPRITE_SCALE,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player)
            .insert(SpriteSize::from(consts::PLAYER_SIZE))
            .insert(Movable {
                auto_despawn: false,
            })
            .insert(Velocity { x: 0., y: 0. });

        player_state.spawned();
        for mut text in &mut text_query {
            text.sections[0].value = get_current_score_text(player_state.current_score);
        }
    }
}

fn player_fire_system(
    mut commands: Commands,
    player_state: ResMut<PlayerState>,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let mut x_offset = consts::PLAYER_SIZE.0 / 2. * consts::SPRITE_SCALE - 5.;

            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.palyer_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + 15., 0.),
                            scale: Vec3::new(consts::SPRITE_SCALE, consts::SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(consts::PLAYER_LASER_SIZE))
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0., y: 1. });
            };

            // FIXME 代码待重构
            match player_state.get_fire_level() {
                GameLevel::Basic => {
                    spawn_laser(0.);
                }
                GameLevel::Middle => {
                    spawn_laser(x_offset);
                    spawn_laser(-x_offset);
                }
                GameLevel::Strong => {
                    spawn_laser(0.);
                    spawn_laser(x_offset);
                    spawn_laser(-x_offset);
                }
                GameLevel::Powerful => {
                    spawn_laser(0.);
                    spawn_laser(x_offset);
                    spawn_laser(-x_offset);
                    x_offset += 10.;
                    spawn_laser(x_offset);
                    spawn_laser(-x_offset);
                }
                GameLevel::Invincible => {
                    let middle_offset = 5.;
                    spawn_laser(middle_offset);
                    spawn_laser(-middle_offset);
                    spawn_laser(x_offset);
                    spawn_laser(-x_offset);
                    x_offset += 10.;
                    spawn_laser(x_offset);
                    spawn_laser(-x_offset);
                }
            }
        }
    }
}

fn player_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(10. / 60.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    win_size: Res<WinSize>,
    mut query: Query<(&Transform, &mut Velocity), With<Player>>,
) {
    if let Ok((t, mut velocity)) = query.get_single_mut() {
        let x_half_size = win_size.w / 2.;
        let y_half_size = win_size.h / 2.;
        velocity.x = if (kb.pressed(KeyCode::Left) || kb.pressed(KeyCode::A))
            && t.translation.x > -x_half_size
        {
            -1.
        } else if (kb.pressed(KeyCode::Right) || kb.pressed(KeyCode::D))
            && t.translation.x < x_half_size
        {
            1.
        } else {
            0.
        };

        velocity.y = if (kb.pressed(KeyCode::Up) || kb.pressed(KeyCode::W))
            && t.translation.y < y_half_size
        {
            1.
        } else if (kb.pressed(KeyCode::Down) || kb.pressed(KeyCode::S))
            && t.translation.y > -y_half_size
        {
            -1.
        } else {
            0.
        }
    }
}
