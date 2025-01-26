use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::{
    assets::sprites::ui::UiAssets,
    audio::GameAudioVolume,
    game::GameState,
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
            container::{UiContainer, UiContainerVariant},
            selector::{UiSelector, UiSelectorItem, UiSelectorItemValue},
            text::{UiText, UiTextSize},
        },
        UiState,
    },
};

pub struct PauseViewUiPlugin;

impl Plugin for PauseViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Pause), ui_init)
            .add_systems(OnExit(UiState::Pause), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::Pause)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
struct SfxVolumeSelector;

#[derive(Component)]
struct MusicVolumeSelector;

#[derive(Component)]
enum PauseButtonAction {
    Close,
    BackToMenu,
}

fn ui_init(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
) {
    commands
        .spawn((
            RootUiComponent,
            UiContainer::new().with_height(Val::Percent(100.0)).center(),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        ))
        .with_children(|parent| {
            parent
                .spawn(
                    UiContainer::new()
                        .with_variant(UiContainerVariant::Primary)
                        .with_width(Val::Px(320.0))
                        .with_padding(UiRect::all(Val::Px(24.0)))
                        .with_row_gap(Val::Px(12.0))
                        .center()
                        .column(),
                )
                .with_children(|parent| {
                    parent.spawn((
                        UiButton::new(),
                        PauseButtonAction::Close,
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
                        .spawn(
                            UiContainer::new()
                                .with_variant(UiContainerVariant::Secondary)
                                .with_padding(UiRect::all(Val::Px(8.0))),
                        )
                        .with_child(UiText::new("ui.pause.title").with_size(UiTextSize::Large));

                    parent.spawn(UiText::new("ui.settings.sfx_volume"));
                    parent.spawn((
                        SfxVolumeSelector,
                        UiSelector::new()
                            .with_options(
                                (0..=20)
                                    .map(|index| {
                                        UiSelectorItem::new(format!("{}%", index * 5)).with_value(
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
                                        UiSelectorItem::new(format!("{}%", index * 5)).with_value(
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
                        .spawn((
                            PauseButtonAction::BackToMenu,
                            UiButton::new().with_variant(UiButtonVariant::Primary),
                        ))
                        .with_child(UiText::new("ui.pause.back_to_menu"));
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
        (&Interaction, &PauseButtonAction),
        (Changed<Interaction>, With<UiButton>),
    >,
    mut pause_selectors: ParamSet<(
        Query<&mut UiSelector, With<SfxVolumeSelector>>,
        Query<&mut UiSelector, With<MusicVolumeSelector>>,
    )>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_audio_volume: ResMut<Persistent<GameAudioVolume>>,
) {
    for mut sfx_volume_selector in pause_selectors.p0().iter_mut() {
        if let Some(changed_item) = sfx_volume_selector.get_changed_item() {
            game_audio_volume
                .update(|volume| {
                    volume.set_sfx_volume(changed_item.value.as_number());
                })
                .unwrap();
        }
    }
    for mut music_volume_selector in pause_selectors.p1().iter_mut() {
        if let Some(changed_item) = music_volume_selector.get_changed_item() {
            game_audio_volume
                .update(|volume| {
                    volume.set_music_volume(changed_item.value.as_number());
                })
                .unwrap();
        }
    }
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                PauseButtonAction::Close => {
                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);
                }
                PauseButtonAction::BackToMenu => {
                    next_ui_state.set(UiState::Menu);
                    next_game_state.set(GameState::Pause);
                }
            }
        }
    }
}
