use bevy::prelude::*;
use lom_game::GameState;

use crate::{
    level::{BuildFence, BuildTurret},
    player::ToolUse,
    tools::{actions::Action, tool_pointer::ToolPointerTile, Tool},
};

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_hammer_use).run_if(in_state(GameState::GamePlay)),
        );
    }
}

pub fn handle_hammer_use(
    mut mr: MessageReader<ToolUse>,
    mut mw_fence: MessageWriter<BuildFence>,
    mut mw_turret: MessageWriter<BuildTurret>,
    q_tool_pointer_tiles: Query<(Entity, &ToolPointerTile)>,
) {
    for message in mr.read() {
        if message.action.is_none() {
            continue;
        }

        let action = message.action.unwrap();

        if let Some((_, tool_pointer_tile)) = q_tool_pointer_tiles.iter().next() {
            match message.tool {
                Tool::Hammer => match action {
                    Action::Fence { width, height } => {
                        mw_fence.write(BuildFence {
                            x: tool_pointer_tile.x,
                            y: tool_pointer_tile.y,
                        });
                    }
                    Action::Turret { width, height } => {
                        mw_turret.write(BuildTurret {
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
