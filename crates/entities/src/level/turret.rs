use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Friction};
use lom_assets::StaticSpriteAssets;

use crate::level::{ground::Ground, GroundStruct};
use crate::level::{BuildTurret, Building};

#[derive(Component)]
pub struct TurretTile {
    pub x: u32,
    pub y: u32,
}

pub fn handle_build_turret(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mr_build_turret: MessageReader<BuildTurret>,
    q_ground: Query<(Entity, &Ground)>,
    q_turret_tiles: Query<(Entity, &ChildOf, &TurretTile)>,
) {
    for message in mr_build_turret.read() {
        if let Some((ground_entity, ground)) = q_ground.iter().next() {
            let turret_does_not_exist = q_turret_tiles
                .iter()
                .filter(|(_, parent, tile)| {
                    parent.parent() == ground_entity && tile.x == message.x && tile.y == message.y
                })
                .count()
                == 0;

            if turret_does_not_exist {
                commands.entity(ground_entity).with_children(
                    |parent: &mut bevy_ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                        parent
                            .spawn((
                                GroundStruct {
                                    x: message.x,
                                    y: message.y,
                                },
                                TurretTile {
                                    x: message.x,
                                    y: message.y,
                                },
                                Building {
                                    x: message.x,
                                    y: message.y,
                                    width: 5,
                                    height: 5,
                                },
                                Transform::IDENTITY,
                            ))
                            .with_children(|parent| {
                                for x in 0..5 {
                                    for y in 0..5 {
                                        parent.spawn((
                                            Transform::from_translation(Vec3::new(
                                                ((message.x + x) * ground.grid_size) as f32,
                                                ((message.y + y) * ground.grid_size) as f32,
                                                0.51,
                                            )),
                                            Name::new("turret tile"),
                                            Collider::cuboid(16., 16.),
                                            Friction::new(1.0),
                                            ActiveEvents::COLLISION_EVENTS,
                                        ));
                                    }
                                }
                            });
                    },
                );

                commands.spawn((
                    Mesh3d(asset_server.load("models/turrets/simple.obj")),
                    MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
                    Transform::from_xyz(
                        (message.x as f32) * 0.62 - 13.61,
                        (message.y as f32) * 0.621 - 8.61,
                        0.0,
                    )
                    .with_rotation(Quat::from_rotation_x(FRAC_PI_4)),
                ));
            }
        }
    }
}
