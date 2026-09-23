use bevy::prelude::*;
use lom_game::GameMode;

use crate::{
    level::HarvestPlant,
    player::ToolUse,
    tools::{actions::Action, tool_pointer::ToolPointerTile, Tool},
};

pub struct NoToolPlugin;

impl Plugin for NoToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_no_tool_use).run_if(in_state(GameMode::GamePlay)),
        );
    }
}

pub fn handle_no_tool_use(
    mut mr: MessageReader<ToolUse>,
    mut mw_harvest_plant: MessageWriter<HarvestPlant>,
    q_tool_pointer_tiles: Query<(Entity, &ToolPointerTile)>,
) {
    for message in mr.read() {
        if message.action.is_none() {
            continue;
        }

        let action = message.action.unwrap();

        if let Some((_, tool_pointer_tile)) = q_tool_pointer_tiles.iter().next() {
            match message.tool {
                Tool::NoTool => match action {
                    Action::Harvest => {
                        mw_harvest_plant.write(HarvestPlant {
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
