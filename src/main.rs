use bevy::{prelude::*};
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const NUMBER_OF_ENEMIES: usize = 4;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)   
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, spawn_player)
    .add_systems(Startup, spawn_enemies)
    .add_systems(Update, player_movement)
    .add_systems(Update,confine_player_movement_screen)
    .run();
}

#[derive(Component)]
pub struct Player {}
#[derive(Component)]
pub struct Enemy {}


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window,With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,    
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
){
    let window: &Window = window_query.get_single().unwrap();
   /*  let layout = TextureAtlasLayout::from_grid(Vec2::new(30.0, 34.0), 24, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 24 };
    commands.spawn((
        SpriteSheetBundle {
            texture:asset_server.load("sprites/sprites.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_xyz(window.width()/2.0, window.height(), 0.0),//Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player{},
    ));*/

       commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(window.width()/2.0, window.height(), 0.0),
                    texture: asset_server.load("sprites/velho.png"),
                    ..default()
                },
                Player {},
            ));

}
pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window,With<PrimaryWindow>>,
){
    let window: &Window = window_query.get_single().unwrap();
   
    commands.spawn(       
            Camera2dBundle{
                transform: Transform::from_xyz(window.width()/2.0, window.height(), 0.0),
                ..default()
            }       
        );

}


//Player movement
pub fn player_movement (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
  //  mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
){
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction: Vec3=Vec3::new(0.0,0.0,0.0);

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA){
            direction += Vec3::new(-1.0,0.0, 0.0);
            
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD){
            direction += Vec3::new(1.0,0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW){
            direction += Vec3::new(0.0,1.0, 0.0);        

            // for (indices, mut timer, mut spritesheet) in &mut query {
            //     spritesheet.index = 23;
            // }
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS){
            direction += Vec3::new(0.0,-1.0, 0.0);
        }

        if direction.length() > 0.0{
            direction= direction.normalize();
        }
        transform.translation += direction* PLAYER_SPEED * time.delta_seconds();
    }
}
pub fn confine_player_movement_screen(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;

        let window_h = window.height()/2.0;
        let y_min = window_h + half_player_size;
        let y_max = window.height()+window_h - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
//End player

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/cannonball.png"),
                ..default()
            },
            Enemy {},
        ));
    }
}




#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);




// pub fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
// ) {
//     for (indices, mut timer, mut atlas) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             atlas.index = if atlas.index == indices.last {
//                 indices.first
//             } else {
//                 atlas.index + 1
//             };
//         }
//     }
// }
// pub fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     let texture = asset_server.load("sprites/sprites.png");
//     let layout = TextureAtlasLayout::from_grid(Vec2::new(29.0, 34.0), 23, 3, None, None);
//     let texture_atlas_layout = texture_atlas_layouts.add(layout);
//     // Use only the subset of sprites in the sheet that make up the run animation
//     let animation_indices = AnimationIndices { first: 11, last: 19 };
//     commands.spawn(Camera2dBundle::default());
//     commands.spawn((
//         SpriteSheetBundle {
//             texture,
//             atlas: TextureAtlas {
//                 layout: texture_atlas_layout,
//                 index: animation_indices.first,
//             },
//             transform: Transform::from_scale(Vec3::splat(6.0)),
//             ..default()
//         },
//         animation_indices,
//         AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
//     ));
// }


