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
    // #[asset(path = "embedded://images/ui/primary/container_background.png")]
    // pub primary_container_background: Handle<Image>,
    // #[asset(path = "embedded://images/ui/primary/container_background_hovered.png")]
    // pub primary_container_background_hovered: Handle<Image>,
    // #[asset(path = "embedded://images/ui/primary/container_background_flat.png")]
    // pub primary_container_background_flat: Handle<Image>,
    // #[asset(path = "embedded://images/ui/primary/container_background_bordered.png")]
    // pub primary_container_background_bordered: Handle<Image>,
    // #[asset(path = "embedded://images/ui/primary/container_background_bordered_flat.png")]
    // pub primary_container_background_bordered_flat: Handle<Image>,

    // #[asset(path = "embedded://images/ui/danger/container_background.png")]
    // pub danger_container_background: Handle<Image>,
    // #[asset(path = "embedded://images/ui/danger/container_background_hovered.png")]
    // pub danger_container_background_hovered: Handle<Image>,
    // #[asset(path = "embedded://images/ui/danger/container_background_flat.png")]
    // pub danger_container_background_flat: Handle<Image>,
    // #[asset(path = "embedded://images/ui/danger/container_background_bordered.png")]
    // pub danger_container_background_bordered: Handle<Image>,
    // #[asset(path = "embedded://images/ui/danger/container_background_bordered_flat.png")]
    // pub danger_container_background_bordered_flat: Handle<Image>,
}
