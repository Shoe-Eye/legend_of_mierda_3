use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use lom_game::GameMode;
use lom_ldtk::ldtk::update_level_selection;
use lom_ldtk::physics::ColliderBundle;

use crate::player::Player;

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

// -------
// Physics
// -------

fn spawn_point_translation(
    project: &LdtkProject,
    level_iid: &str,
    spawn_iid: &str,
) -> Option<Vec3> {
    let level = project.get_raw_level_by_iid(&level_iid.to_string())?;

    let entity = level
        .layer_instances
        .as_ref()?
        .iter()
        .flat_map(|layer| &layer.entity_instances)
        .find(|entity| entity.iid == spawn_iid)?;

    let (world_x, world_y) = match (entity.world_x, entity.world_y) {
        (Some(world_x), Some(world_y)) => (world_x, world_y),
        _ => (level.world_x + entity.px.x, level.world_y + entity.px.y),
    };

    Some(Vec3::new(world_x as f32, -(world_y as f32), 10.))
}

pub fn handle_door_entry(
    mut collision_events: MessageReader<CollisionEvent>,
    q_doors: Query<&Door>,
    mut q_player: Query<&mut Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    projects: Query<&LdtkProjectHandle>,
    project_assets: Res<Assets<LdtkProject>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let contact_1_door = q_doors.get(*e1);
            let contact_2_door = q_doors.get(*e2);
            let is_contact_door = contact_1_door.is_ok() || contact_2_door.is_ok();

            let is_contact_1_player = q_player.get(*e1).is_ok();
            let is_contact_2_player = q_player.get(*e2).is_ok();
            let is_contact_player = is_contact_1_player || is_contact_2_player;

            if is_contact_player && is_contact_door {
                let door = match contact_1_door {
                    Ok(door) => door,
                    Err(_) => contact_2_door.unwrap(),
                };

                let player_entity = if is_contact_1_player { *e1 } else { *e2 };
                let mut player_transform = q_player.get_mut(player_entity).unwrap();

                let project = project_assets.get(projects.single().unwrap().id()).unwrap();

                let Some(spawn_translation) =
                    spawn_point_translation(project, &door.level_ref, &door.spawn_point_ref)
                else {
                    warn!(
                        "spawn point {} not found in level {}",
                        door.spawn_point_ref, door.level_ref
                    );
                    continue;
                };

                player_transform.translation = spawn_translation;
                *level_selection = LevelSelection::Iid(LevelIid::new(door.level_ref.clone()));
            }
        }
    }
}

// ------
// Plugin
// ------

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<DoorBundle>("Door").add_systems(
            Update,
            handle_door_entry
                .run_if(in_state(GameMode::GamePlay))
                .after(update_level_selection),
        );
    }
}
