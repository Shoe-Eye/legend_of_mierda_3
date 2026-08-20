use bevy::prelude::*;
use lom_game::GameState;

use crate::player::{Player, PlayerToolUseEvent};

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
    mut q_player: Query<(Entity, &Transform, &Player)>,
    mut mr: MessageReader<PlayerToolUseEvent>,
) {
    for message in mr.read() {
        if message.tool != super::ui::Tool::Shovel {
            continue;
        }

        if let Some((player_entity, player_transform, _)) = q_player.iter().next() {
            println!("player_transform {}", player_transform.translation);
        }
    }
}
