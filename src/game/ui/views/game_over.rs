use bevy::prelude::*;

use crate::game::{
    assets::{
        levels::LevelCompletionStars,
        sprites::ui::{UiAssets, UiMiscSpriteVariant},
    },
    player::Player,
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
            container::{UiContainer, UiContainerVariant},
            text::{UiText, UiTextSize},
        },
        UiState,
    },
    GameState,
};

pub struct GameOverViewUiPlugin;

impl Plugin for GameOverViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::GameOver), init_ui)
            .add_systems(OnExit(UiState::GameOver), destroy_ui)
            .add_systems(Update, update_ui.run_if(in_state(UiState::GameOver)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component, PartialEq)]
enum ButtonAction {
    BackToMenu,
}

fn init_ui(mut commands: Commands, ui_assets: Res<UiAssets>, player: Res<Player>) {
    commands
        .spawn((
            RootUiComponent,
            UiContainer::new().full().center(),
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
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
                                .with_padding(UiRect::all(Val::Px(12.0)).with_bottom(Val::Px(20.0)))
                                .column()
                                .center(),
                        )
                        .with_children(|parent| {
                            parent.spawn(
                                UiText::new(if player.get_health().is_alive() {
                                    "ui.game_over.player_win"
                                } else {
                                    "ui.game_over.player_lose"
                                })
                                .with_size(UiTextSize::Large),
                            );

                            parent
                                .spawn(UiContainer::new().with_column_gap(Val::Px(4.0)).center())
                                .with_children(|parent| {
                                    for star_index in 1..=3 {
                                        parent.spawn((
                                            UiContainer::new()
                                                .with_width(Val::Px(32.0))
                                                .with_height(Val::Px(32.0)),
                                            ImageNode {
                                                color: if star_index
                                                    <= LevelCompletionStars::from_player_health(
                                                        player.get_health(),
                                                    )
                                                    .as_index()
                                                {
                                                    Color::srgb(1.0, 1.0, 0.0)
                                                } else {
                                                    Color::WHITE
                                                },
                                                image: ui_assets.ui_misc.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: ui_assets.ui_misc_layout.clone(),
                                                    index: UiMiscSpriteVariant::Star as usize,
                                                }),
                                                ..default()
                                            },
                                        ));
                                    }
                                });
                        });

                    parent
                        .spawn((
                            ButtonAction::BackToMenu,
                            UiButton::new().with_variant(UiButtonVariant::Primary),
                        ))
                        .with_child(UiText::new("ui.game_over.back_to_menu"));
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
