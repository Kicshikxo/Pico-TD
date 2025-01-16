use bevy::{prelude::*, ui::widget::NodeImageMode};
use bevy_persistent::Persistent;

use crate::{
    assets::ui::UiAssets,
    audio::GameAudioVolume,
    ui::{
        components::{
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
                        .with_child(UiText::new("ui.settings"));
                    parent.spawn(UiText::new("ui.change_locale"));
                    parent.spawn((
                        LocaleSelector,
                        UiSelector::new(vec![
                            UiSelectorItem::new("ui.english_language".to_string())
                                .with_value(UiSelectorItemValue::String(Locale::En.to_string())),
                            UiSelectorItem::new("ui.russian_language".to_string())
                                .with_value(UiSelectorItemValue::String(Locale::Ru.to_string())),
                        ])
                        .with_default_index(i18n.get_current() as usize),
                    ));
                    parent.spawn(UiText::new("ui.sfx_volume"));
                    parent.spawn((
                        SfxVolumeSelector,
                        UiSelector::new(
                            (0..=10)
                                .map(|index| {
                                    UiSelectorItem::new(format!("{}%", index * 10)).with_value(
                                        UiSelectorItemValue::Number(index as f32 / 10.0),
                                    )
                                })
                                .collect(),
                        )
                        .with_default_index((game_audio_volume.get_sfx_volume() * 10.0) as usize),
                    ));
                    parent.spawn(UiText::new("ui.music_volume"));
                    parent.spawn((
                        MusicVolumeSelector,
                        UiSelector::new(
                            (0..=10)
                                .map(|index| {
                                    UiSelectorItem::new(format!("{}%", index * 10)).with_value(
                                        UiSelectorItemValue::Number(index as f32 / 10.0),
                                    )
                                })
                                .collect(),
                        )
                        .with_default_index((game_audio_volume.get_music_volume() * 10.0) as usize),
                    ));
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
        (Changed<Interaction>, With<Button>),
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
