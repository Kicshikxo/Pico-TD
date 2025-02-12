use bevy::prelude::*;

use crate::{game::GameState, ui::UiState};

#[derive(Resource)]
pub struct PlayerHealth {
    max: u32,
    current: u32,
}

impl Default for PlayerHealth {
    fn default() -> Self {
        Self { max: 0, current: 0 }
    }
}

#[allow(unused)]
impl PlayerHealth {
    pub fn restart(&mut self, max: u32) {
        self.max = max;
        self.current = max;
    }
    pub fn damage(&mut self, damage: u32) {
        self.current = self.current.saturating_sub(damage);
    }
    pub fn heal(&mut self, heal: u32) {
        self.current = self.current.saturating_add(heal).min(self.max);
    }
    pub fn get_current(&self) -> u32 {
        self.current
    }
    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
}

#[derive(Resource)]
pub struct PlayerMoney {
    current: u32,
}

impl Default for PlayerMoney {
    fn default() -> Self {
        Self { current: 0 }
    }
}

impl PlayerMoney {
    pub fn decrease(&mut self, value: u32) {
        self.current = self.current.saturating_sub(value);
    }
    pub fn increase(&mut self, value: u32) {
        self.current = self.current.saturating_add(value);
    }
    pub fn get_current(&self) -> u32 {
        self.current
    }
}

#[derive(Resource)]
pub struct Player {
    pub health: PlayerHealth,
    pub money: PlayerMoney,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            health: PlayerHealth::default(),
            money: PlayerMoney::default(),
        }
    }
}

impl Player {
    pub fn restart(&mut self, max_health: u32, money: u32) {
        self.health.restart(max_health);
        self.money.current = money;
    }
    pub fn get_health(&self) -> &PlayerHealth {
        &self.health
    }
    pub fn get_health_mut(&mut self) -> &mut PlayerHealth {
        &mut self.health
    }
    pub fn get_money(&self) -> &PlayerMoney {
        &self.money
    }
    pub fn get_money_mut(&mut self) -> &mut PlayerMoney {
        &mut self.money
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Player>();

        app.add_systems(
            Update,
            update_player.run_if(in_state(GameState::InGame).and(resource_changed::<Player>)),
        );
    }
}

fn update_player(
    player: Res<Player>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if player.get_health().is_dead() {
        next_ui_state.set(UiState::GameOver);
        next_game_state.set(GameState::Pause);
    }
}
