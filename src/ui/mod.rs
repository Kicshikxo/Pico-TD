pub mod components;
pub mod i18n;
pub mod views;

use bevy::prelude::*;

use components::ComponentsUiPlugin;
use i18n::I18nPlugin;
use views::ViewsUiPlugin;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((I18nPlugin, ComponentsUiPlugin, ViewsUiPlugin));

        app.init_state::<UiState>();
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
    StructureSelect,
    StructureInfo,
    Pause,
    GameOver,
}
