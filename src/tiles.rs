use bevy::prelude::*;

pub struct TilesBgrPlugin;
impl Plugin for TilesBgrPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tile_bgr);
    }
}

pub const TILE_SIZE: f32 = 32.0; // This is the player sprite size.
#[derive(Component)]
struct Tiles {}
//TODO create vector 2
//You have similiar check for both x and y axis
//First you random from 0..X, where x is tile array length
//If same tile as previous => copy from previous
// If different => create transition piece for current by checkin the surrounding pieces
//Additional constrain set change color/tile low and add more rare if transition tile was recently spawned
// you check if t

fn spawn_tile_bgr(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // for t in tiles.iter() {
    //     let (cx, cy) = grid_to_chunk(t.pos.0 as f32, t.pos.1 as f32);
    //     let (x, y) = grid_to_world(t.pos.0 as f32, t.pos.1 as f32);
    //     let (x, y) = center_to_top_left(x, y);
    //     let e = commands
    //         .spawn((
    //             SpriteSheetBundle {
    //                 texture_atlas: handle.clone(),
    //                 sprite: TextureAtlasSprite::new(t.sprite),
    //                 transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
    //                     .with_translation(vec3(x, y, t.z_index as f32)),
    //                 ..default()
    //             },
    //             TileComponent,
    //         ))
    //         .id();
    //     current_chunks
    //         .0
    //         .entry((cx, cy))
    //         .or_insert_with(Vec::new)
    //         .push(e);
    // }

    for i in 0..100 {
        for k in 0..100 {
            let layout =
                TextureAtlasLayout::from_grid(Vec2::new(TILE_SIZE, TILE_SIZE), 1, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let x = TILE_SIZE * i as f32;
            let y = TILE_SIZE * k as f32;
            commands.spawn((
                SpriteSheetBundle {
                    texture: asset_server.load("sprites/t_1.png"), //default
                    atlas: TextureAtlas {
                        index: 0,
                        layout: texture_atlas_layout,
                    },
                    transform: Transform::from_xyz(x, y, 0.0).with_translation(Vec3 {
                        x: x,
                        y: y,
                        z: -5.,
                    }),
                    ..default()
                },
                Tiles {},
            ));
        }
    }
}
