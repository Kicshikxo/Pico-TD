use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
#[allow(unused)]
pub struct UiAudioAssets {
    #[asset(path = "embedded://audio/ui/button_click.ogg")]
    pub button_click: Handle<AudioSource>,
}
