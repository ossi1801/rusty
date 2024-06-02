use crate::player::PLAYER_SIZE;
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
const PROJECTILE_SPEED: f32 = 50.0;
const PROJECTILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const PROJECTILE_RADIUS: f32 = 1.0;

fn player_projectile_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Ok(transform) = query.get_single() {
        //todo atlas from handle
        let layout =
            TextureAtlasLayout::from_grid(Vec2::new(PLAYER_SIZE, PLAYER_SIZE), 1, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        if keyboard_input.pressed(KeyCode::Space) {
            commands.spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(-transform.forward() * PROJECTILE_SPEED),
                    acceleration: Acceleration::new(Vec3::ZERO),
                    collider: Collider::new(PROJECTILE_RADIUS),
                    sprite: SpriteSheetBundle {
                        texture: scene_assets.projectile.clone(),
                        atlas: TextureAtlas {
                            index: 0,
                            layout: texture_atlas_layout,
                        },
                        transform: Transform::from_translation(
                            transform.translation
                                + -transform.forward() * PROJECTILE_FORWARD_SPAWN_SCALAR,
                        ),
                        ..default()
                    },
                },
                //SpaceshipMissile,
            ));
        }
    }
}
