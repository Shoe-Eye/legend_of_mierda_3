use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use lom_ldtk::physics::ColliderBundle;

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Door {
    pub level_ref: String,
    pub spawn_point_ref: String,
}

// -----------
// Compontents
// -----------

#[derive(Default, Bundle, Clone)]
pub struct DoorBundle {
    pub door: Door,
    pub collider_bundle: ColliderBundle,
}

// ----
// LDTK
// ----

impl LdtkEntity for DoorBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlasses: &mut Assets<TextureAtlasLayout>,
    ) -> DoorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        let collider_bundle = ColliderBundle {
            collider: Collider::cuboid(16., 16.),
            rigid_body: RigidBody::Fixed,
            rotation_constraints,
            ..Default::default()
        };

        let level_ref = entity_instance
            .get_string_field("LevelRef")
            .expect("expected entity to have non-nullable LevelRef string field");

        let level_spawn_ref = entity_instance
            .get_string_field("SpawnPointRef")
            .expect("expected entity to have non-nullable SpawnPointRef string field");

        DoorBundle {
            collider_bundle: collider_bundle,
            door: Door {
                level_ref: level_ref.clone(),
                spawn_point_ref: level_spawn_ref.clone(),
            },
        }
    }
}

pub fn handle_door_entry() {}

// ------
// Plugin
// ------

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<DoorBundle>("Door");
    }
}
