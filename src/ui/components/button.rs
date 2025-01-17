// use std::sync::{Arc, Mutex};

use bevy::{
    audio::{PlaybackMode, Volume},
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    ui::widget::NodeImageMode,
};
use bevy_persistent::Persistent;

use crate::{
    assets::{audio::ui::UiAudioAssets, ui::UiAssets},
    audio::GameAudioVolume,
};

#[derive(Default, Clone, PartialEq)]
#[allow(unused)]
pub enum UiButtonVariant {
    None,
    #[default]
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
#[component(on_add = UiButton::on_add)]
pub struct UiButton {
    variant: UiButtonVariant,
    width: Val,
    height: Val,
    padding: UiRect,
    // on_click: Option<Arc<Mutex<dyn FnMut() + Send + Sync>>>,
}

impl Default for UiButton {
    fn default() -> Self {
        Self {
            variant: UiButtonVariant::default(),
            width: Val::Percent(100.0),
            height: Val::Auto,
            padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
            // on_click: None,
        }
    }
}

#[allow(unused)]
impl UiButton {
    pub fn new() -> Self {
        Self { ..default() }
    }
    fn on_add(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
        let ui_button = world.get::<Self>(entity).unwrap();
        let ui_assets = world.get_resource::<UiAssets>().unwrap();

        let width = ui_button.width;
        let height = ui_button.height;
        let padding = ui_button.padding;

        let variant = ui_button.variant.clone();
        let image = ui_assets.small_tilemap.clone();
        let layout = ui_assets.small_tilemap_atlas.clone();

        world.commands().entity(entity).insert(Button);

        if variant != UiButtonVariant::None {
            world.commands().entity(entity).insert((
                Node {
                    width,
                    height,
                    padding,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ImageNode {
                    image,
                    texture_atlas: Some(TextureAtlas {
                        index: variant.as_index(),
                        layout,
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
    // pub fn on_click<F>(mut self, f: F) -> Self
    // where
    //     F: FnMut() + Send + Sync + 'static,
    // {
    //     self.on_click = Some(Arc::new(Mutex::new(f)));
    //     self
    // }
}

pub struct UiButtonPlugin;

impl Plugin for UiButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_update);
    }
}

fn button_update(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<UiButton>),
    >,
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
                commands.spawn((
                    AudioPlayer::new(ui_audio_assets.button_click.clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Once,
                        volume: Volume::new(game_audio_volume.get_sfx_volume()),
                        ..default()
                    },
                ));
            }
            // if let Some(on_click) = ui_button.on_click.as_ref() {
            //     let mut callback = on_click.lock().unwrap();
            //     callback();
            // }
        }
    }
}
