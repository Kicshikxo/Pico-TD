use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::game::{
    assets::images::ui::{UiAssets, UiButtonSpriteVariant},
    audio::GameAudioVolume,
    config::GameConfig,
    entities::enemy::path::EnemyPathVisibility,
    ui::{
        components::{
            button::{UiButton, UiButtonInteraction},
            container::UiContainer,
            selector::{UiSelector, UiSelectorItem, UiSelectorItemValue, UiSelectorSize},
            text::{UiText, UiTextSize},
        },
        UiState,
    },
    GameState,
};

pub struct PauseViewUiPlugin;

impl Plugin for PauseViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Pause), init_ui)
            .add_systems(OnExit(UiState::Pause), destroy_ui)
            .add_systems(Update, update_ui.run_if(in_state(UiState::Pause)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
struct SfxVolumeSelector;

#[derive(Component)]
struct MusicVolumeSelector;

#[derive(Component)]
struct EnemyPathVisibilitySelector;

#[derive(Component)]
enum ButtonAction {
    Close,
    RestartLevel,
    BackToMenu,
}

fn init_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    game_config: Res<Persistent<GameConfig>>,
) {
    commands
        .spawn((
            RootUiComponent,
            UiContainer::new().full().center(),
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
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
                        ButtonAction::Close,
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
                        .with_child(UiText::new("ui.pause.title").with_size(UiTextSize::Large));

                    parent.spawn(UiText::new("ui.settings.sfx_volume"));

                    parent.spawn((
                        SfxVolumeSelector,
                        UiSelector::new()
                            .with_options(
                                (0..=20)
                                    .map(|index| {
                                        UiSelectorItem::new(&format!("{}%", index * 5)).with_value(
                                            UiSelectorItemValue::Number(index as f32 / 20.0),
                                        )
                                    })
                                    .collect(),
                            )
                            .with_default_index(
                                (game_audio_volume.get_sfx_volume() * 20.0) as usize,
                            ),
                    ));

                    parent.spawn(UiText::new("ui.settings.music_volume"));

                    parent.spawn((
                        MusicVolumeSelector,
                        UiSelector::new()
                            .with_options(
                                (0..=20)
                                    .map(|index| {
                                        UiSelectorItem::new(&format!("{}%", index * 5)).with_value(
                                            UiSelectorItemValue::Number(index as f32 / 20.0),
                                        )
                                    })
                                    .collect(),
                            )
                            .with_default_index(
                                (game_audio_volume.get_music_volume() * 20.0) as usize,
                            ),
                    ));

                    parent
                        .spawn(UiContainer::new().with_row_gap(Val::Px(8.0)).column())
                        .with_children(|parent| {
                            parent.spawn(
                                UiText::new("ui.pause.enemy_path_visibility")
                                    .with_size(UiTextSize::Small),
                            );

                            parent.spawn((
                                EnemyPathVisibilitySelector,
                                UiSelector::new()
                                    .with_size(UiSelectorSize::Small)
                                    .with_options(
                                        [
                                            EnemyPathVisibility::PreWaveVisible,
                                            EnemyPathVisibility::AlwaysVisible,
                                            EnemyPathVisibility::NeverVisible,
                                        ]
                                        .iter()
                                        .map(|visibility| {
                                            UiSelectorItem::new(visibility.to_str()).with_value(
                                                UiSelectorItemValue::Number(
                                                    visibility.as_index() as f32
                                                ),
                                            )
                                        })
                                        .collect(),
                                    )
                                    .with_default_index(
                                        game_config.get_enemy_path_visibility().as_index(),
                                    ),
                            ));
                        });

                    parent
                        .spawn((ButtonAction::Close, UiButton::success()))
                        .with_child(UiText::new("ui.pause.resume_game"));

                    parent
                        .spawn((ButtonAction::RestartLevel, UiButton::primary()))
                        .with_child(UiText::new("ui.pause.restart_level"));

                    parent
                        .spawn((ButtonAction::BackToMenu, UiButton::danger()))
                        .with_child(UiText::new("ui.pause.back_to_menu"));
                });
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
    mut pause_selectors: ParamSet<(
        Query<&mut UiSelector, With<SfxVolumeSelector>>,
        Query<&mut UiSelector, With<MusicVolumeSelector>>,
        Query<&mut UiSelector, With<EnemyPathVisibilitySelector>>,
    )>,
    mut game_config: ResMut<Persistent<GameConfig>>,
    mut game_audio_volume: ResMut<Persistent<GameAudioVolume>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for mut sfx_volume_selector in pause_selectors.p0().iter_mut() {
        if let Some(changed_item) = sfx_volume_selector.get_changed_item() {
            game_audio_volume
                .update(|volume| {
                    volume.set_sfx_volume(changed_item.value.as_f32());
                })
                .unwrap();
        }
    }
    for mut music_volume_selector in pause_selectors.p1().iter_mut() {
        if let Some(changed_item) = music_volume_selector.get_changed_item() {
            game_audio_volume
                .update(|volume| {
                    volume.set_music_volume(changed_item.value.as_f32());
                })
                .unwrap();
        }
    }
    for mut enemy_path_visibility_selector in pause_selectors.p2().iter_mut() {
        if let Some(changed_item) = enemy_path_visibility_selector.get_changed_item() {
            game_config
                .update(|game_config| {
                    game_config.set_enemy_path_visibility(EnemyPathVisibility::from_index(
                        changed_item.value.as_f32() as usize,
                    ));
                })
                .unwrap();
        }
    }
    for (ui_button_interaction, button_action) in interaction_query.iter() {
        if *ui_button_interaction != UiButtonInteraction::Clicked {
            continue;
        }
        match button_action {
            ButtonAction::Close => {
                next_ui_state.set(UiState::InGame);
                next_game_state.set(GameState::InGame);
            }
            ButtonAction::RestartLevel => {
                next_game_state.set(GameState::Start);
            }
            ButtonAction::BackToMenu => {
                next_ui_state.set(UiState::Menu);
                next_game_state.set(GameState::Pause);
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_ui_state.set(UiState::InGame);
        next_game_state.set(GameState::InGame);
    }
}
