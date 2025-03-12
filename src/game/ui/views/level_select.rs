use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    ui::widget::NodeImageMode,
};
use bevy_persistent::Persistent;
use rfd::{AsyncFileDialog, MessageDialog, MessageLevel};

use crate::game::{
    assets::{
        audio::ui::UiAudioAssets,
        images::ui::{UiAssets, UiButtonSpriteVariant, UiMiscSpriteVariant},
        levels::{CompletedLevels, Level, LevelCompletionStars, LevelsAssets},
    },
    audio::{GameAudio, GameAudioVolume},
    ui::{
        components::{
            button::{UiButton, UiButtonInteraction, UiButtonVariant},
            container::UiContainer,
            text::{UiText, UiTextSize},
        },
        UiState,
    },
    GameState,
};

pub struct LevelSelectViewUiPlugin;

impl Plugin for LevelSelectViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UploadedLevel::default());

        app.add_systems(OnEnter(UiState::LevelSelect), init_ui)
            .add_systems(OnExit(UiState::LevelSelect), destroy_ui)
            .add_systems(
                Update,
                (update_ui, uploaded_level_update).run_if(in_state(UiState::LevelSelect)),
            );
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum ButtonAction {
    BackToMenu,
    SelectLevel { level_index: usize },
    UploadLevel,
}

#[derive(Resource)]
struct UploadedLevel {
    task: Option<Task<Option<Level>>>,
}

impl Default for UploadedLevel {
    fn default() -> Self {
        Self { task: None }
    }
}

