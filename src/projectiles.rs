use crate::assets_loader::{SceneAssets, SceneAssetsAtlas};
use crate::player::{Player, PlayerDirection};
use bevy::{prelude::*, transform};
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
    query: Query<(&Transform, &Player), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
    scene_atlasses: Res<SceneAssetsAtlas>,
) {
    for (transform, player) in query.iter() {
        if keyboard_input.pressed(KeyCode::Space) {
            info!("{:?}", player.direction);
            let mut dir: Direction3d = transform.up();
            let mut degr: f32 = 0.;
            if player.direction == PlayerDirection::Left {
                degr = 90.;
                dir = transform.left();
            }
            if player.direction == PlayerDirection::Right {
                degr = 0.;
                dir = transform.right();
            }
            if player.direction == PlayerDirection::Up {
                degr = 45.;
                dir = transform.up();
            }
            if player.direction == PlayerDirection::Down {
                degr = -45.;
                dir = transform.down();
            }
            commands.spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(dir * PROJECTILE_SPEED),
                    acceleration: Acceleration::new(Vec3::ZERO),
                    collider: Collider::new(PROJECTILE_RADIUS),
                    sprite: SpriteSheetBundle {
                        texture: scene_assets.projectile.clone(),
                        atlas: TextureAtlas {
                            index: 0,
                            layout: scene_atlasses.projectile.clone().unwrap(),
                        },
                        transform: Transform {
                            translation: transform.translation
                                + -dir * PROJECTILE_FORWARD_SPAWN_SCALAR,
                            rotation: Quat::from_rotation_y(degr),
                            scale: Vec3::new(1f32, 1f32, 1f32),
                        },
                        ..default()
                    },
                },
                PlayerProjecttile,
            ));
        }
    }
}
//TODO Chain keyboard input by storing them in a vector(?) tuple,
// where 0 is key and 1 is time pressed  then iterate if last 3 or match combo do special attack?

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
