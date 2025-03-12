use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    ui::widget::NodeImageMode,
};
use bevy_persistent::Persistent;

use crate::game::{
    assets::{
        audio::ui::UiAudioAssets,
        images::ui::{UiAssets, UiButtonSpriteVariant},
    },
    audio::{GameAudio, GameAudioVolume},
};

#[derive(Component, Default, Clone, Copy, PartialEq)]
pub enum UiButtonInteraction {
    Clicked,
    Hovered,
    #[default]
    None,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum UiButtonVariant {
    #[default]
    None,
    Primary,
    Secondary,
    Success,
    Danger,
}

impl UiButtonVariant {
    pub fn as_index(&self) -> usize {
        match self {
            UiButtonVariant::None => unreachable!(),
            UiButtonVariant::Primary => UiButtonSpriteVariant::Primary as usize,
            UiButtonVariant::Secondary => UiButtonSpriteVariant::Secondary as usize,
            UiButtonVariant::Success => UiButtonSpriteVariant::Success as usize,
            UiButtonVariant::Danger => UiButtonSpriteVariant::Danger as usize,
        }
    }
}

#[derive(Component)]
#[require(Node, UiButtonInteraction)]
pub struct UiButton {
    variant: UiButtonVariant,
    disabled: bool,
    next_disabled_state: bool,
    click_audio: Option<Handle<AudioSource>>,
    width: Val,
    height: Val,
    padding: UiRect,
    aspect_ratio: Option<f32>,
    max_corner_scale: f32,
    previous_interaction: Interaction,
    update_required: bool,
}

impl Default for UiButton {
    fn default() -> Self {
        Self {
            variant: UiButtonVariant::default(),
            disabled: false,
            next_disabled_state: false,
            click_audio: None,
            width: Val::Percent(100.0),
            height: Val::Auto,
            padding: UiRect::all(Val::Px(12.0)),
            aspect_ratio: None,
            max_corner_scale: 4.0,
            previous_interaction: Interaction::default(),
            update_required: true,
        }
    }
}

#[allow(unused)]
impl UiButton {
    pub fn new() -> Self {
        Self { ..default() }
    }
    pub fn primary() -> Self {
        Self::new().with_variant(UiButtonVariant::Primary)
    }
    pub fn secondary() -> Self {
        Self::new().with_variant(UiButtonVariant::Secondary)
    }
    pub fn danger() -> Self {
        Self::new().with_variant(UiButtonVariant::Danger)
    }
    pub fn success() -> Self {
        Self::new().with_variant(UiButtonVariant::Success)
    }
    pub fn with_variant(mut self, variant: UiButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.next_disabled_state = disabled;
        self
    }
    pub fn with_click_audio(mut self, click_audio: Handle<AudioSource>) -> Self {
        self.click_audio = Some(click_audio);
        self
    }
    pub fn get_variant(&self) -> UiButtonVariant {
        self.variant
    }
    pub fn get_disabled(&self) -> bool {
        self.disabled
    }
    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
    pub fn set_next_disabled_state(&mut self, disabled: bool) {
        self.set_update_required(self.next_disabled_state != disabled);
        self.next_disabled_state = disabled;
    }
    pub fn get_next_disabled_state(&self) -> bool {
        self.next_disabled_state
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
    pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }
    pub fn with_max_corner_scale(mut self, max_corner_scale: f32) -> Self {
        self.max_corner_scale = max_corner_scale;
        self
    }
    fn get_previous_interaction(&self) -> Interaction {
        self.previous_interaction
    }
    fn set_previous_interaction(&mut self, interaction: Interaction) {
        self.previous_interaction = interaction;
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
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
                    aspect_ratio: ui_button.aspect_ratio,
                    ..default()
                },
                ImageNode {
                    image: ui_assets.ui_buttons.clone(),
                    texture_atlas: Some(TextureAtlas {
                        index: ui_button.variant.as_index(),
                        layout: ui_assets.ui_buttons_layout.clone(),
                    }),
                    image_mode: NodeImageMode::Sliced(TextureSlicer {
                        border: BorderRect::square(6.0),
                        max_corner_scale: ui_button.max_corner_scale,
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
    mut ui_buttons: ParamSet<(
        Query<
            (
                &Interaction,
                &mut UiButtonInteraction,
                &mut UiButton,
                &mut ImageNode,
            ),
            (Changed<Interaction>, With<UiButton>),
        >,
        Query<(&mut UiButton, &mut ImageNode)>,
    )>,
    touches: Res<Touches>,
    game_audio: Query<Entity, With<GameAudio>>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
    ui_audio_assets: Option<Res<UiAudioAssets>>,
) {
    for (interaction, mut ui_button_interaction, mut ui_button, mut image_node) in
        ui_buttons.p0().iter_mut()
    {
        if ui_button.get_next_disabled_state() == false {
            image_node.color = match *interaction {
                Interaction::Pressed => Color::srgb(0.9, 0.9, 0.9),
                Interaction::Hovered => Color::srgb(0.95, 0.95, 0.95),
                Interaction::None => Color::WHITE,
            };
        }

        *ui_button_interaction =
            if *interaction == Interaction::Hovered || touches.any_just_released() {
                if ui_button.get_previous_interaction() == Interaction::Pressed {
                    UiButtonInteraction::Clicked
                } else {
                    UiButtonInteraction::Hovered
                }
            } else if *interaction == Interaction::Pressed {
                UiButtonInteraction::Hovered
            } else {
                UiButtonInteraction::None
            };
        ui_button.set_previous_interaction(*interaction);

        if *ui_button_interaction == UiButtonInteraction::Clicked {
            if let Some(ui_audio_assets) = &ui_audio_assets {
                let audio_asset = if ui_button.get_disabled() {
                    ui_audio_assets.button_click_error.clone()
                } else {
                    ui_button
                        .click_audio
                        .clone()
                        .unwrap_or_else(|| ui_audio_assets.button_click.clone())
                };

                commands.entity(game_audio.single()).with_child((
                    AudioPlayer::new(audio_asset),
                    PlaybackSettings {
                        mode: PlaybackMode::Remove,
                        volume: Volume::new(game_audio_volume.get_sfx_volume()),
                        ..default()
                    },
                ));
            }
        }
    }
    for (mut ui_button, mut image_node) in ui_buttons.p1().iter_mut() {
        if ui_button.get_update_required() == true {
            let next_disabled_state = ui_button.get_next_disabled_state();
            ui_button.set_disabled(next_disabled_state);

            image_node.color = if next_disabled_state == true
                && ui_button.get_variant() != UiButtonVariant::None
            {
                Color::srgb(0.75, 0.75, 0.75)
            } else {
                Color::WHITE
            };

            ui_button.set_update_required(false);
        }
    }
}
