pub mod in_game;
pub mod level_select;
pub mod menu;
pub mod pause;
pub mod settings;
pub mod structure_info;
pub mod structure_select;

use bevy::prelude::*;

use in_game::InGameViewUiPlugin;
use level_select::LevelSelectViewUiPlugin;
use menu::MenuViewUiPlugin;
use pause::PauseViewUiPlugin;
use settings::SettingsViewUiPlugin;
use structure_info::StructureInfoViewUiPlugin;
use structure_select::StructureSelectViewUiPlugin;

pub struct ViewsUiPlugin;

impl Plugin for ViewsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MenuViewUiPlugin,
            LevelSelectViewUiPlugin,
            SettingsViewUiPlugin,
            InGameViewUiPlugin,
            StructureSelectViewUiPlugin,
            StructureInfoViewUiPlugin,
            PauseViewUiPlugin,
        ));
    }
}
