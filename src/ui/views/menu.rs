use bevy::{prelude::*, ui::widget::NodeImageMode};

use crate::{
    assets::ui::UiAssets,
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
            text::{UiText, UiTextSize},
        },
        UiState,
    },
};

pub struct MenuViewUiPlugin;

impl Plugin for MenuViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Menu), ui_init)
            .add_systems(OnExit(UiState::Menu), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::Menu)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum MenuButtonAction {
    Start,
    Settings,
    Exit,
}

fn ui_init(mut commands: Commands, ui_assets: Res<UiAssets>) {
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
            ImageNode {
                image: ui_assets.small_tilemap.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: 5,
                    layout: ui_assets.small_tilemap_atlas.clone(),
                }),
                image_mode: NodeImageMode::Tiled {
                    tile_x: true,
                    tile_y: true,
                    stretch_value: 8.0,
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(320.0),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        padding: UiRect::all(Val::Px(24.0)),
                        ..default()
                    },
                    ImageNode {
                        image: ui_assets.large_tilemap.clone(),
                        texture_atlas: Some(TextureAtlas {
                            index: 22,
                            layout: ui_assets.large_tilemap_atlas.clone(),
                        }),
                        image_mode: NodeImageMode::Sliced(TextureSlicer {
                            border: BorderRect::square(10.0),
                            max_corner_scale: 2.5,
                            ..default()
                        }),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(8.0)),
                                ..default()
                            },
                            ImageNode {
                                image: ui_assets.large_tilemap.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    index: 3,
                                    layout: ui_assets.large_tilemap_atlas.clone(),
                                }),
                                image_mode: NodeImageMode::Sliced(TextureSlicer {
                                    border: BorderRect::square(10.0),
                                    max_corner_scale: 2.5,
                                    ..default()
                                }),
                                ..default()
                            },
                        ))
                        .with_child(UiText::new("ui.game_title").with_size(UiTextSize::ExtraLarge));

                    parent
                        .spawn((
                            MenuButtonAction::Start,
                            UiButton::new().with_variant(UiButtonVariant::Success),
                        ))
                        .with_child(UiText::new("ui.start_game").with_size(UiTextSize::Large));

                    parent
                        .spawn((MenuButtonAction::Settings, UiButton::new()))
                        .with_child(UiText::new("ui.settings").with_size(UiTextSize::Large));

                    #[cfg(not(target_arch = "wasm32"))]
                    parent
                        .spawn((
                            MenuButtonAction::Exit,
                            UiButton::new().with_variant(UiButtonVariant::Danger),
                        ))
                        .with_child(UiText::new("ui.exit_game").with_size(UiTextSize::Large));
                });
        });
}

fn ui_destroy(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn ui_update(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                MenuButtonAction::Start => {
                    next_ui_state.set(UiState::LevelSelect);
                }
                MenuButtonAction::Settings => {
                    next_ui_state.set(UiState::Settings);
                }
                MenuButtonAction::Exit => {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
    }
}
