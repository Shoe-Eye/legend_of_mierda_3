use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::{assets::LdtkProject, LdtkProjectHandle, LevelEvent, LevelIid};
use lom_game::GameState;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_ground_layer)
                .chain()
                .run_if(in_state(GameState::GamePlay)),
        );
    }
}

#[derive(Component)]
pub struct Ground {
    pub level_iid: String,
    pub width: u32,
    pub height: u32,
    pub grid_size: u32,
}

#[derive(Component)]
pub struct GroundTile {
    pub x: u32,
    pub y: u32,
}

pub fn init_ground_layer(
    mut commands: Commands,
    q_levels: Query<(Entity, &LevelIid)>,
    mut ev_level_event: MessageReader<LevelEvent>,
    projects: Query<&LdtkProjectHandle>,
    project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in ev_level_event.read() {
        if let LevelEvent::Spawned(spawned_level_id) = level_event {
            for (level_entity, level_id) in q_levels.iter() {
                if level_id.as_str() == spawned_level_id.as_str() {
                    commands.entity(level_entity).with_children(|parent| {
                        let project = project_assets.get(projects.single().unwrap().id()).unwrap();
                        if let Some(ldtk_level) =
                            project.get_raw_level_by_iid(&level_id.as_str().to_string())
                        {
                            if let Some(layer_instances) = ldtk_level.layer_instances.clone() {
                                if let Some(layer) = layer_instances.iter().next() {
                                    let width = layer.c_wid as u32;
                                    let height = layer.c_hei as u32;
                                    let grid_size = layer.grid_size as u32;

                                    parent.spawn((
                                        Name::new("Ground"),
                                        InheritedVisibility::default(),
                                        Transform::from_translation(Vec3::new(8.0, 8.0, 0.5)),
                                        Ground {
                                            level_iid: level_id.as_str().to_string(),
                                            width,
                                            height,
                                            grid_size,
                                        },
                                    ));
                                }
                            }
                        }
                    });
                }
            }
        }
    }
}
