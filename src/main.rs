use bevy::prelude::*;

mod assets_loader;
mod camera;
mod enemy;
mod player; //Tell rust what .rs file to scan;
mod projectiles;
mod tiles;

use assets_loader::{AssetLoaderPlugin, SceneAssets};
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use player::{Player, PlayerPlugin, PLAYER_SIZE}; //import from player file player plugin
use projectiles::{MovementPlugin, ProjectilesPlugin};
use tiles::TilesBgrPlugin;

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
        .add_plugins(AssetLoaderPlugin) //textures etc
        .add_plugins(PlayerPlugin) //player
        .add_plugins(EnemyPlugin)
        .add_plugins(MovementPlugin) //Physics
        .add_plugins(ProjectilesPlugin) //Spells
        .add_plugins(CameraPlugin) //main camera
        .add_plugins(TilesBgrPlugin) //Background
        .run();
}
