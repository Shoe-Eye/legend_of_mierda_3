use bevy::prelude::*;
use lom_game::GameState;

use crate::{
    level::PlantWatermelon,
    player::PlayerToolUseEvent,
    tools::{actions::Action, tool_pointer::ToolPointerTile, Tool},
};

pub struct WateringCanPlugin;

impl Plugin for WateringCanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_use_watering_can).run_if(in_state(GameState::GamePlay)),
        );
    }
}

pub fn handle_use_watering_can(
    mut mr: MessageReader<PlayerToolUseEvent>,
    mut mw_fence: MessageWriter<PlantWatermelon>,
    q_tool_pointer_tiles: Query<(Entity, &ToolPointerTile)>,
) {
    for message in mr.read() {
        if message.action.is_none() {
            continue;
        }

        let action = message.action.unwrap();

        if let Some((_, tool_pointer_tile)) = q_tool_pointer_tiles.iter().next() {
            match message.tool {
                Tool::WateringCan => match action {
                    Action::PlantWatermelon { width, height } => {
                        mw_fence.write(PlantWatermelon {
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
