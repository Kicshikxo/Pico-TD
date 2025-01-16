use bevy::{asset::LoadState, prelude::*, ui::widget::NodeImageMode};
#[cfg(not(target_arch = "wasm32"))]
use native_dialog::{FileDialog, MessageDialog, MessageType};

use crate::{
    assets::{
        levels::{Level, LevelsAssets},
        ui::UiAssets,
    },
    game::GameState,
    ui::{
        components::{
            button::UiButton,
            text::{UiText, UiTextSize},
        },
        UiState,
    },
};

pub struct LevelSelectViewUiPlugin;

impl Plugin for LevelSelectViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UploadedLevel::default());

        app.add_systems(OnEnter(UiState::LevelSelect), ui_init)
            .add_systems(OnExit(UiState::LevelSelect), ui_destroy)
            .add_systems(
                Update,
                (
                    ui_update.run_if(in_state(UiState::LevelSelect)),
                    uploaded_level_update.run_if(in_state(UiState::LevelSelect)), // resource_exists::<UploadedLevel>
                ),
            );
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum LevelSelectButtonAction {
    BackToMenu,
    SelectLevel { level_index: usize },
    UploadLevel,
}

#[derive(Resource)]
struct UploadedLevel {
    handle: Option<Handle<Level>>,
}

impl Default for UploadedLevel {
    fn default() -> Self {
        Self { handle: None }
    }
}

fn ui_init(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    levels_assets: Res<LevelsAssets>,
    levels_assets_loader: Res<Assets<Level>>,
) {
    commands
        .spawn((
            RootUiComponent,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ImageNode {
                image: ui_assets.small_tilemap.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: 5,
                    layout: ui_assets.small_tilemap_atlas.clone(),
                }),
                image_mode: NodeImageMode::Tiled {
                    tile_x: true,
                    tile_y: true,
                    stretch_value: 8.0,
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(320.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        padding: UiRect::all(Val::Px(24.0)),
                        ..default()
                    },
                    ImageNode {
                        image: ui_assets.large_tilemap.clone(),
                        texture_atlas: Some(TextureAtlas {
                            index: 22,
                            layout: ui_assets.large_tilemap_atlas.clone(),
                        }),
                        image_mode: NodeImageMode::Sliced(TextureSlicer {
                            border: BorderRect::square(10.0),
                            max_corner_scale: 2.5,
                            ..default()
                        }),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Button,
                        LevelSelectButtonAction::BackToMenu,
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(32.0),
                            top: Val::Px(-6.0),
                            right: Val::Px(38.0),
                            ..default()
                        },
                        ImageNode {
                            image: ui_assets.small_tilemap.clone(),
                            texture_atlas: Some(TextureAtlas {
                                index: 4,
                                layout: ui_assets.small_tilemap_atlas.clone(),
                            }),
                            ..default()
                        },
                    ));
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(8.0)),
                                ..default()
                            },
                            ImageNode {
                                image: ui_assets.large_tilemap.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    index: 3,
                                    layout: ui_assets.large_tilemap_atlas.clone(),
                                }),
                                image_mode: NodeImageMode::Sliced(TextureSlicer {
                                    border: BorderRect::square(10.0),
                                    max_corner_scale: 2.5,
                                    ..default()
                                }),
                                ..default()
                            },
                        ))
                        .with_child(UiText::new("ui.select_level"));
                    parent
                        .spawn(Node {
                            display: Display::Grid,
                            width: Val::Percent(100.0),
                            grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                            row_gap: Val::Px(8.0),
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            for (level_index, level_handle) in
                                levels_assets.compain.iter().enumerate()
                            {
                                let level = levels_assets_loader.get(level_handle).unwrap();

                                parent
                                    .spawn((Node {
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },))
                                    .with_children(|parent| {
                                        parent
                                            .spawn((
                                                Button,
                                                LevelSelectButtonAction::SelectLevel {
                                                    level_index,
                                                },
                                                Node {
                                                    width: Val::Percent(100.0),
                                                    aspect_ratio: Some(1.0),
                                                    align_items: AlignItems::Center,
                                                    justify_content: JustifyContent::Center,
                                                    ..default()
                                                },
                                                ImageNode {
                                                    image: ui_assets.large_tilemap.clone(),
                                                    texture_atlas: Some(TextureAtlas {
                                                        index: if level.error.is_none() {
                                                            16
                                                        } else {
                                                            20
                                                        },
                                                        layout: ui_assets
                                                            .large_tilemap_atlas
                                                            .clone(),
                                                    }),
                                                    image_mode: NodeImageMode::Sliced(
                                                        TextureSlicer {
                                                            border: BorderRect::square(10.0),
                                                            max_corner_scale: 1.75,
                                                            ..default()
                                                        },
                                                    ),
                                                    ..default()
                                                },
                                            ))
                                            .with_child(
                                                UiText::new(&(level_index + 1).to_string())
                                                    .with_size(UiTextSize::Large),
                                            );

                                        parent.spawn(UiText::new(&level.name));
                                    });
                            }
                        });

                    #[cfg(not(target_arch = "wasm32"))]
                    parent
                        .spawn((LevelSelectButtonAction::UploadLevel, UiButton::new()))
                        .with_child(UiText::new("ui.upload_level"));
                });
        });
}

