use bevy::prelude::*;
use lom_game::GameMode;

use crate::{
    level::PlantVegetation,
    player::ToolUse,
    tools::{actions::Action, tool_pointer::ToolPointerTile, Tool},
};

pub struct WateringCanPlugin;

impl Plugin for WateringCanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_use_watering_can).run_if(in_state(GameMode::GamePlay)),
        );
    }
}

pub fn handle_use_watering_can(
    mut mr: MessageReader<ToolUse>,
    mut mw_plant: MessageWriter<PlantVegetation>,
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
                    Action::Plant {
                        plant_type,
                        width,
                        height,
                    } => {
                        mw_plant.write(PlantVegetation {
                            x: tool_pointer_tile.x,
                            y: tool_pointer_tile.y,
                            plant_type: plant_type.clone(),
                        });
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
