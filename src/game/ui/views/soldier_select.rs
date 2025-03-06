use bevy::prelude::*;

use crate::game::{
    assets::sprites::{
        entity::EntityAssets,
        ui::{UiAssets, UiButtonSpriteVariant, UiMiscSpriteVariant},
    },
    entities::{
        soldier::{Soldier, SoldierVariant},
        tile::sprite::TileSprite,
    },
    input::{SelectedSoldier, SelectedTile},
    player::Player,
    ui::{
        components::{
            button::UiButton,
            container::UiContainer,
            text::{UiText, UiTextSize},
        },
        UiState,
    },
    GameState, GameTilemap,
};

pub struct SoldierSelectViewUiPlugin;

impl Plugin for SoldierSelectViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::SoldierSelect), init_ui)
            .add_systems(OnExit(UiState::SoldierSelect), destroy_ui)
            .add_systems(Update, update_ui.run_if(in_state(UiState::SoldierSelect)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum ButtonAction {
    Close,
    Select(SoldierVariant),
}

fn init_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    entity_assets: Res<EntityAssets>,
    player: Res<Player>,
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
                        .with_child(UiText::new("ui.soldier_select.title"));

                    parent
                        .spawn(Node {
                            display: Display::Grid,
                            width: Val::Percent(100.0),
                            grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                            align_items: AlignItems::Start,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            for soldier_variant in [
                                SoldierVariant::Soldier { level: 0 },
                                SoldierVariant::RocketLauncher { level: 0 },
                                SoldierVariant::Sniper { level: 0 },
                            ] {
                                parent
                                    .spawn(
                                        UiContainer::new()
                                            .with_row_gap(Val::Px(4.0))
                                            .center()
                                            .column(),
                                    )
                                    .with_children(|parent| {
                                        parent
                                            .spawn((
                                                ButtonAction::Select(soldier_variant),
                                                UiButton::new().with_disabled(
                                                    soldier_variant.get_config().get_price()
                                                        > player.get_money().get_current(),
                                                ),
                                                UiContainer::secondary()
                                                    .with_width(Val::Auto)
                                                    .with_padding(UiRect::all(Val::Px(16.0)))
                                                    .with_aspect_ratio(1.0)
                                                    .center(),
                                            ))
                                            .with_child((
                                                UiContainer::new()
                                                    .with_width(Val::Px(32.0))
                                                    .with_height(Val::Px(32.0)),
                                                ImageNode {
                                                    image: entity_assets.tilemap.clone(),
                                                    texture_atlas: Some(TextureAtlas {
                                                        index: TileSprite::new(
                                                            soldier_variant.into(),
                                                        )
                                                        .get_variant()
                                                        .as_index(),
                                                        layout: entity_assets
                                                            .tilemap_layout
                                                            .clone(),
                                                    }),
                                                    ..default()
                                                },
                                            ));

                                        parent.spawn(
                                            UiText::new("ui.soldier_select.price")
                                                .with_i18n_arg(
                                                    "price",
                                                    soldier_variant
                                                        .get_config()
                                                        .get_price()
                                                        .to_string(),
                                                )
                                                .with_size(UiTextSize::Small)
                                                .with_color(
                                                    if soldier_variant.get_config().get_price()
                                                        > player.get_money().get_current()
                                                    {
                                                        Color::srgb(1.0, 0.25, 0.25)
                                                    } else {
                                                        Color::WHITE
                                                    },
                                                ),
                                        );

                                        parent.spawn(
                                            UiText::new(soldier_variant.to_str())
                                                .with_size(UiTextSize::Small)
                                                .with_linebreak(LineBreak::WordOrCharacter),
                                        );
                                    });
                            }
                        });
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
    game_tilemap: Query<Entity, With<GameTilemap>>,
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
            ButtonAction::Select(variant) => {
                if player.get_money().get_current() < variant.get_config().get_price() {
                    continue;
                }

                commands.entity(game_tilemap.single()).with_child((
                    Soldier::new(variant.clone()),
                    selected_soldier.tile_position.clone(),
                ));

                player
                    .get_money_mut()
                    .decrease(variant.get_config().get_price());

                selected_tile
                    .tile_position
                    .set_from_vec2(selected_soldier.tile_position.as_vec2());

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
