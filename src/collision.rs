use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::geometry::ContactPair};
use rand::prelude::*;

use crate::{
    assets_loader::SceneAssetBundles, damage::*, enemy::Enemy, player::Player, PlayerProjecttile,
};

pub struct CollisionSystemPlugin;
impl Plugin for CollisionSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_world_collider)
            .add_systems(
                PostStartup,
                modify_collider_restitution.after(spawn_world_collider),
            )
            .add_event::<CreateWallEvent>()
            .add_systems(PostStartup, spawn_buildings.after(spawn_world_collider))
            .add_systems(Update, spawn_wall_with_collider)
            .add_systems(Update, register_player_collide_with_enemy)
            .add_systems(Update, register_projectile_hits);
    }
}
fn spawn_world_collider(mut commands: Commands) {
    let width: f32 = 3200.0; //100 tiles * tile size 32px
    let thickness: f32 = 10.;
    let half_size: f32 = width / 2.0;
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(half_size, thickness))
        .insert(TransformBundle::from(Transform::from_xyz(
            half_size, -thickness, 0.0,
        ))); //bot
    commands
        .spawn(Collider::cuboid(half_size, thickness))
        .insert(TransformBundle::from(Transform::from_xyz(
            half_size, width, 0.0,
        ))); //top
    commands
        .spawn(Collider::cuboid(thickness, half_size))
        .insert(TransformBundle::from(Transform::from_xyz(width, 0.0, 0.0))); //right
    commands
        .spawn(Collider::cuboid(10.0, half_size))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0))); //left
}

fn spawn_buildings(
    mut ev_writer: EventWriter<CreateWallEvent>,
    scene_asset_bundles: Res<SceneAssetBundles>,
) {
    let block_size: f32 = 32.;
    for i in 1..10 {
        ev_writer.send(CreateWallEvent {
            sprite_index: 1,
            collider_size: Vec2::new(block_size / 2., block_size / 2.),
            sprite_position: Vec3::new(100., 500. + (block_size * i as f32), 0.),
            ..default()
        });
    }

    //Tree mesh generate test

    let tree_size_x: f32 = 60.;
    let tree_size_y: f32 = 74.;
    for i in 1..100 {
        for k in 1..100 {
            let rnd = rand::thread_rng().gen_range(0..1000);
            if rnd % 20 == 0 {
                ev_writer.send(CreateWallEvent {
                    sprite_override: Some(scene_asset_bundles.object.clone()),
                    sprite_index: 0,
                    collider_size: Vec2::new(8., 8.),
                    sprite_position: Vec3::new(
                        100. + (tree_size_x * i as f32),
                        100. + (tree_size_y * k as f32),
                        0.,
                    ),
                    collider_offset: Some(Vec3::new(0., -28., 0.)),
                });
            }
        }
    }
}

fn modify_collider_restitution(mut restitutions: Query<&mut Restitution>) {
    for mut restitution in restitutions.iter_mut() {
        restitution.coefficient = 0.0;
        restitution.combine_rule = CoefficientCombineRule::Min;
    }
}

// fn display_events(
//     mut collision_events: EventReader<CollisionEvent>,
//     mut contact_force_events: EventReader<ContactForceEvent>,
// ) {
//     for collision_event in collision_events.read() {
//         info!("Received collision event: {:?}", collision_event);
//     }
//     for contact_force_event in contact_force_events.read() {
//         info!("Received contact force event: {:?}", contact_force_event);
//     }
// }

fn register_player_collide_with_enemy(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(&mut Player, Entity), (With<Player>, Without<Enemy>)>,
    mut collision_query: Query<(&Enemy, Entity), (With<Enemy>, Without<Player>)>,
) {
    if let Ok((mut player, player_entity)) = player_query.get_single_mut() {
        for (enemy, enemy_entity) in collision_query.iter_mut() {
            /* Find the intersection pair, if it exists, between two colliders. */
            //if let Some(contact) = rapier_context.contact_pair(player_entity, enemy_entity) { //test wall
            if rapier_context.intersection_pair(player_entity, enemy_entity) == Some(true) {
                //info!("intersection Colliding");
                player.hp += -(enemy.damage * time.delta_seconds());
            }
        }
    }
}

fn register_projectile_hits(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut events: EventWriter<DamageEvent>,
    mut reciever_query: Query<(&mut Enemy, Entity), (With<Enemy>, Without<PlayerProjecttile>)>,
    mut collision_query: Query<
        (&PlayerProjecttile, Entity),
        (With<Sensor>, With<PlayerProjecttile>, Without<Enemy>),
    >,
) {
    for (mut reciever, reciever_entity) in reciever_query.iter_mut() {
        for (pp, projectile_entity) in collision_query.iter_mut() {
            /* Find the intersection pair, if it exists, between two colliders. */
            if rapier_context.intersection_pair(reciever_entity, projectile_entity) == Some(true) {
                //player.hp += -1;
                //if player shot reduce enemy hp else reduce lpayer hp
                commands.entity(projectile_entity).despawn();
                events.send(DamageEvent {
                    damage: pp.0,
                    target: reciever_entity,
                });
            }
        }
    }
}

#[derive(Event, Default)]
pub struct CreateWallEvent {
    sprite_index: usize,
    sprite_override: Option<SpriteSheetBundle>,
    sprite_position: Vec3,
    collider_size: Vec2,
    collider_offset: Option<Vec3>,
}

fn spawn_wall_with_collider(
    mut commands: Commands,
    scene_asset_bundles: Res<SceneAssetBundles>,
    mut ev: EventReader<CreateWallEvent>,
) {
    for e in ev.read() {
        //Create new sprite if overrides else use default
        let mut sprite = scene_asset_bundles.wall.clone();
        if e.sprite_override.is_some() {
            sprite = e.sprite_override.clone().expect("sprite_override error");
        }
        sprite.atlas.index = e.sprite_index;
        let parent = commands
            .spawn(sprite)
            .insert(TransformBundle::from(Transform::from_xyz(
                e.sprite_position.x,
                e.sprite_position.y,
                e.sprite_position.z,
            )))
            .id();

        //Creating collider: You can set the offset by creating a child entity and setting localtransform as offset value
        let collider = commands
            .spawn(Collider::cuboid(e.collider_size.x, e.collider_size.y))
            .id();
        let mut offset: Vec3 = Vec3::new(0., 0., 0.);
        if e.collider_offset.is_some() {
            offset = e.collider_offset.expect("sprite_override error");
        }
        commands.entity(collider).insert(TransformBundle {
            local: Transform::from_xyz(offset.x, offset.y, offset.z),
            ..default()
        });
        //Binding collider with parent
        commands.entity(parent).push_children(&[collider]);
    }
}
