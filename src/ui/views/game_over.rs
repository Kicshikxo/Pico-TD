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
};

pub struct GameOverViewUiPlugin;

impl Plugin for GameOverViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::GameOver), ui_init)
            .add_systems(OnExit(UiState::GameOver), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::GameOver)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component, PartialEq)]
enum GameOverButtonAction {
    BackToMenu,
}

fn ui_init(mut commands: Commands) {
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
                    parent
                        .spawn(
                            UiContainer::new()
                                .with_variant(UiContainerVariant::Secondary)
                                .with_padding(UiRect::all(Val::Px(8.0))),
                        )
                        .with_child(
                            UiText::new("ui.in_game.game_over").with_size(UiTextSize::Large),
                        );
                    parent
                        .spawn((
                            GameOverButtonAction::BackToMenu,
                            UiButton::new().with_variant(UiButtonVariant::Primary),
                        ))
                        .with_child(UiText::new("ui.in_game.back_to_menu"));
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
        (&Interaction, &GameOverButtonAction),
        (Changed<Interaction>, With<UiButton>),
    >,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                GameOverButtonAction::BackToMenu => {
                    next_ui_state.set(UiState::Menu);
                    next_game_state.set(GameState::Pause);
                }
            }
        }
    }
}
