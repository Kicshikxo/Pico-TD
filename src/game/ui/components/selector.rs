use bevy::prelude::*;

use crate::game::ui::{
    components::{
        button::{UiButton, UiButtonVariant},
        text::UiText,
    },
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
    pub fn is_previous_allowed(&self) -> bool {
        self.cycle || self.current_index > 0
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
    pub fn is_next_allowed(&self) -> bool {
        self.cycle || self.current_index < self.options.len().saturating_sub(1)
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
    pub fn get_changed_item(&mut self) -> Option<&UiSelectorItem> {
        if self.value_changed == true {
            self.value_changed = false;
            self.get_current_item()
        } else {
            None
        }
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
                            .with_width(Val::Px(48.0))
                            .with_height(Val::Px(48.0))
                            .with_padding(UiRect::ZERO)
                            .with_aspect_ratio(1.0),
                    ))
                    .with_child(UiText::new("<").without_i18n());
                parent.spawn((
                    UiSelectorText,
                    UiText::new(&ui_selector.get_current_item().unwrap().text)
                        .with_width(Val::Auto),
                ));
                parent
                    .spawn((
                        UiSelectorIncreaseButton,
                        UiButton::new()
                            .with_variant(UiButtonVariant::Primary)
                            .with_width(Val::Px(48.0))
                            .with_height(Val::Px(48.0))
                            .with_padding(UiRect::ZERO)
                            .with_aspect_ratio(1.0),
                    ))
                    .with_child(UiText::new(">").without_i18n());
            });
    }
}

fn update_ui_selector(
    ui_selector_buttons_interactions: Query<
        (
            &Interaction,
            &Parent,
            Option<&UiSelectorDecreaseButton>,
            Option<&UiSelectorIncreaseButton>,
        ),
        Changed<Interaction>,
    >,
    mut ui_selectors: Query<(&mut UiSelector, &Children)>,
    mut ui_selector_buttons: Query<(
        &mut UiButton,
        Option<&UiSelectorDecreaseButton>,
        Option<&UiSelectorIncreaseButton>,
    )>,
    mut ui_selector_texts: Query<&mut I18nComponent, With<UiSelectorText>>,
) {
    for (interaction, parent, ui_selector_decrease_buttons, ui_selector_increase_buttons) in
        ui_selector_buttons_interactions.iter()
    {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok((mut ui_selector, _ui_selector_children)) = ui_selectors.get_mut(parent.get()) {
            if ui_selector_decrease_buttons.is_some() && ui_selector.is_previous_allowed() {
                ui_selector.select_previous();
            }
            if ui_selector_increase_buttons.is_some() && ui_selector.is_next_allowed() {
                ui_selector.select_next();
            }
        }
    }

    for (mut ui_selector, ui_selector_children) in ui_selectors.iter_mut() {
        if ui_selector.get_update_required() == false {
            continue;
        }

        for child in ui_selector_children.iter() {
            if let Ok((
                mut ui_selector_button,
                ui_selector_decrease_buttons,
                ui_selector_increase_buttons,
            )) = ui_selector_buttons.get_mut(*child)
            {
                if ui_selector_decrease_buttons.is_some() {
                    ui_selector_button.set_disabled(ui_selector.is_previous_allowed() == false);
                }
                if ui_selector_increase_buttons.is_some() {
                    ui_selector_button.set_disabled(ui_selector.is_next_allowed() == false);
                }
            }
            if let Ok(mut ui_selector_text_i18n) = ui_selector_texts.get_mut(*child) {
                ui_selector_text_i18n.change_i18n_key(
                    ui_selector
                        .get_current_item()
                        .unwrap_or(&UiSelectorItem::default())
                        .text
                        .clone(),
                );
            }
        }
        ui_selector.set_update_required(false);
    }
}
