use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct UtilAssets {
    #[asset(path = "embedded://images/icon.png")]
    pub window_icon: Handle<Image>,
}
