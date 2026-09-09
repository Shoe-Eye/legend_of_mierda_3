use bevy::prelude::*;
use bevy_rapier2d::{parry::simba::scalar::SupersetOf, prelude::*};
use lom_assets::{loading::StaticSpriteTextureAtlasLayoutAssets, StaticSpriteAssets};
use lom_game::GameState;

use crate::{
    level::{
        fence::{get_sprite_index, FenceTile},
        ground::Ground,
        BuildFenceMessage, BuildTurret, Building,
    },
    player::PlayerToolUseEvent,
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
    mut mr: MessageReader<PlayerToolUseEvent>,
    mut mw_fence: MessageWriter<BuildFenceMessage>,
    mut mw_turret: MessageWriter<BuildTurret>,
    q_tool_pointer_tiles: Query<(Entity, &ToolPointerTile)>,
) {
    for message in mr.read() {
        if message.action.is_none() {
            continue;
        }

        let action = message.action.unwrap();

        if let Some((_, tool_pointer_tile)) = q_tool_pointer_tiles.iter().next() {
            if (message.tool == Tool::Hammer && action == Action::Fence) {
                mw_fence.write(BuildFenceMessage {
                    x: tool_pointer_tile.x,
                    y: tool_pointer_tile.y,
                });
            }

            if (message.tool == Tool::Hammer && action == Action::Turret) {
                mw_turret.write(BuildTurret {
                    x: tool_pointer_tile.x,
                    y: tool_pointer_tile.y,
                });
            }
        }
    }
}
