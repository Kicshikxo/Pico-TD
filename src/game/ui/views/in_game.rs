use bevy::prelude::*;

use crate::game::{
    assets::images::ui::{UiAssets, UiMiscSpriteVariant},
    player::Player,
    speed::GameSpeed,
    ui::{
        components::{
            button::UiButton,
            container::UiContainer,
            selector::{UiSelector, UiSelectorItem, UiSelectorItemValue, UiSelectorSize},
            text::{UiText, UiTextSize},
        },
        i18n::I18nComponent,
        UiState,
    },
    waves::GameWaves,
    GameState,
};

pub struct InGameViewUiPlugin;

impl Plugin for InGameViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::InGame), init_ui)
            .add_systems(OnExit(UiState::InGame), destroy_ui)
            .add_systems(Update, update_ui.run_if(in_state(UiState::InGame)))
            .add_systems(
                Update,
                update_ui_after_player_change
                    .run_if(in_state(UiState::InGame).and(resource_changed::<Player>)),
            )
            .add_systems(
                Update,
                update_ui_after_wave_change
                    .run_if(in_state(UiState::InGame).and(resource_changed::<GameWaves>)),
            );
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
struct HealthTextComponent;
#[derive(Component)]
struct MoneyTextComponent;
#[derive(Component)]
struct WaveTextComponent;

#[derive(Component)]
struct SpeedSelector;

#[derive(Component, PartialEq)]
enum ButtonAction {
    Pause,
    NextWave,
}

