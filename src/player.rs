use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::assets_loader::{SceneAssets, SceneAssetsAtlas};
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {}
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 32.0; // This is the player sprite size.

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, player_movement);
        //   .add_systems(Update, player_projectile_controls.after(player_movement));
        //.add_systems(Update,confine_player_movement_screen)
    }
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    scene_assets: Res<SceneAssets>,
    scene_atlasses: Res<SceneAssetsAtlas>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteSheetBundle {
            texture: scene_assets.player.clone(),
            atlas: TextureAtlas {
                index: 0,
                layout: scene_atlasses.player.clone().unwrap(), //texture_atlas_layout,
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height(), 0.0),
            ..default()
        },
        Player {},
    ));
}

//Player movement
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut TextureAtlas), With<Player>>,
    time: Res<Time>,
    //  mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut transform, mut textures) in player_query.iter_mut() {
        //if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            textures.index = 0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            textures.index = 2;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            textures.index = 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
            textures.index = 3;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        //}
    }
}

//TODO: lock to playable area instead of "screen"
// fn confine_player_movement_screen(
//     mut player_query: Query<&mut Transform, With<Player>>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
// ) {
//     if let Ok(mut player_transform) = player_query.get_single_mut() {
//         let window = window_query.get_single().unwrap();
//         let width: f32 = PLAY_AREA_SIZE_X as f32; //window.width();
//         let height: f32 = PLAY_AREA_SIZE_Y as f32; //window.height();
//         let half_player_size = PLAYER_SIZE / 2.0;
//         let x_min = 0.0 + half_player_size;
//         let x_max = width - half_player_size;
//         let window_h = height / 2.0;
//         let y_min = window_h + half_player_size;
//         let y_max = height + window_h - half_player_size;
//         let mut translation = player_transform.translation;
//         // Bound the player x position
//         if translation.x < x_min {
//             translation.x = x_min;
//         } else if translation.x > x_max {
//             translation.x = x_max;
//         }
//         // Bound the players y position.
//         if translation.y < y_min {
//             translation.y = y_min;
//         } else if translation.y > y_max {
//             translation.y = y_max;
//         }
//         player_transform.translation = translation;
//     }
// }
