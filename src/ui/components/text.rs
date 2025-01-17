use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

use crate::{assets::ui::UiAssets, ui::i18n::I18nComponent};

#[derive(Default)]
#[allow(unused)]
pub enum UiTextSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    Custom(f32),
}

impl UiTextSize {
    pub fn as_f32(&self) -> f32 {
        match self {
            UiTextSize::Small => 16.0,
            UiTextSize::Medium => 24.0,
            UiTextSize::Large => 32.0,
            UiTextSize::ExtraLarge => 64.0,
            UiTextSize::Custom(size) => *size,
        }
    }
}

#[derive(Component)]
#[component(on_add = UiText::on_add)]
pub struct UiText {
    size: UiTextSize,
    justify: JustifyText,
    i18n_key: String,
    i18n_args: Vec<(String, String)>,
    enable_i18n: bool,
}

impl Default for UiText {
    fn default() -> Self {
        Self {
            size: UiTextSize::default(),
            justify: JustifyText::Center,
            i18n_key: String::new(),
            i18n_args: Vec::new(),
            enable_i18n: true,
        }
    }
}

#[allow(unused)]
impl UiText {
    pub fn new(i18n_key: &str) -> Self {
        Self {
            i18n_key: i18n_key.into(),
            ..default()
        }
    }
    fn on_add(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
        let ui_text = world.get::<Self>(entity).unwrap();
        let ui_assets = world.get_resource::<UiAssets>().unwrap();

        let font = ui_assets.primary_font.clone();
        let font_size = ui_text.size.as_f32();
        let justify = ui_text.justify.clone();

        let i18n_key = ui_text.i18n_key.clone();
        let i18n_args = ui_text.i18n_args.clone();
        let enable_i18n = ui_text.enable_i18n.clone();

        world.commands().entity(entity).insert((
            Node {
                width: Val::Percent(100.0),
                ..default()
            },
            TextFont {
                font,
                font_size,
                ..default()
            },
            TextLayout {
                justify,
                ..default()
            },
        ));
        if enable_i18n {
            let i18n_component = I18nComponent::new(i18n_key).with_args(i18n_args);
            world
                .commands()
                .entity(entity)
                .insert((Text::new(i18n_component.translate()), i18n_component));
        } else {
            world.commands().entity(entity).insert(Text::new(i18n_key));
        }
    }
    pub fn with_size(mut self, size: UiTextSize) -> Self {
        self.size = size;
        self
    }
    pub fn with_justify(mut self, justify: JustifyText) -> Self {
        self.justify = justify;
        self
    }
    pub fn with_arg(mut self, key: &str, value: String) -> Self {
        self.i18n_args.push((key.to_string(), value));
        self
    }
    pub fn without_i18n(mut self) -> Self {
        self.enable_i18n = false;
        self
    }
}
