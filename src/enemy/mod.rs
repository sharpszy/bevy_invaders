use std::{f32::consts::PI, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::{thread_rng, Rng};

use crate::{
    components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity},
    consts::{self},
    entity::{EnemyState, GameTextures, WinSize},
};

use self::formation::{Formation, FormationMaker};

mod formation;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FormationMaker::default())
            .add_system(enemy_spawn_system.run_if(on_timer(Duration::from_millis(250))))
            .add_system(enemy_fire_system.run_if(enemy_fire_criteria))
            .add_system(enemy_movement_system);
    }
}

fn enemy_fire_criteria() -> bool {
    if thread_rng().gen_bool(1. / 60.) {
        true
    } else {
        false
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_state: ResMut<EnemyState>,
    mut formation_maker: ResMut<FormationMaker>,
    win_size: Res<WinSize>,
) {
    if enemy_state.count < enemy_state.level_count {
        // get formation and start x/y
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;

        commands
            .spawn(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, consts::Z_COORDINATE),
                    scale: Vec3::new(consts::SPRITE_SCALE, consts::SPRITE_SCALE, 1.),
                    rotation: Quat::from_rotation_x(PI),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(formation)
            .insert(SpriteSize::from(consts::ENEMY_SIZE));

        enemy_state.count += 1;
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_state: ResMut<EnemyState>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);
        // spawn enemy laser sprite;
        commands
            .spawn(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15., 0.),
                    scale: Vec3::new(consts::SPRITE_SCALE, consts::SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(consts::ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable { auto_despawn: true })
            .insert(Velocity {
                x: 0.,
                y: enemy_state.velocity,
            });
    }
}

fn enemy_movement_system(mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>) {
    for (mut transform, mut formation) in query.iter_mut() {
        // current position
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);

        // max distance
        let max_distance = consts::TIME_STEP * formation.speed;

        // fixeture (hardcode for now)
        let dir: f32 = if formation.start.0 < 0. { 1. } else { -1. }; // -1 for counter clockwise, -1 clockwise
        let (x_pivot, y_pivot) = formation.pivot;
        let (x_radius, y_radius) = formation.radius;

        // compute next angle (based on time for now)
        let angle = formation.angle
            + dir * formation.speed * consts::TIME_STEP / (x_radius.min(y_radius) * PI / 2.);
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

        // start rotating the formation angle only when sprite is on or close to ellipse
        if distance < max_distance * formation.speed / 20. {
            formation.angle = angle;
        }

        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
        // translation.x += BASE_SPEED * TIME_STEP / 4.; // ->> SLOWMO
        // translation.y += BASE_SPEED * TIME_STEP / 4.; // ->> SLOWMO
    }
}
