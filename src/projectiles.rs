use crate::assets_loader::{SceneAssetBundles, SceneAssets, SceneAssetsAtlas};
use crate::player::{Player, PlayerDirection};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PROJECTILE_SPEED: f32 = 1500.0;

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_projectile_controls);
    }
}

#[derive(Component, Debug)]
pub struct PlayerProjecttile(pub f32);

fn player_projectile_controls(
    mut commands: Commands,
    query: Query<(&Transform, &Player), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scene_asset_bundles: ResMut<SceneAssetBundles>,
) {
    for (transform, player) in query.iter() {
        if keyboard_input.pressed(KeyCode::Space) {
            let mut velocity: Vec2 = Vec2::new(0., 0.);
            match player.direction {
                PlayerDirection::Left => velocity = Vec2::new(-1., 0.),
                PlayerDirection::Right => velocity = Vec2::new(1., 0.),
                PlayerDirection::Up => velocity = Vec2::new(0., 1.),
                PlayerDirection::Down => velocity = Vec2::new(0., -1.),
            }
            scene_asset_bundles.projectile.transform = Transform {
                translation: transform.translation,
                scale: Vec3::new(1f32, 1f32, 1f32),
                ..default()
            };
            let projectile_id = commands
                .spawn((
                    scene_asset_bundles.projectile.clone(),
                    PlayerProjecttile(10.0),
                    RigidBody::Dynamic,
                ))
                .insert(Collider::cuboid(8., 8.))
                .insert(Sensor)
                .insert(Velocity {
                    linvel: velocity * PROJECTILE_SPEED,
                    angvel: 0.0,
                })
                .id();
            //info!("{:?}", projectile_id);
        }
    }
}
//TODO Chain keyboard input by storing them in a vector(?) tuple,
// where 0 is key and 1 is time pressed  then iterate if last 3 or match combo do special attack?
