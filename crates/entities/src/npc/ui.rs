use bevy::prelude::*;
use lom_assets::FontAssets;

use crate::{npc::NPC, player::Player};

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

            if distance <= 60.0 {
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
