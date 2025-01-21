use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    ui::widget::NodeImageMode,
};
use bevy_persistent::Persistent;

use crate::{
    assets::{audio::ui::UiAudioAssets, ui::UiAssets},
    audio::{GameAudio, GameAudioVolume},
};

#[derive(Default, Clone, PartialEq)]
#[allow(unused)]
pub enum UiButtonVariant {
    #[default]
    None,
    Primary,
    Success,
    Danger,
}

impl UiButtonVariant {
    pub fn as_index(&self) -> usize {
        match self {
            UiButtonVariant::None => 0,
            UiButtonVariant::Primary => 65,
            UiButtonVariant::Success => 64,
            UiButtonVariant::Danger => 63,
        }
    }
}

#[derive(Component)]
#[require(Node)]
pub struct UiButton {
    variant: UiButtonVariant,
    width: Val,
    height: Val,
    padding: UiRect,
}

impl Default for UiButton {
    fn default() -> Self {
        Self {
            variant: UiButtonVariant::default(),
            width: Val::Percent(100.0),
            height: Val::Auto,
            padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
        }
    }
}

#[allow(unused)]
impl UiButton {
    pub fn new() -> Self {
        Self { ..default() }
    }
    pub fn with_variant(mut self, variant: UiButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    pub fn with_height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }
    pub fn with_padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }
}

pub struct UiButtonPlugin;

impl Plugin for UiButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_ui_button);
        app.add_systems(Update, update_ui_button);
    }
}

fn init_ui_button(
    mut commands: Commands,
    ui_buttons: Query<(Entity, &UiButton), Added<UiButton>>,
    ui_assets: Option<Res<UiAssets>>,
) {
    for (ui_button_entity, ui_button) in ui_buttons.iter() {
        let Some(ui_assets) = &ui_assets else {
            return;
        };

        commands.entity(ui_button_entity).insert(Button);

        if ui_button.variant != UiButtonVariant::None {
            commands.entity(ui_button_entity).insert((
                Node {
                    width: ui_button.width,
                    height: ui_button.height,
                    padding: ui_button.padding,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ImageNode {
                    image: ui_assets.small_tilemap.clone(),
                    texture_atlas: Some(TextureAtlas {
                        index: ui_button.variant.as_index(),
                        layout: ui_assets.small_tilemap_atlas.clone(),
                    }),
                    image_mode: NodeImageMode::Sliced(TextureSlicer {
                        border: BorderRect::square(6.0),
                        max_corner_scale: 2.5,
                        ..default()
                    }),
                    ..default()
                },
            ));
        }
    }
}

fn update_ui_button(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<UiButton>),
    >,
    game_audio: Single<Entity, With<GameAudio>>,
    ui_audio_assets: Option<Res<UiAudioAssets>>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
) {
    for (interaction, mut image_node) in &mut interaction_query {
        image_node.color = match *interaction {
            Interaction::Pressed => Color::srgb(0.9, 0.9, 0.9).into(),
            Interaction::Hovered => Color::srgb(0.95, 0.95, 0.95).into(),
            Interaction::None => Color::WHITE.into(),
        };
        if *interaction == Interaction::Pressed {
            if let Some(ui_audio_assets) = ui_audio_assets.as_ref() {
                commands.entity(*game_audio).with_child((
                    AudioPlayer::new(ui_audio_assets.button_click.clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Remove,
                        volume: Volume::new(game_audio_volume.get_sfx_volume()),
                        ..default()
                    },
                ));
            }
        }
    }
}
