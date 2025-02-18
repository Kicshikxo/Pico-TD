use bevy::prelude::*;

use crate::{
    game::GameState,
    player::Player,
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
enum ButtonAction {
    BackToMenu,
}

fn ui_init(mut commands: Commands, player: Res<Player>) {
    commands
        .spawn((
            RootUiComponent,
            UiContainer::new().full().center(),
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
                        .with_child(UiText::new("ui.game_over.title").with_size(UiTextSize::Large));
                    parent
                        .spawn(
                            UiContainer::new()
                                .with_variant(if player.get_health().is_alive() {
                                    UiContainerVariant::Success
                                } else {
                                    UiContainerVariant::Danger
                                })
                                .with_padding(UiRect::all(Val::Px(8.0))),
                        )
                        .with_child(
                            UiText::new(if player.get_health().is_alive() {
                                "ui.game_over.player_win"
                            } else {
                                "ui.game_over.player_lose"
                            })
                            .with_size(UiTextSize::Large),
                        );
                    parent
                        .spawn((
                            ButtonAction::BackToMenu,
                            UiButton::new().with_variant(UiButtonVariant::Primary),
                        ))
                        .with_child(UiText::new("ui.game_over.back_to_menu"));
                });
        });
}

fn ui_destroy(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn ui_update(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<UiButton>)>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::BackToMenu => {
                    next_ui_state.set(UiState::Menu);
                    next_game_state.set(GameState::Pause);
                }
            }
        }
    }
}
