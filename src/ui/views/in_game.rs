use bevy::prelude::*;

use crate::{
    game::GameState,
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
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
            .add_systems(Update, ui_update.run_if(in_state(UiState::InGame)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum InGameButtonAction {
    NextWave,
    Pause,
}

fn ui_init(mut commands: Commands) {
    commands
        .spawn((
            RootUiComponent,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|parent| {
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
                            InGameButtonAction::NextWave,
                            UiButton::new()
                                .with_variant(UiButtonVariant::Primary)
                                .with_padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0))),
                        ))
                        .with_child(
                            UiText::new("ui.in_game.next_wave").with_size(UiTextSize::Small),
                        );

                    parent
                        .spawn((
                            InGameButtonAction::Pause,
                            UiButton::new()
                                .with_variant(UiButtonVariant::Primary)
                                .with_padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0))),
                        ))
                        .with_child(UiText::new("ui.in_game.pause").with_size(UiTextSize::Small));
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
            }
        }
    }
}
