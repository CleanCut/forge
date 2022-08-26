use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseLocation(Vec2::ZERO))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(change_sprite))
            .add_system(update_mouse_location);
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas =
        TextureAtlas::from_grid(textures.materials_sheet.clone(), Vec2::splat(16.0), 10, 25);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mouse_location: ResMut<MouseLocation>,
) {
    // if actions.player_movement.is_none() {
    //     return;
    // }
    // let speed = 150.;
    // let movement = Vec3::new(
    //     actions.player_movement.unwrap().x * speed * time.delta_seconds(),
    //     actions.player_movement.unwrap().y * speed * time.delta_seconds(),
    //     0.,
    // );
    for mut player_transform in &mut player_query {
        player_transform.translation = mouse_location.0.extend(0.0);
    }
}

fn change_sprite(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>), With<Player>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (mut sprite, texture_atlas_handle) in &mut query {
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        if keyboard_input.just_pressed(KeyCode::Left) {
            sprite.index = if sprite.index > 0 {
                sprite.index - 1
            } else {
                texture_atlas.textures.len() - 1
            };
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct MouseLocation(Vec2);

fn update_mouse_location(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_location: ResMut<MouseLocation>,
    windows: Res<Windows>,
) {
    // Get window dimensions
    // It's possible to not have window dimensions for the first frame or two
    let window_dimensions;
    if let Some(window) = windows.get_primary() {
        window_dimensions = Vec2::new(window.width(), window.height());
    } else {
        return;
    }

    // Update the mouse location
    if let Some(event) = cursor_moved_events.iter().last() {
        mouse_location.0 = event.position - (window_dimensions * 0.5);
    }
}
