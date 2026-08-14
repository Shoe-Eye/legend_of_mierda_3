use std::cmp::min;

use super::item::{create_item_bundle, Item, ItemStepOverEvent, ItemType};
use crate::{physics::ColliderBundle, player::Player};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use lom_assets::{load_texture_atlas, sprites::PIZZA_ASSET_SHEET};

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Pizza {
    pub is_dummy: bool,
}

#[derive(Clone, Default, Bundle)]
pub struct PizzaBundle {
    pub item: Item,
    pub collider_bundle: ColliderBundle,
    pub sensor: Sensor,
    pub sprite: Sprite,
}

impl LdtkEntity for PizzaBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlasses: &mut Assets<TextureAtlasLayout>,
    ) -> PizzaBundle {
        let is_dummy = *entity_instance
            .get_bool_field("is_dummy")
            .expect("expected entity to have non-nullable name string field");
        let bundle = create_item_bundle(asset_server, texture_atlasses, is_dummy, ItemType::Pizza);

        let layout = load_texture_atlas(
            PIZZA_ASSET_SHEET.to_string(),
            asset_server,
            1,
            1,
            None,
            Vec2::ONE * 16.,
            texture_atlasses,
        );
        let image = asset_server.load(PIZZA_ASSET_SHEET.clone());

        PizzaBundle {
            collider_bundle: bundle.collider_bundle,
            item: bundle.item,
            sensor: bundle.sensor,
            sprite: Sprite::from_atlas_image(
                image,
                TextureAtlas {
                    layout: layout,
                    index: 0,
                },
            ),
        }
    }
}

// --------------
// Event Handlers
// --------------

pub fn event_on_pizza_step_over(
    mut commands: Commands,
    mut er_item_step_over: MessageReader<ItemStepOverEvent>,
    mut q_items: Query<(Entity, &Item)>,
    mut q_player: Query<(Entity, &mut Player)>,
) {
    for e in er_item_step_over.read() {
        if e.item_type != ItemType::Pizza {
            continue;
        }
        for (_, mut player) in q_player.iter_mut() {
            player.health = min(player.health + 10, 100);
        }

        for (e_item, _) in q_items
            .iter_mut()
            .filter(|(_, i)| i.item_type == ItemType::Pizza)
        {
            if e_item != e.entity {
                continue;
            }
            commands.entity(e_item).despawn();
        }
    }
}

// ------
// Plugin
// ------

pub struct PizzaPlugin;

impl Plugin for PizzaPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PizzaBundle>("Pizza")
            .add_systems(Update, event_on_pizza_step_over);
    }
}
