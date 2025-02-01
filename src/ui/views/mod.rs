pub mod game_over;
pub mod in_game;
pub mod level_select;
pub mod menu;
pub mod pause;
pub mod settings;
pub mod soldier_info;
pub mod soldier_select;

use bevy::prelude::*;

use game_over::GameOverViewUiPlugin;
use in_game::InGameViewUiPlugin;
use level_select::LevelSelectViewUiPlugin;
use menu::MenuViewUiPlugin;
use pause::PauseViewUiPlugin;
use settings::SettingsViewUiPlugin;
use soldier_info::SoldierInfoViewUiPlugin;
use soldier_select::SoldierSelectViewUiPlugin;

pub struct ViewsUiPlugin;

impl Plugin for ViewsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MenuViewUiPlugin,
            LevelSelectViewUiPlugin,
            SettingsViewUiPlugin,
            InGameViewUiPlugin,
            SoldierSelectViewUiPlugin,
            SoldierInfoViewUiPlugin,
            PauseViewUiPlugin,
            GameOverViewUiPlugin,
        ));
    }
}
