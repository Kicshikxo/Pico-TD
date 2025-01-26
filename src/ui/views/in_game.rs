use bevy::prelude::*;

use crate::{
    game::{GameSpeed, GameState},
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
            container::UiContainer,
            text::{UiText, UiTextSize},
        },
        i18n::I18nComponent,
        UiState,
    },
    waves::Wave,
};

pub struct InGameViewUiPlugin;

impl Plugin for InGameViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::InGame), ui_init)
            .add_systems(OnExit(UiState::InGame), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::InGame)))
            .add_systems(
                Update,
                ui_update_after_wave_change.run_if(resource_changed::<Wave>),
            );
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
struct WaveInfoTextComponent;

#[derive(Component)]
struct CurrentSpeedTextComponent;

#[derive(Component, PartialEq)]
enum InGameButtonAction {
    ChangeSpeed,
    Pause,
    NextWave,
}

fn ui_init(mut commands: Commands, wave: Res<Wave>, game_speed: Res<GameSpeed>) {
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
                    right: Val::Px(8.0),
                    ..default()
                })
                .with_child((
                    WaveInfoTextComponent,
                    UiText::new("ui.in_game.wave_info")
                        .with_arg("current", (wave.get_current() + 1).to_string())
                        .with_arg("total", wave.get_total().to_string()),
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
                                .with_disabled(wave.is_next_wave_allowed() == false)
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
    mut wave: ResMut<Wave>,
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
                    if wave.is_next_wave_allowed() == true {
                        wave.next_wave();
                    }
                }
            }
        }
    }
}

fn ui_update_after_wave_change(
    wave: Res<Wave>,
    mut next_wave_button: Query<(&mut UiButton, &InGameButtonAction)>,
    mut wave_info_text: Query<&mut I18nComponent, With<WaveInfoTextComponent>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for mut wave_info_text_i18n in wave_info_text.iter_mut() {
        wave_info_text_i18n.change_arg("current", (wave.get_current() + 1).to_string());
        wave_info_text_i18n.change_arg("total", wave.get_total().to_string())
    }
    for (mut ui_button, button_action) in next_wave_button.iter_mut() {
        ui_button.set_disabled(
            *button_action == InGameButtonAction::NextWave && wave.is_next_wave_allowed() == false,
        );
    }
    if wave.is_fully_completed() == true {
        next_ui_state.set(UiState::GameOver);
        next_game_state.set(GameState::Pause);
    }
}
