use crate::assets_loader::SceneAssetBundles;
use crate::damage;
use crate::player::{Player, PlayerDirection};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

const PROJECTILE_SPEED: f32 = 1000.0;

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_projectile_controls)
            .add_event::<ProjectileLaunchEvent>()
            //.add_systems(Update, spinning_projectile)
            .add_systems(Update, projectile_event);
    }
}

#[derive(Component, Debug)]
pub struct PlayerProjecttile(pub f32);

fn player_projectile_controls(
    time: Res<Time>,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut ev_projectile: EventWriter<ProjectileLaunchEvent>,
    mut query: Query<(&Transform, &mut Player), With<Player>>,
    // mut weapon_query: Query<(&mut Transform, &Weapon), (With<Weapon>, Without<Player>)>, // mut scene_asset_bundles: ResMut<SceneAssetBundles>,
) {
    for (transform, mut player) in query.iter_mut() {
        let input = KeyCode::Space;
        //let weapon_input = KeyCode::KeyL;
        // //Weapon once pressed no hold
        // if keyboard_input.just_pressed(weapon_input) {
        //     ev_projectile.send(ProjectileLaunchEvent {
        //         // parent_entity: Entity,
        //         parent_position: transform.translation,
        //         projectile_multiplier: 0.5,
        //         projectile_damage: 0.5,
        //         projectile_speed: 100.,
        //         projectile_velocity: Vec2::new(3.14, 1.),
        //     });
        //     //do event
        // }

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
            // match player.direction {
            //     PlayerDirection::Left => velocity = Vec2::new(-1., 0.),
            //     PlayerDirection::Right => velocity = Vec2::new(1., 0.),
            //     PlayerDirection::Up => velocity = Vec2::new(0., 1.),
            //     PlayerDirection::Down => velocity = Vec2::new(0., -1.),
            // }
            let mut a: f32 = -0.5;
            for _ in 1..4 {
                match player.direction {
                    PlayerDirection::Left => velocity = Vec2::new(-1., a),
                    PlayerDirection::Right => velocity = Vec2::new(1., a),
                    PlayerDirection::Up => velocity = Vec2::new(a, 1.),
                    PlayerDirection::Down => velocity = Vec2::new(a, -1.),
                }

                ev_projectile.send(ProjectileLaunchEvent {
                    // parent_entity: Entity,
                    parent_position: transform.translation,
                    projectile_multiplier: percentage,
                    projectile_damage: damage,
                    projectile_velocity: velocity,
                    projectile_speed: PROJECTILE_SPEED,
                });
                a += 0.5;
            }
            // scene_asset_bundles.projectile.transform = Transform {
            //     translation: transform.translation,
            //     scale: Vec3::new(1f32, 1f32, 1f32) * percentage,
            //     ..default()
            // };
            player.shoot_btn_timer.reset();
            //info!("{:?}", projectile_id);
            //keyboard_input.release(input);
        }
    }
}
//TODO Chain keyboard input by storing them in a vector(?) tuple,
// where 0 is key and 1 is time pressed  then iterate if last 3 or match combo do special attack?
#[derive(Event)]
pub struct ProjectileLaunchEvent {
    //  parent_entity: Entity,
    parent_position: Vec3,
    projectile_multiplier: f32,
    projectile_damage: f32,
    projectile_velocity: Vec2,
    projectile_speed: f32,
}

fn projectile_event(
    mut ev_projectile: EventReader<ProjectileLaunchEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for ev in ev_projectile.read() {
        let chandle = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));
        let mut meshbundle = MaterialMesh2dBundle {
            mesh: chandle,
            material: materials.add(Color::rgb(1., 0., 0.)), //materials.add(color),
            ..default()
        };
        meshbundle.transform = Transform {
            translation: ev.parent_position,
            scale: Vec3::new(1f32, 1f32, 1f32) * ev.projectile_multiplier,
            ..default()
        };
        commands
            .spawn((
                meshbundle,
                PlayerProjecttile(ev.projectile_damage),
                RigidBody::Dynamic,
            ))
            .insert(Collider::ball(50.))
            .insert(Sensor)
            .insert(Velocity {
                linvel: ev.projectile_velocity * ev.projectile_speed,
                angvel: 0.0,
            });
    }
}

// fn spinning_projectile(
//     time: Res<Time>,
//     mut projectiles: Query<(&mut Transform, &mut PlayerProjecttile), Without<Player>>,
//     player: Query<&Transform, With<Player>>,
// ) {
//     let p = player.get_single().expect("xd");
//     for (mut transform, prc) in &mut projectiles {
//         let look_at_sphere = transform.looking_at(p.translation, *transform.local_y());
//         let incremental_turn_weight = 100. * time.delta_seconds();
//         let old_rotation = transform.rotation;
//         transform.rotation = old_rotation.lerp(look_at_sphere.rotation, incremental_turn_weight);
//     }
// }
