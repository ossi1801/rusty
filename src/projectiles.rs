use crate::assets_loader::{SceneAssets, SceneAssetsAtlas};
use crate::player::{player_movement, Player, PLAYER_SIZE};
use bevy::prelude::*;
//Proejcttiles
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}
impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}
#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}
impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}
#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}
impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
    pub sprite: SpriteSheetBundle,
}
const PROJECTILE_SPEED: f32 = 1500.0;
const PROJECTILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const PROJECTILE_RADIUS: f32 = 0.25;

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_projectile_controls);
    }
}

#[derive(Component, Debug)]
pub struct PlayerProjecttile;
fn player_projectile_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
    scene_atlasses: Res<SceneAssetsAtlas>,
) {
    if let Ok(transform) = query.get_single() {
        if keyboard_input.pressed(KeyCode::Space) {
            commands.spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(-transform.up() * PROJECTILE_SPEED),
                    acceleration: Acceleration::new(Vec3::ZERO),
                    collider: Collider::new(PROJECTILE_RADIUS),
                    sprite: SpriteSheetBundle {
                        texture: scene_assets.projectile.clone(),
                        atlas: TextureAtlas {
                            index: 0,
                            layout: scene_atlasses.projectile.clone().unwrap(),
                        },
                        transform: Transform::from_translation(
                            transform.translation
                                + -transform.forward() * PROJECTILE_FORWARD_SPAWN_SCALAR,
                        ),
                        ..default()
                    },
                },
                PlayerProjecttile,
            ));
        }
    }
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
