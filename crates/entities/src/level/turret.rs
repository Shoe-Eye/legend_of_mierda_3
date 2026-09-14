use std::f32::consts::{FRAC_PI_4, PI, TAU};
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Friction};
use lom_game::GameState;

use crate::level::ground::FoundationTile;
use crate::level::{ground::Ground, GroundStruct};
use crate::level::{BuildTurret, Building};
use crate::weapons::speargun::{Speargun, SpeargunShootEvent, SpeargunTimer};

pub const TURRET_SIZE_X: u32 = 4;
pub const TURRET_SIZE_Y: u32 = 4;

#[derive(Component)]
pub struct Turret {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct TurretModel {
    pub game_entity: Entity,
    pub angle: f32,
}

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
                let mut turret_entity: Option<Entity> = None;

                commands.entity(ground_entity).with_children(
                    |parent: &mut bevy_ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                        let mut ec = parent.spawn((
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
                            Speargun,
                            SpeargunTimer(Timer::new(
                                Duration::from_secs_f32(1.0),
                                TimerMode::Repeating,
                            )),
                        ));

                        let entity = ec.id();
                        ec.with_children(|parent| {
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

                        turret_entity = Some(entity.clone());
                    },
                );

                commands.spawn((
                    Mesh3d(asset_server.load("models/turrets/simple.obj")),
                    MeshMaterial3d(materials.add(StandardMaterial { ..default() })),
                    TurretModel {
                        game_entity: turret_entity.unwrap(),
                        angle: 0.0,
                    },
                    Transform::from_xyz(
                        (message.x as f32) * 0.75 - 16.5,
                        (message.y as f32) * 0.75 - 10.5,
                        0.0,
                    )
                    .with_rotation(Quat::from_rotation_x(FRAC_PI_4)),
                ));

                break;
            }
        }
    }
}

pub fn handle_turret_rotation(
    mut turrets: Query<(&mut Transform, &mut TurretModel)>,
    timer: Res<Time>,
) {
    for (mut transform, mut model) in &mut turrets {
        let angle = 0.1 * TAU * timer.delta_secs();
        let delta = Quat::from_rotation_y(angle);
        transform.rotation *= delta;

        model.angle = (model.angle + angle + PI).rem_euclid(TAU) - PI;
    }
}

fn handle_turret_attack(
    mut q_speargun: Query<(Entity, &Speargun, &Turret, &mut SpeargunTimer)>,
    mut ev_arrow_attack: MessageWriter<SpeargunShootEvent>,
    time: Res<Time>,
) {
    for (entity, speargun, turret, mut timer) in q_speargun.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            ev_arrow_attack.write(SpeargunShootEvent {
                entity: entity.clone(),
            });
        }
    }
}

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_build_turret,
                handle_turret_rotation,
                handle_turret_attack,
            )
                .run_if(in_state(GameState::GamePlay)),
        )
        .add_message::<BuildTurret>();
    }
}
