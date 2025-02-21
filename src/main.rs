#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod assets;
mod audio;
mod entities;
mod game;
mod input;
mod player;
mod ui;
mod utils;
mod waves;

use bevy::{
    prelude::*,
    window::{EnabledButtons, PresentMode, WindowResolution, WindowTheme},
};

use game::GamePlugin;

rust_i18n::i18n!("src/locales", fallback = "en");

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
                        resize_constraints: WindowResizeConstraints {
                            min_height: 640.0,
                            min_width: 640.0,
                            ..default()
                        },
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            maximize: false,
                            ..default()
                        },
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        canvas: Some("#pico-td".into()),
                        window_theme: Some(WindowTheme::Dark),
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
