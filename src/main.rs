use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod assets_loader;
mod camera;
mod collision;
mod debug;
mod enemy;
mod player; //Tell rust what .rs file to scan;
mod projectiles;
mod tiles;
mod ui;

use assets_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use collision::CollisionSystemPlugin;
use debug::DebugPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin; //import from player file player plugin
use projectiles::*;
use tiles::TilesBgrPlugin;
use ui::UserInterfacePlugin;

pub const SCALE: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Wizard".into(),
                        name: Some("Wizard test".into()),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        //.add_plugins(DebugPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            physics_pipeline_active: true,
            query_pipeline_active: true,
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            },
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: true,
        })
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CollisionSystemPlugin)
        .add_plugins(AssetLoaderPlugin) //textures etc
        .add_plugins(UserInterfacePlugin)
        .add_plugins(PlayerPlugin) //player
        .add_plugins(EnemyPlugin)
        .add_plugins(ProjectilesPlugin) //Spells
        .add_plugins(CameraPlugin) //main camera
        .add_plugins(TilesBgrPlugin) //Background
        .run();
}
