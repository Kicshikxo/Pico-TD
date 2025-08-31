use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::game::ui::i18n::Locale;

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "embedded://fonts/Fairfax.ttf")]
    pub primary_font: Handle<Font>,
    #[asset(path = "embedded://fonts/ark-pixel-16px-proportional-zh_cn.ttf")]
    pub chinese_font: Handle<Font>,
    #[asset(path = "embedded://fonts/ark-pixel-16px-proportional-ja.ttf")]
    pub japanese_font: Handle<Font>,
    #[asset(path = "embedded://fonts/ark-pixel-16px-proportional-ko.ttf")]
    pub korean_font: Handle<Font>,
}

impl FontAssets {
    pub fn get_locale_based_font(&self, locale: Locale) -> Handle<Font> {
        match locale {
            Locale::Zh => self.chinese_font.clone(),
            Locale::Ja => self.japanese_font.clone(),
            Locale::Ko => self.korean_font.clone(),
            _ => self.primary_font.clone(),
        }
    }
}
