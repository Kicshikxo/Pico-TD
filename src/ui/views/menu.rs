use bevy::{prelude::*, ui::widget::NodeImageMode};

use crate::{
    assets::sprites::ui::{UiAssets, UiMiscSpriteVariant},
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
            container::{UiContainer, UiContainerVariant},
            text::{UiText, UiTextSize},
        },
        UiState,
    },
};

pub struct MenuViewUiPlugin;

impl Plugin for MenuViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Menu), init_ui)
            .add_systems(OnExit(UiState::Menu), destroy_ui)
            .add_systems(Update, update_ui.run_if(in_state(UiState::Menu)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum ButtonAction {
    Start,
    Settings,
    Exit,
}

fn init_ui(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn((
            RootUiComponent,
            UiContainer::new().full().center(),
            ImageNode {
                image: ui_assets.ui_misc.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: UiMiscSpriteVariant::Background as usize,
                    layout: ui_assets.ui_misc_layout.clone(),
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
                    parent
                        .spawn(
                            UiContainer::new()
                                .with_variant(UiContainerVariant::Secondary)
                                .with_padding(UiRect::all(Val::Px(8.0))),
                        )
                        .with_child(
                            UiText::new("ui.menu.game_title").with_size(UiTextSize::ExtraLarge),
                        );

                    parent
                        .spawn((
                            ButtonAction::Start,
                            UiButton::new().with_variant(UiButtonVariant::Success),
                        ))
                        .with_child(UiText::new("ui.menu.start_game").with_size(UiTextSize::Large));

                    parent
                        .spawn((
                            ButtonAction::Settings,
                            UiButton::new().with_variant(UiButtonVariant::Primary),
                        ))
                        .with_child(UiText::new("ui.menu.settings").with_size(UiTextSize::Large));

                    #[cfg(not(target_arch = "wasm32"))]
                    parent
                        .spawn((
                            ButtonAction::Exit,
                            UiButton::new().with_variant(UiButtonVariant::Danger),
                        ))
                        .with_child(UiText::new("ui.menu.exit_game").with_size(UiTextSize::Large));
                });
        });
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_ui(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<UiButton>)>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::Start => {
                    next_ui_state.set(UiState::LevelSelect);
                }
                ButtonAction::Settings => {
                    next_ui_state.set(UiState::Settings);
                }
                ButtonAction::Exit => {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
    }
}
