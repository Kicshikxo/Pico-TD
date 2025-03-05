use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct UiAudioAssets {
    #[asset(path = "embedded://audio/ui/button_click.ogg")]
    pub button_click: Handle<AudioSource>,
    #[asset(path = "embedded://audio/ui/button_click_error.ogg")]
    pub button_click_error: Handle<AudioSource>,
    #[asset(path = "embedded://audio/ui/selector_click.ogg")]
    pub selector_click: Handle<AudioSource>,
    #[asset(path = "embedded://audio/ui/tilemap_click.ogg")]
    pub tilemap_click: Handle<AudioSource>,

    #[asset(path = "embedded://audio/ui/level_select.ogg")]
    pub level_select: Handle<AudioSource>,

    #[asset(path = "embedded://audio/ui/soldier_upgrade.ogg")]
    pub soldier_upgrade: Handle<AudioSource>,
    #[asset(path = "embedded://audio/ui/soldier_sell.ogg")]
    pub soldier_sell: Handle<AudioSource>,
}
