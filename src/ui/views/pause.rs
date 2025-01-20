use bevy::prelude::*;

use crate::{
    assets::ui::UiAssets,
    game::{GameBackgroundSound, GameState, GameTilemap},
    ui::{
        components::{
            button::{UiButton, UiButtonVariant},
            container::{UiContainer, UiContainerVariant},
            text::UiText,
        },
        UiState,
    },
};

pub struct PauseViewUiPlugin;

impl Plugin for PauseViewUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Pause), ui_init)
            .add_systems(OnExit(UiState::Pause), ui_destroy)
            .add_systems(Update, ui_update.run_if(in_state(UiState::Pause)));
    }
}

#[derive(Component)]
struct RootUiComponent;

#[derive(Component)]
enum PauseButtonAction {
    Close,
    BackToMenu,
}

fn ui_init(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mut background_sound: Query<&mut AudioSink, With<GameBackgroundSound>>,
) {
    if let Ok(background_sound_sink) = background_sound.get_single_mut() {
        background_sound_sink.pause();
    }
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
                        PauseButtonAction::Close,
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
                        .with_child(UiText::new("ui.pause.title"));
                    parent
                        .spawn((
                            PauseButtonAction::BackToMenu,
                            UiButton::new().with_variant(UiButtonVariant::Primary),
                        ))
                        .with_child(UiText::new("ui.pause.back_to_menu"));
                });
        });
}

fn ui_destroy(
    mut commands: Commands,
    query: Query<Entity, With<RootUiComponent>>,
    mut background_sound: Query<&mut AudioSink, With<GameBackgroundSound>>,
) {
    if let Ok(background_sound_sink) = background_sound.get_single_mut() {
        background_sound_sink.play();
    }
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn ui_update(
    mut commands: Commands,
    interaction_query: Query<
        (&Interaction, &PauseButtonAction),
        (Changed<Interaction>, With<UiButton>),
    >,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                PauseButtonAction::Close => {
                    next_ui_state.set(UiState::InGame);
                    next_game_state.set(GameState::InGame);
                }
                PauseButtonAction::BackToMenu => {
                    commands.entity(game_tilemap.single()).despawn_recursive();

                    next_ui_state.set(UiState::Menu);
                    next_game_state.set(GameState::Pause);
                }
            }
        }
    }
}
