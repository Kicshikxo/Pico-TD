use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::Rng;

#[derive(AssetCollection, Resource)]
pub struct GameAudioAssets {
    #[asset(path = "embedded://audio/game/background.ogg")]
    pub background: Handle<AudioSource>,

    #[asset(path = "embedded://audio/game/shoot_variant_0.ogg")]
    pub shoot_variant_0: Handle<AudioSource>,
    #[asset(path = "embedded://audio/game/shoot_variant_1.ogg")]
    pub shoot_variant_1: Handle<AudioSource>,
    #[asset(path = "embedded://audio/game/shoot_variant_2.ogg")]
    pub shoot_variant_2: Handle<AudioSource>,
}

impl GameAudioAssets {
    pub fn get_random_shoot(&self) -> Handle<AudioSource> {
        let index = rand::rng().random_range(0..3);
        match index {
            0 => self.shoot_variant_0.clone(),
            1 => self.shoot_variant_1.clone(),
            2 => self.shoot_variant_2.clone(),
            _ => unreachable!(),
        }
    }
}
