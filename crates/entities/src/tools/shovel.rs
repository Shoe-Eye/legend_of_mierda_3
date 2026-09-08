use bevy::prelude::*;
use lom_assets::StaticSpriteAssets;
use lom_game::GameState;

use crate::{
    level::{
        ground::{FoundationTile, Ground, GroundTile},
        BuildFoundationMessage, Building, DigGroundMessage, GroundStruct,
    },
    player::PlayerToolUseEvent,
    tools::{actions::Action, tool_pointer::ToolPointerTile, Tool},
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
    mut mw_build_foundation: MessageWriter<BuildFoundationMessage>,
    mut mw_dig_ground: MessageWriter<DigGroundMessage>,
    q_ground_structs: Query<(Entity, &GroundStruct)>,
    q_tool_pointer_tiles: Query<(Entity, &ToolPointerTile)>,
    q_buildings: Query<(Entity, &Building)>,
) {
    for message in mr.read() {
        if message.action.is_none() {
            continue;
        }

        let action = message.action.unwrap();

        if let Some((_, tool_pointer_tile)) = q_tool_pointer_tiles.iter().next() {
            let building_exists = q_buildings
                .iter()
                .filter(|(_, building)| {
                    building.x == tool_pointer_tile.x && building.y == tool_pointer_tile.y
                })
                .count()
                > 0;

            if building_exists {
                continue;
            }

            if message.tool == Tool::Shovel {
                for (entity, ground_struct) in q_ground_structs.iter() {
                    if ground_struct.x == tool_pointer_tile.x
                        && ground_struct.y == tool_pointer_tile.y
                    {
                        commands.entity(entity).despawn();
                    }
                }

                if action == Action::Dig {
                    mw_dig_ground.write(DigGroundMessage {
                        x: tool_pointer_tile.x,
                        y: tool_pointer_tile.y,
                    });
                }

                if action == Action::Foundation {
                    mw_build_foundation.write(BuildFoundationMessage {
                        x: tool_pointer_tile.x,
                        y: tool_pointer_tile.y,
                    });
                }
            }
        }
    }
}
