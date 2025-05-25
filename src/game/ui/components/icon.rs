use bevy::prelude::*;

use crate::game::assets::images::ui::{UiAssets, UiIconSpriteVariant};

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum UiIconVariant {
    Settings,
    Globe,
    Sound,
    Music,
    Home,
    Play,
    Next,
    Pause,
    Exit,
    Upload,
    Restart,
    Delete,
}

impl UiIconVariant {
    pub fn as_index(&self) -> usize {
        match self {
            UiIconVariant::Settings => UiIconSpriteVariant::Settings as usize,
            UiIconVariant::Globe => UiIconSpriteVariant::Globe as usize,
            UiIconVariant::Sound => UiIconSpriteVariant::Sound as usize,
            UiIconVariant::Music => UiIconSpriteVariant::Music as usize,
            UiIconVariant::Home => UiIconSpriteVariant::Home as usize,
            UiIconVariant::Play => UiIconSpriteVariant::Play as usize,
            UiIconVariant::Next => UiIconSpriteVariant::Next as usize,
            UiIconVariant::Pause => UiIconSpriteVariant::Pause as usize,
            UiIconVariant::Exit => UiIconSpriteVariant::Exit as usize,
            UiIconVariant::Upload => UiIconSpriteVariant::Upload as usize,
            UiIconVariant::Restart => UiIconSpriteVariant::Restart as usize,
            UiIconVariant::Delete => UiIconSpriteVariant::Delete as usize,
        }
    }
}

#[allow(unused)]
#[derive(Clone, Copy, Default)]
pub enum UiIconSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    Custom(f32),
}

impl UiIconSize {
    pub fn as_f32(&self) -> f32 {
        match self {
            UiIconSize::Small => 16.0,
            UiIconSize::Medium => 24.0,
            UiIconSize::Large => 32.0,
            UiIconSize::ExtraLarge => 64.0,
            UiIconSize::Custom(size) => *size,
        }
    }
}

#[derive(Component)]
#[require(Node)]
pub struct UiIcon {
    variant: UiIconVariant,
    size: UiIconSize,
}

impl UiIcon {
    pub fn new(variant: UiIconVariant) -> Self {
        Self {
            variant,
            size: UiIconSize::default(),
        }
    }
    pub fn with_size(mut self, size: UiIconSize) -> Self {
        self.size = size;
        self
    }
}

pub struct UiIconPlugin;

impl Plugin for UiIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_ui_icon);
    }
}

fn init_ui_icon(
    mut commands: Commands,
    ui_icons: Query<(Entity, &UiIcon), Added<UiIcon>>,
    ui_assets: Option<Res<UiAssets>>,
) {
    let Some(ui_assets) = &ui_assets else {
        return;
    };

    for (ui_icon_entity, ui_icon) in ui_icons.iter() {
        commands.entity(ui_icon_entity).insert((
            Node {
                width: Val::Px(ui_icon.size.as_f32()),
                height: Val::Px(ui_icon.size.as_f32()),
                ..default()
            },
            ImageNode {
                image: ui_assets.ui_icons.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: ui_icon.variant.as_index(),
                    layout: ui_assets.ui_icons_layout.clone(),
                }),
                ..default()
            },
        ));
    }
}
