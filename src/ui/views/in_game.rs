use bevy::prelude::*;

use crate::{
    game::GameState,
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
            container::{UiContainer, UiContainerVariant},
            text::{UiText, UiTextSize},
        },
        UiState,
    },
    waves::{CurrentWave, WaveState},
};

pub struct InGameViewUiPlugin;

impl Plugin for InGameViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::InGame), ui_init)
            .add_systems(OnExit(UiState::InGame), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::InGame)))
            .add_systems(
                Update,
                ui_update_after_wave_change.run_if(resource_changed::<CurrentWave>),
            );
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
struct GameOverComponent;

#[derive(Component, PartialEq)]
enum InGameButtonAction {
    NextWave,
    Pause,
    BackToMenu,
}

fn ui_init(mut commands: Commands, current_wave: Res<CurrentWave>) {
    commands
        .spawn((
            RootUiComponent,
            UiContainer::new().with_height(Val::Percent(100.0)),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    GameOverComponent,
                    UiContainer::new()
                        .with_display(if current_wave.is_fully_completed() == true {
                            Display::Flex
                        } else {
                            Display::None
                        })
                        .with_height(Val::Percent(100.0))
                        .center(),
                    ZIndex(1),
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
                            parent
                                .spawn(
                                    UiContainer::new()
                                        .with_variant(UiContainerVariant::Secondary)
                                        .with_padding(UiRect::all(Val::Px(8.0))),
                                )
                                .with_child(
                                    UiText::new("ui.in_game.game_over")
                                        .with_size(UiTextSize::Large),
                                );
                            parent
                                .spawn((
                                    InGameButtonAction::BackToMenu,
                                    UiButton::new().with_variant(UiButtonVariant::Primary),
                                ))
                                .with_child(UiText::new("ui.in_game.back_to_menu"));
                        });
                });

            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(8.0),
                    right: Val::Px(8.0),
                    align_items: AlignItems::End,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            InGameButtonAction::Pause,
                            UiButton::new()
                                .with_variant(UiButtonVariant::Primary)
                                .with_padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0))),
                        ))
                        .with_child(UiText::new("ui.in_game.pause").with_size(UiTextSize::Small));

                    parent
                        .spawn((
                            InGameButtonAction::NextWave,
                            UiButton::new()
                                .with_variant(UiButtonVariant::Primary)
                                .with_disabled(
                                    current_wave.get_state() != WaveState::Completed
                                        || current_wave.is_last_wave() == true,
                                )
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
    mut current_wave: ResMut<CurrentWave>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                InGameButtonAction::NextWave => {
                    if current_wave.get_state() != WaveState::Completed {
                        return;
                    }
                    current_wave.next_wave();
                }
                InGameButtonAction::Pause => {
                    next_ui_state.set(UiState::Pause);
                    next_game_state.set(GameState::Pause);
                }
                InGameButtonAction::BackToMenu => {
                    next_ui_state.set(UiState::Menu);
                    next_game_state.set(GameState::Pause);
                }
            }
        }
    }
}

fn ui_update_after_wave_change(
    current_wave: Res<CurrentWave>,
    mut next_wave_button: Query<(&mut UiButton, &InGameButtonAction)>,
    mut game_over_component: Query<&mut Node, With<GameOverComponent>>,
) {
    for (mut ui_button, button_action) in next_wave_button.iter_mut() {
        ui_button.set_disabled(
            *button_action == InGameButtonAction::NextWave
                && (current_wave.get_state() != WaveState::Completed
                    || current_wave.is_last_wave() == true),
        );
    }
    if let Ok(mut game_over_node) = game_over_component.get_single_mut() {
        game_over_node.display = if current_wave.is_fully_completed() == true {
            Display::Flex
        } else {
            Display::None
        }
    }
}
