use bevy::{
    ecs::{
        component::{Component, ComponentId},
        world::DeferredWorld,
    },
    prelude::*,
};

use crate::ui::{
    components::{button::UiButton, text::UiText},
    i18n::I18nComponent,
};

#[derive(Component)]
pub struct UiSelectorDecreaseButton;

#[derive(Component)]
pub struct UiSelectorText;

#[derive(Component)]
pub struct UiSelectorIncreaseButton;

pub struct UiSelectorItem {
    pub text: String,
    pub value: UiSelectorItemValue,
}

#[derive(Default)]
pub enum UiSelectorItemValue {
    #[default]
    None,
    String(String),
    Number(f32),
}

impl UiSelectorItemValue {
    pub fn as_string(&self) -> String {
        match self {
            UiSelectorItemValue::None => String::new(),
            UiSelectorItemValue::String(value) => value.clone(),
            UiSelectorItemValue::Number(value) => value.to_string(),
        }
    }
    pub fn as_number(&self) -> f32 {
        match self {
            UiSelectorItemValue::None => 0.0,
            UiSelectorItemValue::String(value) => value.parse::<f32>().unwrap(),
            UiSelectorItemValue::Number(value) => *value,
        }
    }
}

impl Default for UiSelectorItem {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            value: UiSelectorItemValue::default(),
        }
    }
}

impl UiSelectorItem {
    pub fn new(text: String) -> Self {
        Self { text, ..default() }
    }
    pub fn with_value(mut self, value: UiSelectorItemValue) -> Self {
        self.value = value;
        self
    }
}

#[derive(Component)]
#[component(on_add = UiSelector::on_add)]
pub struct UiSelector {
    current_index: usize,
    options: Vec<UiSelectorItem>,
    need_update: bool,
    value_changed: bool,
}

impl Default for UiSelector {
    fn default() -> Self {
        Self {
            current_index: 0,
            options: Vec::new(),
            need_update: true,
            value_changed: false,
        }
    }
}

impl UiSelector {
    pub fn new(options: Vec<UiSelectorItem>) -> UiSelector {
        Self {
            options,
            ..default()
        }
    }
    fn on_add(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
        let selector = world.get::<Self>(entity).unwrap();
        let current_option_text = selector.get_current_item().unwrap().text.clone();

        world
            .commands()
            .entity(entity)
            .insert(Node {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn((
                        UiSelectorDecreaseButton,
                        UiButton::new()
                            .with_width(Val::Auto)
                            .with_padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0))),
                    ))
                    .with_child(UiText::new("<"));
                parent.spawn((UiSelectorText, UiText::new(&current_option_text)));
                parent
                    .spawn((
                        UiSelectorIncreaseButton,
                        UiButton::new()
                            .with_width(Val::Auto)
                            .with_padding(UiRect::axes(Val::Px(16.0), Val::Px(8.0))),
                    ))
                    .with_child(UiText::new(">"));
            });
    }
    pub fn with_default_index(mut self, index: usize) -> Self {
        self.current_index = index;
        self
    }
    pub fn select_previous(&mut self) {
        self.current_index = self.current_index.saturating_sub(1);
        self.need_update = true;
        self.value_changed = true;
    }
    pub fn select_next(&mut self) {
        self.current_index = self
            .current_index
            .saturating_add(1)
            .min(self.options.len().saturating_sub(1));
        self.need_update = true;
        self.value_changed = true;
    }
    pub fn set_index(&mut self, index: usize) {
        self.current_index = index;
        self.need_update = true;
        self.value_changed = true;
    }
    pub fn get_current_index(&self) -> usize {
        self.current_index
    }
    pub fn get_current_item(&self) -> Option<&UiSelectorItem> {
        self.options.get(self.current_index)
    }
    pub fn get_value_changed(&mut self) -> bool {
        if self.value_changed {
            self.value_changed = false;
            return true;
        } else {
            return false;
        }
    }
}

pub struct UiSelectorPlugin;

impl Plugin for UiSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, selector_update);
    }
}

fn selector_update(
    decrease_button_interaction: Query<
        (&Interaction, &Parent),
        (
            Changed<Interaction>,
            With<UiSelectorDecreaseButton>,
            Without<UiSelectorIncreaseButton>,
        ),
    >,
    increase_button_interaction: Query<
        (&Interaction, &Parent),
        (
            Changed<Interaction>,
            With<UiSelectorIncreaseButton>,
            Without<UiSelectorDecreaseButton>,
        ),
    >,
    mut ui_selector: Query<(&mut UiSelector, &Children)>,
    mut ui_selector_text: Query<(&mut Text, &mut I18nComponent), With<UiSelectorText>>,
) {
    for (interaction, parent) in decrease_button_interaction.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok((mut selector, _selector_children)) = ui_selector.get_mut(parent.get()) {
                selector.select_previous();
            }
        }
    }
    for (interaction, parent) in increase_button_interaction.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok((mut selector, _selector_children)) = ui_selector.get_mut(parent.get()) {
                selector.select_next();
            }
        }
    }
    for (mut selector, selector_children) in ui_selector.iter_mut() {
        if selector.need_update == false {
            continue;
        }

        for child in selector_children.iter() {
            if let Ok((mut text, mut text_i18n)) = ui_selector_text.get_mut(*child) {
                text_i18n.change_i18n_key(
                    selector
                        .get_current_item()
                        .unwrap_or(&UiSelectorItem::default())
                        .text
                        .clone(),
                );
                text.0 = text_i18n.translate();
            }
        }
        selector.need_update = false;
    }
}
