use bevy::prelude::*;

use crate::game::{
    assets::sprites::ui::{UiAssets, UiMiscSpriteVariant},
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
    {GameSpeed, GameState},
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
enum ButtonAction {
    ChangeSpeed,
    Pause,
    NextWave,
}

fn init_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    player: Res<Player>,
    game_wave: Res<GameWave>,
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
                                            .with_arg(
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
                        .with_arg(
                            "current",
                            game_wave.get_current().saturating_add(1).to_string(),
                        )
                        .with_arg("total", game_wave.get_total().to_string()),
                ));

            parent
                .spawn(
                    UiContainer::new()
                        .with_right(Val::Px(8.0))
                        .with_bottom(Val::Px(8.0))
                        .with_width(Val::Auto)
                        .with_row_gap(Val::Px(8.0))
                        .grid()
                        .absolute(),
                )
                .with_children(|parent| {
                    parent
                        .spawn(UiContainer::new().with_column_gap(Val::Px(8.0)))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonAction::ChangeSpeed,
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
                                    ButtonAction::Pause,
                                    UiButton::new()
                                        .with_variant(UiButtonVariant::Danger)
                                        .with_padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0))),
                                ))
                                .with_child(
                                    UiText::new("ui.in_game.pause").with_size(UiTextSize::Small),
                                );
                        });

                    parent
                        .spawn((
                            ButtonAction::NextWave,
                            UiButton::new()
                                .with_variant(UiButtonVariant::Success)
                                .with_disabled(game_wave.is_next_wave_allowed() == false)
                                .with_padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0))),
                        ))
                        .with_child(
                            UiText::new("ui.in_game.next_wave").with_size(UiTextSize::Small),
                        );
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
    mut current_speed_text: Query<&mut I18nComponent, With<CurrentSpeedTextComponent>>,
    mut game_wave: ResMut<GameWave>,
    mut game_speed: ResMut<GameSpeed>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::ChangeSpeed => {
                    game_speed.toggle();
                    if let Ok(mut current_speed_text_i18n) = current_speed_text.get_single_mut() {
                        current_speed_text_i18n
                            .change_arg("speed", game_speed.as_f32().to_string());
                    }
                }
                ButtonAction::Pause => {
                    next_ui_state.set(UiState::Pause);
                    next_game_state.set(GameState::Pause);
                }
                ButtonAction::NextWave => {
                    if game_wave.is_next_wave_allowed() == true {
                        game_wave.next_wave();
                    }
                }
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_ui_state.set(UiState::Pause);
        next_game_state.set(GameState::Pause);
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        if game_wave.is_next_wave_allowed() == true {
            game_wave.next_wave();
        }
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
        health_text_i18n.change_arg("health", player.get_health().get_current().to_string());
    }
    for mut money_text_i18n in money_text.iter_mut() {
        money_text_i18n.change_arg("money", player.get_money().get_current().to_string());
    }
}

fn update_ui_after_wave_change(
    game_wave: Res<GameWave>,
    mut next_wave_button: Query<(&mut UiButton, &ButtonAction)>,
    mut wave_text: Query<&mut I18nComponent, With<WaveTextComponent>>,
) {
    for mut wave_text_i18n in wave_text.iter_mut() {
        wave_text_i18n.change_arg(
            "current",
            game_wave.get_current().saturating_add(1).to_string(),
        );
        wave_text_i18n.change_arg("total", game_wave.get_total().to_string())
    }
    for (mut ui_button, button_action) in next_wave_button.iter_mut() {
        ui_button.set_disabled(
            *button_action == ButtonAction::NextWave && game_wave.is_next_wave_allowed() == false,
        );
    }
}
