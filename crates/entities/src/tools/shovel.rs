use bevy::prelude::*;
use lom_assets::StaticSpriteAssets;
use lom_game::GameState;

use crate::{
    level::ground::{Ground, GroundTile},
    player::PlayerToolUseEvent,
    tools::{tool_pointer::ToolPointerTile, Tool},
};

pub struct ShovelPlugin;

impl Plugin for ShovelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_shovel_use).run_if(in_state(GameState::GamePlay)),
        );
    }
}

pub fn handle_shovel_use(
    mut commands: Commands,
    mut mr: MessageReader<PlayerToolUseEvent>,
    q_ground: Query<(Entity, &Ground)>,
    q_ground_tiles: Query<(Entity, &ChildOf, &GroundTile)>,
    q_tool_pointer_tiles: Query<(Entity, &ToolPointerTile)>,
    static_sprite_assets: Res<StaticSpriteAssets>,
) {
    for message in mr.read() {
        if message.tool != Tool::Shovel {
            continue;
        }

        if let Some((_, tool_pointer_tile)) = q_tool_pointer_tiles.iter().next() {
            if let Some((ground_entity, ground)) = q_ground.iter().next() {
                if q_ground_tiles
                    .iter()
                    .filter(|(_, parent, tile)| {
                        parent.parent() == ground_entity
                            && tile.x == tool_pointer_tile.x
                            && tile.y == tool_pointer_tile.y
                    })
                    .count()
                    == 0
                {
                    commands.entity(ground_entity).with_children(
                        |parent: &mut bevy_ecs::relationship::RelatedSpawnerCommands<
                            '_,
                            ChildOf,
                        >| {
                            parent.spawn((
                                Sprite::from_image(static_sprite_assets.earth_1.clone()),
                                Transform::from_translation(Vec3::new(
                                    (tool_pointer_tile.x * ground.grid_size) as f32,
                                    (tool_pointer_tile.y * ground.grid_size) as f32,
                                    0.51,
                                )),
                                Name::new("ground tile"),
                                GroundTile {
                                    x: tool_pointer_tile.x,
                                    y: tool_pointer_tile.y,
                                },
                            ));
                        },
                    );
                }
            }
        }
    }
}
