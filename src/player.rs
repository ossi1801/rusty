use crate::assets_loader::{SceneAssets, SceneAssetsAtlas};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub direction: PlayerDirection,
    pub hp: i32,
}
#[derive(PartialEq, Debug)]
pub enum PlayerDirection {
    Left,
    Right,
    Up,
    Down,
}
pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_SIZE: f32 = 32.0; // This is the player sprite size.
pub const PLAYER_COLLIDER_SIZE: f32 = PLAYER_SIZE / 2.0;

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
    let player_id: Entity = commands
        .spawn((
            SpriteSheetBundle {
                texture: scene_assets.player.clone(),
                atlas: TextureAtlas {
                    index: 0,
                    layout: scene_atlasses.player.clone().unwrap(), //texture_atlas_layout,
                },
                transform: Transform::from_xyz(window.width() / 2.0, window.height(), 0.0),
                ..default()
            },
            Player {
                direction: PlayerDirection::Left,
                hp: 100,
            },
            RigidBody::Dynamic,
        ))
        .insert(Collider::cuboid(PLAYER_COLLIDER_SIZE, PLAYER_COLLIDER_SIZE))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .id();
    //.insert(Sensor);
}

//Player movement
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut TextureAtlas, &mut Player), With<Player>>,
    time: Res<Time>,
    //  mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut transform, mut textures, mut player) in player_query.iter_mut() {
        //if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            textures.index = 0;
            player.direction = PlayerDirection::Left;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            textures.index = 2;
            player.direction = PlayerDirection::Right;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            textures.index = 1;
            player.direction = PlayerDirection::Up;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
            textures.index = 3;
            player.direction = PlayerDirection::Down;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        //}
    }
}
