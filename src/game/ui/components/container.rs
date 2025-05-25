use bevy::{prelude::*, ui::widget::NodeImageMode};

use crate::game::assets::images::ui::{UiAssets, UiContainerSpriteVariant};

#[derive(Default, Clone, PartialEq)]
pub enum UiContainerVariant {
    #[default]
    None,
    Primary,
    Secondary,
    Success,
    Danger,
}

impl UiContainerVariant {
    pub fn as_index(&self) -> usize {
        match self {
            UiContainerVariant::None => unreachable!(),
            UiContainerVariant::Primary => UiContainerSpriteVariant::Primary as usize,
            UiContainerVariant::Secondary => UiContainerSpriteVariant::Secondary as usize,
            UiContainerVariant::Success => UiContainerSpriteVariant::Success as usize,
            UiContainerVariant::Danger => UiContainerSpriteVariant::Danger as usize,
        }
    }
}

#[derive(Component)]
#[require(Node)]
pub struct UiContainer {
    variant: UiContainerVariant,
    display: Display,
    position_type: PositionType,
    position: UiRect,
    width: Val,
    min_width: Val,
    max_width: Val,
    height: Val,
    min_height: Val,
    max_height: Val,
    padding: UiRect,
    align_items: AlignItems,
    justify_content: JustifyContent,
    flex_direction: FlexDirection,
    aspect_ratio: Option<f32>,
    row_gap: Val,
    column_gap: Val,
    max_corner_scale: f32,
}

impl Default for UiContainer {
    fn default() -> Self {
        Self {
            variant: UiContainerVariant::default(),
            display: Display::Flex,
            position_type: PositionType::Relative,
            position: UiRect::all(Val::Auto),
            width: Val::Percent(100.0),
            min_width: Val::Auto,
            max_width: Val::Auto,
            height: Val::Auto,
            min_height: Val::Auto,
            max_height: Val::Auto,
            padding: UiRect::all(Val::ZERO),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Row,
            aspect_ratio: None,
            row_gap: Val::ZERO,
            column_gap: Val::ZERO,
            max_corner_scale: 4.0,
        }
    }
}

#[allow(unused)]
impl UiContainer {
    pub fn new() -> Self {
        Self { ..default() }
    }
    pub fn primary() -> Self {
        Self::new().with_variant(UiContainerVariant::Primary)
    }
    pub fn secondary() -> Self {
        Self::new().with_variant(UiContainerVariant::Secondary)
    }
    pub fn success() -> Self {
        Self::new().with_variant(UiContainerVariant::Success)
    }
    pub fn danger() -> Self {
        Self::new().with_variant(UiContainerVariant::Danger)
    }
    pub fn with_variant(mut self, variant: UiContainerVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn with_display(mut self, display: Display) -> Self {
        self.display = display;
        self
    }
    pub fn with_position_type(mut self, position_type: PositionType) -> Self {
        self.position_type = position_type;
        self
    }
    pub fn with_position(mut self, position: UiRect) -> Self {
        self.position = position;
        self
    }
    pub fn with_left(mut self, left: Val) -> Self {
        self.position.left = left;
        self
    }
    pub fn with_right(mut self, right: Val) -> Self {
        self.position.right = right;
        self
    }
    pub fn with_top(mut self, top: Val) -> Self {
        self.position.top = top;
        self
    }
    pub fn with_bottom(mut self, bottom: Val) -> Self {
        self.position.bottom = bottom;
        self
    }
    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    pub fn with_min_width(mut self, min_width: Val) -> Self {
        self.min_width = min_width;
        self
    }
    pub fn with_max_width(mut self, max_width: Val) -> Self {
        self.max_width = max_width;
        self
    }
    pub fn with_height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }
    pub fn with_min_height(mut self, min_height: Val) -> Self {
        self.min_height = min_height;
        self
    }
    pub fn with_max_height(mut self, max_height: Val) -> Self {
        self.max_height = max_height;
        self
    }
    pub fn with_padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }
    pub fn with_align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = align_items;
        self
    }
    pub fn with_justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }
    pub fn with_flex_direction(mut self, flex_direction: FlexDirection) -> Self {
        self.flex_direction = flex_direction;
        self
    }
    pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }
    pub fn with_row_gap(mut self, row_gap: Val) -> Self {
        self.row_gap = row_gap;
        self
    }
    pub fn with_column_gap(mut self, column_gap: Val) -> Self {
        self.column_gap = column_gap;
        self
    }
    pub fn with_gap(mut self, gap: Val) -> Self {
        self.with_row_gap(gap).with_column_gap(gap)
    }
    pub fn with_max_corner_scale(mut self, max_corner_scale: f32) -> Self {
        self.max_corner_scale = max_corner_scale;
        self
    }
    pub fn grid(mut self) -> Self {
        self.with_display(Display::Grid)
    }
    pub fn absolute(self) -> Self {
        self.with_position_type(PositionType::Absolute)
    }
    pub fn auto_width(self) -> Self {
        self.with_width(Val::Auto)
    }
    pub fn full(self) -> Self {
        self.with_width(Val::Percent(100.0))
            .with_height(Val::Percent(100.0))
    }
    pub fn center(self) -> Self {
        self.with_align_items(AlignItems::Center)
            .with_justify_content(JustifyContent::Center)
    }
    pub fn column(mut self) -> Self {
        self.with_flex_direction(FlexDirection::Column)
    }
}

pub struct UiContainerPlugin;

impl Plugin for UiContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_ui_container);
    }
}

fn init_ui_container(
    mut commands: Commands,
    ui_containers: Query<(Entity, &UiContainer), Added<UiContainer>>,
    ui_assets: Option<Res<UiAssets>>,
) {
    for (ui_container_entity, ui_container) in ui_containers.iter() {
        let Some(ui_assets) = &ui_assets else {
            return;
        };

        commands.entity(ui_container_entity).insert(Node {
            display: ui_container.display,
            position_type: ui_container.position_type,
            left: ui_container.position.left,
            right: ui_container.position.right,
            top: ui_container.position.top,
            bottom: ui_container.position.bottom,
            width: ui_container.width,
            min_width: ui_container.min_width,
            max_width: ui_container.max_width,
            height: ui_container.height,
            min_height: ui_container.min_height,
            max_height: ui_container.max_height,
            padding: ui_container.padding,
            align_items: ui_container.align_items,
            justify_content: ui_container.justify_content,
            flex_direction: ui_container.flex_direction,
            aspect_ratio: ui_container.aspect_ratio,
            row_gap: ui_container.row_gap,
            column_gap: ui_container.column_gap,
            ..default()
        });

        if ui_container.variant != UiContainerVariant::None {
            commands.entity(ui_container_entity).insert(ImageNode {
                image: ui_assets.ui_containers.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: ui_container.variant.as_index(),
                    layout: ui_assets.ui_containers_layout.clone(),
                }),
                image_mode: NodeImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(8.0),
                    max_corner_scale: ui_container.max_corner_scale,
                    ..default()
                }),
                ..default()
            });
        }
    }
}
