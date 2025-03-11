use bevy::prelude::*;

use crate::game::assets::images::entity::EnemySpriteVariant;

pub struct EnemyConfig {
    health: u32,
    damage: u32,
    kill_reward: u32,
    sprite_scale: f32,
    sprite_variant: EnemySpriteVariant,
}

impl EnemyConfig {
    pub fn get_health(&self) -> u32 {
        self.health
    }
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
    pub fn get_kill_reward(&self) -> u32 {
        self.kill_reward
    }
    pub fn get_sprite_scale(&self) -> Vec3 {
        Vec3::new(self.sprite_scale, self.sprite_scale, 1.0)
    }
    pub fn get_sprite_variant(&self) -> EnemySpriteVariant {
        self.sprite_variant
    }
}

pub const DRON_LEVELS: [EnemyConfig; 5] = [
    EnemyConfig {
        health: 100,
        damage: 1,
        kill_reward: 1,
        sprite_scale: 0.67,
        sprite_variant: EnemySpriteVariant::DronGray,
    },
    EnemyConfig {
        health: 200,
        damage: 2,
        kill_reward: 2,
        sprite_scale: 0.67,
        sprite_variant: EnemySpriteVariant::DronRed,
    },
    EnemyConfig {
        health: 300,
        damage: 3,
        kill_reward: 3,
        sprite_scale: 0.67,
        sprite_variant: EnemySpriteVariant::DronGreen,
    },
    EnemyConfig {
        health: 400,
        damage: 4,
        kill_reward: 4,
        sprite_scale: 0.67,
        sprite_variant: EnemySpriteVariant::DronBlue,
    },
    EnemyConfig {
        health: 500,
        damage: 5,
        kill_reward: 5,
        sprite_scale: 0.67,
        sprite_variant: EnemySpriteVariant::DronYellow,
    },
];

pub const TRUCK_LEVELS: [EnemyConfig; 5] = [
    EnemyConfig {
        health: 100,
        damage: 1,
        kill_reward: 1,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::TruckGray,
    },
    EnemyConfig {
        health: 200,
        damage: 2,
        kill_reward: 2,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::TruckRed,
    },
    EnemyConfig {
        health: 300,
        damage: 3,
        kill_reward: 3,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::TruckGreen,
    },
    EnemyConfig {
        health: 400,
        damage: 4,
        kill_reward: 4,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::TruckBlue,
    },
    EnemyConfig {
        health: 500,
        damage: 5,
        kill_reward: 5,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::TruckYellow,
    },
];

pub const TANK_LEVELS: [EnemyConfig; 5] = [
    EnemyConfig {
        health: 1000,
        damage: 10,
        kill_reward: 10,
        sprite_scale: 0.9,
        sprite_variant: EnemySpriteVariant::TankGray,
    },
    EnemyConfig {
        health: 1200,
        damage: 12,
        kill_reward: 12,
        sprite_scale: 0.9,
        sprite_variant: EnemySpriteVariant::TankRed,
    },
    EnemyConfig {
        health: 1400,
        damage: 14,
        kill_reward: 14,
        sprite_scale: 0.9,
        sprite_variant: EnemySpriteVariant::TankGreen,
    },
    EnemyConfig {
        health: 1600,
        damage: 16,
        kill_reward: 16,
        sprite_scale: 0.9,
        sprite_variant: EnemySpriteVariant::TankBlue,
    },
    EnemyConfig {
        health: 1800,
        damage: 18,
        kill_reward: 18,
        sprite_scale: 0.9,
        sprite_variant: EnemySpriteVariant::TankYellow,
    },
];

pub const PLANE_LEVELS: [EnemyConfig; 5] = [
    EnemyConfig {
        health: 500,
        damage: 5,
        kill_reward: 5,
        sprite_scale: 1.0,
        sprite_variant: EnemySpriteVariant::PlaneGray,
    },
    EnemyConfig {
        health: 600,
        damage: 6,
        kill_reward: 6,
        sprite_scale: 1.0,
        sprite_variant: EnemySpriteVariant::PlaneRed,
    },
    EnemyConfig {
        health: 700,
        damage: 7,
        kill_reward: 7,
        sprite_scale: 1.0,
        sprite_variant: EnemySpriteVariant::PlaneGreen,
    },
    EnemyConfig {
        health: 800,
        damage: 8,
        kill_reward: 8,
        sprite_scale: 1.0,
        sprite_variant: EnemySpriteVariant::PlaneBlue,
    },
    EnemyConfig {
        health: 900,
        damage: 9,
        kill_reward: 9,
        sprite_scale: 1.0,
        sprite_variant: EnemySpriteVariant::PlaneYellow,
    },
];

pub const HELICOPTER_LEVELS: [EnemyConfig; 5] = [
    EnemyConfig {
        health: 300,
        damage: 3,
        kill_reward: 3,
        sprite_scale: 1.0,
        sprite_variant: EnemySpriteVariant::HelicopterGray,
    },
    EnemyConfig {
        health: 400,
        damage: 4,
        kill_reward: 4,
        sprite_scale: 1.0,
        sprite_variant: EnemySpriteVariant::HelicopterRed,
    },
    EnemyConfig {
        health: 500,
        damage: 5,
        kill_reward: 5,
        sprite_scale: 1.0,
        sprite_variant: EnemySpriteVariant::HelicopterGreen,
    },
    EnemyConfig {
        health: 600,
        damage: 6,
        kill_reward: 6,
        sprite_scale: 1.0,
        sprite_variant: EnemySpriteVariant::HelicopterBlue,
    },
    EnemyConfig {
        health: 700,
        damage: 7,
        kill_reward: 7,
        sprite_scale: 1.0,
        sprite_variant: EnemySpriteVariant::HelicopterYellow,
    },
];

pub const BOAT_LEVELS: [EnemyConfig; 5] = [
    EnemyConfig {
        health: 200,
        damage: 2,
        kill_reward: 2,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::BoatGray,
    },
    EnemyConfig {
        health: 300,
        damage: 3,
        kill_reward: 3,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::BoatRed,
    },
    EnemyConfig {
        health: 400,
        damage: 4,
        kill_reward: 4,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::BoatGreen,
    },
    EnemyConfig {
        health: 500,
        damage: 5,
        kill_reward: 5,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::BoatBlue,
    },
    EnemyConfig {
        health: 600,
        damage: 6,
        kill_reward: 6,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::BoatYellow,
    },
];

pub const SUBMARINE_LEVELS: [EnemyConfig; 5] = [
    EnemyConfig {
        health: 500,
        damage: 5,
        kill_reward: 5,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::SubmarineGray,
    },
    EnemyConfig {
        health: 600,
        damage: 6,
        kill_reward: 6,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::SubmarineRed,
    },
    EnemyConfig {
        health: 700,
        damage: 7,
        kill_reward: 7,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::SubmarineGreen,
    },
    EnemyConfig {
        health: 800,
        damage: 8,
        kill_reward: 8,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::SubmarineBlue,
    },
    EnemyConfig {
        health: 900,
        damage: 9,
        kill_reward: 9,
        sprite_scale: 0.75,
        sprite_variant: EnemySpriteVariant::SubmarineYellow,
    },
];
