use bevy::prelude::*;

use crate::{
    assets::sprites::ui::{UiAssets, UiMiscSpriteVariant},
    game::{GameSpeed, GameState},
    player::Player,
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
            container::UiContainer,
            text::{UiText, UiTextSize},
        },
        i18n::I18nComponent,
        UiState,
    },
    waves::GameWave,
};

pub struct InGameViewUiPlugin;

impl Plugin for InGameViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::InGame), ui_init)
            .add_systems(OnExit(UiState::InGame), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::InGame)))
            .add_systems(
                Update,
                ui_update_after_player_change
                    .run_if(in_state(UiState::InGame).and(resource_changed::<Player>)),
            )
            .add_systems(
                Update,
                ui_update_after_wave_change
                    .run_if(in_state(UiState::InGame).and(resource_changed::<GameWave>)),
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
struct CurrentSpeedTextComponent;

#[derive(Component, PartialEq)]
enum InGameButtonAction {
    ChangeSpeed,
    Pause,
    NextWave,
}

fn ui_init(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    player: Res<Player>,
    game_wave: Res<GameWave>,
    game_speed: Res<GameSpeed>,
) {
    commands
        .spawn((
            RootUiComponent,
            UiContainer::new().with_height(Val::Percent(100.0)),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(8.0),
                    left: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(UiContainer::new().column())
                        .with_children(|parent| {
                            parent
                                .spawn(UiContainer::new().with_column_gap(Val::Px(8.0)).center())
                                .with_children(|parent| {
                                    parent.spawn((
                                        Node {
                                            width: Val::Px(32.0),
                                            height: Val::Px(32.0),
                                            ..default()
                                        },
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
                                            .with_arg(
                                                "health",
                                                player.get_health().get_current().to_string(),
                                            ),
                                    ));
                                });

                            parent
                                .spawn(UiContainer::new().with_column_gap(Val::Px(8.0)).center())
                                .with_children(|parent| {
                                    parent.spawn((
                                        Node {
                                            width: Val::Px(32.0),
                                            height: Val::Px(32.0),
                                            ..default()
                                        },
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
                                            .with_arg(
                                                "money",
                                                player.get_money().get_current().to_string(),
                                            ),
                                    ));
                                });
                        });
                });

            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(8.0),
                    right: Val::Px(8.0),
                    ..default()
                })
                .with_child((
                    WaveTextComponent,
                    UiText::new("ui.in_game.wave")
                        .with_arg("current", (game_wave.get_current() + 1).to_string())
                        .with_arg("total", game_wave.get_total().to_string()),
                ));

            parent
                .spawn(Node {
                    display: Display::Grid,
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(8.0),
                    right: Val::Px(8.0),
                    row_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(UiContainer::new().with_column_gap(Val::Px(8.0)))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    InGameButtonAction::ChangeSpeed,
                                    UiButton::new()
                                        .with_variant(UiButtonVariant::Primary)
                                        .with_padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0))),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        CurrentSpeedTextComponent,
                                        UiText::new("ui.in_game.game_speed")
                                            .with_arg("speed", game_speed.as_f32().to_string())
                                            .with_size(UiTextSize::Small)
                                            .no_wrap(),
                                    ));
                                });

                            parent
                                .spawn((
                                    InGameButtonAction::Pause,
                                    UiButton::new()
                                        .with_variant(UiButtonVariant::Primary)
                                        .with_padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0))),
                                ))
                                .with_child(
                                    UiText::new("ui.in_game.pause").with_size(UiTextSize::Small),
                                );
                        });

                    parent
                        .spawn((
                            InGameButtonAction::NextWave,
                            UiButton::new()
                                .with_variant(UiButtonVariant::Primary)
                                .with_disabled(game_wave.is_next_wave_allowed() == false)
                                .with_padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0))),
                        ))
                        .with_child(
                            UiText::new("ui.in_game.next_wave").with_size(UiTextSize::Small),
                        );
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
        (&Interaction, &InGameButtonAction),
        (Changed<Interaction>, With<UiButton>),
    >,
    mut current_speed_text: Query<&mut I18nComponent, With<CurrentSpeedTextComponent>>,
    mut game_wave: ResMut<GameWave>,
    mut game_speed: ResMut<GameSpeed>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                InGameButtonAction::ChangeSpeed => {
                    game_speed.toggle();
                    if let Ok(mut current_speed_text_i18n) = current_speed_text.get_single_mut() {
                        current_speed_text_i18n
                            .change_arg("speed", game_speed.as_f32().to_string());
                    }
                }
                InGameButtonAction::Pause => {
                    next_ui_state.set(UiState::Pause);
                    next_game_state.set(GameState::Pause);
                }
                InGameButtonAction::NextWave => {
                    if game_wave.is_next_wave_allowed() == true {
                        game_wave.next_wave();
                    }
                }
            }
        }
    }
}

fn ui_update_after_player_change(
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
        health_text_i18n.change_arg("health", player.get_health().get_current().to_string());
    }
    for mut money_text_i18n in money_text.iter_mut() {
        money_text_i18n.change_arg("money", player.get_money().get_current().to_string());
    }
}

fn ui_update_after_wave_change(
    game_wave: Res<GameWave>,
    mut next_wave_button: Query<(&mut UiButton, &InGameButtonAction)>,
    mut wave_text: Query<&mut I18nComponent, With<WaveTextComponent>>,
) {
    for mut wave_text_i18n in wave_text.iter_mut() {
        wave_text_i18n.change_arg("current", (game_wave.get_current() + 1).to_string());
        wave_text_i18n.change_arg("total", game_wave.get_total().to_string())
    }
    for (mut ui_button, button_action) in next_wave_button.iter_mut() {
        ui_button.set_disabled(
            *button_action == InGameButtonAction::NextWave
                && game_wave.is_next_wave_allowed() == false,
        );
    }
}
