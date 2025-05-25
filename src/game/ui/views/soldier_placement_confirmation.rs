use bevy::prelude::*;

use crate::game::{
    GameState,
    assets::images::ui::{UiAssets, UiMiscSpriteVariant},
    entities::{soldier::Soldier, tile::position::TilePosition},
    input::SelectedSoldier,
    player::Player,
    ui::{
        UiState,
        components::{
            button::{UiButton, UiButtonInteraction},
            container::UiContainer,
            text::{UiText, UiTextSize},
        },
    },
    waves::GameWaves,
};

pub struct SoldierPlacementConfirmationViewUiPlugin;

impl Plugin for SoldierPlacementConfirmationViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::SoldierPlacementConfirmation), init_ui)
            .add_systems(OnExit(UiState::SoldierPlacementConfirmation), destroy_ui)
            .add_systems(
                Update,
                update_ui.run_if(in_state(UiState::SoldierPlacementConfirmation)),
            );
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component, PartialEq)]
enum ButtonAction {
    Confirm,
    Cancel,
}

fn init_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    player: Res<Player>,
    game_waves: Res<GameWaves>,
) {
    commands.spawn((
        RootUiComponent,
        UiContainer::new().full(),
        children![
            (
                UiContainer::new()
                    .with_left(Val::Px(8.0))
                    .with_top(Val::Px(8.0))
                    .absolute(),
                children![(
                    UiContainer::new().column(),
                    children![
                        (
                            UiContainer::new().with_column_gap(Val::Px(8.0)).center(),
                            children![
                                (
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
                                    }
                                ),
                                UiText::new("ui.in_game.health")
                                    .with_justify(JustifyText::Left)
                                    .with_i18n_arg(
                                        "health",
                                        player.get_health().get_current().to_string(),
                                    ),
                            ],
                        ),
                        (
                            UiContainer::new().with_column_gap(Val::Px(8.0)).center(),
                            children![
                                (
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
                                    }
                                ),
                                UiText::new("ui.in_game.money")
                                    .with_justify(JustifyText::Left)
                                    .with_i18n_arg(
                                        "money",
                                        player.get_money().get_current().to_string(),
                                    ),
                            ],
                        ),
                    ],
                )],
            ),
            (
                UiContainer::new()
                    .with_right(Val::Px(8.0))
                    .with_top(Val::Px(8.0))
                    .auto_width()
                    .absolute(),
                children![
                    UiText::new("ui.in_game.wave")
                        .with_i18n_arg(
                            "current",
                            game_waves.get_current().saturating_add(1).to_string(),
                        )
                        .with_i18n_arg(
                            "total",
                            game_waves.get_total().saturating_add(1).to_string(),
                        ),
                ],
            ),
            (
                UiContainer::new()
                    .with_right(Val::Px(8.0))
                    .with_bottom(Val::Px(8.0))
                    .with_row_gap(Val::Px(8.0))
                    .grid()
                    .absolute()
                    .auto_width(),
                children![
                    (
                        (
                            ButtonAction::Confirm,
                            UiButton::success()
                                .with_height(Val::Px(32.0))
                                .with_padding(UiRect::horizontal(Val::Px(16.0)))
                        ),
                        children![
                            UiText::new("ui.soldier_placement_confirmation.confirm")
                                .with_size(UiTextSize::Small)
                        ],
                    ),
                    (
                        (
                            ButtonAction::Cancel,
                            UiButton::danger()
                                .with_height(Val::Px(32.0))
                                .with_padding(UiRect::horizontal(Val::Px(16.0)))
                        ),
                        children![
                            UiText::new("ui.soldier_placement_confirmation.cancel")
                                .with_size(UiTextSize::Small)
                        ],
                    ),
                ],
            ),
        ],
    ));
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_ui(
    mut commands: Commands,
    interaction_query: Query<
        (&UiButtonInteraction, &ButtonAction),
        (Changed<UiButtonInteraction>, With<UiButton>),
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut soldiers: Query<(Entity, &mut Soldier, &TilePosition)>,
    mut player: ResMut<Player>,
    selected_soldier: Res<SelectedSoldier>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut cancel_soldier_placement = false;

    for (ui_button_interaction, button_action) in interaction_query.iter() {
        if *ui_button_interaction != UiButtonInteraction::Clicked {
            continue;
        }
        match button_action {
            ButtonAction::Confirm => {
                next_ui_state.set(UiState::InGame);
                next_game_state.set(GameState::InGame);
            }
            ButtonAction::Cancel => {
                cancel_soldier_placement = true;
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Enter) {
        next_ui_state.set(UiState::InGame);
        next_game_state.set(GameState::InGame);
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        cancel_soldier_placement = true;
    }

    if cancel_soldier_placement == true {
        for (soldier_entity, soldier, soldier_tile_position) in soldiers.iter_mut() {
            if soldier_tile_position.as_vec2() != selected_soldier.tile_position.as_vec2() {
                continue;
            }

            commands.entity(soldier_entity).despawn();
            player
                .get_money_mut()
                .increase(soldier.get_config().get_price());

            break;
        }
        next_ui_state.set(UiState::InGame);
        next_game_state.set(GameState::InGame);
    }
}
