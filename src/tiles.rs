use bevy::{
    prelude::*,
    render::view::visibility,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::prelude::*;

use crate::assets_loader::SceneAssetBundles;
pub struct TilesBgrPlugin;
impl Plugin for TilesBgrPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tile_bgr)
            .add_systems(Update, show_debug_tiles);
    }
}

pub const TILE_SIZE: f32 = 32.0; //
#[derive(Component)]
pub struct Tiles {
    pub tile_walkable: bool,
    pub tile_position: Vec2,
}

#[derive(Component, Reflect)]
struct DebugTiles {}

//TODO create vector 2
//You have similiar check for both x and y axis
//First you random from 0..X, where x is tile array length
//If same tile as previous => copy from previous
// If different => create transition piece for current by checkin the surrounding pieces
//Additional constrain set change color/tile low and add more rare if transition tile was recently spawned
// you check if t

pub fn spawn_tile_bgr(
    mut commands: Commands,

    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    scene_asset_bundles: Res<SceneAssetBundles>,
) {
    for i in 0..100 {
        for k in 0..100 {
            let x = TILE_SIZE * i as f32;
            let y = TILE_SIZE * k as f32;
            let mut tile = scene_asset_bundles.tile.clone();
            tile.transform =
                Transform::from_xyz(x, y, 0.0).with_translation(Vec3 { x: x, y: y, z: -5. });
            let mut debug_tile = scene_asset_bundles.debug_tile.clone();
            debug_tile.transform.translation.z = 4.;
            //debug_tile.visibility = Visibility::Hidden;
            //debug_tile.transform.scale = Vec3::new(0.4, 0.4, 1.);
            let parent_tile = commands
                .spawn((
                    tile,
                    Tiles {
                        tile_position: Vec2::new(x, y),
                        tile_walkable: true,
                    },
                ))
                .id();
            let child_tile = commands.spawn((debug_tile, DebugTiles {})).id();
            commands.entity(parent_tile).push_children(&[child_tile]);

            //Grass &other foliage
            let rnd = rand::thread_rng().gen_range(0..100);
            if rnd % 10 == 0 {
                let layout = TextureAtlasLayout::from_grid(
                    Vec2::new(TILE_SIZE, TILE_SIZE),
                    2,
                    1,
                    None,
                    None,
                );
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                commands.spawn((SpriteSheetBundle {
                    texture: asset_server.load("sprites/tile_detail.png"), //default
                    atlas: TextureAtlas {
                        index: rnd, //index loops around if greater
                        layout: texture_atlas_layout,
                    },
                    transform: Transform::from_xyz(x, y, 0.0).with_translation(Vec3 {
                        x: x,
                        y: y,
                        z: -4.,
                    }),
                    ..default()
                },));
            }
        }
    }
}
fn show_debug_tiles(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tile_query: Query<(&Tiles, &Children)>,
    mut debug_tile_query: Query<(&mut Visibility, &mut TextureAtlas), With<DebugTiles>>,
    //mut debug_tile_query: Query<(&mut Visibility, &mut TextureAtlas, &Parent), With<DebugTiles>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        for (tile, children) in tile_query.iter() {
            for &child in children.iter() {
                if let Ok((visibility, mut atlas)) = debug_tile_query.get_mut(child) {
                    atlas.index = if tile.tile_walkable { 0 } else { 1 };
                }
            }
        }

        //for each debug tile
        // for (mut vis, mut debug_tiles_atlas, parent) in debug_tile_query.iter_mut() {
        //     //set visibility
        //     if *vis == Visibility::Hidden {
        //         *vis = Visibility::Inherited;
        //     } else {
        //         *vis = Visibility::Hidden;
        //     }
        //     // if let Ok(parent) = tile_query.get(parent.get()) {
        //     //     if parent.tile_walkable {
        //     //         debug_tiles_atlas.index = 0
        //     //     } else {
        //     //         debug_tiles_atlas.index = 1
        //     //     };
        //     // }
        // }
    }
}
