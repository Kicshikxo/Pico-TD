pub mod components;
pub mod i18n;
pub mod views;

use bevy::prelude::*;

use crate::game::ui::{components::ComponentsUiPlugin, i18n::I18nPlugin, views::ViewsUiPlugin};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UiState>();

        app.add_plugins((I18nPlugin, ComponentsUiPlugin, ViewsUiPlugin));
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UiState {
    #[default]
    None,
    Menu,
    LevelSelect,
    Settings,
    InGame,
    SoldierSelect,
    SoldierPlacementConfirmation,
    SoldierInfo,
    Pause,
    GameOver,
}
