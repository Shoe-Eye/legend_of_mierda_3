use std::collections::{HashMap, HashSet};

use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub const LEVEL_1_IID: &str = "d53f9950-c640-11ed-8430-4942c04951ff";
pub const LIBRARY_IID: &str = "bd25c9f0-bde0-11f1-b8c4-c3a3469c37aa";

// Events

#[derive(Component, Default)]
pub struct Player;

#[derive(Message, Clone)]
pub struct LevelChangeEvent {
    #[allow(dead_code)]
    pub(crate) level_id: usize,
}

impl LevelChangeEvent {
    pub fn new(level_id: usize) -> Self {
        Self { level_id }
    }

    pub fn level_id(&self) -> usize {
        self.level_id
    }
}

// Entities

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
    sensor: Sensor,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PortalWallBundle {
    wall: Wall,
    sensor: Sensor,
}

pub fn update_level_selection(
    mut commands: Commands,
    level_query: Query<(&LevelIid, &Transform), Without<Player>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    projects: Query<&LdtkProjectHandle>,
    project_assets: Res<Assets<LdtkProject>>,
) {
    for (level_iid, level_transform) in level_query.iter() {
        let project = project_assets.get(projects.single().unwrap().id()).unwrap();

        if let Some(ldtk_level) = project.get_raw_level_by_iid(level_iid.get()) {
            let level_bounds = Rect {
                min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
                max: Vec2::new(
                    level_transform.translation.x + ldtk_level.px_wid as f32,
                    level_transform.translation.y + ldtk_level.px_hei as f32,
                ),
            };

            for player_transform in &player_query {
                let player_within_x_bounds = player_transform.translation().x < level_bounds.max.x
                    && player_transform.translation().x > level_bounds.min.x;

                let player_within_y_bounds = player_transform.translation().y < level_bounds.max.y
                    && player_transform.translation().y > level_bounds.min.y;

                if player_within_x_bounds && player_within_y_bounds {
                    let new_level = LevelSelection::Iid(LevelIid::new(ldtk_level.iid.clone()));
                    if *level_selection != new_level {
                        *level_selection = new_level;

                        let level_id = ldtk_level.get_int_field("LevelID");
                        if level_id.is_ok() {
                            let level_id = *level_id.unwrap() as usize;
                            commands.write_message(LevelChangeEvent { level_id });
                        }
                    }
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut params: ParamSet<(
        Query<(&mut bevy::camera::Projection, &mut Camera, &mut Transform), With<Camera2d>>,
        Query<&GlobalTransform, With<Player>>,
        Query<(&Transform, &LevelIid), Without<Player>>,
    )>,
    level_selection: Res<LevelSelection>,
    projects: Query<&LdtkProjectHandle>,
    project_assets: Res<Assets<LdtkProject>>,
) -> Result {
    if params.p1().is_empty() {
        return Ok(());
        // return Err(BevyError::from("player not found"));
    }

    let player_translation = params.p1().single().unwrap().translation();

    let _project = project_assets.get(projects.single().unwrap().id()).unwrap();

    // Collect level data first so we hold no borr.2ow on params
    let level_data: Vec<(Transform, LevelIid)> = params
        .p2()
        .iter()
        .map(|(t, iid)| (*t, iid.clone()))
        .collect();

    let mut camera_query = params.p0();

    let (mut projection, _camera, mut camera_transform) = camera_query.single_mut().unwrap();

    let Projection::Orthographic(_orthographic_projection) = &mut *projection else {
        return Err(BevyError::from("non-orthographic projection found"));
    };

    for (_level_transform, level_iid) in &level_data {
        let ldtk_project = project_assets
            .get(projects.single()?)
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        if level_selection.is_match(&LevelIndices::default(), level) {
            camera_transform.translation.x = player_translation.x;
            camera_transform.translation.y = player_translation.y;
        }
    }

    Ok(())
}

pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &ChildOf), Added<Wall>>,
    parent_query: Query<&ChildOf, Without<Wall>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    // let project = project_assets.get(projects.single()).unwrap();

    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    for (&grid_coords, parent) in wall_query.iter() {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
            commands.entity(parent.get()).insert(Visibility::Hidden);
        }
    }

    if !wall_query.is_empty() {
        for (level_entity, level_iid) in level_query.iter() {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single().unwrap().id())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                // combine wall tiles into flat "plates" in each indWallividual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    for wall_rect in wall_rects {
                        level.spawn_empty().insert((
                            Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ),
                            ActiveEvents::COLLISION_EVENTS,
                            RigidBody::Fixed,
                            Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ),
                            Name::new("Wall Collision"),
                            GlobalTransform::default(),
                        ));
                    }
                });
            }
        }
    }
}

pub fn spawn_game_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle::from(asset_server.load("levels/main.ldtk")),
        ..Default::default()
    });

    commands.write_message(LevelChangeEvent { level_id: 1 });
}

pub fn despawn_game_world(mut commands: Commands, level_query: Query<(Entity, &LevelSet)>) {
    for (entity, _) in level_query.iter() {
        commands.entity(entity).despawn();
    }
}
