use bevy::prelude::*;

use crate::game::{
    assets::sprites::{
        entity::EntityAssets,
        ui::{UiAssets, UiButtonSpriteVariant, UiMiscSpriteVariant},
    },
    entities::{
        soldier::Soldier,
        tile::{position::TilePosition, sprite::TileSprite},
    },
    input::{SelectedSoldier, SelectedTile},
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

pub struct SoldierInfoViewUiPlugin;

impl Plugin for SoldierInfoViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::SoldierInfo), init_ui)
            .add_systems(OnExit(UiState::SoldierInfo), destroy_ui)
            .add_systems(Update, update_ui.run_if(in_state(UiState::SoldierInfo)));
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
    SellSoldier,
}

fn init_ui(
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
                                            UiText::new("ui.soldier_info.soldier_level")
                                                .with_arg(
                                                    "level",
                                                    soldier
                                                        .get_variant()
                                                        .get_level()
                                                        .saturating_add(1)
                                                        .to_string(),
                                                )
                                                .with_arg(
                                                    "max_level",
                                                    soldier
                                                        .get_variant()
                                                        .get_max_level()
                                                        .saturating_add(1)
                                                        .to_string(),
                                                )
                                                .with_size(UiTextSize::Small)
                                                .with_justify(JustifyText::Left),
                                        );
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
                                if soldier.get_variant().is_next_level_allowed() == true {
                                    parent
                                        .spawn((
                                            ButtonAction::UpgradeSoldier,
                                            UiButton::new()
                                                .with_variant(UiButtonVariant::Success)
                                                .with_disabled(
                                                    player.get_money().get_current()
                                                        < soldier
                                                            .get_variant()
                                                            .get_next_level_config()
                                                            .get_price(),
                                                )
                                                .with_padding(UiRect::all(Val::Px(8.0))),
                                        ))
                                        .with_child(
                                            UiText::new("ui.soldier_info.upgrade_soldier")
                                                .with_arg(
                                                    "price",
                                                    soldier
                                                        .get_variant()
                                                        .get_next_level_config()
                                                        .get_price()
                                                        .to_string(),
                                                ),
                                        );
                                }

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

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_ui(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<UiButton>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut soldiers: Query<(Entity, &mut Soldier, &TilePosition)>,
    mut player: ResMut<Player>,
    selected_soldier: Res<SelectedSoldier>,
    mut selected_tile: ResMut<SelectedTile>,
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
                    for (_soldier_entity, mut soldier, soldier_tile_position) in soldiers.iter_mut()
                    {
                        if soldier_tile_position.as_vec2()
                            == selected_soldier.tile_position.as_vec2()
                        {
                            let next_level_price =
                                soldier.get_variant().get_next_level_config().get_price();

                            if player.get_money().get_current() < next_level_price {
                                break;
                            }

                            soldier.get_variant_mut().next_level();
                            player.get_money_mut().decrease(next_level_price);

                            selected_tile
                                .tile_position
                                .set_from_vec2(soldier_tile_position.as_vec2());

                            next_ui_state.set(UiState::InGame);
                            next_game_state.set(GameState::InGame);
                            break;
                        }
                    }
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
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_ui_state.set(UiState::InGame);
        next_game_state.set(GameState::InGame);
    }
}
