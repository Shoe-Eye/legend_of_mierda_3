use bevy::prelude::*;
use lom_dialogue::ActiveDialogue;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;

use crate::npc::{NPC, NPCControlMessage};

#[derive(Component, Deref, DerefMut, Clone, Default, Reflect)]
pub struct BehaviourTimer(pub Timer);

pub fn npc_ai(
    mut q_npc: Query<(Entity, &mut NPC, &mut BehaviourTimer)>,
    time: Res<Time>,
    active_dialogue: Res<ActiveDialogue>,
    mut mw_control_npc: MessageWriter<NPCControlMessage>,
) {
    for (entity, npc, mut timer) in q_npc.iter_mut() {
        timer.tick(time.delta());

        if !npc.is_ai_driven || active_dialogue.npc == Some(entity) {
            continue;
        }

        if timer.just_finished() {
            let npc_behaviour_options = vec![
                NPCControlMessage {
                    go_up: true,
                    go_down: false,
                    go_left: false,
                    go_right: false,
                    idle: false,
                    entity: entity.clone(),
                },
                NPCControlMessage {
                    go_down: true,
                    go_up: false,
                    go_left: false,
                    go_right: false,
                    idle: false,
                    entity: entity.clone(),
                },
                NPCControlMessage {
                    go_left: true,
                    go_up: false,
                    go_down: false,
                    go_right: false,
                    idle: false,
                    entity: entity.clone(),
                },
                NPCControlMessage {
                    go_right: true,
                    go_up: false,
                    go_down: false,
                    go_left: false,
                    idle: false,
                    entity: entity.clone(),
                },
                NPCControlMessage {
                    idle: true,
                    go_up: false,
                    go_down: false,
                    go_left: false,
                    go_right: false,
                    entity: entity.clone(),
                },
            ];

            let mut rng = ThreadRng::default();

            let behaviour = npc_behaviour_options.choose(&mut rng).unwrap();
            mw_control_npc.write(*behaviour);
        }
    }
}
