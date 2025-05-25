use bevy::{prelude::*, window::WindowResized};

use crate::game::{GameState, GameTilemap, assets::levels::Level, entities::tilemap::Tilemap};

#[derive(Component)]
pub struct GameCamera;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera.run_if(in_state(GameState::InGame)));
    }
}

fn update_camera(
    windows: Query<&Window>,
    mut game_camera: Query<&mut Transform, With<GameCamera>>,
    game_tilemap: Single<&Tilemap, With<GameTilemap>>,
    mut resize_reader: EventReader<WindowResized>,
    selected_level: Res<Level>,
    game_state: Res<State<GameState>>,
) {
    let Ok(mut camera_transform) = game_camera.single_mut() else {
        return;
    };
    if selected_level.get_error().is_some() {
        return;
    }

    let mut update_camera_scale = |width: f32, height: f32| {
        let viewport_size = selected_level
            .get_viewport_size()
            .unwrap_or(selected_level.get_map_size());

        let tilemap_width = viewport_size.x as f32 * game_tilemap.get_tile_size() as f32;
        let tilemap_height = viewport_size.y as f32 * game_tilemap.get_tile_size() as f32;

        let scale_x = tilemap_width / width;
        let scale_y = tilemap_height / height;

        camera_transform.scale = Vec3::splat(scale_x.max(scale_y)).with_z(1.0);
    };

    for event in resize_reader.read() {
        update_camera_scale(event.width, event.height);
    }

    if game_state.is_changed() {
        if let Ok(window) = windows.single() {
            update_camera_scale(window.resolution.width(), window.resolution.height());
        }
    }
}
