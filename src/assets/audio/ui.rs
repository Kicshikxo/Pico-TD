use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
#[allow(unused)]
pub struct UiAudioAssets {
    #[asset(path = "embedded://audio/ui/button_click.ogg")]
    pub button_click: Handle<AudioSource>,
    #[asset(path = "embedded://audio/ui/button_back_click.ogg")]
    pub button_back_click: Handle<AudioSource>,
    #[asset(path = "embedded://audio/ui/level_select.ogg")]
    pub level_select: Handle<AudioSource>,
}
