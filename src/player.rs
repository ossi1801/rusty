use std::time::Duration;

use crate::assets_loader::{SceneAssetBundles, SceneAssets, SceneAssetsAtlas};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub direction: PlayerDirection,
    pub hp: f32,
    pub shoot_btn_timer: Timer,
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
    }
}

fn spawn_player(mut commands: Commands, scene_asset_bundles: Res<SceneAssetBundles>) {
    let player_id: Entity = commands
        .spawn((
            scene_asset_bundles.player.clone(), //spritesheet
            Player {
                direction: PlayerDirection::Left,
                hp: 100.,
                shoot_btn_timer: Timer::new(Duration::from_millis(1200), TimerMode::Once),
            },
            RigidBody::KinematicPositionBased,
        ))
        .insert(Collider::cuboid(PLAYER_COLLIDER_SIZE, PLAYER_COLLIDER_SIZE))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(KinematicCharacterController::default())
        .id();
    //.insert(Sensor);
}

//Player movement
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (
            &mut TextureAtlas,
            &mut Player,
            &mut KinematicCharacterController,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    for (mut textures, mut player, mut controller) in player_query.iter_mut() {
        let mut direction: Vec2 = Vec2::new(0.0, 0.0);

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec2::new(-1.0, 0.0);
            textures.index = 0;
            player.direction = PlayerDirection::Left;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec2::new(1.0, 0.0);
            textures.index = 2;
            player.direction = PlayerDirection::Right;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec2::new(0.0, 1.0);
            textures.index = 1;
            player.direction = PlayerDirection::Up;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec2::new(0.0, -1.0);
            textures.index = 3;
            player.direction = PlayerDirection::Down;
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        controller.translation = Some(direction * PLAYER_SPEED * time.delta_seconds());
    }
}
