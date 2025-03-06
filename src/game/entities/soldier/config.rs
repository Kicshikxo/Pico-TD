use std::time::Duration;

use crate::game::{
    assets::sprites::entity::SoldierSpriteVariant, entities::soldier::projectile::ProjectileVariant,
};

pub struct SoldierConfig {
    price: u32,
    sell_price: u32,
    damage: u32,
    fire_radius: f32,
    fire_rate_secs: f32,
    sprite_variant: SoldierSpriteVariant,
    projectile_variant: ProjectileVariant,
}

impl SoldierConfig {
    pub fn get_price(&self) -> u32 {
        self.price
    }
    pub fn get_sell_price(&self) -> u32 {
        self.sell_price
    }
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
    pub fn get_fire_radius(&self) -> f32 {
        self.fire_radius
    }
    pub fn get_fire_rate(&self) -> Duration {
        Duration::from_secs_f32(self.fire_rate_secs)
    }
    pub fn get_sprite_variant(&self) -> SoldierSpriteVariant {
        self.sprite_variant
    }
    pub fn get_projectile_variant(&self) -> ProjectileVariant {
        self.projectile_variant
    }
}

pub const SOLDIER_LEVELS: [SoldierConfig; 3] = [
    SoldierConfig {
        price: 150,
        sell_price: 105,
        damage: 100,
        fire_radius: 2.5,
        fire_rate_secs: 0.5,
        sprite_variant: SoldierSpriteVariant::SoldierGray,
        projectile_variant: ProjectileVariant::Bullet,
    },
    SoldierConfig {
        price: 100,
        sell_price: 175,
        damage: 200,
        fire_radius: 3.0,
        fire_rate_secs: 0.5,
        sprite_variant: SoldierSpriteVariant::SoldierYellow,
        projectile_variant: ProjectileVariant::Bullet,
    },
    SoldierConfig {
        price: 200,
        sell_price: 315,
        damage: 300,
        fire_radius: 3.5,
        fire_rate_secs: 0.5,
        sprite_variant: SoldierSpriteVariant::SoldierRed,
        projectile_variant: ProjectileVariant::Bullet,
    },
];

pub const ROCKET_LAUNCHER_LEVELS: [SoldierConfig; 3] = [
    SoldierConfig {
        price: 200,
        sell_price: 140,
        damage: 200,
        fire_radius: 3.5,
        fire_rate_secs: 1.5,
        sprite_variant: SoldierSpriteVariant::RocketLauncherGray,
        projectile_variant: ProjectileVariant::Rocket { blast_radius: 1.0 },
    },
    SoldierConfig {
        price: 150,
        sell_price: 245,
        damage: 300,
        fire_radius: 4.0,
        fire_rate_secs: 1.5,
        sprite_variant: SoldierSpriteVariant::RocketLauncherYellow,
        projectile_variant: ProjectileVariant::Rocket { blast_radius: 1.5 },
    },
    SoldierConfig {
        price: 200,
        sell_price: 385,
        damage: 300,
        fire_radius: 4.0,
        fire_rate_secs: 1.5,
        sprite_variant: SoldierSpriteVariant::RocketLauncherRed,
        projectile_variant: ProjectileVariant::Rocket { blast_radius: 2.0 },
    },
];

pub const SNIPER_LEVELS: [SoldierConfig; 2] = [
    SoldierConfig {
        price: 250,
        sell_price: 175,
        damage: 500,
        fire_radius: 4.0,
        fire_rate_secs: 2.0,
        sprite_variant: SoldierSpriteVariant::SoldierGreen,
        projectile_variant: ProjectileVariant::Bullet,
    },
    SoldierConfig {
        price: 200,
        sell_price: 315,
        damage: 1000,
        fire_radius: 5.0,
        fire_rate_secs: 2.0,
        sprite_variant: SoldierSpriteVariant::SoldierBlue,
        projectile_variant: ProjectileVariant::Bullet,
    },
];
