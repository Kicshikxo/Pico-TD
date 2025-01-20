pub mod button;
pub mod container;
pub mod selector;
pub mod text;

use bevy::prelude::*;

use button::UiButtonPlugin;
use container::UiContainerPlugin;
use selector::UiSelectorPlugin;
use text::UiTextPlugin;

pub struct ComponentsUiPlugin;

impl Plugin for ComponentsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiButtonPlugin,
            UiContainerPlugin,
            UiSelectorPlugin,
            UiTextPlugin,
        ));
    }
}
