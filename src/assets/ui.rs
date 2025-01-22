use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "embedded://fonts/Fifaks.ttf")]
    pub primary_font: Handle<Font>,

    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 13, rows = 7))]
    pub large_tilemap_atlas: Handle<TextureAtlasLayout>,
    #[asset(path = "embedded://images/ui/tilemap.png")]
    pub large_tilemap: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 23, rows = 7))]
    pub small_tilemap_atlas: Handle<TextureAtlasLayout>,
    #[asset(path = "embedded://images/ui/tilemap_small.png")]
    pub small_tilemap: Handle<Image>,
}
