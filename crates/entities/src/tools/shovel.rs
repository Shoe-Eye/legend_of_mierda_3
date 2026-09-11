use bevy::prelude::*;
use lom_game::GameState;

use crate::{
    level::{BuildFoundationMessage, BuildTrailMessage, Building, DigGroundMessage},
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
    mut mr: MessageReader<PlayerToolUseEvent>,
    mut mw_build_foundation: MessageWriter<BuildFoundationMessage>,
    mut mw_dig_ground: MessageWriter<DigGroundMessage>,
    mut mw_build_trail: MessageWriter<BuildTrailMessage>,
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

            match message.tool {
                Tool::Shovel => match action {
                    Action::Dig { width, height } => {
                        mw_dig_ground.write(DigGroundMessage {
                            x: tool_pointer_tile.x,
                            y: tool_pointer_tile.y,
                        });
                    }
                    Action::Foundation { width, height } => {
                        mw_build_foundation.write(BuildFoundationMessage {
                            x: tool_pointer_tile.x,
                            y: tool_pointer_tile.y,
                        });
                    }
                    Action::Trail { width, height } => {
                        mw_build_trail.write(BuildTrailMessage {
                            x: tool_pointer_tile.x,
                            y: tool_pointer_tile.y,
                        });
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
