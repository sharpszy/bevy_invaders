use std::collections::HashSet;

use bevy::{
    app::AppExit,
    math::Vec3Swizzles,
    prelude::*,
    sprite::collide_aabb::collide,
    window::{PrimaryWindow, WindowResized, WindowResolution},
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use components::{
    CurrentScoreText, Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer,
    GameOverText, HistoryScoreText, Laser, LifeText, Movable, Player, SpriteSize, TotalScoreText,
    Velocity,
};
use enemy::EnemyPlugin;
use entity::{EnemyState, GameState, GameTextures, PlayerState, WinSize};
use player::PlayerPlugin;
use text::TextPlugin;

mod components;
mod consts;
mod enemy;
mod entity;
mod player;
mod text;
mod tools;

#[macro_use]
extern crate lazy_static;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(
            DefaultPlugins
                .build()
                // package asset files to binary
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Indavers!".to_string(),
                        resolution: WindowResolution::new(598.0, 676.0),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        // .add_system(window_resize_listener) // FIXME, this will be exe every tick time
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(TextPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_enemy_system)
        .add_system(enemy_laser_hit_player_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .add_system(game_over_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // capture window size
    let window = primary_query.get_single_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // add WinSize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // create explosion texture altas
    let texture_handle = asset_server.load(consts::EXPLOSION_SHEET);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4, None, None);
    let explosion = texture_atlases.add(texture_atlas);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(consts::PLAYER_SPRITE),
        palyer_laser: asset_server.load(consts::PLAYER_LASER_SPRITE),
        enemy: asset_server.load(consts::ENEMY_SPRITE),
        enemy_laser: asset_server.load(consts::ENEMY_LASER_SPRITE),
        explosion,
    };
    commands.insert_resource(game_textures);
    commands.insert_resource(EnemyState::default());

    // add game state resource
    commands.insert_resource(GameState::default());
}

#[allow(dead_code)]
fn window_resize_listener(
    mut win_size: ResMut<WinSize>,
    mut resize_events: EventReader<WindowResized>,
) {
    for e in resize_events.iter() {
        win_size.w = e.width;
        win_size.h = e.height;
    }
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * consts::TIME_STEP * consts::BASE_SPEED;
        translation.y += velocity.y * consts::TIME_STEP * consts::BASE_SPEED;

        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_state: ResMut<EnemyState>,
    mut player_state: ResMut<PlayerState>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
    mut text_set: ParamSet<(
        Query<&mut Text, With<CurrentScoreText>>,
        Query<&mut Text, With<TotalScoreText>>,
    )>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // iterate through the laser
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }

        let laser_scale = Vec2::from(laser_tf.scale.xy());

        // iterate through the enemies;
        for (enemy_enity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_enity)
                || despawned_entities.contains(&laser_entity)
            {
                continue;
            }

            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // determine if collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            // perform collision
            if let Some(_) = collision {
                // remove the enemy
                commands.entity(enemy_enity).despawn();
                despawned_entities.insert(enemy_enity);
                enemy_state.count -= 1;

                // remove the laser
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);

                // spawn the ExplosionToSpawn
                commands.spawn(ExplosionToSpawn(enemy_tf.translation.clone()));

                // update the score
                player_state.increase_score();

                // udpate enemy state
                enemy_state.update(player_state.get_game_level());

                // update score text
                CurrentScoreText::update(text_set.p0(), player_state.current_score);
                TotalScoreText::update(text_set.p1(), player_state.total_score);
            }
        }
    }
}

fn enemy_laser_hit_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
    text_query: Query<&mut Text, With<LifeText>>,
) {
    if let Ok((player_entity, player_tf, player_size)) = player_query.get_single() {
        let player_scale = player_tf.scale.xy();

        for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = laser_tf.scale.xy();

            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                player_tf.translation,
                player_size.0 * player_scale,
            );

            // perform the collision
            if let Some(_) = collision {
                if !player_state.hit_to_die() {
                    break;
                }

                // remove the player
                commands.entity(player_entity).despawn();
                if player_state.shot(time.elapsed_seconds_f64()) == 0 {
                    game_state.is_over = true;
                }

                // remove the laser
                commands.entity(laser_entity).despawn();

                // spawn the ExplosionToSpawn
                commands.spawn(ExplosionToSpawn(player_tf.translation.clone()));

                // update life text
                LifeText::update(text_query, player_state.lives);

                break;
            }
        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        // spawn the explosion sprite
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // move to next sprite cell
            if sprite.index >= consts::EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn game_over_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>,
    kb: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut game_state: ResMut<GameState>,
    mut player_state: ResMut<PlayerState>,
    mut text_set: ParamSet<(
        Query<Entity, With<GameOverText>>,
        Query<&mut Text, With<LifeText>>,
        Query<&mut Text, With<CurrentScoreText>>,
        Query<&mut Text, With<TotalScoreText>>,
        Query<&mut Text, With<HistoryScoreText>>,
    )>,
) {
    if !game_state.is_over {
        return;
    }
    if !game_state.show {
        game_state.show = true;
        HistoryScoreText::update(text_set.p4(), player_state.total_score);
        text::game_over_text_bundle(&mut commands, asset_server, win_size);
    }

    if kb.just_pressed(KeyCode::P) {
        // despawn game over text
        let game_over_text = text_set.p0();
        for entity in game_over_text.iter() {
            // despawn_recursive 消除警告
            commands.entity(entity).despawn_recursive();
        }
        game_state.reset();
        player_state.replay();

        // update life text
        LifeText::update(text_set.p1(), player_state.lives);

        // update score text
        CurrentScoreText::update(text_set.p2(), player_state.current_score);
        TotalScoreText::update(text_set.p3(), player_state.total_score);
    } else if kb.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
