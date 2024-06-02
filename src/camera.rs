use crate::player::player_movement;
use crate::player::Player;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PixelCameraPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow.after(player_movement));
    }
}

pub const PLAY_AREA_SIZE_X: i32 = 600; //600
pub const PLAY_AREA_SIZE_Y: i32 = 300; //300

#[derive(Component, Debug)]
pub struct MainCamera {}
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height(), 0.0),
            ..default()
        },
        //Creates pixel effect on regular size screens
        PixelZoom::FitSize {
            width: PLAY_AREA_SIZE_X,
            height: PLAY_AREA_SIZE_Y,
        },
        PixelViewport,
        MainCamera {},
    ));
}
pub fn camera_follow(
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
) {
    let mut cam = camera.single_mut();
    let p = &player.single();
    cam.translation.x = p.translation.x;
    cam.translation.y = p.translation.y;
}
