use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowMode;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_play_button))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu));
    }
}

struct ButtonColors {
    normal: UiColor,
    hovered: UiColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}

// Mark things to clean up when we leave the menu
#[derive(Component)]
struct PartOfMenu;

#[derive(Component)]
struct ButtonName {
    name: &'static str,
}

impl PartialEq<str> for ButtonName {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    textures: Res<TextureAssets>,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(720.0);
    commands.spawn_bundle(camera_bundle);

    // Play button
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: button_colors.normal,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        })
        .insert(ButtonName { name: "play" })
        .insert(PartOfMenu);

    // Fullscreen button
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(350.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: button_colors.normal,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Toggle Fullscreen".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        })
        .insert(ButtonName { name: "fullscreen" })
        .insert(PartOfMenu);

    // Title Screen Image
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.title_screen.clone(),
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..Default::default()
        })
        .insert(PartOfMenu);
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<(&Interaction, &mut UiColor, &ButtonName), Changed<Interaction>>,
    mut windows: ResMut<Windows>,
) {
    for (interaction, mut color, button_name) in &mut interaction_query {
        if button_name == "play" {
            match *interaction {
                Interaction::Clicked => {
                    state.set(GameState::Playing).unwrap();
                }
                Interaction::Hovered => {
                    *color = button_colors.hovered;
                }
                Interaction::None => {
                    *color = button_colors.normal;
                }
            }
        } else if button_name == "fullscreen" {
            match *interaction {
                Interaction::Clicked => {
                    use WindowMode::*;
                    let window = windows
                        .get_primary_mut()
                        .expect("couldn't find window to make fullscreen");
                    let new_mode = if window.mode() == BorderlessFullscreen {
                        Windowed
                    } else {
                        BorderlessFullscreen
                    };
                    window.set_mode(new_mode);
                }
                Interaction::Hovered => {
                    *color = button_colors.hovered;
                }
                Interaction::None => {
                    *color = button_colors.normal;
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<PartOfMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
