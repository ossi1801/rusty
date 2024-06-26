use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

use crate::player::player_movement;
use crate::player::Player;

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, weapon_follow_player.after(player_movement));
    }
}

#[derive(Component)]
pub struct Weapon {
    pub rigidbody: RigidBody,
    pub sensor: Sensor,
    pub collider: Collider,
    pub meshbundle: MaterialMesh2dBundle<ColorMaterial>,
}

// for (mut transform, weapon) in weapon_query.iter_mut() {
//     //transform.translation
//     transform.rotate(Quat::from_rotation_z(-180. * 100. * time.delta_seconds()));
// }
fn weapon_follow_player(
    mut weapon: Query<&mut Transform, With<Weapon>>,
    player: Query<&Transform, (With<Player>, Without<Weapon>)>,
) {
    for mut transform in &mut weapon {
        for player_transform in &player {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
    }
}
