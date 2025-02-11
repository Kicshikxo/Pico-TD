pub mod cooldown_bar;
pub mod projectile;

use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    sprite::Anchor,
};
use bevy_persistent::Persistent;
use cooldown_bar::CooldownBar;
use projectile::{Projectile, ProjectilePlugin, ProjectileVariant};

use crate::{
    assets::audio::game::GameAudioAssets,
    audio::{GameAudio, GameAudioVolume},
    game::{GameSpeed, GameState, GameTilemap},
};

use super::{
    enemy::Enemy,
    tile::{
        movement::TileMovement,
        position::TilePosition,
        sprite::{TileSprite, TileSpriteVariant},
    },
};

pub struct SoldierVariantConfig {
    price: u32,
    damage: u32,
    fire_radius: f32,
    fire_rate: Duration,
    projectile_variant: ProjectileVariant,
}

impl SoldierVariantConfig {
    pub fn get_price(&self) -> u32 {
        self.price
    }
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
    pub fn get_fire_radius(&self) -> f32 {
        self.fire_radius
    }
    pub fn get_fire_rate(&self) -> Duration {
        self.fire_rate
    }
    pub fn get_projectile_variant(&self) -> ProjectileVariant {
        self.projectile_variant
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SoldierVariant {
    Soldier,
    SoldierFast,
    SoldierStrong,
    SoldierSniper,
    RocketLauncher,
}

impl SoldierVariant {
    pub fn to_string(&self) -> String {
        match self {
            SoldierVariant::Soldier => "ui.soldier.soldier".to_string(),
            SoldierVariant::SoldierFast => "ui.soldier.soldier_fast".to_string(),
            SoldierVariant::SoldierStrong => "ui.soldier.soldier_strong".to_string(),
            SoldierVariant::SoldierSniper => "ui.soldier.soldier_sniper".to_string(),
            SoldierVariant::RocketLauncher => "ui.soldier.rocket_launcher".to_string(),
        }
    }
    pub fn get_config(&self) -> SoldierVariantConfig {
        match self {
            SoldierVariant::Soldier => SoldierVariantConfig {
                price: 50,
                damage: 25,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(0.5),
                projectile_variant: ProjectileVariant::Bullet,
            },
            SoldierVariant::SoldierFast => SoldierVariantConfig {
                price: 100,
                damage: 10,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(0.2),
                projectile_variant: ProjectileVariant::Bullet,
            },
            SoldierVariant::SoldierStrong => SoldierVariantConfig {
                price: 150,
                damage: 50,
                fire_radius: 3.0,
                fire_rate: Duration::from_secs_f32(1.0),
                projectile_variant: ProjectileVariant::Bullet,
            },
            SoldierVariant::SoldierSniper => SoldierVariantConfig {
                price: 200,
                damage: 150,
                fire_radius: 7.0,
                fire_rate: Duration::from_secs_f32(5.0),
                projectile_variant: ProjectileVariant::Bullet,
            },
            SoldierVariant::RocketLauncher => SoldierVariantConfig {
                price: 250,
                damage: 100,
                fire_radius: 5.0,
                fire_rate: Duration::from_secs_f32(2.0),
                projectile_variant: ProjectileVariant::Rocket,
            },
        }
    }
}

#[derive(Component, Clone)]
#[require(TilePosition)]
pub struct Soldier {
    variant: SoldierVariant,
    damage: u32,
    fire_radius: f32,
    fire_rate: Duration,
    cooldown: Duration,
    update_required: bool,
}

impl Soldier {
    pub fn new(variant: SoldierVariant) -> Self {
        let config = variant.get_config();

        Self {
            variant,
            damage: config.damage,
            fire_radius: config.fire_radius,
            fire_rate: config.fire_rate,
            cooldown: Duration::ZERO,
            update_required: false,
        }
    }
    pub fn get_variant(&self) -> SoldierVariant {
        self.variant
    }
    pub fn set_variant(&mut self, variant: SoldierVariant) {
        self.variant = variant;
        self.update_required = true;
    }
    pub fn set_damage(&mut self, damage: u32) {
        self.damage = damage;
    }
    pub fn get_damage(&self) -> u32 {
        self.damage
    }
    pub fn set_fire_radius(&mut self, fire_radius: f32) {
        self.fire_radius = fire_radius;
    }
    pub fn get_fire_radius(&self) -> f32 {
        self.fire_radius
    }
    pub fn set_fire_rate(&mut self, fire_rate: Duration) {
        self.fire_rate = fire_rate;
    }
    pub fn get_fire_rate(&self) -> Duration {
        self.fire_rate
    }
    pub fn get_cooldown(&self) -> Duration {
        self.cooldown
    }
    pub fn decrease_cooldown(&mut self, delta_time: Duration) {
        self.cooldown = self.cooldown.checked_sub(delta_time).unwrap_or_default();
    }
    pub fn update_cooldown(&mut self) {
        self.cooldown = self.fire_rate;
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
}

pub struct SoldierPlugin;

impl Plugin for SoldierPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProjectilePlugin);

        app.add_systems(
            Update,
            (init_soldier, update_soldier, update_soldier_cooldown)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_soldier(
    mut commands: Commands,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    soldiers: Query<(Entity, &Soldier), Added<Soldier>>,
) {
    for (soldier_entity, soldier) in soldiers.iter() {
        commands
            .entity(soldier_entity)
            .insert(TileSprite::new(soldier.get_variant().into()));

        commands.entity(game_tilemap.single()).with_child((
            CooldownBar::new(soldier_entity),
            Sprite {
                custom_size: Some(Vec2::new(2.0, 16.0)),
                color: Color::srgba(1.0, 1.0, 0.0, 0.75),
                anchor: Anchor::BottomRight,
                ..default()
            },
        ));
    }
}

fn update_soldier(
    mut commands: Commands,
    mut soldiers: Query<(&mut Soldier, &TilePosition, &mut TileSprite, &mut Transform)>,
    game_tilemap: Query<Entity, With<GameTilemap>>,
    enemies: Query<(Entity, &TileMovement, &TilePosition), With<Enemy>>,
    game_audio: Query<Entity, With<GameAudio>>,
    game_audio_assets: Res<GameAudioAssets>,
    game_audio_volume: Res<Persistent<GameAudioVolume>>,
) {
    let mut sorted_enemies = enemies.iter().collect::<Vec<_>>();
    sorted_enemies.sort_by(|(_, enemy_a_movement, _), (_, enemy_b_movement, _)| {
        enemy_b_movement
            .get_progress()
            .partial_cmp(&enemy_a_movement.get_progress())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for (mut soldier, soldier_tile_position, mut soldier_tile_sprite, mut soldier_transform) in
        soldiers.iter_mut()
    {
        if soldier.get_update_required() == true {
            soldier_tile_sprite
                .set_variant(TileSpriteVariant::Soldier(soldier.get_variant().into()));
            let config = soldier.get_variant().get_config();
            soldier.set_damage(config.get_damage());
            soldier.set_fire_radius(config.get_fire_radius());
            soldier.set_fire_rate(config.get_fire_rate());
            soldier.set_update_required(false);
        }

        if soldier.get_cooldown() > Duration::ZERO {
            continue;
        }

        for (enemy_entity, enemy_movement, enemy_tile_position) in &sorted_enemies {
            if soldier_tile_position
                .as_vec2()
                .distance(enemy_tile_position.as_vec2())
                <= soldier.get_fire_radius()
            {
                let projectile_variant =
                    soldier.get_variant().get_config().get_projectile_variant();
                let projectile_duration = projectile_variant.get_config().get_duration();

                let enemy_progress_on_hit = enemy_movement.get_progress()
                    + projectile_duration.as_secs_f32()
                        / enemy_movement.get_duration().as_secs_f32();

                commands.entity(game_tilemap.single()).with_child((
                    Projectile::new(projectile_variant, *enemy_entity, soldier.get_damage()),
                    TileMovement::new(
                        vec![
                            soldier_tile_position.as_vec2(),
                            enemy_movement.position_at_progress(enemy_progress_on_hit),
                        ],
                        projectile_duration,
                        None,
                    ),
                ));
                commands.entity(game_audio.single()).with_child((
                    AudioPlayer::new(game_audio_assets.get_random_shoot().clone()),
                    PlaybackSettings {
                        mode: PlaybackMode::Remove,
                        volume: Volume::new(game_audio_volume.get_sfx_volume()),
                        ..default()
                    },
                ));

                soldier.update_cooldown();

                let enemy_direction = soldier_tile_position.as_vec2()
                    - enemy_movement.position_at_progress(enemy_progress_on_hit);
                let scale_x = if enemy_direction.x < 0.0 { 1.0 } else { -1.0 };
                soldier_transform.translation.z = 1.0;
                soldier_transform.scale.x = scale_x;

                break;
            }
        }
    }
}

fn update_soldier_cooldown(
    mut commands: Commands,
    mut soldiers: Query<(&mut Soldier, &Transform)>,
    mut cooldown_bars: Query<(Entity, &CooldownBar, &Sprite, &mut Transform), Without<Soldier>>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    for (mut soldier, _soldier_transform) in soldiers.iter_mut() {
        if soldier.get_cooldown() > Duration::ZERO {
            soldier.decrease_cooldown(Duration::from_secs_f32(
                time.delta_secs() * game_speed.as_f32(),
            ));
            continue;
        }
    }
    for (cooldown_bar_entity, cooldown_bar, cooldown_bar_sprite, mut cooldown_bar_transform) in
        cooldown_bars.iter_mut()
    {
        if let Ok((soldier, soldier_transform)) = soldiers.get(cooldown_bar.get_soldier_entity()) {
            let cooldown_percentage =
                soldier.get_cooldown().as_secs_f32() / soldier.get_fire_rate().as_secs_f32();
            cooldown_bar_transform.scale = Vec3::new(1.0, cooldown_percentage, 1.0);

            let cooldown_bar_sprite_size = cooldown_bar_sprite.custom_size.unwrap();
            cooldown_bar_transform.translation = soldier_transform.translation
                + Vec3::new(8.0, cooldown_bar_sprite_size.y / 2.0 * -1.0, 1.0);
        } else {
            commands.entity(cooldown_bar_entity).despawn();
        }
    }
}