fn init_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    player: Res<Player>,
    game_waves: Res<GameWaves>,
    game_speed: Res<GameSpeed>,
) {
    commands
        .spawn((RootUiComponent, UiContainer::new().full()))
        .with_children(|parent| {
            parent
                .spawn(
                    UiContainer::new()
                        .with_left(Val::Px(8.0))
                        .with_top(Val::Px(8.0))
                        .absolute(),
                )
                .with_children(|parent| {
                    parent
                        .spawn(UiContainer::new().column())
                        .with_children(|parent| {
                            parent
                                .spawn(UiContainer::new().with_column_gap(Val::Px(8.0)).center())
                                .with_children(|parent| {
                                    parent.spawn((
                                        UiContainer::new()
                                            .with_width(Val::Px(32.0))
                                            .with_height(Val::Px(32.0)),
                                        ImageNode {
                                            image: ui_assets.ui_misc.clone(),
                                            texture_atlas: Some(TextureAtlas {
                                                index: UiMiscSpriteVariant::Health as usize,
                                                layout: ui_assets.ui_misc_layout.clone(),
                                            }),
                                            ..default()
                                        },
                                    ));
                                    parent.spawn((
                                        HealthTextComponent,
                                        UiText::new("ui.in_game.health")
                                            .with_justify(JustifyText::Left)
                                            .with_i18n_arg(
                                                "health",
                                                player.get_health().get_current().to_string(),
                                            ),
                                    ));
                                });

                            parent
                                .spawn(UiContainer::new().with_column_gap(Val::Px(8.0)).center())
                                .with_children(|parent| {
                                    parent.spawn((
                                        UiContainer::new()
                                            .with_width(Val::Px(32.0))
                                            .with_height(Val::Px(32.0)),
                                        ImageNode {
                                            image: ui_assets.ui_misc.clone(),
                                            texture_atlas: Some(TextureAtlas {
                                                index: UiMiscSpriteVariant::Money as usize,
                                                layout: ui_assets.ui_misc_layout.clone(),
                                            }),
                                            ..default()
                                        },
                                    ));
                                    parent.spawn((
                                        MoneyTextComponent,
                                        UiText::new("ui.in_game.money")
                                            .with_justify(JustifyText::Left)
                                            .with_i18n_arg(
                                                "money",
                                                player.get_money().get_current().to_string(),
                                            ),
                                    ));
                                });
                        });
                });

            parent
                .spawn(
                    UiContainer::new()
                        .with_right(Val::Px(8.0))
                        .with_top(Val::Px(8.0))
                        .with_width(Val::Auto)
                        .absolute(),
                )
                .with_child((
                    WaveTextComponent,
                    UiText::new("ui.in_game.wave")
                        .with_i18n_arg(
                            "current",
                            game_waves.get_current().saturating_add(1).to_string(),
                        )
                        .with_i18n_arg(
                            "total",
                            game_waves.get_total().saturating_add(1).to_string(),
                        ),
                ));

            parent
                .spawn((
                    Button,
                    UiContainer::new()
                        .with_right(Val::Px(8.0))
                        .with_bottom(Val::Px(8.0))
                        .with_width(Val::Auto)
                        .with_row_gap(Val::Px(8.0))
                        .grid()
                        .absolute(),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonAction::NextWave,
                            UiButton::success()
                                .with_disabled(game_waves.is_next_wave_allowed() == false)
                                .with_height(Val::Px(32.0))
                                .with_padding(UiRect::horizontal(Val::Px(16.0))),
                        ))
                        .with_child(
                            UiText::new("ui.in_game.next_wave").with_size(UiTextSize::Small),
                        );

                    parent.spawn((
                        SpeedSelector,
                        UiSelector::new()
                            .with_size(UiSelectorSize::Small)
                            .with_options(
                                (1..=5)
                                    .map(|index| {
                                        let game_speed = GameSpeed::from_f32(index as f32);

                                        UiSelectorItem::new("ui.in_game.game_speed")
                                            .with_i18n_arg("speed", game_speed.as_f32().to_string())
                                            .with_value(UiSelectorItemValue::Number(
                                                game_speed.as_f32(),
                                            ))
                                    })
                                    .collect::<Vec<_>>(),
                            )
                            .with_default_index(game_speed.as_index()),
                    ));

                    parent
                        .spawn((
                            ButtonAction::Pause,
                            UiButton::danger()
                                .with_height(Val::Px(32.0))
                                .with_padding(UiRect::horizontal(Val::Px(16.0))),
                        ))
                        .with_child(UiText::new("ui.in_game.pause").with_size(UiTextSize::Small));
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
    mut speed_selector: Query<&mut UiSelector, With<SpeedSelector>>,
    mut game_waves: ResMut<GameWaves>,
    mut game_speed: ResMut<GameSpeed>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(mut speed_selector) = speed_selector.get_single_mut() {
        if let Some(changed_item) = speed_selector.get_changed_item() {
            game_speed.set(GameSpeed::from_f32(changed_item.value.as_f32()));
        }
    }
    for (interaction, button_action) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button_action {
            ButtonAction::Pause => {
                next_ui_state.set(UiState::Pause);
                next_game_state.set(GameState::Pause);
            }
            ButtonAction::NextWave => {
                if game_waves.is_next_wave_allowed() == true {
                    game_waves.next_wave();
                }
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_ui_state.set(UiState::Pause);
        next_game_state.set(GameState::Pause);
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        if game_waves.is_next_wave_allowed() == true {
            game_waves.next_wave();
        }
    }
    if let Ok(mut speed_selector) = speed_selector.get_single_mut() {
        let speed = if keyboard_input.just_pressed(KeyCode::Digit1) {
            GameSpeed::Normal
        } else if keyboard_input.just_pressed(KeyCode::Digit2) {
            GameSpeed::Double
        } else if keyboard_input.just_pressed(KeyCode::Digit3) {
            GameSpeed::Triple
        } else if keyboard_input.just_pressed(KeyCode::Digit4) {
            GameSpeed::Quadruple
        } else if keyboard_input.just_pressed(KeyCode::Digit5) {
            GameSpeed::Quintuple
        } else {
            return;
        };

        game_speed.set(speed);
        speed_selector.set_index(game_speed.as_index());
    }
}

fn update_ui_after_player_change(
    player: Res<Player>,
    mut health_text: Query<
        &mut I18nComponent,
        (With<HealthTextComponent>, Without<MoneyTextComponent>),
    >,
    mut money_text: Query<
        &mut I18nComponent,
        (With<MoneyTextComponent>, Without<HealthTextComponent>),
    >,
) {
    for mut health_text_i18n in health_text.iter_mut() {
        health_text_i18n.change_i18n_arg("health", player.get_health().get_current().to_string());
    }
    for mut money_text_i18n in money_text.iter_mut() {
        money_text_i18n.change_i18n_arg("money", player.get_money().get_current().to_string());
    }
}

fn update_ui_after_wave_change(
    game_waves: Res<GameWaves>,
    mut next_wave_button: Query<(&mut UiButton, &ButtonAction)>,
    mut wave_text: Query<&mut I18nComponent, With<WaveTextComponent>>,
) {
    for mut wave_text_i18n in wave_text.iter_mut() {
        wave_text_i18n.change_i18n_arg(
            "current",
            game_waves.get_current().saturating_add(1).to_string(),
        );
        wave_text_i18n.change_i18n_arg(
            "total",
            game_waves.get_total().saturating_add(1).to_string(),
        )
    }
    for (mut ui_button, button_action) in next_wave_button.iter_mut() {
        ui_button.set_next_disabled_state(
            *button_action == ButtonAction::NextWave && game_waves.is_next_wave_allowed() == false,
        );
    }
}
