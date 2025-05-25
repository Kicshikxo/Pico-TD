use bevy::{prelude::*, ui::widget::NodeImageMode};

use crate::game::{
    assets::images::ui::{UiAssets, UiMiscSpriteVariant},
    ui::{
        UiState,
        components::{
            button::{UiButton, UiButtonInteraction},
            container::UiContainer,
            icon::{UiIcon, UiIconVariant},
            text::{UiText, UiTextSize},
        },
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
    #[allow(unused)]
    Exit,
}

fn init_ui(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((
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
        children![
            (
                UiContainer::primary()
                    .with_min_width(Val::Px(320.0))
                    .with_padding(UiRect::all(Val::Px(24.0)))
                    .with_row_gap(Val::Px(12.0))
                    .auto_width()
                    .center()
                    .column(),
                children![
                    (
                        UiContainer::secondary().with_padding(UiRect::all(Val::Px(8.0))),
                        children![
                            UiText::new("ui.menu.game_title").with_size(UiTextSize::ExtraLarge),
                        ]
                    ),
                    (
                        (ButtonAction::Start, UiButton::success()),
                        children![
                            UiIcon::new(UiIconVariant::Play),
                            UiText::new("ui.menu.start_game")
                                .with_size(UiTextSize::Large)
                                .auto_width(),
                        ]
                    ),
                    (
                        (ButtonAction::Settings, UiButton::primary()),
                        children![
                            UiIcon::new(UiIconVariant::Settings),
                            UiText::new("ui.menu.settings")
                                .with_size(UiTextSize::Large)
                                .auto_width(),
                        ]
                    ),
                    #[cfg(not(target_arch = "wasm32"))]
                    (
                        (ButtonAction::Exit, UiButton::danger()),
                        children![
                            UiIcon::new(UiIconVariant::Exit),
                            UiText::new("ui.menu.exit_game")
                                .with_size(UiTextSize::Large)
                                .auto_width(),
                        ]
                    ),
                ]
            ),
            (
                UiContainer::new()
                    .with_right(Val::Px(8.0))
                    .with_bottom(Val::Px(8.0))
                    .absolute(),
                children![
                    UiText::new("ui.version")
                        .with_size(UiTextSize::Small)
                        .with_justify(JustifyText::Right)
                        .with_i18n_arg("version", env!("CARGO_PKG_VERSION").to_string())
                ]
            )
        ],
    ));
}

fn destroy_ui(mut commands: Commands, query: Query<Entity, With<RootUiComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_ui(
    interaction_query: Query<
        (&UiButtonInteraction, &ButtonAction),
        (Changed<UiButtonInteraction>, With<UiButton>),
    >,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (ui_button_interaction, button_action) in interaction_query.iter() {
        if *ui_button_interaction != UiButtonInteraction::Clicked {
            continue;
        }
        match button_action {
            ButtonAction::Start => {
                next_ui_state.set(UiState::LevelSelect);
            }
            ButtonAction::Settings => {
                next_ui_state.set(UiState::Settings);
            }
            ButtonAction::Exit => {
                app_exit_events.write(AppExit::Success);
            }
        }
    }
}
