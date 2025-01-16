pub mod in_game;
pub mod level_select;
pub mod menu;
pub mod pause;
pub mod settings;

use bevy::prelude::*;

use in_game::InGameViewUiPlugin;
use level_select::LevelSelectViewUiPlugin;
use menu::MenuViewUiPlugin;
use pause::PauseViewUiPlugin;
use settings::SettingsViewUiPlugin;

pub struct ViewsUiPlugin;

impl Plugin for ViewsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MenuViewUiPlugin,
            LevelSelectViewUiPlugin,
            SettingsViewUiPlugin,
            InGameViewUiPlugin,
            PauseViewUiPlugin,
        ));
    }
}
