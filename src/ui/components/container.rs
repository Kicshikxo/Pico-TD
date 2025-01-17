use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    ui::widget::NodeImageMode,
};

use crate::assets::ui::UiAssets;

#[derive(Default, Clone)]
#[allow(unused)]
pub enum UiContainerVariant {
    #[default]
    Default,
    Primary,
    Success,
    Danger,
}

impl UiContainerVariant {
    pub fn as_index(&self) -> usize {
        match self {
            UiContainerVariant::Default => 3,
            UiContainerVariant::Primary => 22,
            UiContainerVariant::Success => 21,
            UiContainerVariant::Danger => 20,
        }
    }
}

#[derive(Component)]
#[component(on_add = UiContainer::on_add)]
pub struct UiContainer {
    variant: UiContainerVariant,
    width: Val,
    height: Val,
    padding: UiRect,
    align_items: AlignItems,
    justify_content: JustifyContent,
    flex_direction: FlexDirection,
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
    fn on_add(mut world: DeferredWorld, entity: Entity, _component_id: ComponentId) {
        let ui_container = world.get::<Self>(entity).unwrap();
        let ui_assets = world.get_resource::<UiAssets>().unwrap();

        let width = ui_container.width;
        let height = ui_container.height;
        let padding = ui_container.padding;
        let align_items = ui_container.align_items;
        let justify_content = ui_container.justify_content;
        let flex_direction = ui_container.flex_direction;
        let row_gap = ui_container.row_gap;
        let column_gap = ui_container.column_gap;

        let variant = ui_container.variant.clone();
        let image = ui_assets.large_tilemap.clone();
        let layout = ui_assets.large_tilemap_atlas.clone();

        world.commands().entity(entity).insert((
            Node {
                width,
                height,
                padding,
                align_items,
                justify_content,
                flex_direction,
                row_gap,
                column_gap,
                ..default()
            },
            ImageNode {
                image,
                texture_atlas: Some(TextureAtlas {
                    index: variant.as_index(),
                    layout,
                }),
                image_mode: NodeImageMode::Sliced(TextureSlicer {
                    border: BorderRect::square(10.0),
                    max_corner_scale: 2.5,
                    ..default()
                }),
                ..default()
            },
        ));
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
    pub fn with_row_gap(mut self, row_gap: Val) -> Self {
        self.row_gap = row_gap;
        self
    }
    pub fn with_column_gap(mut self, column_gap: Val) -> Self {
        self.column_gap = column_gap;
        self
    }
    pub fn center(mut self) -> Self {
        self.align_items = AlignItems::Center;
        self.justify_content = JustifyContent::Center;
        self
    }
    pub fn column(mut self) -> Self {
        self.flex_direction = FlexDirection::Column;
        self
    }
    pub fn gap(mut self, gap: Val) -> Self {
        self.row_gap = gap;
        self.column_gap = gap;
        self
    }
}
