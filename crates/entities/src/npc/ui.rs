use bevy::prelude::*;
use lom_assets::FontAssets;
use lom_dialogue::{ActiveDialogue, AdvanceDialogue, StartNPCDialogue};

use crate::{
    controls::PlayerControlEvent,
    npc::{NPC, control::NPCControlMessage},
    player::Player,
};

/// How close the player must be to an NPC to see its name and talk to it.
pub const NPC_INTERACTION_DISTANCE: f32 = 60.0;

#[derive(Component)]
pub struct UINPCNameLabel;

#[derive(Message, Clone)]
pub struct SpawnNPCNameLabel {
    pub text: String,
    pub entity: Entity,
}

#[derive(Message, Clone, Copy)]
pub struct DespawnNPCNameLabel {
    pub entity: Entity,
}

pub fn handle_spawn_npc_name_label(
    mut commands: Commands,
    mut mr_spawn_name: MessageReader<SpawnNPCNameLabel>,
    font_assets: Res<FontAssets>,
    q_npc_labels: Query<(Entity, &ChildOf, &UINPCNameLabel)>,
) {
    for event in mr_spawn_name.read() {
        let mut label_spawned = false;

        for (_, child_of, _) in q_npc_labels.iter() {
            if child_of.parent() == event.entity {
                label_spawned = true;
                break;
            }
        }

        if label_spawned {
            continue;
        }

        commands.entity(event.entity).with_children(|parent| {
            parent.spawn((
                Text2d::new(event.text.clone()),
                TextFont {
                    font: font_assets.pixeloid_mono.clone(),
                    font_size: 15.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
                TextColor(Color::WHITE),
                Transform::from_xyz(0.0, -23.0, 10.0),
                UINPCNameLabel,
            ));
        });
    }
}

pub fn handle_despawn_npc_name_label(
    mut commands: Commands,
    mut mr_despawn_name: MessageReader<DespawnNPCNameLabel>,
    q_npc_labels: Query<(Entity, &ChildOf, &UINPCNameLabel)>,
) {
    for event in mr_despawn_name.read() {
        for (enitity, child_of, _) in q_npc_labels.iter() {
            if child_of.parent() == event.entity {
                commands.entity(enitity).despawn();
            }
        }
    }
}

pub fn handle_player_npc_proximity(
    q_player: Query<&GlobalTransform, With<Player>>,
    q_npc: Query<(Entity, &GlobalTransform, &NPC)>,
    mut mw_spawn_name: MessageWriter<SpawnNPCNameLabel>,
    mut mw_despawn_name: MessageWriter<DespawnNPCNameLabel>,
) {
    if let Some(player_transform) = q_player.iter().next() {
        for (entity, npc_transform, npc) in q_npc.iter() {
            let mut vec = player_transform.translation() - npc_transform.translation();
            vec.z = 0.0;
            let distance = vec.length();

            if distance == 0.0 {
                continue;
            }

            if distance <= NPC_INTERACTION_DISTANCE {
                mw_spawn_name.write(SpawnNPCNameLabel {
                    text: npc.name.clone(),
                    entity: entity.clone(),
                });
            } else {
                mw_despawn_name.write(DespawnNPCNameLabel {
                    entity: entity.clone(),
                });
            }
        }
    }
}

pub fn handle_start_npc_dialogue(
    mut ev_control: MessageReader<PlayerControlEvent>,
    active_dialogue: Res<ActiveDialogue>,
    q_player: Query<&GlobalTransform, With<Player>>,
    q_npc: Query<(Entity, &GlobalTransform, &NPC)>,
    mut mw_start_dialogue: MessageWriter<StartNPCDialogue>,
    mut mw_advance_dialogue: MessageWriter<AdvanceDialogue>,
    mut mw_control_npc: MessageWriter<NPCControlMessage>,
) {
    if !ev_control.read().any(|event| event.interact) {
        return;
    }

    if active_dialogue.is_active() {
        mw_advance_dialogue.write(AdvanceDialogue);
        return;
    }

    let Some(player_transform) = q_player.iter().next() else {
        return;
    };
    let player_position = player_transform.translation().truncate();

    let nearest = q_npc
        .iter()
        .map(|(entity, transform, npc)| {
            let distance = player_position.distance(transform.translation().truncate());
            (entity, npc, distance)
        })
        .filter(|(_, _, distance)| *distance <= NPC_INTERACTION_DISTANCE)
        .min_by(|a, b| a.2.total_cmp(&b.2));

    let Some((entity, npc, _)) = nearest else {
        return;
    };

    // stop the NPC so it doesn't walk away mid-sentence
    mw_control_npc.write(NPCControlMessage {
        go_up: false,
        go_down: false,
        go_left: false,
        go_right: false,
        idle: true,
        entity,
    });

    mw_start_dialogue.write(StartNPCDialogue {
        entity,
        speaker: npc.name.clone(),
        lines: npc.dialogue.clone(),
    });
}
