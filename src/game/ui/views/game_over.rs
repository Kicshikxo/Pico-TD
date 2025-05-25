use bevy::{ecs::spawn::SpawnIter, prelude::*};

use crate::game::{
    GameState,
    assets::{
        images::ui::{UiAssets, UiMiscSpriteVariant},
        levels::LevelCompletionStars,
    },
    player::Player,
    ui::{
        UiState,
        components::{
            button::{UiButton, UiButtonInteraction},
            container::{UiContainer, UiContainerVariant},
            icon::{UiIcon, UiIconVariant},
            text::{UiText, UiTextSize},
        },
    },
    waves::GameWaves,
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
    RetryLevel,
    BackToMenu,
}

fn init_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    player: Res<Player>,
    game_waves: Res<GameWaves>,
) {
    commands.spawn((
        RootUiComponent,
        UiContainer::new().full().center(),
        BackgroundColor(Color::BLACK.with_alpha(0.5)),
        children![(
            UiContainer::new()
                .with_variant(if player.get_health().is_alive() {
                    UiContainerVariant::Success
                } else {
                    UiContainerVariant::Danger
                })
                .with_min_width(Val::Px(320.0))
                .with_padding(UiRect::all(Val::Px(24.0)).with_top(Val::Px(40.0)))
                .with_row_gap(Val::Px(12.0))
                .auto_width()
                .center()
                .column(),
            children![
                (
                    (
                        UiContainer::new()
                            .with_top(Val::Px(-12.0))
                            .absolute()
                            .center(),
                        ZIndex(1)
                    ),
                    Children::spawn(SpawnIter(
                        (1..=3)
                            .map(|star_index| {
                                (
                                    UiContainer::new()
                                        .with_bottom(if star_index == 2 {
                                            Val::Px(12.0)
                                        } else {
                                            Val::Px(0.0)
                                        })
                                        .with_width(Val::Px(48.0))
                                        .with_height(Val::Px(48.0)),
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
                                )
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                    )),
                ),
                (
                    UiContainer::secondary()
                        .with_padding(UiRect::all(Val::Px(12.0)))
                        .column()
                        .center(),
                    children![
                        UiText::new(if player.get_health().is_alive() {
                            "ui.game_over.player_win"
                        } else {
                            "ui.game_over.player_lose"
                        })
                        .with_size(UiTextSize::Large),
                        if player.get_health().is_dead() {
                            UiText::new("ui.game_over.waves_survived")
                                .with_i18n_arg("current_wave", game_waves.get_current().to_string())
                                .with_i18n_arg(
                                    "total_waves",
                                    game_waves.get_total().saturating_add(1).to_string(),
                                )
                        } else {
                            UiText::default()
                        }
                    ],
                ),
                (
                    (ButtonAction::RetryLevel, UiButton::success()),
                    children![
                        UiIcon::new(UiIconVariant::Restart),
                        UiText::new("ui.game_over.retry_level").auto_width()
                    ],
                ),
                (
                    (ButtonAction::BackToMenu, UiButton::primary()),
                    children![
                        UiIcon::new(UiIconVariant::Home),
                        UiText::new("ui.game_over.back_to_menu").auto_width()
                    ],
                ),
            ],
        ),],
    ));
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_ui(
    interaction_query: Query<
        (&UiButtonInteraction, &ButtonAction),
        (Changed<UiButtonInteraction>, With<UiButton>),
    >,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (ui_button_interaction, button_action) in interaction_query.iter() {
        if *ui_button_interaction != UiButtonInteraction::Clicked {
            continue;
        }
        match button_action {
            ButtonAction::RetryLevel => {
                next_game_state.set(GameState::Start);
            }
            ButtonAction::BackToMenu => {
                next_ui_state.set(UiState::Menu);
                next_game_state.set(GameState::Pause);
            }
        }
    }
}
