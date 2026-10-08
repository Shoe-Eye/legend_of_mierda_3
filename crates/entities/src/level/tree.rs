use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use lom_assets::loading::MAPLE_TREE_ASSET_SHEET;
use lom_ldtk::physics::ColliderBundle;

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Tree;

// -----------
// Compontents
// -----------

#[derive(Default, Bundle, Clone)]
pub struct MapleTreeBundle {
    pub tree: Tree,
    pub collider_bundle: ColliderBundle,
    pub sprite: Sprite,
}

// ----
// LDTK
// ----

impl LdtkEntity for MapleTreeBundle {
    fn bundle_entity(
        _entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlasses: &mut Assets<TextureAtlasLayout>,
    ) -> MapleTreeBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        let collider_bundle = ColliderBundle {
            collider: Collider::cuboid(16., 24.),
            rigid_body: RigidBody::Fixed,
            friction: Friction {
                coefficient: 20.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            rotation_constraints,
            ..Default::default()
        };

        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(32, 48),
            5,
            1,
            Some(UVec2::ZERO),
            Some(UVec2::ZERO),
        );
        let layout_handle = texture_atlasses.add(layout);
        let image = asset_server.load(MAPLE_TREE_ASSET_SHEET.to_string());

        MapleTreeBundle {
            collider_bundle: collider_bundle,
            tree: Tree {},
            sprite: Sprite::from_atlas_image(
                image,
                TextureAtlas {
                    layout: layout_handle,
                    index: 3,
                },
            ),
        }
    }
}

// ------
// Plugin
// ------

pub struct MapleTreePlugin;

impl Plugin for MapleTreePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<MapleTreeBundle>("Tree");
    }
}
