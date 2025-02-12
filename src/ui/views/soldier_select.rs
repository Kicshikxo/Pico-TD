use bevy::prelude::*;

use crate::{
    assets::sprites::{
        tile::TileAssets,
        ui::{UiAssets, UiButtonSpriteVariant},
    },
    entities::{
        soldier::{Soldier, SoldierVariant},
        tile::{position::TilePosition, sprite::TileSprite},
    },
    game::{GameState, GameTilemap},
    input::SelectedSoldier,
    player::Player,
    ui::{
        components::{
            button::UiButton,
            container::{UiContainer, UiContainerVariant},
            text::{UiText, UiTextSize},
        },
        UiState,
    },
};

pub struct SoldierSelectViewUiPlugin;

impl Plugin for SoldierSelectViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::SoldierSelect), ui_init)
            .add_systems(OnExit(UiState::SoldierSelect), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::SoldierSelect)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum SoldierSelectButtonAction {
    Close,
    Select(SoldierVariant),
}

fn ui_init(mut commands: Commands, ui_assets: Res<UiAssets>, tile_assets: Res<TileAssets>) {
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
                    parent.spawn((
                        UiButton::new(),
                        SoldierSelectButtonAction::Close,
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(32.0),
                            top: Val::Px(-6.0),
                            right: Val::Px(38.0),
                            ..default()
                        },
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
                        .with_child(UiText::new("ui.soldier_select.title"));

                    parent
                        .spawn(Node {
                            display: Display::Grid,
                            width: Val::Percent(100.0),
                            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                            row_gap: Val::Px(8.0),
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            for variant in [
                                SoldierVariant::Soldier,
                                SoldierVariant::SoldierFast,
                                SoldierVariant::SoldierStrong,
                                SoldierVariant::SoldierSniper,
                                SoldierVariant::RocketLauncher,
                            ] {
                                parent
                                    .spawn(
                                        UiContainer::new()
                                            .with_row_gap(Val::Px(4.0))
                                            .with_align_items(AlignItems::Start)
                                            .column(),
                                    )
                                    .with_children(|parent| {
                                        parent
                                            .spawn((
                                                SoldierSelectButtonAction::Select(variant),
                                                UiButton::new(),
                                                UiContainer::new()
                                                    .with_variant(UiContainerVariant::Secondary)
                                                    .with_aspect_ratio(1.0)
                                                    .center(),
                                            ))
                                            .with_child((
                                                UiContainer::new()
                                                    .with_width(Val::Px(32.0))
                                                    .with_height(Val::Px(32.0)),
                                                ImageNode {
                                                    image: tile_assets.entities.clone(),
                                                    texture_atlas: Some(TextureAtlas {
                                                        index: TileSprite::new(variant.into())
                                                            .get_variant()
                                                            .as_index(),
                                                        layout: tile_assets.entities_layout.clone(),
                                                    }),
                                                    ..default()
                                                },
                                            ));

                                        parent.spawn(
                                            UiText::new("ui.soldier_select.price")
                                                .with_arg(
                                                    "price",
                                                    variant.get_config().get_price().to_string(),
                                                )
                                                .with_size(UiTextSize::Small),
                                        );
                                        parent.spawn(
                                            UiText::new(&variant.to_string())
                                                .with_size(UiTextSize::Small),
                                        );
                                    });
                            }
                        });
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
    interaction_query: Query<
        (&Interaction, &SoldierSelectButtonAction),
        (Changed<Interaction>, With<UiButton>),
    >,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    mut soldiers: Query<(&mut Soldier, &TilePosition)>,
    mut player: ResMut<Player>,
    selected_soldier: Res<SelectedSoldier>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                SoldierSelectButtonAction::Close => {
                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);
                }
                SoldierSelectButtonAction::Select(variant) => {
                    let mut current_soldier: Option<&mut Soldier> = None;
                    for (soldier, tile_position) in soldiers.iter_mut() {
                        if tile_position.as_vec2() == selected_soldier.tile_position.as_vec2() {
                            current_soldier = Some(soldier.into_inner());
                            break;
                        }
                    }
                    if player.get_money().get_current()
                        + if let Some(current_soldier) = &current_soldier {
                            current_soldier.get_variant().get_config().get_price()
                        } else {
                            0
                        }
                        < variant.get_config().get_price()
                    {
                        continue;
                    }

                    if let Some(current_soldier) = current_soldier {
                        player
                            .get_money_mut()
                            .increase(current_soldier.get_variant().get_config().get_price());
                        current_soldier.set_variant(variant.clone());
                    } else {
                        commands.entity(game_tilemap.single()).with_child((
                            Soldier::new(variant.clone()),
                            selected_soldier.tile_position.clone(),
                        ));
                    }
                    player
                        .get_money_mut()
                        .decrease(variant.get_config().get_price());

                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);
                }
            }
        }
    }
}
