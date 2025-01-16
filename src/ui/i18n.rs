use std::path::Path;

use bevy::prelude::*;
use bevy_persistent::prelude::*;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Locale {
    En,
    Ru,
}

impl Locale {
    pub fn to_string(&self) -> String {
        match self {
            Locale::En => "en".to_string(),
            Locale::Ru => "ru".to_string(),
        }
    }
    pub fn from_string(string: &str) -> Locale {
        match &string[0..2] {
            "en" => Locale::En,
            "ru" => Locale::Ru,
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
        // info!("Current locale: {}", rust_i18n::locale().to_string());
        // info!("Available locales: {:?}", rust_i18n::available_locales!());
        // info!("System locale: {}", system_locale);
        // info!(
        //     "System locales: {:?}",
        //     sys_locale::get_locales().collect::<String>()
        // );
        Self {
            current: Locale::from_string(&system_locale),
        }
    }
}

#[derive(Component)]
pub struct I18nComponent {
    key: String,
    args: Vec<(String, String)>,
}

impl Default for I18nComponent {
    fn default() -> Self {
        Self {
            key: String::new(),
            args: Vec::new(),
        }
    }
}

impl I18nComponent {
    pub fn new(key: String) -> Self {
        Self { key, ..default() }
    }
    pub fn with_args(mut self, args: Vec<(String, String)>) -> Self {
        self.args = args;
        self
    }
    pub fn change_i18n_key(&mut self, key: String) {
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

        app.add_systems(
            Update,
            update_locale.run_if(resource_changed::<Persistent<I18n>>),
        );
    }
}

fn update_locale(mut components: Query<(&mut Text, &I18nComponent)>, i18n: Res<Persistent<I18n>>) {
    rust_i18n::set_locale(&i18n.get_current().to_string());
    for (mut text, component) in components.iter_mut() {
        text.0 = component.translate();
    }
}
