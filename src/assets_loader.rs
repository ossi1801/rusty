use bevy::prelude::*;

pub struct CharacterSheet {
    pub handle: Handle<TextureAtlas>,
    pub player_up: [usize; 3],
    pub player_down: [usize; 3],
    pub player_left: [usize; 3],
    pub player_right: [usize; 3],
}

impl GraphicsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let image = assets.load("characters.png");
        let atlas =
            TextureAtlas::from_grid_with_padding(image, Vec2::splat(16.0), 12, 8, Vec2::splat(2.0));
        let atlas_handle = texture_atlases.add(atlas);

        let columns = 12;

        commands.insert_resource(CharacterSheet {
            handle: atlas_handle,
            player_down: [3, 4, 5],
            player_left: [columns + 3, columns + 4, columns + 5],
            player_right: [columns * 2 + 3, columns * 2 + 4, columns * 2 + 5],
            player_up: [columns * 3 + 3, columns * 3 + 4, columns * 3 + 5],
        });
    }
}
