// #![windows_subsystem = "windows"]
mod assets;
mod audio;
mod entities;
mod game;
mod input;
mod ui;
mod waves;

use bevy::{
    prelude::*,
    window::{EnabledButtons, PresentMode, WindowResolution, WindowTheme},
};

use game::GamePlugin;

rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pico TD".into(),
                        name: Some("Pico TD".into()),
                        resolution: WindowResolution::new(640.0, 640.0),
                        // .with_scale_factor_override(1.0),
                        present_mode: PresentMode::AutoVsync,
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: EnabledButtons {
                            maximize: false,
                            ..default()
                        },
                        resizable: false,
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            GamePlugin,
        ))
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .run();
}
