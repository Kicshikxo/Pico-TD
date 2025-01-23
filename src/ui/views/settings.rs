use bevy::{prelude::*, ui::widget::NodeImageMode};
use bevy_persistent::Persistent;

use crate::{
    assets::sprites::ui::UiAssets,
    audio::GameAudioVolume,
    ui::{
        components::{
            button::UiButton,
            container::{UiContainer, UiContainerVariant},
            selector::{UiSelector, UiSelectorItem, UiSelectorItemValue},
            text::UiText,
        },
        i18n::{I18n, Locale},
        UiState,
    },
};

pub struct SettingsViewUiPlugin;

impl Plugin for SettingsViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Settings), ui_init)
            .add_systems(OnExit(UiState::Settings), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::Settings)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum SettingsButtonAction {
    BackToMenu,
}

#[derive(Component)]
struct LocaleSelector;

#[derive(Component)]
struct SfxVolumeSelector;

#[derive(Component)]
struct MusicVolumeSelector;

fn ui_init(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    i18n: Res<Persistent<I18n>>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
) {
    commands
        .spawn((
            RootUiComponent,
            UiContainer::new().with_height(Val::Percent(100.0)).center(),
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
                        SettingsButtonAction::BackToMenu,
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
                        .with_child(UiText::new("ui.settings.title"));

                    parent
                        .spawn(UiContainer::new().with_row_gap(Val::Px(4.0)).column())
                        .with_children(|parent| {
                            parent.spawn(UiText::new("ui.settings.change_locale"));
                            parent.spawn((
                                LocaleSelector,
                                UiSelector::new()
                                    .with_options(vec![
                                        UiSelectorItem::new(
                                            "ui.settings.english_language".to_string(),
                                        )
                                        .with_value(
                                            UiSelectorItemValue::String(Locale::En.to_string()),
                                        ),
                                        UiSelectorItem::new(
                                            "ui.settings.russian_language".to_string(),
                                        )
                                        .with_value(
                                            UiSelectorItemValue::String(Locale::Ru.to_string()),
                                        ),
                                    ])
                                    .with_default_index(i18n.get_current() as usize)
                                    .cycle(),
                            ));
                            parent.spawn(UiText::new("ui.settings.sfx_volume"));
                            parent.spawn((
                                SfxVolumeSelector,
                                UiSelector::new()
                                    .with_options(
                                        (0..=20)
                                            .map(|index| {
                                                UiSelectorItem::new(format!("{}%", index * 5))
                                                    .with_value(UiSelectorItemValue::Number(
                                                        index as f32 / 20.0,
                                                    ))
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
                                                UiSelectorItem::new(format!("{}%", index * 5))
                                                    .with_value(UiSelectorItemValue::Number(
                                                        index as f32 / 20.0,
                                                    ))
                                            })
                                            .collect(),
                                    )
                                    .with_default_index(
                                        (game_audio_volume.get_music_volume() * 20.0) as usize,
                                    ),
                            ));
                        });
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
        (&Interaction, &SettingsButtonAction),
        (Changed<Interaction>, With<UiButton>),
    >,
    mut locale_selector: Query<
        &mut UiSelector,
        (
            With<LocaleSelector>,
            Without<SfxVolumeSelector>,
            Without<MusicVolumeSelector>,
        ),
    >,
    mut sfx_volume_selector: Query<
        &mut UiSelector,
        (
            With<SfxVolumeSelector>,
            Without<LocaleSelector>,
            Without<MusicVolumeSelector>,
        ),
    >,
    mut music_volume_selector: Query<
        &mut UiSelector,
        (
            With<MusicVolumeSelector>,
            Without<LocaleSelector>,
            Without<SfxVolumeSelector>,
        ),
    >,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut i18n: ResMut<Persistent<I18n>>,
    mut game_audio_volume: ResMut<Persistent<GameAudioVolume>>,
) {
    for mut selector in locale_selector.iter_mut() {
        if selector.get_value_changed() {
            i18n.update(|i18n| {
                i18n.set_locale(Locale::from_string(
                    &selector.get_current_item().unwrap().value.as_string(),
                ))
            })
            .unwrap()
        }
    }
    for mut selector in sfx_volume_selector.iter_mut() {
        if selector.get_value_changed() {
            game_audio_volume
                .update(|volume| {
                    volume.set_sfx_volume(selector.get_current_item().unwrap().value.as_number());
                })
                .unwrap();
        }
    }
    for mut selector in music_volume_selector.iter_mut() {
        if selector.get_value_changed() {
            game_audio_volume
                .update(|volume| {
                    volume.set_music_volume(selector.get_current_item().unwrap().value.as_number());
                })
                .unwrap();
        }
    }
    for (interaction, button_action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button_action {
                SettingsButtonAction::BackToMenu => {
                    next_ui_state.set(UiState::Menu);
                }
            }
        }
    }
}
