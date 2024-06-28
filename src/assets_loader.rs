use crate::{player::PLAYER_SIZE, tiles::TILE_SIZE};
use bevy::{prelude::*, sprite::*};

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,

    pub projectile: Handle<Image>,
}
#[derive(Resource, Debug, Default)]
pub struct SceneAssetsAtlas {
    pub player: Option<Handle<TextureAtlasLayout>>,
    pub enemy: Option<Handle<TextureAtlasLayout>>,
    pub projectile: Option<Handle<TextureAtlasLayout>>,
}

#[derive(Resource, Default)]
pub struct SceneAssetBundles {
    pub player: SpriteSheetBundle,
    pub enemy: SpriteSheetBundle,
    pub projectile: SpriteSheetBundle,
    pub wall: SpriteSheetBundle,   //walls
    pub object: SpriteSheetBundle, //treee,rocks, etc
    pub tile: SpriteSheetBundle,

    pub debug_tile: SpriteSheetBundle, //MaterialMesh2dBundle<ColorMaterial>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .init_resource::<SceneAssetsAtlas>()
            .init_resource::<SceneAssetBundles>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
    mut bundles: ResMut<SceneAssetBundles>,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut h_atlas: ResMut<SceneAssetsAtlas>,
) {
    *scene_assets = SceneAssets {
        player: asset_server.load("sprites/spritesheet.png"),
        enemy: asset_server.load("sprites/blobs.png"),
        projectile: asset_server.load("sprites/p0.png"),
    };

    //Player  ---------------------------
    let layout_p =
        TextureAtlasLayout::from_grid(Vec2::new(PLAYER_SIZE, PLAYER_SIZE), 4, 1, None, None);
    let tal_p = texture_atlas_layouts.add(layout_p);
    h_atlas.player = Some(tal_p);

    bundles.player = SpriteSheetBundle {
        texture: scene_assets.player.clone(),
        atlas: TextureAtlas {
            index: 0,
            layout: h_atlas.player.clone().unwrap(), //texture_atlas_layout,
        },
        transform: Transform::from_xyz(500.0, 500.0, 0.0),
        ..default()
    };

    //enemy ---------------------------
    let layout_e =
        TextureAtlasLayout::from_grid(Vec2::new(PLAYER_SIZE, PLAYER_SIZE), 4, 1, None, None);
    let tal_e = texture_atlas_layouts.add(layout_e);
    h_atlas.enemy = Some(tal_e);

    bundles.enemy = SpriteSheetBundle {
        texture: scene_assets.enemy.clone(),
        atlas: TextureAtlas {
            index: 0,
            layout: h_atlas.enemy.clone().unwrap(), //texture_atlas_layout,
        },
        //transform: Transform::from_xyz(random_x, random_y, 0.0),
        ..default()
    };

    //Projectile ---------------------------
    let layout_projectile = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 1, 1, None, None);
    let tal_projectile = texture_atlas_layouts.add(layout_projectile);
    h_atlas.projectile = Some(tal_projectile);

    bundles.projectile = SpriteSheetBundle {
        texture: scene_assets.projectile.clone(),
        atlas: TextureAtlas {
            index: 0,
            layout: h_atlas.projectile.clone().unwrap(),
        },
        ..default()
    };
    //---------------

    //Walls-----------
    let layout_wall = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 2, 1, None, None);
    let tal_wall = texture_atlas_layouts.add(layout_wall);
    bundles.wall = SpriteSheetBundle {
        texture: asset_server.load("sprites/tiles.png"),
        atlas: TextureAtlas {
            layout: tal_wall,
            index: 1,
        },
        ..default()
    };
    //--------

    //Trees ---------
    let layout_object = TextureAtlasLayout::from_grid(Vec2::new(60., 72.), 11, 1, None, None);
    let tal_object = texture_atlas_layouts.add(layout_object);
    bundles.object = SpriteSheetBundle {
        texture: asset_server.load("sprites/craftpix/trees.png"),
        atlas: TextureAtlas {
            layout: tal_object,
            index: 1,
        },
        ..default()
    };
    //--------------

    //Tile --------------
    let layout = TextureAtlasLayout::from_grid(Vec2::new(TILE_SIZE, TILE_SIZE), 1, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    bundles.tile = SpriteSheetBundle {
        texture: asset_server.load("sprites/green.PNG"), //default
        atlas: TextureAtlas {
            index: 0,
            layout: texture_atlas_layout,
        },
        ..default()
    };
    //-----------------

    //Debug tile ------------
    let layout = TextureAtlasLayout::from_grid(Vec2::new(TILE_SIZE, TILE_SIZE), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    bundles.debug_tile = SpriteSheetBundle {
        texture: asset_server.load("sprites/debug.png"), //default
        atlas: TextureAtlas {
            index: 0,
            layout: texture_atlas_layout,
        },
        //visibility: Visibility::Hidden,
        ..default()
    };
    // meshbundle.transform = Transform::from_xyz(0., 0., 0.0).with_translation(Vec3 {
    //     x: 0.,
    //     y: 0.,
    //     z: 1.,
    // });
    //---------------
}
