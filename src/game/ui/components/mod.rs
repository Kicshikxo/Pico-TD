pub mod button;
pub mod container;
pub mod selector;
pub mod text;

use bevy::prelude::*;

use crate::game::ui::components::{
    button::UiButtonPlugin, container::UiContainerPlugin, selector::UiSelectorPlugin,
    text::UiTextPlugin,
};

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
