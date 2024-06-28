use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::assets_loader::SceneAssetBundles;

use crate::damage::Health;
use crate::player::{player_movement, Player};
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemies)
            .add_systems(Update, update_enemy_position.after(player_movement))
            .add_systems(Update, animate_enemys)
            .add_systems(Update, despawn_timer_tick);
    }
}

#[derive(Component, PartialEq)]
pub enum ColliderFlag {
    Enemy,
    Player,
    Other,
}
#[derive(Component)]
pub struct Enemy {
    spawn_location: Vec3,
    w: f32,
    h: f32,
    speed: f32,
    vision_radius: f32,
    //pub health: Health,
    pub damage: f32,             //todo damge comp
    pub death_anim_timer: Timer, //Death animation timer
    pub collider_flag: ColliderFlag,
}

pub const NUMBER_OF_ENEMIES: usize = 16;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut scene_asset_bundles: ResMut<SceneAssetBundles>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width() as f32;
        let random_y = random::<f32>() * window.height() as f32;
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        scene_asset_bundles.enemy.transform = Transform::from_xyz(random_x, random_y, 0.0);
        //scene_asset_bundles.enemy.sprite.color.set_a(0.7);

        commands
            .spawn((
                scene_asset_bundles.enemy.clone(), //spritesheet
                Enemy {
                    spawn_location: Vec3::new(random_x, random_y, 0.0),
                    w: 32.0, //todo proper height and width values
                    h: 32.0,
                    speed: 150.,
                    vision_radius: 250.,
                    damage: 5.0,
                    death_anim_timer: Timer::new(Duration::from_millis(700), TimerMode::Once),
                    collider_flag: ColliderFlag::Enemy,
                },
                Health {
                    hp: 5.,
                    is_dead: false,
                },
                animation_indices, //anims
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)), //anims
                ActiveCollisionTypes::KINEMATIC_STATIC,
                KinematicCharacterController::default(),
                Collider::cuboid(8.0, 8.0),
                Sensor, //moves through player
            ))
            .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC);
        //ActiveCollisionTypes is used to check between 2 KinematicCharacterControllers (enemy<->player)
        //KinematicCharacterController also takes care of enemy running into walls ;)
    }
}

pub fn update_enemy_position(
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<
        (
            &mut Transform,
            &mut Enemy,
            &Health,
            &mut KinematicCharacterController,
            Entity,
        ),
        (With<Enemy>, Without<Player>),
    >,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    //custom_data_query: Query<&ColliderFlag>,
) {
    let p = player_query.get_single_mut().expect("player query failed");
    let p_translation: Vec3 = p.0.translation; //.normalize();
    for (mut t, e, health, mut controller, enemy_entity) in enemy_query.iter_mut() {
        if health.is_dead {
            continue; // skip dead enemys
        }
        let tmp: Vec3 = t.translation;

        //Check if player in range
        //if is_in_range(&tmp, &p_translation, e.vision_radius) {
        let enemy_pos: Vec2 = Vec2::new(tmp.x, tmp.y);
        //let player_pos: Vec2 = Vec2::new(p_translation.x, p_translation.y);
        if let Some((entity, _)) = rapier_context.cast_ray(
            enemy_pos,
            from_to_vec2_normalize(tmp, p_translation),
            e.vision_radius,
            true,
            QueryFilter::new().exclude_collider(enemy_entity),
        ) {
            if let Ok(_) = player_query.get_mut(entity) {
                controller.translation = Some(
                    from_to_vec2_normalize(tmp, p_translation) * e.speed * time.delta_seconds(),
                );
            }
        } else if is_in_range(&tmp, &e.spawn_location, 30.0) == false {
            // if is not in  home
            controller.translation = Some(
                from_to_vec2_normalize(tmp, e.spawn_location) * e.speed * time.delta_seconds(),
            );
        }
    }
}
pub fn from_to_vec2_normalize(from: Vec3, to: Vec3) -> Vec2 {
    return Vec2::new(to.x - from.x, to.y - from.y).normalize();
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

fn despawn_timer_tick(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Enemy, Entity, &mut Sprite, &Health), With<Enemy>>,
) {
    for (mut reciever_transform, mut reciever, reciever_entity, mut sprite, health) in &mut query {
        if health.is_dead {
            //Turn dead and fade opacity for dramatic effect
            reciever.death_anim_timer.tick(time.delta());
            let percentage: f32 = reciever.death_anim_timer.elapsed().as_secs_f32()
                / reciever.death_anim_timer.duration().as_secs_f32();
            let opacity: f32 = 1.0 - percentage;
            let angle: f32 = (0.35 * reciever.death_anim_timer.duration().as_secs_f32());
            reciever_transform.rotate(Quat::from_rotation_z(angle));
            sprite.color.set_a(opacity);
            //info!("{}", angle);

            //Despawn after animation
            if reciever.death_anim_timer.finished() {
                commands.entity(reciever_entity).despawn();
            }
        }
    }
}