fn ui_destroy(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn ui_update(
    interaction_query: Query<
        (&Interaction, &LevelSelectButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    levels_assets: Res<LevelsAssets>,
    levels_assets_loader: Res<Assets<Level>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    mut uploaded_level: ResMut<UploadedLevel>,
    mut selected_level: ResMut<Level>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                LevelSelectButtonAction::BackToMenu => {
                    next_ui_state.set(UiState::Menu);
                }
                LevelSelectButtonAction::SelectLevel { level_index } => {
                    let level: &Level = levels_assets_loader
                        .get(&levels_assets.compain[*level_index])
                        .unwrap();
                    if level.error.is_some() {
                        return;
                    }
                    *selected_level = level.clone();

                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::Start);
                }
                LevelSelectButtonAction::UploadLevel => {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(path) = FileDialog::new()
                        .add_filter("RON Files", &["ron"])
                        .show_open_single_file()
                        .unwrap()
                    {
                        let level_handle =
                            asset_server.load::<Level>(path.to_string_lossy().to_string());

                        uploaded_level.handle = Some(level_handle.clone());
                    }
                }
            }
        }
    }
}

fn uploaded_level_update(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    asset_server: Res<AssetServer>,
    levels_assets_loader: Res<Assets<Level>>,
    mut uploaded_level: ResMut<UploadedLevel>,
    mut selected_level: ResMut<Level>,
) {
    if let Some(uploaded_level_handle) = &uploaded_level.handle {
        match asset_server.get_load_state(uploaded_level_handle).unwrap() {
            bevy::asset::LoadState::Loaded => {
                if let Some(level) = levels_assets_loader.get(uploaded_level_handle) {
                    if level.error.is_some() {
                        uploaded_level.handle = None;
                        #[cfg(not(target_arch = "wasm32"))]
                        MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title(&rust_i18n::t!("level_select.file_reading_error.title"))
                            .set_text(&level.error.as_ref().unwrap())
                            .show_alert()
                            .unwrap();
                        return;
                    }
                    *selected_level = level.clone();

                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::Start);
                } else {
                    error!("Level data is missing even though it's marked as loaded.");
                }
                uploaded_level.handle = None;
            }
            LoadState::Failed(error) => {
                uploaded_level.handle = None;
                error!("Failed to load level file: {}", error);
                #[cfg(not(target_arch = "wasm32"))]
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_title(&rust_i18n::t!("level_select.file_upload_error.title"))
                    .set_text(&rust_i18n::t!("level_select.file_upload_error.description"))
                    .show_alert()
                    .unwrap();
            }
            _ => {
                info!("Level is still loading...");
            }
        }
    }
}
