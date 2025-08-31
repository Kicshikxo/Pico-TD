use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct UtilsAssets {
    #[asset(path = "embedded://images/icon.png")]
    pub window_icon: Handle<Image>,

    #[asset(path = "embedded://fonts/Fairfax-Fusion.ttf")]
    pub primary_font: Handle<Font>,
}
