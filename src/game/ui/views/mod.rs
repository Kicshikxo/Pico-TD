pub mod game_over;
pub mod in_game;
pub mod level_select;
pub mod menu;
pub mod pause;
pub mod settings;
pub mod soldier_info;
pub mod soldier_placement_confirm;
pub mod soldier_select;

use bevy::prelude::*;

use crate::game::ui::views::{
    game_over::GameOverViewUiPlugin, in_game::InGameViewUiPlugin,
    level_select::LevelSelectViewUiPlugin, menu::MenuViewUiPlugin, pause::PauseViewUiPlugin,
    settings::SettingsViewUiPlugin, soldier_info::SoldierInfoViewUiPlugin,
    soldier_placement_confirm::SoldierPlacementConfirmViewUiPlugin,
    soldier_select::SoldierSelectViewUiPlugin,
};

pub struct ViewsUiPlugin;

impl Plugin for ViewsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MenuViewUiPlugin,
            LevelSelectViewUiPlugin,
            SettingsViewUiPlugin,
            InGameViewUiPlugin,
            SoldierSelectViewUiPlugin,
            SoldierPlacementConfirmViewUiPlugin,
            SoldierInfoViewUiPlugin,
            PauseViewUiPlugin,
            GameOverViewUiPlugin,
        ));
    }
}
