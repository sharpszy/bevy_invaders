use std::{
    collections::HashSet,
    time::{Duration, SystemTime},
};

use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, window::WindowResized};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use components::{
    Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer, Laser, LifeText,
    Movable, Player, ScoreText, SpriteSize, Velocity,
};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use text::{get_current_score_text, get_lives_text, TextPlugin};

use crate::text::get_total_score_text;

mod components;
mod enemy;
mod player;
mod text;
mod tools;

// region: --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;

const SPRITE_SCALE: f32 = 0.5;

// endregion: --- Asset Constatns

// region: --- Game Constants

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

const PLAYER_RESPAWN_DELAY: f64 = 2.;
const PLAYER_MAX_LIVES: u32 = 5;
const PLAYER_INVINCIBLE_DURATION: Duration = Duration::from_secs(3); // 玩家无敌持续时间

const ENEMY_MAX: u32 = 4;
const FORMATION_MEMBERS_MAX: u32 = 2;

// endregion: --- Game Constatns

// region: --- Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    palyer_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
    // score: Handle<>
}

#[derive(Resource)]
struct EnemyCount(u32);

#[derive(Resource)]
struct PlayerState {
    on: bool,
    last_shot: f64,
    born: SystemTime,
    is_invincible: bool,
    current_score: u32,
    total_score: u32,
    lives: u32,
}

enum FireLevel {
    Base,
    Middle,
    Strong,
    Powerful,
    Invincible,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.,
            born: SystemTime::now(),
            is_invincible: true,
            current_score: 0,
            total_score: 0,
            lives: PLAYER_MAX_LIVES,
        }
    }
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
        if self.lives > 0 {
            self.lives -= 1;
        }
    }

    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
        self.born = SystemTime::now();
        self.is_invincible = true;
        self.current_score = 0;
    }

    pub fn increase_score(&mut self) {
        self.total_score += 1;
        self.current_score += 1;
    }

    pub fn hit_to_die(&mut self) -> bool {
        if self.is_invincible {
            if SystemTime::now()
                .duration_since(self.born)
                .unwrap()
                .gt(&PLAYER_INVINCIBLE_DURATION)
            {
                self.is_invincible = false;
            }
        }
        !self.is_invincible
    }

    pub fn get_level(&self) -> FireLevel {
        if self.current_score < 10 {
            FireLevel::Base
        } else if self.current_score >= 10 && self.current_score < 30 {
            FireLevel::Middle
        } else if self.current_score >= 30 && self.current_score < 60 {
            FireLevel::Strong
        } else if self.current_score >= 60 && self.current_score < 100 {
            FireLevel::Powerful
        } else {
            FireLevel::Invincible
        }
    }

    pub fn game_over(&self) -> bool {
        self.lives <= 0
    }
}

// endregion: --- Resources

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(
            DefaultPlugins
                .build()
                // package asset files to binary
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Bevy Indavers!".to_string(),
                        width: 598.0,
                        height: 676.0,
                        resizable: false,
                        ..Default::default()
                    },
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
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // add WinSize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // create explosion texture altas
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4, None, None);
    let explosion = texture_atlases.add(texture_atlas);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        palyer_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
    };
    commands.insert_resource(game_textures);
    commands.insert_resource(EnemyCount(0));
}

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
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

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
    mut enemy_counter: ResMut<EnemyCount>,
    mut player_state: ResMut<PlayerState>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
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
                enemy_counter.0 -= 1;

                // remove the laser
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);

                // spawn the ExplosionToSpawn
                commands.spawn(ExplosionToSpawn(enemy_tf.translation.clone()));

                // update the score
                player_state.increase_score();
                for mut text in &mut text_query {
                    text.sections[0].value = get_current_score_text(player_state.current_score);
                    text.sections[1].value = get_total_score_text(player_state.total_score);
                }
            }
        }
    }
}

fn enemy_laser_hit_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
    mut text_query: Query<&mut Text, With<LifeText>>,
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
                player_state.shot(time.elapsed_seconds_f64());

                // remove the laser
                commands.entity(laser_entity).despawn();

                // spawn the ExplosionToSpawn
                commands.spawn(ExplosionToSpawn(player_tf.translation.clone()));

                // update life text
                for mut text in &mut text_query {
                    text.sections[0].value = get_lives_text(player_state.lives);
                }

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
            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}
