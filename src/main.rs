mod camera;
mod player; //Tell rust what .rs file to scan;
use camera::CameraPlugin;
use player::{Player, PlayerPlugin}; //import from player file player plugin

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const NUMBER_OF_ENEMIES: usize = 16;
pub const SCALE: f32 = 2.0;
pub const PLAY_AREA_SIZE_X: i32 = 600;
pub const PLAY_AREA_SIZE_Y: i32 = 300;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Wizard".into(),
                        name: Some("Wizard test".into()),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, confine_player_movement_collisions) //static walls (enemys as test)
        .run();
}

#[derive(Component)]
pub struct Enemy {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}
#[derive(Component)]
pub struct Collision {}

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
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width() as f32;
        let random_y = random::<f32>() * window.height() as f32;
        println!("x:{random_x} y: {random_y}");
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/cannonball.png"),
                ..default()
            },
            Enemy {
                x: random_x,
                y: random_y,
                w: 32.0, //todo proper height and width values
                h: 32.0,
            },
            Collision {}, // add collision to enemys  as well?
        ));
    }
}
