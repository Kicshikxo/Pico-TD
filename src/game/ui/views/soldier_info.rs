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
            button::UiButton,
            container::UiContainer,
            text::{UiText, UiTextSize},
        },
        i18n::I18nComponent,
        UiState,
    },
    GameState,
};

pub struct SoldierInfoViewUiPlugin;

impl Plugin for SoldierInfoViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::SoldierInfo), init_ui)
            .add_systems(OnExit(UiState::SoldierInfo), destroy_ui)
            .add_systems(
                Update,
                (update_ui, update_soldier_info).run_if(in_state(UiState::SoldierInfo)),
            );
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum SoldierInfoComponent {
    Level,
    Damage,
    FireRadius,
    BlastRadius,
    FireRate,
}

#[derive(Component, PartialEq)]
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
                                            .with_i18n_arg(
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
                                    parent.spawn(
                                        UiText::new("ui.in_game.money")
                                            .with_justify(JustifyText::Left)
                                            .with_i18n_arg(
                                                "money",
                                                player.get_money().get_current().to_string(),
                                            ),
                                    );
                                });
                        });
                });

            parent
                .spawn(
                    UiContainer::primary()
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
                        .spawn(UiContainer::secondary().with_padding(UiRect::all(Val::Px(8.0))))
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
                                        UiContainer::secondary()
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
                                                    UiText::new(soldier.to_str())
                                                        .with_width(Val::Auto)
                                                        .with_size(UiTextSize::Small)
                                                        .with_justify(JustifyText::Left),
                                                );
                                            });
                                        parent.spawn((
                                            SoldierInfoComponent::Level,
                                            UiText::new("ui.soldier_info.soldier_level")
                                                .with_i18n_arg(
                                                    "level",
                                                    soldier
                                                        .get_variant()
                                                        .get_level()
                                                        .saturating_add(1)
                                                        .to_string(),
                                                )
                                                .with_i18n_arg(
                                                    "max_level",
                                                    soldier
                                                        .get_variant()
                                                        .get_max_level()
                                                        .saturating_add(1)
                                                        .to_string(),
                                                )
                                                .with_size(UiTextSize::Small)
                                                .with_justify(JustifyText::Left),
                                        ));
                                        parent.spawn((
                                            SoldierInfoComponent::Damage,
                                            UiText::new("ui.soldier_info.soldier_damage")
                                                .with_i18n_arg(
                                                    "damage",
                                                    soldier.get_damage().to_string(),
                                                )
                                                .with_size(UiTextSize::Small)
                                                .with_justify(JustifyText::Left),
                                        ));
                                        parent.spawn((
                                            SoldierInfoComponent::FireRadius,
                                            UiText::new("ui.soldier_info.soldier_fire_radius")
                                                .with_i18n_arg(
                                                    "fire_radius",
                                                    soldier.get_fire_radius().to_string(),
                                                )
                                                .with_size(UiTextSize::Small)
                                                .with_justify(JustifyText::Left),
                                        ));
                                        if let Some(blast_radius) = soldier
                                            .get_config()
                                            .get_projectile_variant()
                                            .get_blast_radius()
                                        {
                                            parent.spawn((
                                                SoldierInfoComponent::BlastRadius,
                                                UiText::new("ui.soldier_info.soldier_blast_radius")
                                                    .with_i18n_arg(
                                                        "blast_radius",
                                                        blast_radius.to_string(),
                                                    )
                                                    .with_size(UiTextSize::Small)
                                                    .with_justify(JustifyText::Left),
                                            ));
                                        }
                                        parent.spawn((
                                            SoldierInfoComponent::FireRate,
                                            UiText::new("ui.soldier_info.soldier_fire_rate")
                                                .with_i18n_arg(
                                                    "fire_rate",
                                                    ((1.0 / soldier.get_fire_rate().as_secs_f32()
                                                        * 100.0)
                                                        .round()
                                                        / 100.0)
                                                        .to_string(),
                                                )
                                                .with_size(UiTextSize::Small)
                                                .with_justify(JustifyText::Left),
                                        ));
                                    });
                            });

                        parent
                            .spawn(UiContainer::new().with_row_gap(Val::Px(8.0)).column())
                            .with_children(|parent| {
                                if soldier.is_next_level_allowed() == true {
                                    parent
                                        .spawn((
                                            ButtonAction::UpgradeSoldier,
                                            UiButton::success()
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
                                                .with_i18n_arg(
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
                                        UiButton::danger().with_padding(UiRect::all(Val::Px(8.0))),
                                    ))
                                    .with_child(
                                        UiText::new("ui.soldier_info.sell_soldier").with_i18n_arg(
                                            "sell_price",
                                            soldier.get_config().get_sell_price().to_string(),
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
    for (interaction, button_action) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button_action {
            ButtonAction::Close => {
                next_ui_state.set(UiState::InGame);
                next_game_state.set(GameState::InGame);
            }
            ButtonAction::UpgradeSoldier => {
                for (_soldier_entity, mut soldier, soldier_tile_position) in soldiers.iter_mut() {
                    if soldier_tile_position.as_vec2() != selected_soldier.tile_position.as_vec2() {
                        continue;
                    }

                    let next_level_price = soldier.get_next_level_config().get_price();

                    if soldier.is_next_level_allowed() == false
                        || player.get_money().get_current() < next_level_price
                    {
                        break;
                    }

                    soldier.get_variant_mut().set_next_level();
                    player.get_money_mut().decrease(next_level_price);

                    selected_tile
                        .tile_position
                        .set_from_vec2(soldier_tile_position.as_vec2());

                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);

                    break;
                }
            }
            ButtonAction::SellSoldier => {
                for (soldier_entity, soldier, soldier_tile_position) in soldiers.iter_mut() {
                    if soldier_tile_position.as_vec2() != selected_soldier.tile_position.as_vec2() {
                        continue;
                    }

                    commands.entity(soldier_entity).despawn_recursive();
                    player
                        .get_money_mut()
                        .increase(soldier.get_config().get_sell_price());
                    break;
                }
                next_ui_state.set(UiState::InGame);
                next_game_state.set(GameState::InGame);
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_ui_state.set(UiState::InGame);
        next_game_state.set(GameState::InGame);
    }
}

fn update_soldier_info(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<UiButton>)>,
    mut soldier_info_components: Query<(&mut TextColor, &mut I18nComponent, &SoldierInfoComponent)>,
    soldiers: Query<(&Soldier, &TilePosition)>,
    player: Res<Player>,
    selected_soldier: Res<SelectedSoldier>,
) {
    for (interaction, button_action) in interaction_query.iter() {
        if *button_action != ButtonAction::UpgradeSoldier {
            continue;
        }

        for (soldier, soldier_tile_position) in soldiers.iter() {
            if soldier_tile_position.as_vec2() != selected_soldier.tile_position.as_vec2() {
                continue;
            }

            let current_config = soldier.get_config();
            let next_level_config = soldier.get_next_level_config();

            if soldier.is_next_level_allowed() == false
                || player.get_money().get_current() < next_level_config.get_price()
            {
                break;
            }

            let show_next_level =
                matches!(interaction, &Interaction::Hovered | &Interaction::Pressed);

            let dispayed_config = if show_next_level {
                next_level_config
            } else {
                current_config
            };

            for (
                mut soldier_info_component_text_color,
                mut soldier_info_i18n_component,
                soldier_info_component,
            ) in soldier_info_components.iter_mut()
            {
                let (key, value, changed) = match soldier_info_component {
                    SoldierInfoComponent::Level => (
                        "level",
                        if show_next_level {
                            soldier.get_next_level().saturating_add(1).to_string()
                        } else {
                            soldier.get_level().saturating_add(1).to_string()
                        },
                        true,
                    ),
                    SoldierInfoComponent::Damage => (
                        "damage",
                        dispayed_config.get_damage().to_string(),
                        current_config.get_damage() != next_level_config.get_damage(),
                    ),
                    SoldierInfoComponent::FireRadius => (
                        "fire_radius",
                        dispayed_config.get_fire_radius().to_string(),
                        current_config.get_fire_radius() != next_level_config.get_fire_radius(),
                    ),
                    SoldierInfoComponent::BlastRadius => {
                        if let Some(blast_radius) =
                            dispayed_config.get_projectile_variant().get_blast_radius()
                        {
                            (
                                "blast_radius",
                                blast_radius.to_string(),
                                current_config.get_projectile_variant().get_blast_radius()
                                    != next_level_config
                                        .get_projectile_variant()
                                        .get_blast_radius(),
                            )
                        } else {
                            continue;
                        }
                    }
                    SoldierInfoComponent::FireRate => (
                        "fire_rate",
                        ((1.0 / dispayed_config.get_fire_rate().as_secs_f32() * 100.0).round()
                            / 100.0)
                            .to_string(),
                        current_config.get_fire_rate() != next_level_config.get_fire_rate(),
                    ),
                };

                soldier_info_i18n_component.change_i18n_arg(key, value);
                soldier_info_component_text_color.0 = if show_next_level && changed {
                    Color::srgb(0.5, 1.0, 0.5)
                } else {
                    Color::WHITE
                };
            }

            break;
        }
    }
}
