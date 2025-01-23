use bevy::prelude::*;

use crate::{
    assets::sprites::{tile::TileAssets, ui::UiAssets},
    entities::{
        structure::Structure,
        tile::{position::TilePosition, sprite::TileSprite},
    },
    game::GameState,
    input::SelectedStructure,
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
            container::{UiContainer, UiContainerVariant},
            text::{UiText, UiTextSize},
        },
        UiState,
    },
};

pub struct StructureInfoViewUiPlugin;

impl Plugin for StructureInfoViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::StructureInfo), ui_init)
            .add_systems(OnExit(UiState::StructureInfo), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::StructureInfo)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum StructureInfoButtonAction {
    Close,
    UpgradeStructure,
    SellStructure,
}

fn ui_init(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    tile_assets: Res<TileAssets>,
    structures: Query<(&Structure, &TilePosition)>,
    selected_structure: Option<Res<SelectedStructure>>,
) {
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
                        StructureInfoButtonAction::Close,
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(32.0),
                            top: Val::Px(-6.0),
                            right: Val::Px(38.0),
                            ..default()
                        },
                        ImageNode {
                            image: ui_assets.small_tilemap.clone(),
                            texture_atlas: Some(TextureAtlas {
                                index: 4,
                                layout: ui_assets.small_tilemap_atlas.clone(),
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
                        .with_child(UiText::new("ui.structure_info.title"));

                    if let Some(selected_structure) = selected_structure {
                        for (structure, tile_position) in structures.iter() {
                            if tile_position.as_vec2() == selected_structure.tile_position.as_vec2()
                            {
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
                                                    image: tile_assets.entities.clone(),
                                                    texture_atlas: Some(TextureAtlas {
                                                        index: TileSprite::new(
                                                            structure.get_variant().into(),
                                                        )
                                                        .get_variant()
                                                        .as_index(),
                                                        layout: tile_assets.entities_layout.clone(),
                                                    }),
                                                    ..default()
                                                },
                                            ));

                                        parent.spawn(UiContainer::new().column()).with_children(
                                            |parent| {
                                                parent
                                                    .spawn(
                                                        UiContainer::new()
                                                            .with_column_gap(Val::Px(8.0)),
                                                    )
                                                    .with_children(|parent| {
                                                        parent.spawn(
                                                            UiText::new(
                                                                "ui.structure_info.structure_name",
                                                            )
                                                            .with_width(Val::Auto)
                                                            .with_size(UiTextSize::Small)
                                                            .with_justify(JustifyText::Left),
                                                        );
                                                        parent.spawn(
                                                            UiText::new(
                                                                &structure
                                                                    .get_variant()
                                                                    .to_string(),
                                                            )
                                                            .with_width(Val::Auto)
                                                            .with_size(UiTextSize::Small)
                                                            .with_justify(JustifyText::Left),
                                                        );
                                                    });
                                                parent.spawn(
                                                    UiText::new(
                                                        "ui.structure_info.structure_damage",
                                                    )
                                                    .with_arg(
                                                        "damage",
                                                        structure.get_damage().to_string(),
                                                    )
                                                    .with_size(UiTextSize::Small)
                                                    .with_justify(JustifyText::Left),
                                                );
                                                parent.spawn(
                                                    UiText::new(
                                                        "ui.structure_info.structure_fire_radius",
                                                    )
                                                    .with_arg(
                                                        "fire_radius",
                                                        structure.get_fire_radius().to_string(),
                                                    )
                                                    .with_size(UiTextSize::Small)
                                                    .with_justify(JustifyText::Left),
                                                );
                                                parent.spawn(
                                                    UiText::new(
                                                        "ui.structure_info.structure_fire_rate",
                                                    )
                                                    .with_arg(
                                                        "fire_rate",
                                                        ((1.0
                                                            / structure
                                                                .get_fire_rate()
                                                                .as_secs_f32()
                                                            * 100.0)
                                                            .round()
                                                            / 100.0)
                                                            .to_string(),
                                                    )
                                                    .with_size(UiTextSize::Small)
                                                    .with_justify(JustifyText::Left),
                                                );
                                            },
                                        );
                                    });
                                break;
                            }
                        }
                    }

                    parent
                        .spawn(UiContainer::new().with_row_gap(Val::Px(8.0)).column())
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    StructureInfoButtonAction::UpgradeStructure,
                                    UiButton::new()
                                        .with_variant(UiButtonVariant::Success)
                                        .with_padding(UiRect::all(Val::Px(8.0))),
                                ))
                                .with_child(UiText::new("ui.structure_info.upgrade_structure"));

                            parent
                                .spawn((
                                    StructureInfoButtonAction::SellStructure,
                                    UiButton::new()
                                        .with_variant(UiButtonVariant::Danger)
                                        .with_padding(UiRect::all(Val::Px(8.0))),
                                ))
                                .with_child(UiText::new("ui.structure_info.sell_structure"));
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
        (&Interaction, &StructureInfoButtonAction),
        (Changed<Interaction>, With<UiButton>),
    >,
    mut structures: Query<(Entity, &TilePosition), With<Structure>>,
    selected_structure: Option<Res<SelectedStructure>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                StructureInfoButtonAction::Close => {
                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);
                }
                StructureInfoButtonAction::UpgradeStructure => {
                    next_ui_state.set(UiState::StructureSelect);
                    next_game_state.set(GameState::Pause);
                }
                StructureInfoButtonAction::SellStructure => {
                    if let Some(selected_structure) = selected_structure.as_ref() {
                        for (structure_entity, structure_tile_position) in structures.iter_mut() {
                            if structure_tile_position.as_vec2()
                                == selected_structure.tile_position.as_vec2()
                            {
                                commands.entity(structure_entity).despawn_recursive();
                                break;
                            }
                        }
                    }
                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);
                }
            }
        }
    }
}
