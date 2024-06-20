use bevy::{prelude::*, transform};
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::assets_loader::{SceneAssets, SceneAssetsAtlas};
use crate::player::{player_movement, Player, PLAYER_SIZE};
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemies)
            .add_systems(Update, confine_player_movement_collisions) //static walls (enemys as test)
            .add_systems(Update, update_enemy_position.after(player_movement));
    }
}

#[derive(Component)]
pub struct Enemy {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,

}
#[derive(Component)]
pub struct Collision {}

pub const NUMBER_OF_ENEMIES: usize = 16;

pub fn confine_player_movement_collisions(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut collision_query: Query<&Enemy, With<Collision>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        for collider in collision_query.iter_mut() {
            //let collision_object = collision_query.get_single().unwrap();

            let half_player_size = PLAYER_SIZE / 2.0; // 32.0
            let mut translation = player_transform.translation;

            if collider.x < translation.x + half_player_size
                && collider.x + collider.w > translation.x
                && collider.y < translation.y + half_player_size
                && collider.y + collider.h > translation.y
            {
                let tmp = player_transform.translation.z; // z uselss
                player_transform.translation -=
                    Vec3::new(collider.x - translation.x, collider.y - translation.y, tmp);
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
        println!("x:{random_x} y: {random_y}");
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
                x: random_x,
                y: random_y,
                w: 32.0, //todo proper height and width values
                h: 32.0,
                speed: 150.,
            },
            Collision {}, // add collision to enemys  as well?
        ));
    }
}


pub fn update_enemy_position (
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>,Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy), (With<Enemy>,Without<Player>)>,
    time: Res<Time>,
) {
    let p = player_query.get_single_mut().expect("player query failed");
    let p_translation: Vec3 = p.0.translation;//.normalize();
    for (mut t ,e) in enemy_query.iter_mut() {
        let tmp: Vec3= t.translation;
        t.translation += from_to_vec3_normalize(tmp,p_translation) * e.speed * time.delta_seconds();
        
    }
}
pub fn from_to_vec3_normalize(from:Vec3,to:Vec3)-> Vec3{
    return Vec3::new(to.x-from.x,to.y-from.y,0.).normalize();
}
