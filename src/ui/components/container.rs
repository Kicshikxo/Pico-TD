use bevy::{prelude::*, ui::widget::NodeImageMode};

use crate::assets::sprites::ui::UiAssets;

#[derive(Default, Clone, PartialEq)]
#[allow(unused)]
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
            UiContainerVariant::None => 0,
            UiContainerVariant::Primary => 22,
            UiContainerVariant::Secondary => 3,
            UiContainerVariant::Success => 21,
            UiContainerVariant::Danger => 20,
        }
    }
}
#[derive(Component)]
#[require(Node)]
pub struct UiContainer {
    variant: UiContainerVariant,
    width: Val,
    height: Val,
    padding: UiRect,
    align_items: AlignItems,
    justify_content: JustifyContent,
    flex_direction: FlexDirection,
    aspect_ratio: Option<f32>,
    row_gap: Val,
    column_gap: Val,
}

impl Default for UiContainer {
    fn default() -> Self {
        Self {
            variant: UiContainerVariant::default(),
            width: Val::Percent(100.0),
            height: Val::Auto,
            padding: UiRect::all(Val::ZERO),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Row,
            aspect_ratio: None,
            row_gap: Val::ZERO,
            column_gap: Val::ZERO,
        }
    }
}

#[allow(unused)]
impl UiContainer {
    pub fn new() -> Self {
        Self { ..default() }
    }
    pub fn with_variant(mut self, variant: UiContainerVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    pub fn with_height(mut self, height: Val) -> Self {
        self.height = height;
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
    pub fn center(self) -> Self {
        self.with_align_items(AlignItems::Center)
            .with_justify_content(JustifyContent::Center)
    }
    pub fn column(mut self) -> Self {
        self.with_flex_direction(FlexDirection::Column)
    }
    pub fn gap(mut self, gap: Val) -> Self {
        self.with_row_gap(gap).with_column_gap(gap)
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
            width: ui_container.width,
            height: ui_container.height,
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
                image: ui_assets.large_tilemap.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: ui_container.variant.as_index(),
                    layout: ui_assets.large_tilemap_atlas.clone(),
                }),
                image_mode: NodeImageMode::Sliced(TextureSlicer {
                    border: BorderRect::square(10.0),
                    max_corner_scale: 2.5,
                    ..default()
                }),
                ..default()
            });
        }
    }
}
