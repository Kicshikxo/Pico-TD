use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "embedded://fonts/Fairfax.ttf")]
    pub primary_font: Handle<Font>,

    #[asset(path = "embedded://images/ui/containers.png")]
    pub ui_containers: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 4, rows = 1))]
    pub ui_containers_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "embedded://images/ui/buttons.png")]
    pub ui_buttons: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 5, rows = 1))]
    pub ui_buttons_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "embedded://images/ui/misc.png")]
    pub ui_misc: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1))]
    pub ui_misc_layout: Handle<TextureAtlasLayout>,
}

pub enum UiContainerSpriteVariant {
    Primary = 1,
    Secondary = 0,
    Success = 2,
    Danger = 3,
}

pub enum UiButtonSpriteVariant {
    Primary = 1,
    Secondary = 0,
    Success = 2,
    Danger = 3,
    Close = 4,
}

#[derive(Copy, Clone)]
pub enum UiMiscSpriteVariant {
    Background = 0,
    Health = 1,
    Money = 2,
    Star = 3,
}
