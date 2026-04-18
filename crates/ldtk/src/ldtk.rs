use std::collections::{HashMap, HashSet};

use bevy::image::TextureAtlasLayout;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::physics::ColliderBundle;
use bevy::ecs::relationship::Relationship;

const ASPECT_RATIO: f32 = 1.0;
pub const LEVEL_1_IID: &str = "d53f9950-c640-11ed-8430-4942c04951ff";

// Events

#[derive(Component)]
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
    mut camera_query: Query<(&mut Camera, &mut Transform), With<Camera>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    level_query: Query<(&Transform, &LevelIid), Without<Player>>,
    level_selection: Res<LevelSelection>,
    projects: Query<&LdtkProjectHandle>,
    project_assets: Res<Assets<LdtkProject>>,
) {
    if player_query.is_empty() {
        return;
    }

    let project = project_assets.get(projects.single().unwrap().id()).unwrap();

    let player_translation = player_query.single().unwrap().translation();

    let (_camera, mut camera_transform) = camera_query.single_mut().unwrap();

    for (level_transform, level_iid) in level_query.iter() {
        if let Some(ldtk_level) = project.get_raw_level_by_iid(level_iid.get()) {
            let level = &ldtk_level;
            if level_selection.is_match(
                &LevelIndices {
                    level: 0,
                    ..default()
                },
                level,
            ) {
                let level_ratio = level.px_wid as f32 / ldtk_level.px_hei as f32;
                if level_ratio > ASPECT_RATIO {
                    // level is wider than the screen
                    let height = (level.px_hei as f32 / 9.).round() * 9.;
                    let width = height * ASPECT_RATIO;
                    camera_transform.translation.x =
                        (player_translation.x - level_transform.translation.x - width / 2.)
                            .clamp(0., level.px_wid as f32 - width);
                    camera_transform.translation.y = 0.;
                } else {
                    // level is taller than the screen
                    let mut width = (level.px_wid as f32 / 16.).round() * 16.;
                    let mut height = width / ASPECT_RATIO;

                    width *= 0.7;
                    height *= 0.7;

                    camera_transform.translation.y =
                        (player_translation.y - level_transform.translation.y - height / 2.)
                            .clamp(0., level.px_hei as f32 - height);
                    camera_transform.translation.x =
                        (player_translation.x - level_transform.translation.x - width / 2.)
                            .clamp(0., level.px_wid as f32 - width);
                }

                camera_transform.translation.x += level_transform.translation.x;
                camera_transform.translation.y += level_transform.translation.y;
            }
        }
    }
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

                // combine wall tiles into flat "plates" in each individual row
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
                        level
                            .spawn_empty()
                            .insert((
                                Collider::cuboid(
                                    (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                        * grid_size as f32
                                        / 2.,
                                    (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                        * grid_size as f32
                                        / 2.,
                                ),
                                // Sensor {},
                                ActiveEvents::COLLISION_EVENTS,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub is_dummy: bool,
    pub enemy_type: i32,
}

#[derive(Component)]
pub struct Item {
    pub is_dummy: bool,
    pub item_type: i32,
}

pub struct EnemyBundle {
    pub collider_bundle: ColliderBundle,
    pub direction_update_time: f32,
    pub animated_character_sprite: (),
    pub character_animation: (),
    pub animation_timer: (),
}

pub struct ItemBundle {
    pub collider_bundle: ColliderBundle,
}

pub fn create_enemy_bundle(
    _asset_server: &AssetServer,
    _texture_atlasses: &Assets<TextureAtlasLayout>,
    _is_dummy: bool,
    _enemy_type: i32,
) -> EnemyBundle {
    EnemyBundle {
        collider_bundle: ColliderBundle::default(),
        direction_update_time: 0.0,
        animated_character_sprite: (),
        character_animation: (),
        animation_timer: (),
    }
}

pub fn create_item_bundle(
    _asset_server: &AssetServer,
    _texture_atlasses: &Assets<TextureAtlasLayout>,
    _is_dummy: bool,
    _item_type: i32,
) -> ItemBundle {
    ItemBundle {
        collider_bundle: ColliderBundle::default(),
    }
}

#[allow(unused_variables)]
pub fn hide_dummy_entities(
    commands: Commands,
    _level_selection: Res<LevelSelection>,
    set: ParamSet<(
        Query<(Entity, &mut Visibility, &Enemy)>,
        Query<(Entity, &mut Visibility, &Item)>,
    )>,
) {
}

#[allow(unused_variables)]
pub fn fix_missing_ldtk_entities(
    asset_server: Res<AssetServer>,
    texture_atlasses: ResMut<Assets<TextureAtlasLayout>>,
    commands: Commands,
    q_enemies: Query<(Entity, &Enemy), Without<Collider>>,
    q_items: Query<(Entity, &Item), Without<Collider>>,
) {
    // Placeholder implementation - user needs to define Enemy and Item components
    // with actual enemy/item types and bundle creation logic
}

pub fn spawn_game_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle::from(asset_server.load("levels/example.ldtk")),
        ..Default::default()
    });

    commands.write_message(LevelChangeEvent { level_id: 1 });
}

pub fn despawn_game_world(mut commands: Commands, level_query: Query<(Entity, &LevelSet)>) {
    for (entity, _) in level_query.iter() {
        commands.entity(entity).despawn();
    }
}
