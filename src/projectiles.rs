use crate::assets_loader::SceneAssetBundles;
use crate::damage;
use crate::player::{Player, PlayerDirection};
use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

const PROJECTILE_SPEED: f32 = 1000.0;

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_projectile_controls);
    }
}

#[derive(Component, Debug)]
pub struct PlayerProjecttile(pub f32);

fn player_projectile_controls(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut commands: Commands,
    mut query: Query<(&Transform, &mut Player), With<Player>>,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    // mut scene_asset_bundles: ResMut<SceneAssetBundles>,
) {
    for (transform, mut player) in query.iter_mut() {
        let input = KeyCode::Space;

        //If player hold down shoot button increase volume of spell
        if keyboard_input.pressed(input) {
            player.shoot_btn_timer.tick(time.delta());
            if player.shoot_btn_timer.finished() {
                keyboard_input.release(input);
                //player.shoot_btn_timer.reset();
            }
        }

        if keyboard_input.just_released(input) {
            let mut percentage = player.shoot_btn_timer.elapsed().as_secs_f32()
                / player.shoot_btn_timer.duration().as_secs_f32();
            let max_damage: f32 = 10.;
            if percentage < 0.25 {
                percentage = 0.2
            }
            let damage: f32 = max_damage * percentage;

            let mut velocity: Vec2 = Vec2::new(0., 0.);
            match player.direction {
                PlayerDirection::Left => velocity = Vec2::new(-1., 0.),
                PlayerDirection::Right => velocity = Vec2::new(1., 0.),
                PlayerDirection::Up => velocity = Vec2::new(0., 1.),
                PlayerDirection::Down => velocity = Vec2::new(0., -1.),
            }
            // scene_asset_bundles.projectile.transform = Transform {
            //     translation: transform.translation,
            //     scale: Vec3::new(1f32, 1f32, 1f32) * percentage,
            //     ..default()
            // };
            let chandle = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));
            let mut meshbundle = MaterialMesh2dBundle {
                mesh: chandle,
                material: materials.add(Color::rgb(1., 0., 0.)), //materials.add(color),
                ..default()
            };
            meshbundle.transform = Transform {
                translation: transform.translation,
                scale: Vec3::new(1f32, 1f32, 1f32) * percentage,
                ..default()
            };
            commands
                .spawn((
                    //scene_asset_bundles.projectile.clone(),
                    meshbundle,
                    PlayerProjecttile(damage),
                    RigidBody::Dynamic,
                ))
                .insert(Collider::ball(50.))
                .insert(Sensor)
                .insert(Velocity {
                    linvel: velocity * PROJECTILE_SPEED,
                    angvel: 0.0,
                });
            player.shoot_btn_timer.reset();
            //info!("{:?}", projectile_id);
            //keyboard_input.release(input);
        }
    }
}
//TODO Chain keyboard input by storing them in a vector(?) tuple,
// where 0 is key and 1 is time pressed  then iterate if last 3 or match combo do special attack?
