use std::path::Path;

use bevy::prelude::*;
use bevy_persistent::prelude::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Locale {
    En,
    Ru,
    De,
    Fr,
    Es,
    It,
    Nl,
    Tr,
    Pl,
}

impl Locale {
    pub fn to_string(&self) -> String {
        match self {
            Locale::En => "en".to_string(),
            Locale::Ru => "ru".to_string(),
            Locale::De => "de".to_string(),
            Locale::Fr => "fr".to_string(),
            Locale::Es => "es".to_string(),
            Locale::It => "it".to_string(),
            Locale::Nl => "nl".to_string(),
            Locale::Tr => "tr".to_string(),
            Locale::Pl => "pl".to_string(),
        }
    }
    pub fn from_string(string: &str) -> Locale {
        match &string[0..2] {
            "en" => Locale::En,
            "ru" => Locale::Ru,
            "de" => Locale::De,
            "fr" => Locale::Fr,
            "es" => Locale::Es,
            "it" => Locale::It,
            "nl" => Locale::Nl,
            "tr" => Locale::Tr,
            "pl" => Locale::Pl,
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
            self.update_required = true;
        }
    }
    pub fn change_i18n_args(&mut self, args: Vec<(String, String)>) {
        self.args = args;
        self.update_required = true;
    }
    pub fn change_i18n_key(&mut self, key: String) {
        self.key = key;
        self.update_required = true;
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
                    if let Some(proj_dirs) = ProjectDirs::from("ru", "kicshikxo", "pico-td") {
                        proj_dirs.preference_dir().join("i18n.ron")
                    } else {
                        Path::new("local").join("i18n")
                    },
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
    mut i18n_components: Query<(&mut Text, &I18nComponent)>,
    i18n: Res<Persistent<I18n>>,
) {
    rust_i18n::set_locale(&i18n.get_current().to_string());
    for (mut i18n_text, i18n_component) in i18n_components.iter_mut() {
        i18n_text.0 = i18n_component.translate();
    }
}
