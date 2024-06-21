use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CollisionSystemPlugin;
impl Plugin for CollisionSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_world_collider)
            // .add_systems(
            //     PostStartup,
            //     modify_body_gravity_scale.after(spawn_world_collider),
            // )
            ;
    }
}
fn spawn_world_collider(mut commands: Commands) {
    commands.spawn(Collider::cuboid(500.0, 50.0));
}
/* Set the gravity scale inside of a system. */
fn modify_body_gravity_scale(mut grav_scale: Query<&mut GravityScale>) {
    for mut grav_scale in grav_scale.iter_mut() {
        grav_scale.0 = 0.0;
    }
}
