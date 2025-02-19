use bevy::prelude::*;

use crate::{
    assets::sprites::{
        entity::EntityAssets,
        ui::{UiAssets, UiButtonSpriteVariant, UiMiscSpriteVariant},
    },
    entities::{
        soldier::Soldier,
        tile::{position::TilePosition, sprite::TileSprite},
    },
    game::GameState,
    input::SelectedSoldier,
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

pub struct SoldierInfoViewUiPlugin;

impl Plugin for SoldierInfoViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::SoldierInfo), ui_init)
            .add_systems(OnExit(UiState::SoldierInfo), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::SoldierInfo)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
struct MoneyTextComponent;

#[derive(Component)]
enum ButtonAction {
    Close,
    UpgradeSoldier,
    ChangeSoldier,
    SellSoldier,
}

fn ui_init(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    entity_assets: Res<EntityAssets>,
    player: Res<Player>,
    soldiers: Query<(&Soldier, &TilePosition)>,
    selected_soldier: Res<SelectedSoldier>,
) {
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
                                    parent.spawn(
                                        UiText::new("ui.in_game.health")
                                            .with_justify(JustifyText::Left)
                                            .with_arg(
                                                "health",
                                                player.get_health().get_current().to_string(),
                                            ),
                                    );
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
                        .with_variant(UiContainerVariant::Primary)
                        .with_width(Val::Px(320.0))
                        .with_padding(UiRect::all(Val::Px(24.0)))
                        .with_row_gap(Val::Px(12.0))
                        .center()
                        .column(),
                )
                .with_children(|parent| {
                    parent.spawn((
                        ButtonAction::Close,
                        UiButton::new(),
                        UiContainer::new()
                            .with_width(Val::Px(32.0))
                            .with_right(Val::Px(38.0))
                            .with_top(Val::Px(-6.0))
                            .absolute(),
                        ImageNode {
                            image: ui_assets.ui_buttons.clone(),
                            texture_atlas: Some(TextureAtlas {
                                index: UiButtonSpriteVariant::Close as usize,
                                layout: ui_assets.ui_buttons_layout.clone(),
                            }),
                            ..default()
                        },
                    ));
                    parent
                        .spawn(
                            UiContainer::new()
                                .with_variant(UiContainerVariant::Secondary)
                                .with_padding(UiRect::all(Val::Px(8.0))),
                        )
                        .with_child(
                            UiText::new("ui.soldier_info.title").with_size(UiTextSize::Large),
                        );

                    let mut current_soldier: Option<&Soldier> = None;
                    for (soldier, tile_position) in soldiers.iter() {
                        if tile_position.as_vec2() == selected_soldier.tile_position.as_vec2() {
                            current_soldier = Some(soldier);
                            break;
                        }
                    }

                    if let Some(soldier) = current_soldier {
                        parent
                            .spawn(UiContainer::new().with_column_gap(Val::Px(8.0)))
                            .with_children(|parent| {
                                parent
                                    .spawn(
                                        UiContainer::new()
                                            .with_variant(UiContainerVariant::Secondary)
                                            .with_width(Val::Px(64.0))
                                            .with_height(Val::Px(64.0))
                                            .center(),
                                    )
                                    .with_child((
                                        UiContainer::new()
                                            .with_width(Val::Px(32.0))
                                            .with_height(Val::Px(32.0)),
                                        ImageNode {
                                            image: entity_assets.tilemap.clone(),
                                            texture_atlas: Some(TextureAtlas {
                                                index: TileSprite::new(
                                                    soldier.get_variant().into(),
                                                )
                                                .get_variant()
                                                .as_index(),
                                                layout: entity_assets.tilemap_layout.clone(),
                                            }),
                                            ..default()
                                        },
                                    ));

                                parent
                                    .spawn(UiContainer::new().with_width(Val::Auto).column())
                                    .with_children(|parent| {
                                        parent
                                            .spawn(UiContainer::new().with_column_gap(Val::Px(8.0)))
                                            .with_children(|parent| {
                                                parent.spawn(
                                                    UiText::new("ui.soldier_info.soldier_name")
                                                        .with_width(Val::Auto)
                                                        .with_size(UiTextSize::Small)
                                                        .with_justify(JustifyText::Left),
                                                );
                                                parent.spawn(
                                                    UiText::new(&soldier.get_variant().to_string())
                                                        .with_width(Val::Auto)
                                                        .with_size(UiTextSize::Small)
                                                        .with_justify(JustifyText::Left),
                                                );
                                            });
                                        parent.spawn(
                                            UiText::new("ui.soldier_info.soldier_damage")
                                                .with_arg(
                                                    "damage",
                                                    soldier.get_damage().to_string(),
                                                )
                                                .with_size(UiTextSize::Small)
                                                .with_justify(JustifyText::Left),
                                        );
                                        parent.spawn(
                                            UiText::new("ui.soldier_info.soldier_fire_radius")
                                                .with_arg(
                                                    "fire_radius",
                                                    soldier.get_fire_radius().to_string(),
                                                )
                                                .with_size(UiTextSize::Small)
                                                .with_justify(JustifyText::Left),
                                        );
                                        parent.spawn(
                                            UiText::new("ui.soldier_info.soldier_fire_rate")
                                                .with_arg(
                                                    "fire_rate",
                                                    ((1.0 / soldier.get_fire_rate().as_secs_f32()
                                                        * 100.0)
                                                        .round()
                                                        / 100.0)
                                                        .to_string(),
                                                )
                                                .with_size(UiTextSize::Small)
                                                .with_justify(JustifyText::Left),
                                        );
                                    });
                            });

                        parent
                            .spawn(UiContainer::new().with_row_gap(Val::Px(8.0)).column())
                            .with_children(|parent| {
                                parent
                                    .spawn((
                                        ButtonAction::UpgradeSoldier,
                                        UiButton::new()
                                            .with_variant(UiButtonVariant::Success)
                                            .with_padding(UiRect::all(Val::Px(8.0))),
                                    ))
                                    .with_child(UiText::new("ui.soldier_info.upgrade_soldier"));

                                parent
                                    .spawn((
                                        ButtonAction::ChangeSoldier,
                                        UiButton::new()
                                            .with_variant(UiButtonVariant::Primary)
                                            .with_padding(UiRect::all(Val::Px(8.0))),
                                    ))
                                    .with_child(UiText::new("ui.soldier_info.change_soldier"));

                                parent
                                    .spawn((
                                        ButtonAction::SellSoldier,
                                        UiButton::new()
                                            .with_variant(UiButtonVariant::Danger)
                                            .with_padding(UiRect::all(Val::Px(8.0))),
                                    ))
                                    .with_child(
                                        UiText::new("ui.soldier_info.sell_soldier").with_arg(
                                            "sell_price",
                                            soldier
                                                .get_variant()
                                                .get_config()
                                                .get_sell_price()
                                                .to_string(),
                                        ),
                                    );
                            });
                    }
                });
        });
}

fn ui_destroy(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn ui_update(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<UiButton>)>,
    mut soldiers: Query<(Entity, &Soldier, &TilePosition)>,
    mut player: ResMut<Player>,
    selected_soldier: Res<SelectedSoldier>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::Close => {
                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);
                }
                ButtonAction::UpgradeSoldier => {
                    info!("Upgrade soldier");
                }
                ButtonAction::ChangeSoldier => {
                    next_ui_state.set(UiState::SoldierSelect);
                    next_game_state.set(GameState::Pause);
                }
                ButtonAction::SellSoldier => {
                    for (soldier_entity, soldier, soldier_tile_position) in soldiers.iter_mut() {
                        if soldier_tile_position.as_vec2()
                            == selected_soldier.tile_position.as_vec2()
                        {
                            commands.entity(soldier_entity).despawn_recursive();
                            player
                                .get_money_mut()
                                .increase(soldier.get_variant().get_config().get_sell_price());
                            break;
                        }
                    }
                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);
                }
            }
        }
    }
}
