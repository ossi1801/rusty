use crate::player::{player_movement, Player, PLAYER_SIZE};
use bevy::prelude::*;
pub struct UserInterfacePlugin;
impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, hp_update);
    }
}

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct HPText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Relative,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        HPText,
    ));

    // Text with multiple sections
    // commands.spawn((
    //     // Create a TextBundle that has a Text with a list of sections.
    //     TextBundle::from_sections([
    //         TextSection::new(
    //             "FPS: ",
    //             TextStyle {
    //                 // This font is loaded and will be used instead of the default font.
    //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                 font_size: 60.0,
    //                 ..default()
    //             },
    //         ),
    //         TextSection::from_style(if cfg!(feature = "default_font") {
    //             TextStyle {
    //                 font_size: 60.0,
    //                 color: Color::GOLD,
    //                 // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
    //                 ..default()
    //             }
    //         } else {
    //             // "default_font" feature is unavailable, load a font to use instead.
    //             TextStyle {
    //                 font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                 font_size: 60.0,
    //                 color: Color::GOLD,
    //             }
    //         }),
    //     ]),
    //     FpsText,
    // ));

    // #[cfg(feature = "default_font")]
    // commands.spawn(
    //     // Here we are able to call the `From` method instead of creating a new `TextSection`.
    //     // This will use the default font (a minimal subset of FiraMono) and apply the default styling.
    //     TextBundle::from("From an &str into a TextBundle with the default font!").with_style(
    //         Style {
    //             position_type: PositionType::Absolute,
    //             bottom: Val::Px(5.0),
    //             left: Val::Px(15.0),
    //             ..default()
    //         },
    //     ),
    // );
    // #[cfg(not(feature = "default_font"))]
    // commands.spawn(
    //     TextBundle::from_section(
    //         "Default font disabled",
    //         TextStyle {
    //             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //             ..default()
    //         },
    //     )
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         bottom: Val::Px(5.0),
    //         left: Val::Px(15.0),
    //         ..default()
    //     }),
    // );
}

// fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
//     for mut text in &mut query {
//         let seconds = time.elapsed_seconds();

//         // Update the color of the first and only section.
//         text.sections[0].style.color = Color::Rgba {
//             red: (1.25 * seconds).sin() / 2.0 + 0.5,
//             green: (0.75 * seconds).sin() / 2.0 + 0.5,
//             blue: (0.50 * seconds).sin() / 2.0 + 0.5,
//             alpha: 1.0,
//         };
//     }
// }

fn hp_update(mut text_q: Query<&mut Text, With<HPText>>, player_q: Query<&Player, With<Player>>) {
    let mut text = text_q.get_single_mut().expect("UI text query failed");
    let player = player_q.get_single().expect("UI player query failed");
    text.sections[0].value = player.hp.to_string();
}
