#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod game;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution, WindowTheme},
};

use game::GamePlugin;

rust_i18n::i18n!("src/game/locales", fallback = "en");

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pico TD".into(),
                        name: Some("Pico TD".into()),
                        present_mode: PresentMode::AutoVsync,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(640.0, 640.0),
                        #[cfg(not(target_arch = "wasm32"))]
                        resize_constraints: WindowResizeConstraints {
                            min_height: 640.0,
                            min_width: 640.0,
                            ..default()
                        },
                        fit_canvas_to_parent: true,
                        #[cfg(not(debug_assertions))]
                        canvas: Some("#pico-td".into()),
                        window_theme: Some(WindowTheme::Dark),
                        #[cfg(not(target_os = "windows"))]
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            GamePlugin,
        ))
        .run();
}
