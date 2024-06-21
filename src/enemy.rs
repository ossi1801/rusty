use bevy::window::PrimaryWindow;
use bevy::{prelude::*, transform};
use rand::prelude::*;

use crate::assets_loader::{SceneAssets, SceneAssetsAtlas};
use crate::player::{player_movement, Player, PLAYER_SIZE};
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemies)
            .add_systems(Update, confine_player_movement_collisions) //static walls (enemys as test)
            .add_systems(Update, update_enemy_position.after(player_movement))
            .add_systems(Update, animate_enemys);
    }
}

#[derive(Component)]
pub struct Enemy {
    spawn_location: Vec3,
    w: f32,
    h: f32,
    speed: f32,
    vision_radius: f32,
}
#[derive(Component)]
pub struct Collision {}

pub const NUMBER_OF_ENEMIES: usize = 16;

pub fn confine_player_movement_collisions(
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>, Without<Enemy>)>,
    mut collision_query: Query<(&Transform, &Enemy), (With<Collision>, Without<Player>)>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        for q in collision_query.iter_mut() {
            let collider = q.0.translation;
            let half_player_size = PLAYER_SIZE / 2.0; // 32.0
            let mut translation = player_transform.translation;

            if collider.x < translation.x + half_player_size
                && collider.x + q.1.w > translation.x
                && collider.y < translation.y + half_player_size
                && collider.y + q.1.h > translation.y
            {
                let tmp = player_transform.translation.z; // z uselss
                player_transform.translation -=
                    Vec3::new(collider.x - translation.x, collider.y - translation.y, tmp);
                player.hp += -1;
            }
        }
    }
}

//End player

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    scene_assets: Res<SceneAssets>,
    scene_atlasses: Res<SceneAssetsAtlas>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width() as f32;
        let random_y = random::<f32>() * window.height() as f32;
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        commands.spawn((
            SpriteSheetBundle {
                texture: scene_assets.enemy.clone(),
                atlas: TextureAtlas {
                    index: 0,
                    layout: scene_atlasses.enemy.clone().unwrap(), //texture_atlas_layout,
                },
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..default()
            },
            Enemy {
                spawn_location: Vec3::new(random_x, random_y, 0.0),
                w: 32.0, //todo proper height and width values
                h: 32.0,
                speed: 150.,
                vision_radius: 250.,
            },
            Collision {},      // add collision to enemys  as well?
            animation_indices, //anims
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)), //anims
        ));
    }
}

pub fn update_enemy_position(
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy), (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
) {
    let p = player_query.get_single_mut().expect("player query failed");
    let p_translation: Vec3 = p.0.translation; //.normalize();
    for (mut t, e) in enemy_query.iter_mut() {
        let tmp: Vec3 = t.translation;
        if is_in_range(&tmp, &p_translation, e.vision_radius) {
            // check if player in range
            t.translation +=
                from_to_vec3_normalize(tmp, p_translation) * e.speed * time.delta_seconds();
        } else if is_in_range(&tmp, &e.spawn_location, 30.0) == false {
            // if is not in  home
            t.translation +=
                from_to_vec3_normalize(tmp, e.spawn_location) * e.speed * time.delta_seconds();
        }
    }
}
pub fn from_to_vec3_normalize(from: Vec3, to: Vec3) -> Vec3 {
    return Vec3::new(to.x - from.x, to.y - from.y, 0.).normalize();
}
pub fn is_in_range(from: &Vec3, to: &Vec3, range_float: f32) -> bool {
    return ((to.x - from.x).abs() + (to.y - from.y).abs()) < range_float;
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_enemys(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), With<Enemy>>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
