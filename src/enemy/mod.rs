use std::f32::consts::PI;

use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, prelude::*};
use rand::{thread_rng, Rng};

use crate::{
    components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity},
    EnemyCount, GameTextures, WinSize, BASE_SPEED, ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE,
    SPRITE_SCALE, TIME_STEP,
};

use self::formation::FormationMaker;

mod formation;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FormationMaker::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.))
                    .with_system(enemy_spawn_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(enemy_fire_criteria)
                    .with_system(enemy_fire_system),
            )
            .add_system(enemy_movement_system);
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1. / 60.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    mut formation_maker: ResMut<FormationMaker>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        // compute the x/y
        // let mut rng = thread_rng();
        // let w_span = win_size.w / 2. - 100.;
        // let h_span = win_size.h / 2. - 100.;
        // let x = rng.gen_range(-w_span..w_span);
        // let y = rng.gen_range(-h_span..h_span);

        // get formation and start x/y
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    rotation: Quat::from_rotation_x(PI),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);
        // spawn enemy laser sprite;
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15., 0.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable { auto_despawn: true })
            .insert(Velocity { x: 0., y: -1. });
    }
}

fn enemy_movement_system(time: Res<Time>, mut query: Query<&mut Transform, With<Enemy>>) {
    let now = time.seconds_since_startup() as f32;

    for mut transform in query.iter_mut() {
        // current position
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);

        // max distance
        let max_distance = TIME_STEP * BASE_SPEED;

        // fixeture (hardcode for now)
        let dir: f32 = -1.; // -1 for counter clockwise, -1 clockwise
        let (x_pivot, y_pivot) = (0., 0.);
        let (x_radius, y_radius) = (200., 130.);

        // compute next angle (based on time for now)
        let angle = dir * BASE_SPEED * TIME_STEP / 4. * now % 360. / PI;
        let x_dst = x_radius * angle.cos() + x_pivot;
        let y_dst = y_radius * angle.sin() + y_pivot;

        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance != 0. {
            max_distance / distance
        } else {
            0.
        };

        // compute final x/y
        let x = x_org - dx * distance_ratio;
        let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
        let y = y_org - dy * distance_ratio;
        let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
        // translation.x += BASE_SPEED * TIME_STEP / 4.; // ->> SLOWMO
        // translation.y += BASE_SPEED * TIME_STEP / 4.; // ->> SLOWMO
    }
}
