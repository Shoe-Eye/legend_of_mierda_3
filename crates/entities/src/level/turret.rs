use std::f32::consts::{FRAC_PI_4, TAU};

use bevy::prelude::*;
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Friction};

use crate::level::ground::FoundationTile;
use crate::level::{ground::Ground, GroundStruct};
use crate::level::{BuildTurret, Building};

pub const TURRET_SIZE_X: u32 = 4;
pub const TURRET_SIZE_Y: u32 = 4;

#[derive(Component)]
pub struct Turret {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct TurretModel;

pub fn handle_build_turret(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mr_build_turret: MessageReader<BuildTurret>,
    q_ground: Query<(Entity, &Ground)>,
    q_turret_tiles: Query<(Entity, &ChildOf, &Turret)>,
    q_foundation_tiles: Query<(Entity, &FoundationTile)>,
    // q_buildings
) {
    for message in mr_build_turret.read() {
        if let Some((ground_entity, ground)) = q_ground.iter().next() {
            let turret_does_not_exist = q_turret_tiles
                .iter()
                .filter(|(_, parent, tile)| {
                    parent.parent() == ground_entity
                        && (tile.x == message.x && tile.y == message.y)
                        && (message.x > tile.x
                            && message.x < tile.x + TURRET_SIZE_X
                            && message.y > tile.y
                            && message.y < tile.y + TURRET_SIZE_Y)
                })
                .count()
                == 0;

            let foundation_tiles: Vec<FoundationTile> = q_foundation_tiles
                .iter()
                .map(|(_, foundation)| foundation.clone())
                .collect();
            let mut n_foundation_tiles = 0;
            for x in 0..TURRET_SIZE_X {
                for y in 0..TURRET_SIZE_Y {
                    let ground_tile_exist = foundation_tiles
                        .iter()
                        .filter(|ft| ft.x == x + message.x && ft.y == y + message.y)
                        .count()
                        != 0;

                    if ground_tile_exist {
                        n_foundation_tiles += 1;
                    }
                }
            }

            if turret_does_not_exist && n_foundation_tiles == TURRET_SIZE_X * TURRET_SIZE_Y {
                commands.entity(ground_entity).with_children(
                    |parent: &mut bevy_ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                        parent
                            .spawn((
                                GroundStruct {
                                    x: message.x,
                                    y: message.y,
                                },
                                Turret {
                                    x: message.x,
                                    y: message.y,
                                },
                                Building {
                                    x: message.x,
                                    y: message.y,
                                    width: TURRET_SIZE_X,
                                    height: TURRET_SIZE_Y,
                                },
                                Name::new("turret tile"),
                                Transform::IDENTITY,
                            ))
                            .with_children(|parent| {
                                for x in 0..TURRET_SIZE_X {
                                    for y in 0..TURRET_SIZE_Y {
                                        parent.spawn((
                                            Transform::from_translation(Vec3::new(
                                                ((message.x + x) * ground.grid_size) as f32,
                                                ((message.y + y) * ground.grid_size) as f32,
                                                0.51,
                                            )),
                                            Collider::cuboid(16., 16.),
                                            Friction::new(1.0),
                                            ActiveEvents::COLLISION_EVENTS,
                                            Name::new("turret tile collider"),
                                        ));
                                    }
                                }
                            });
                    },
                );

                commands.spawn((
                    Mesh3d(asset_server.load("models/turrets/simple.obj")),
                    MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
                    TurretModel,
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

pub fn handle_turret_rotation(
    mut turrets: Query<(&mut Transform, &TurretModel)>,
    timer: Res<Time>,
) {
    for (mut transform, _) in &mut turrets {
        transform.rotation *= Quat::from_rotation_y(0.1 * TAU * timer.delta_secs());
    }
}
