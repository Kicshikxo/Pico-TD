use bevy::prelude::*;

use crate::ui::{
    components::{button::UiButton, text::UiText},
    i18n::I18nComponent,
};

use super::{button::UiButtonVariant, text::UiTextSize};

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
            UiSelectorItemValue::String(value) => value.parse::<f32>().unwrap_or(0.0),
            UiSelectorItemValue::Number(value) => *value,
        }
    }
}

impl Default for UiSelectorItem {
    fn default() -> Self {
        Self {
            text: String::new(),
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
#[require(Node)]

pub struct UiSelector {
    current_index: usize,
    options: Vec<UiSelectorItem>,
    cycle: bool,
    update_required: bool,
    value_changed: bool,
}

impl Default for UiSelector {
    fn default() -> Self {
        Self {
            current_index: 0,
            options: Vec::new(),
            cycle: false,
            update_required: true,
            value_changed: false,
        }
    }
}

#[allow(unused)]
impl UiSelector {
    pub fn new() -> UiSelector {
        Self { ..default() }
    }
    pub fn with_options(mut self, options: Vec<UiSelectorItem>) -> Self {
        self.options = options;
        self
    }
    pub fn add_option(mut self, option: UiSelectorItem) -> Self {
        self.options.push(option);
        self
    }
    pub fn with_default_index(mut self, index: usize) -> Self {
        self.current_index = index;
        self
    }
    pub fn cycle(mut self) -> Self {
        self.cycle = true;
        self
    }
    pub fn select_previous(&mut self) {
        if self.current_index == 0 && self.cycle == false {
            return;
        }
        self.current_index = if self.cycle && self.current_index == 0 {
            self.options.len().saturating_sub(1)
        } else {
            self.current_index.saturating_sub(1)
        };
        self.update_required = true;
        self.value_changed = true;
    }
    pub fn select_next(&mut self) {
        let last_index = self.options.len().saturating_sub(1);
        if self.current_index >= last_index && self.cycle == false {
            return;
        }
        self.current_index = if self.cycle && self.current_index == last_index {
            0
        } else {
            self.current_index.saturating_add(1).min(last_index)
        };
        self.update_required = true;
        self.value_changed = true;
    }
    pub fn set_index(&mut self, index: usize) {
        self.current_index = index;
        self.update_required = true;
        self.value_changed = true;
    }
    pub fn get_current_index(&self) -> usize {
        self.current_index
    }
    pub fn get_current_item(&self) -> Option<&UiSelectorItem> {
        self.options.get(self.current_index)
    }
    pub fn get_update_required(&self) -> bool {
        self.update_required
    }
    pub fn set_update_required(&mut self, value: bool) {
        self.update_required = value;
    }
    pub fn get_value_changed(&mut self) -> bool {
        if self.value_changed {
            self.value_changed = false;
            return true;
        } else {
            return false;
        }
    }
    pub fn is_previous_allowed(&self) -> bool {
        self.cycle || self.current_index > 0
    }
    pub fn is_next_allowed(&self) -> bool {
        self.cycle || self.current_index < self.options.len().saturating_sub(1)
    }
}

pub struct UiSelectorPlugin;

impl Plugin for UiSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_ui_selector);
        app.add_systems(Update, update_ui_selector);
    }
}

fn init_ui_selector(
    mut commands: Commands,
    ui_selectors: Query<(Entity, &UiSelector), Added<UiSelector>>,
) {
    for (ui_selector_entity, ui_selector) in ui_selectors.iter() {
        commands
            .entity(ui_selector_entity)
            .insert(Node {
                width: Val::Percent(100.0),
                height: Val::Px(48.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn((
                        UiSelectorDecreaseButton,
                        UiButton::new()
                            .with_variant(UiButtonVariant::Primary)
                            .with_width(Val::Px(48.0 + UiTextSize::Medium.as_f32()))
                            .with_height(Val::Px(48.0))
                            .with_padding(UiRect::ZERO),
                    ))
                    .with_child(UiText::new("<").without_i18n());
                parent.spawn((
                    UiSelectorText,
                    UiText::new(&ui_selector.get_current_item().unwrap().text),
                ));
                parent
                    .spawn((
                        UiSelectorIncreaseButton,
                        UiButton::new()
                            .with_variant(UiButtonVariant::Primary)
                            .with_width(Val::Px(48.0 + UiTextSize::Medium.as_f32()))
                            .with_height(Val::Px(48.0))
                            .with_padding(UiRect::ZERO),
                    ))
                    .with_child(UiText::new(">").without_i18n());
            });
    }
}

fn update_ui_selector(
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
    mut ui_selector_buttons: Query<(
        &mut UiButton,
        Option<&UiSelectorDecreaseButton>,
        Option<&UiSelectorIncreaseButton>,
    )>,
    mut ui_selector_texts: Query<(&mut Text, &mut I18nComponent), With<UiSelectorText>>,
) {
    for (interaction, parent) in decrease_button_interaction.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok((mut selector, _selector_children)) = ui_selector.get_mut(parent.get()) {
                if selector.is_previous_allowed() == true {
                    selector.select_previous();
                }
            }
        }
    }
    for (interaction, parent) in increase_button_interaction.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok((mut selector, _selector_children)) = ui_selector.get_mut(parent.get()) {
                if selector.is_next_allowed() == true {
                    selector.select_next();
                }
            }
        }
    }
    for (mut selector, selector_children) in ui_selector.iter_mut() {
        if selector.get_update_required() == false {
            continue;
        }

        for child in selector_children.iter() {
            if let Ok((
                mut ui_selector_button,
                ui_selector_decrease_buttons,
                ui_selector_increase_buttons,
            )) = ui_selector_buttons.get_mut(*child)
            {
                if ui_selector_decrease_buttons.is_some() {
                    ui_selector_button.set_disabled(!selector.is_previous_allowed());
                }
                if ui_selector_increase_buttons.is_some() {
                    ui_selector_button.set_disabled(!selector.is_next_allowed());
                }
            }
            if let Ok((mut ui_selector_text, mut ui_selector_text_i18n)) =
                ui_selector_texts.get_mut(*child)
            {
                ui_selector_text_i18n.change_i18n_key(
                    selector
                        .get_current_item()
                        .unwrap_or(&UiSelectorItem::default())
                        .text
                        .clone(),
                );
                ui_selector_text.0 = ui_selector_text_i18n.translate();
            }
        }
        selector.set_update_required(false);
    }
}
