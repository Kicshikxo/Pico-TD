use bevy::prelude::*;

use crate::{assets::sprites::ui::UiAssets, ui::i18n::I18nComponent};

#[derive(Default)]
pub enum UiTextSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    #[allow(unused)]
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
#[require(Node)]
pub struct UiText {
    size: UiTextSize,
    justify: JustifyText,
    linebreak: LineBreak,
    color: Color,
    i18n_key: String,
    i18n_args: Vec<(String, String)>,
    enable_i18n: bool,
    width: Val,
}

impl Default for UiText {
    fn default() -> Self {
        Self {
            size: UiTextSize::default(),
            justify: JustifyText::Center,
            linebreak: LineBreak::default(),
            color: Color::WHITE,
            i18n_key: String::new(),
            i18n_args: Vec::new(),
            enable_i18n: true,
            width: Val::Percent(100.0),
        }
    }
}

impl UiText {
    pub fn new(i18n_key: &str) -> Self {
        Self {
            i18n_key: i18n_key.into(),
            ..default()
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
    pub fn with_linebreak(mut self, linebreak: LineBreak) -> Self {
        self.linebreak = linebreak;
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn no_wrap(self) -> Self {
        self.with_linebreak(LineBreak::NoWrap)
    }
    pub fn with_arg(mut self, key: &str, value: String) -> Self {
        self.i18n_args.push((key.to_string(), value));
        self
    }
    pub fn without_i18n(mut self) -> Self {
        self.enable_i18n = false;
        self
    }
    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
}

pub struct UiTextPlugin;

impl Plugin for UiTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_ui_text);
    }
}

fn init_ui_text(
    mut commands: Commands,
    ui_texts: Query<(Entity, &UiText), Added<UiText>>,
    ui_assets: Option<Res<UiAssets>>,
) {
    for (ui_text_entity, ui_text) in ui_texts.iter() {
        let Some(ui_assets) = &ui_assets else {
            return;
        };

        commands.entity(ui_text_entity).insert((
            Node {
                width: ui_text.width,
                ..default()
            },
            TextFont {
                font: ui_assets.primary_font.clone(),
                font_size: ui_text.size.as_f32(),
                ..default()
            },
            TextLayout {
                justify: ui_text.justify.clone(),
                linebreak: ui_text.linebreak.clone(),
                ..default()
            },
            TextColor(ui_text.color.clone()),
        ));

        if ui_text.enable_i18n {
            let i18n_component =
                I18nComponent::new(ui_text.i18n_key.clone()).with_args(ui_text.i18n_args.clone());
            commands
                .entity(ui_text_entity)
                .insert((Text::new(i18n_component.translate()), i18n_component));
        } else {
            commands
                .entity(ui_text_entity)
                .insert(Text::new(ui_text.i18n_key.clone()));
        }
    }
}
