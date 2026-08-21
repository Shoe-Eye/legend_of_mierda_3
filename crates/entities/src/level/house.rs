use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use lom_ldtk::physics::ColliderBundle;

pub struct HousePlugin;

impl Plugin for HousePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<HouseBundle>("House");
    }
}

#[derive(Clone, Default, Bundle)]
pub struct HouseBundle {
    pub collider_bundle: ColliderBundle,
    pub sensor: Sensor,
}

impl LdtkEntity for HouseBundle {
    fn bundle_entity(
        _entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlasses: &mut Assets<TextureAtlasLayout>,
    ) -> HouseBundle {
        println!("setting up collider bundle");

        HouseBundle {
            collider_bundle: ColliderBundle {
                collider: Collider::cuboid(16., 16.),
                rigid_body: RigidBody::Fixed,
                friction: Friction {
                    coefficient: 20.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                ..Default::default()
            },
            sensor: Sensor {},
        }
    }
}
