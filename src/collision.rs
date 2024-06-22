use bevy::{ecs::entity, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{assets_loader::SceneAssets, enemy::Enemy, player::Player};

pub struct CollisionSystemPlugin;
impl Plugin for CollisionSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_world_collider)
            .add_systems(Update, display_intersection_info);
    }
}
fn spawn_world_collider(mut commands: Commands) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(5000.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(1.0, 0.0, 0.0))); //bot
    commands
        .spawn(Collider::cuboid(5000.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(1.0, 1000.0, 0.0))); //top
    commands
        .spawn(Collider::cuboid(10.0, 5000.0))
        .insert(TransformBundle::from(Transform::from_xyz(1500.0, 0.0, 0.0))); //right
    commands
        .spawn(Collider::cuboid(10.0, 5000.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0))); //left
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        info!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        info!("Received contact force event: {:?}", contact_force_event);
    }
}

fn display_intersection_info(
    rapier_context: Res<RapierContext>,
    // custom_info: Res<SceneAssets>,
    mut player_query: Query<(&mut Transform, &mut Player, Entity), (With<Player>, Without<Enemy>)>,
    mut collision_query: Query<(&Transform, &Enemy, Entity), (With<Sensor>, Without<Player>)>,
) {
    if let Ok((mut player_transform, mut player, player_entity)) = player_query.get_single_mut() {
        for q in collision_query.iter_mut() {
            /* Find the intersection pair, if it exists, between two colliders. */
            if rapier_context.intersection_pair(player_entity, q.2) == Some(true) {
                player.hp += -1;
                println!(
                    "The entities {:?} and {:?} have intersecting colliders!",
                    player_entity, q.2
                );
            }
        }
    }
}

// #[derive(Resource, Debug, Default)]
// pub struct EntitiesInScene {
//     pub player: Entity,
//     pub enemy: Handle<Image>,
//     pub projectile: Handle<Image>,
// }
