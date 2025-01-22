use bevy::prelude::*;

use crate::{
    assets::sprites::{tile::TileAssets, ui::UiAssets},
    entities::{
        structure::{Structure, StructureVariant},
        tile::{position::TilePosition, sprite::TileSprite},
    },
    game::{GameState, GameTilemap},
    input::SelectedStructure,
    ui::{
        components::{
            button::UiButton,
            container::{UiContainer, UiContainerVariant},
            text::{UiText, UiTextSize},
        },
        UiState,
    },
};

pub struct StructureSelectViewUiPlugin;

impl Plugin for StructureSelectViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::StructureSelect), ui_init)
            .add_systems(OnExit(UiState::StructureSelect), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::StructureSelect)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum StructureSelectButtonAction {
    Close,
    Select(StructureVariant),
}

fn ui_init(mut commands: Commands, ui_assets: Res<UiAssets>, tile_assets: Res<TileAssets>) {
    commands
        .spawn((
            RootUiComponent,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
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
                        StructureSelectButtonAction::Close,
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
                        .with_child(UiText::new("ui.structure_select.title"));

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
                                StructureVariant::Soldier,
                                StructureVariant::SoldierFast,
                                StructureVariant::SoldierStrong,
                                StructureVariant::RocketLauncher,
                            ] {
                                parent
                                    .spawn(Node {
                                        align_items: AlignItems::Center,
                                        flex_direction: FlexDirection::Column,
                                        row_gap: Val::Px(4.0),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent
                                            .spawn((
                                                StructureSelectButtonAction::Select(variant),
                                                UiButton::new(),
                                                UiContainer::new()
                                                    .with_variant(UiContainerVariant::Secondary)
                                                    .with_aspect_ratio(1.0)
                                                    .center(),
                                            ))
                                            .with_child((
                                                Node {
                                                    width: Val::Px(32.0),
                                                    height: Val::Px(32.0),
                                                    ..default()
                                                },
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
        (&Interaction, &StructureSelectButtonAction),
        (Changed<Interaction>, With<UiButton>),
    >,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    mut structures: Query<(&mut Structure, &TilePosition)>,
    selected_structure: Res<SelectedStructure>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                StructureSelectButtonAction::Close => {
                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);
                }
                StructureSelectButtonAction::Select(variant) => {
                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);

                    for (mut structure, tile_position) in structures.iter_mut() {
                        if tile_position.as_vec2() == selected_structure.tile_position.as_vec2() {
                            structure.set_variant(variant.clone());
                            return;
                        }
                    }

                    commands.entity(game_tilemap.single()).with_child((
                        Structure::new(variant.clone()),
                        selected_structure.tile_position.clone(),
                    ));
                }
            }
        }
    }
}
