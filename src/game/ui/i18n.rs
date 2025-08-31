use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{assets::fonts::FontAssets, ui::components::text::UiText};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Locale {
    En,
    Ru,
    Uk,
    De,
    Fr,
    Es,
    Pt,
    It,
    Nl,
    Tr,
    Pl,
    Zh,
    Ja,
    Ko,
}

impl Locale {
    pub fn to_string(&self) -> String {
        match self {
            Locale::En => "en",
            Locale::Ru => "ru",
            Locale::Uk => "uk",
            Locale::De => "de",
            Locale::Fr => "fr",
            Locale::Es => "es",
            Locale::Pt => "pt",
            Locale::It => "it",
            Locale::Nl => "nl",
            Locale::Tr => "tr",
            Locale::Pl => "pl",
            Locale::Zh => "zh",
            Locale::Ja => "ja",
            Locale::Ko => "ko",
        }
        .to_string()
    }
    pub fn from_string(string: &str) -> Locale {
        match &string[0..2] {
            "en" => Locale::En,
            "ru" => Locale::Ru,
            "uk" => Locale::Uk,
            "de" => Locale::De,
            "fr" => Locale::Fr,
            "es" => Locale::Es,
            "pt" => Locale::Pt,
            "it" => Locale::It,
            "nl" => Locale::Nl,
            "tr" => Locale::Tr,
            "pl" => Locale::Pl,
            "zh" => Locale::Zh,
            "ja" => Locale::Ja,
            "ko" => Locale::Ko,
            _ => Locale::En,
        }
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct I18n {
    current: Locale,
}

impl I18n {
    pub fn set_locale(&mut self, locale: Locale) {
        self.current = locale;
    }
    pub fn get_current(&self) -> Locale {
        self.current
    }
}

impl Default for I18n {
    fn default() -> Self {
        let system_locale = sys_locale::get_locale().unwrap_or(Locale::En.to_string());

        Self {
            current: Locale::from_string(&system_locale),
        }
    }
}

#[derive(Component)]
pub struct I18nComponent {
    key: String,
    args: Vec<(String, String)>,
    update_required: bool,
}

impl Default for I18nComponent {
    fn default() -> Self {
        Self {
            key: String::new(),
            args: Vec::new(),
            update_required: false,
        }
    }
}

impl I18nComponent {
    pub fn new(key: String) -> Self {
        Self { key, ..default() }
    }
    pub fn with_i18n_args(mut self, args: Vec<(String, String)>) -> Self {
        self.args = args;
        self
    }
    pub fn change_i18n_arg(&mut self, key: &str, new_value: String) {
        if let Some((_arg_key, arg_value)) = self
            .args
            .iter_mut()
            .find(|(arg_key, _arg_value)| *arg_key == key)
        {
            *arg_value = new_value;
            self.set_update_required(true);
        }
    }
    pub fn change_i18n_args(&mut self, args: Vec<(String, String)>) {
        self.set_update_required(true);
        self.args = args;
    }
    pub fn change_i18n_key(&mut self, key: String) {
        self.set_update_required(self.key != key);
        self.key = key;
    }
    pub fn translate(&self) -> String {
        let (patterns, values): (Vec<&str>, Vec<String>) = self
            .args
            .iter()
            .map(|(key, value)| (key.as_str(), value.clone()))
            .unzip();
        rust_i18n::replace_patterns(
            &rust_i18n::t!(&self.key),
            patterns.as_slice(),
            values.as_slice(),
        )
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
}

pub struct I18nPlugin;

impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            Persistent::<I18n>::builder()
                .name("i18n")
                .format(StorageFormat::Ron)
                .default(I18n::default())
                .path(
                    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
                    directories::ProjectDirs::from("ru", "kicshikxo", "pico-td")
                        .unwrap()
                        .preference_dir()
                        .join("i18n.ron"),
                    #[cfg(target_arch = "wasm32")]
                    std::path::Path::new("local").join("i18n"),
                    #[cfg(target_os = "android")]
                    "/data/data/ru.kicshikxo.pico_td/files/i18n.ron",
                )
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .unwrap(),
        );

        app.add_systems(Update, update_i18n);
        app.add_systems(
            Update,
            update_locale.run_if(resource_changed::<Persistent<I18n>>),
        );
    }
}

fn update_i18n(mut i18n_components: Query<(&mut Text, &mut I18nComponent)>) {
    for (mut i18n_text, mut i18n_component) in i18n_components.iter_mut() {
        if i18n_component.get_update_required() == true {
            i18n_text.0 = i18n_component.translate();
            i18n_component.set_update_required(false);
        }
    }
}

fn update_locale(
    mut i18n_components: Query<(&UiText, &mut TextFont, &mut Text, &I18nComponent)>,
    font_assets: Option<Res<FontAssets>>,
    i18n: Res<Persistent<I18n>>,
) {
    rust_i18n::set_locale(&i18n.get_current().to_string());
    for (ui_text, mut ui_text_font, mut text, i18n_component) in i18n_components.iter_mut() {
        text.0 = i18n_component.translate();
        if let Some(font_assets) = &font_assets
            && ui_text.get_i18n_enabled()
        {
            ui_text_font.font = font_assets.get_locale_based_font(i18n.get_current())
        }
    }
}
