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
            UiContainer::new().with_height(Val::Percent(100.0)).center(),
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
                            MenuButtonAction::Start,
                            UiButton::new().with_variant(UiButtonVariant::Success),
                        ))
                        .with_child(UiText::new("ui.menu.start_game").with_size(UiTextSize::Large));

                    parent
                        .spawn((
                            MenuButtonAction::Settings,
                            UiButton::new().with_variant(UiButtonVariant::Primary),
                        ))
                        .with_child(UiText::new("ui.menu.settings").with_size(UiTextSize::Large));

                    #[cfg(not(target_arch = "wasm32"))]
                    parent
                        .spawn((
                            MenuButtonAction::Exit,
                            UiButton::new().with_variant(UiButtonVariant::Danger),
                        ))
                        .with_child(UiText::new("ui.menu.exit_game").with_size(UiTextSize::Large));
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
        (Changed<Interaction>, With<UiButton>),
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
