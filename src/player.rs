use bevy::{ecs::schedule::ShouldRun, prelude::*, time::FixedTimestep};
use rand::{thread_rng, Rng};

use crate::{
    components::{FromPlayer, Laser, Movable, Player, SpriteSize, Velocity},
    FireLevel, GameTextures, PlayerState, WinSize, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY,
    PLAYER_SIZE, SPRITE_SCALE,
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
                        bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.,
                        10.,
                    ),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, SPRITE_SCALE),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player)
            .insert(SpriteSize::from(PLAYER_SIZE))
            .insert(Movable {
                auto_despawn: false,
            })
            .insert(Velocity { x: 0., y: 0. });

        player_state.spawned();
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
            let mut x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;

            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.palyer_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + 15., 0.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0., y: 1. });
            };

            spawn_laser(x_offset);
            spawn_laser(-x_offset);
            match player_state.get_level() {
                FireLevel::Middle => {
                    spawn_laser(0.);
                }
                FireLevel::Highest => {
                    spawn_laser(0.);
                    x_offset += 10.;
                    spawn_laser(x_offset);
                    spawn_laser(-x_offset);
                }
                _ => {}
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
        velocity.x = if kb.pressed(KeyCode::Left) && t.translation.x >= -x_half_size {
            -1.
        } else if kb.pressed(KeyCode::Right) && t.translation.x <= x_half_size {
            1.
        } else {
            0.
        };
    }
}