fn init_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    ui_audio_assets: Res<UiAudioAssets>,
    levels_assets: Res<LevelsAssets>,
    levels_assets_loader: Res<Assets<Level>>,
    mut images: ResMut<Assets<Image>>,
    completed_levels: Res<Persistent<CompletedLevels>>,
) {
    commands
        .spawn((
            RootUiComponent,
            UiContainer::new().full().center(),
            ImageNode {
                image: ui_assets.ui_misc.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: UiMiscSpriteVariant::Background as usize,
                    layout: ui_assets.ui_misc_layout.clone(),
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
                .spawn(
                    UiContainer::primary()
                        .with_width(Val::Px(320.0))
                        .with_padding(UiRect::all(Val::Px(24.0)))
                        .with_row_gap(Val::Px(12.0))
                        .center()
                        .column(),
                )
                .with_children(|parent| {
                    parent.spawn((
                        ButtonAction::BackToMenu,
                        UiButton::new(),
                        UiContainer::new()
                            .with_width(Val::Px(32.0))
                            .with_right(Val::Px(38.0))
                            .with_top(Val::Px(-6.0))
                            .absolute(),
                        ImageNode {
                            image: ui_assets.ui_buttons.clone(),
                            texture_atlas: Some(TextureAtlas {
                                index: UiButtonSpriteVariant::Close as usize,
                                layout: ui_assets.ui_buttons_layout.clone(),
                            }),
                            ..default()
                        },
                    ));

                    parent
                        .spawn(UiContainer::secondary().with_padding(UiRect::all(Val::Px(8.0))))
                        .with_child(
                            UiText::new("ui.level_select.title").with_size(UiTextSize::Large),
                        );

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

                                let level_completion =
                                    completed_levels.get_completion(&level.get_name());
                                let level_stars = if level_completion.is_some() {
                                    level_completion.unwrap().get_stars()
                                } else {
                                    &LevelCompletionStars::Zero
                                };

                                parent
                                    .spawn(UiContainer::new().with_row_gap(Val::Px(4.0)).column())
                                    .with_children(|parent| {
                                        parent
                                            .spawn((
                                                ButtonAction::SelectLevel { level_index },
                                                UiButton::new()
                                                    .with_variant(if level.get_error().is_some() {
                                                        UiButtonVariant::Danger
                                                    } else {
                                                        if level_completion.is_some() {
                                                            UiButtonVariant::Success
                                                        } else {
                                                            UiButtonVariant::Secondary
                                                        }
                                                    })
                                                    .with_click_audio(
                                                        ui_audio_assets.level_select.clone(),
                                                    )
                                                    .with_padding(UiRect::all(Val::Px(8.0)))
                                                    .with_aspect_ratio(1.0),
                                            ))
                                            .with_children(|parent| {
                                                if level.get_error().is_some() {
                                                    return;
                                                }

                                                parent
                                                    .spawn((
                                                        UiContainer::new()
                                                            .with_bottom(Val::Px(-4.0))
                                                            .absolute()
                                                            .center(),
                                                        ZIndex(1),
                                                    ))
                                                    .with_children(|parent| {
                                                        for star_index in 1..=3 {
                                                            parent.spawn((
                                                                UiContainer::new()
                                                                    .with_bottom(
                                                                        if star_index == 2 {
                                                                            Val::Px(4.0)
                                                                        } else {
                                                                            Val::Px(0.0)
                                                                        },
                                                                    )
                                                                    .with_width(Val::Px(16.0))
                                                                    .with_height(Val::Px(16.0)),
                                                                ImageNode {
                                                                    color: if star_index
                                                                        <= level_stars.as_index()
                                                                    {
                                                                        Color::srgb(1.0, 1.0, 0.0)
                                                                    } else {
                                                                        Color::WHITE
                                                                    },
                                                                    image: ui_assets
                                                                        .ui_misc
                                                                        .clone(),
                                                                    texture_atlas: Some(
                                                                        TextureAtlas {
                                                                            layout: ui_assets
                                                                                .ui_misc_layout
                                                                                .clone(),
                                                                            index: UiMiscSpriteVariant::Star
                                                                                as usize,
                                                                        },
                                                                    ),
                                                                    ..default()
                                                                },
                                                            ));
                                                        }
                                                    });

                                                parent.spawn((
                                                    UiContainer::new().full(),
                                                    ImageNode {
                                                        image: images.add(level.get_preview()),
                                                        ..default()
                                                    },
                                                ));
                                            });

                                        parent.spawn(
                                            UiText::new(&format!("level.{}", level.get_name()))
                                                .with_size(UiTextSize::Small),
                                        );
                                    });
                            }
                        });

                    parent
                        .spawn((ButtonAction::UploadLevel, UiButton::primary()))
                        .with_child(UiText::new("ui.level_select.upload_level"));
                });

            parent
                .spawn(
                    UiContainer::new()
                        .with_right(Val::Px(8.0))
                        .with_bottom(Val::Px(8.0))
                        .absolute(),
                )
                .with_child(
                    UiText::new("ui.version")
                        .with_size(UiTextSize::Small)
                        .with_justify(JustifyText::Right)
                        .with_i18n_arg("version", env!("CARGO_PKG_VERSION").to_string()),
                );
        });
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_ui(
    interaction_query: Query<
        (&UiButtonInteraction, &ButtonAction),
        (Changed<UiButtonInteraction>, With<UiButton>),
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    levels_assets: Res<LevelsAssets>,
    levels_assets_loader: Res<Assets<Level>>,
    mut uploaded_level: ResMut<UploadedLevel>,
    mut selected_level: ResMut<Level>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (ui_button_interaction, button_action) in interaction_query.iter() {
        if *ui_button_interaction != UiButtonInteraction::Clicked {
            continue;
        }
        match button_action {
            ButtonAction::BackToMenu => {
                next_ui_state.set(UiState::Menu);
            }
            ButtonAction::SelectLevel { level_index } => {
                if let Some(level) = levels_assets_loader.get(&levels_assets.compain[*level_index])
                {
                    if level.get_error().is_none() {
                        *selected_level = level.clone();
                        next_game_state.set(GameState::Start);
                    }
                }
            }
            ButtonAction::UploadLevel => {
                if uploaded_level.task.is_some() {
                    continue;
                }
                uploaded_level.task = Some(AsyncComputeTaskPool::get().spawn(async move {
                    if let Some(file) = AsyncFileDialog::new()
                        .add_filter("RON Files", &["ron"])
                        .pick_file()
                        .await
                    {
                        let bytes = file.read().await;
                        let source = std::str::from_utf8(&bytes).unwrap_or_default();
                        Some(Level::from_source(source))
                    } else {
                        None
                    }
                }));
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_ui_state.set(UiState::Menu);
    }
}

fn uploaded_level_update(
    mut commands: Commands,
    game_audio: Query<Entity, With<GameAudio>>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    ui_audio_assets: Res<UiAudioAssets>,
    mut uploaded_level: ResMut<UploadedLevel>,
    mut selected_level: ResMut<Level>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Some(mut uploaded_level_task) = uploaded_level.task.take() {
        if let Some(uploaded_level_task_result) =
            bevy::tasks::block_on(bevy::tasks::poll_once(&mut uploaded_level_task))
        {
            if let Some(level) = uploaded_level_task_result {
                if level.get_error().is_some() {
                    MessageDialog::new()
                        .set_level(MessageLevel::Error)
                        .set_title(rust_i18n::t!("error.level_select.file_reading_error.title"))
                        .set_description(level.get_error().unwrap())
                        .show();
                } else {
                    *selected_level = level.clone();
                    next_game_state.set(GameState::Start);

                    commands.entity(game_audio.single()).with_child((
                        AudioPlayer::new(ui_audio_assets.level_select.clone()),
                        PlaybackSettings {
                            mode: PlaybackMode::Remove,
                            volume: Volume::new(game_audio_volume.get_sfx_volume()),
                            ..default()
                        },
                    ));
                }
            }
        } else {
            uploaded_level.task = Some(uploaded_level_task);
        }
    }
}
