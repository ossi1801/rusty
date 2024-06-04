use crate::player::PLAYER_SIZE;
use bevy::prelude::*;

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
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .init_resource::<SceneAssetsAtlas>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut h_atlas: ResMut<SceneAssetsAtlas>,
) {
    *scene_assets = SceneAssets {
        player: asset_server.load("sprites/spritesheet.png"),
        enemy: asset_server.load("sprites/spritesheet.png"),
        projectile: asset_server.load("sprites/red.png"),
    };

    //Texture atlasses
    let layout_p =
        TextureAtlasLayout::from_grid(Vec2::new(PLAYER_SIZE, PLAYER_SIZE), 4, 1, None, None);
    let tal_p = texture_atlas_layouts.add(layout_p);
    h_atlas.player = Some(tal_p);

    let layout_e =
        TextureAtlasLayout::from_grid(Vec2::new(PLAYER_SIZE, PLAYER_SIZE), 4, 1, None, None);
    let tal_e = texture_atlas_layouts.add(layout_e);
    h_atlas.enemy = Some(tal_e);

    let layout_projectile = TextureAtlasLayout::from_grid(Vec2::new(8., 8.), 1, 1, None, None);
    let tal_projectile = texture_atlas_layouts.add(layout_projectile);
    h_atlas.projectile = Some(tal_projectile);
}

// pub struct CharacterSheet {
//     pub handle: Handle<TextureAtlas>,
//     pub player_up: [usize; 3],
//     pub player_down: [usize; 3],
//     pub player_left: [usize; 3],
//     pub player_right: [usize; 3],
// }

// impl GraphicsPlugin {
//     fn load_graphics(
//         mut commands: Commands,
//         assets: Res<AssetServer>,
//         mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//     ) {
//         let image = assets.load("characters.png");
//         let atlas =
//             TextureAtlas::from_grid_with_padding(image, Vec2::splat(16.0), 12, 8, Vec2::splat(2.0));
//         let atlas_handle = texture_atlases.add(atlas);

//         let columns = 12;

//         commands.insert_resource(CharacterSheet {
//             handle: atlas_handle,
//             player_down: [3, 4, 5],
//             player_left: [columns + 3, columns + 4, columns + 5],
//             player_right: [columns * 2 + 3, columns * 2 + 4, columns * 2 + 5],
//             player_up: [columns * 3 + 3, columns * 3 + 4, columns * 3 + 5],
//         });
//     }
// }
