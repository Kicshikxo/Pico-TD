use bevy::{prelude::*, ui::widget::NodeImageMode};
use bevy_persistent::Persistent;

use crate::game::{
    assets::{
        levels::CompletedLevels,
        sprites::ui::{UiAssets, UiButtonSpriteVariant, UiMiscSpriteVariant},
    },
    audio::GameAudioVolume,
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
            container::{UiContainer, UiContainerVariant},
            selector::{UiSelector, UiSelectorItem, UiSelectorItemValue},
            text::{UiText, UiTextSize},
        },
        i18n::{I18n, Locale},
        UiState,
    },
};

pub struct SettingsViewUiPlugin;

impl Plugin for SettingsViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Settings), init_ui)
            .add_systems(OnExit(UiState::Settings), destroy_ui)
            .add_systems(Update, update_ui.run_if(in_state(UiState::Settings)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
struct LocaleSelector;

#[derive(Component)]
struct SfxVolumeSelector;

#[derive(Component)]
struct MusicVolumeSelector;

#[derive(Component)]
enum ButtonAction {
    BackToMenu,
    ResetProgress,
}

fn init_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    i18n: Res<Persistent<I18n>>,
    completed_levels: Res<Persistent<CompletedLevels>>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
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
                        .spawn(
                            UiContainer::new()
                                .with_variant(UiContainerVariant::Secondary)
                                .with_padding(UiRect::all(Val::Px(8.0))),
                        )
                        .with_child(UiText::new("ui.settings.title").with_size(UiTextSize::Large));

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

                    parent
                        .spawn((
                            ButtonAction::ResetProgress,
                            UiButton::new()
                                .with_variant(UiButtonVariant::Danger)
                                .with_disabled(completed_levels.is_empty()),
                        ))
                        .with_child(UiText::new("ui.settings.reset_progress"));
                });
        });
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_ui(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<UiButton>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut settings_selectors: ParamSet<(
        Query<&mut UiSelector, With<LocaleSelector>>,
        Query<&mut UiSelector, With<SfxVolumeSelector>>,
        Query<&mut UiSelector, With<MusicVolumeSelector>>,
    )>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut i18n: ResMut<Persistent<I18n>>,
    mut completed_levels: ResMut<Persistent<CompletedLevels>>,
    mut game_audio_volume: ResMut<Persistent<GameAudioVolume>>,
) {
    for mut locale_selector in settings_selectors.p0().iter_mut() {
        if let Some(changed_item) = locale_selector.get_changed_item() {
            i18n.update(|i18n| {
                i18n.set_locale(Locale::from_string(&changed_item.value.as_string()))
            })
            .unwrap();
        }
    }
    for mut sfx_volume_selector in settings_selectors.p1().iter_mut() {
        if let Some(changed_item) = sfx_volume_selector.get_changed_item() {
            game_audio_volume
                .update(|volume| {
                    volume.set_sfx_volume(changed_item.value.as_number());
                })
                .unwrap();
        }
    }
    for mut music_volume_selector in settings_selectors.p2().iter_mut() {
        if let Some(changed_item) = music_volume_selector.get_changed_item() {
            game_audio_volume
                .update(|volume| {
                    volume.set_music_volume(changed_item.value.as_number());
                })
                .unwrap();
        }
    }
    for (interaction, button_action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::BackToMenu => {
                    next_ui_state.set(UiState::Menu);
                }
                ButtonAction::ResetProgress => {
                    if completed_levels.is_empty() {
                        continue;
                    }
                    completed_levels.update(|levels| levels.reset()).unwrap();
                    next_ui_state.set(UiState::Menu);
                }
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_ui_state.set(UiState::Menu);
    }
}
