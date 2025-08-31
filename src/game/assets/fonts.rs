use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::game::ui::i18n::Locale;

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "embedded://fonts/Fairfax.ttf")]
    pub primary_font: Handle<Font>,
    #[asset(path = "embedded://fonts/fusion-pixel-12px-monospaced-zh_hans.ttf")]
    pub cjk_font: Handle<Font>,
}

impl FontAssets {
    pub fn get_locale_based_font(&self, locale: Locale) -> Handle<Font> {
        match locale {
            Locale::Zh | Locale::Ja | Locale::Ko => self.cjk_font.clone(),
            _ => self.primary_font.clone(),
        }
    }
}
