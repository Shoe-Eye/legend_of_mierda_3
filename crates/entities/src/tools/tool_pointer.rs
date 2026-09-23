use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::{assets::LdtkProject, LdtkProjectHandle, LevelEvent, LevelIid};
use lom_assets::sprites::CharacterAnimation;
use lom_game::GameMode;

use crate::controls::ControlEvent;
use crate::level::Building;
use crate::player::Player;

#[derive(Component)]
pub struct ToolPointerLayer {
    pub level_iid: String,
    pub width: u32,
    pub height: u32,
    pub grid_size: u32,

    pub pointer_size_x: u32,
    pub pointer_size_y: u32,

    pub enabled: bool,
}

#[derive(Default, Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    #[default]
    Up,
    Down,
}

#[derive(Component, Default)]
pub struct ToolPointer;

#[derive(Component, Default)]
pub struct ToolPointerTile {
    pub x: u32,
    pub y: u32,
    direction: Direction,
}

pub fn init_tool_pointer_layer(
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
                                        Name::new("ToolPointer"),
                                        InheritedVisibility::default(),
                                        Transform::from_translation(Vec3::new(8.0, 8.0, 1.0)),
                                        ToolPointerLayer {
                                            level_iid: level_id.as_str().to_string(),
                                            width,
                                            height,
                                            grid_size,
                                            pointer_size_x: 1,
                                            pointer_size_y: 1,
                                            enabled: true,
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

pub fn draw_tool_pointer(
    mut commands: Commands,
    q_player: Query<(Entity, &Transform, &CharacterAnimation, &Player)>,
    mut q_tool_pointer_layer: Query<(Entity, &mut ToolPointerLayer)>,
    q_tool_pointer_tiles: Query<(Entity, &ChildOf, &ToolPointerTile)>,
    q_buildings: Query<(&Building)>,
) {
    let mut direction = Direction::default();

    for (entity, _, tile) in q_tool_pointer_tiles.iter() {
        direction = tile.direction;
        commands.entity(entity).despawn();
    }

    for (_, player_transform, _character_animation, _player) in q_player.iter() {
        if let Some((tool_pointer_entity, mut tool_pointer_layer)) =
            q_tool_pointer_layer.iter_mut().next()
        {
            let mut x =
                f32::floor(player_transform.translation.x / (tool_pointer_layer.grid_size as f32))
                    as u32;
            let mut y =
                f32::floor(player_transform.translation.y / (tool_pointer_layer.grid_size as f32))
                    as u32;

            match direction {
                Direction::Left => {
                    if x >= tool_pointer_layer.pointer_size_x {
                        x -= tool_pointer_layer.pointer_size_x;
                    }

                    if y >= tool_pointer_layer.pointer_size_y / 2 {
                        y -= tool_pointer_layer.pointer_size_y / 2;
                    }
                }
                Direction::Right => {
                    x += 1;
                    if y >= tool_pointer_layer.pointer_size_y / 2 {
                        y -= tool_pointer_layer.pointer_size_y / 2;
                    }
                }
                Direction::Up => {
                    y += 1;
                    if x >= tool_pointer_layer.pointer_size_x / 2 {
                        x -= tool_pointer_layer.pointer_size_x / 2;
                    }
                }
                Direction::Down => {
                    if y >= tool_pointer_layer.pointer_size_y {
                        y -= tool_pointer_layer.pointer_size_y;
                    }

                    if x >= tool_pointer_layer.pointer_size_x / 2 {
                        x -= tool_pointer_layer.pointer_size_x / 2;
                    }
                }
            }

            let buildings: Vec<Building> = q_buildings.iter().map(|e| e.clone()).collect();
            let building_blocks = |x: u32, y: u32| -> bool {
                return buildings
                    .iter()
                    .filter(|b| {
                        (x >= b.x && x < (b.x + b.width)) && (y >= b.y && y < (b.y + b.height))
                    })
                    .count()
                    != 0;
            };

            tool_pointer_layer.enabled = true;
            commands.entity(tool_pointer_entity).with_children(
                |parent: &mut bevy_ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                    for delta_x in 0..tool_pointer_layer.pointer_size_x {
                        for delta_y in 0..tool_pointer_layer.pointer_size_y {
                            if building_blocks(x + delta_x, y + delta_y) {
                                tool_pointer_layer.enabled = false;
                            }
                        }
                    }

                    for delta_x in 0..tool_pointer_layer.pointer_size_x {
                        for delta_y in 0..tool_pointer_layer.pointer_size_y {
                            parent.spawn((
                                Sprite::from_color(
                                    match tool_pointer_layer.enabled {
                                        false => Color::srgba(1.0, 0.0, 0.0, 0.5),
                                        true => Color::srgba(0.0, 1.0, 0.0, 0.5),
                                    },
                                    Vec2::new(16.0, 16.0),
                                ),
                                Transform::from_translation(Vec3::new(
                                    ((x + delta_x) * tool_pointer_layer.grid_size) as f32,
                                    ((y + delta_y) * tool_pointer_layer.grid_size) as f32,
                                    0.6,
                                )),
                                Name::new("ground tile"),
                                ToolPointerTile { x, y, direction },
                            ));
                        }
                    }
                },
            );
        }
    }
}

pub fn control_pointer(
    mut ev_control: MessageReader<ControlEvent>,
    mut q_tool_pointer_tiles: Query<(Entity, &mut ToolPointerTile)>,
) {
    for control in ev_control.read() {
        for (_, mut tile) in q_tool_pointer_tiles.iter_mut() {
            if control.arrow_down {
                tile.direction = Direction::Down;
            }
            if control.arrow_up {
                tile.direction = Direction::Up;
            }
            if control.arrow_right {
                tile.direction = Direction::Right;
            }
            if control.arrow_left {
                tile.direction = Direction::Left;
            }
        }
    }
}

pub struct ToolPointerPlugin;

impl Plugin for ToolPointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_tool_pointer_layer, draw_tool_pointer, control_pointer)
                .run_if(in_state(GameMode::GamePlay)),
        );
    }
}
