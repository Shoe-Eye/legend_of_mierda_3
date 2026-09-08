use bevy::prelude::*;
use lom_assets::StaticSpriteAssets;

use crate::level::BuildFoundationMessage;
use crate::level::{
    ground::{FoundationTile, Ground, GroundTile},
    GroundStruct,
};

pub fn handle_build_foundation(
    mut commands: Commands,
    mut mr_build_foundation: MessageReader<BuildFoundationMessage>,
    q_ground: Query<(Entity, &Ground)>,
    q_ground_tiles: Query<(Entity, &ChildOf, &GroundTile)>,
    q_foundation_tiles: Query<(Entity, &ChildOf, &FoundationTile)>,
    static_sprite_assets: Res<StaticSpriteAssets>,
) {
    for message in mr_build_foundation.read() {
        if let Some((ground_entity, ground)) = q_ground.iter().next() {
            let foundation_does_no_exits = q_foundation_tiles
                .iter()
                .filter(|(_, parent, tile)| {
                    parent.parent() == ground_entity && tile.x == message.x && tile.y == message.y
                })
                .count()
                == 0;

            let ground_digged = q_ground_tiles
                .iter()
                .filter(|(_, parent, tile)| {
                    parent.parent() == ground_entity && tile.x == message.x && tile.y == message.y
                })
                .count()
                == 1;

            if foundation_does_no_exits && ground_digged {
                commands.entity(ground_entity).with_children(
                    |parent: &mut bevy_ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                        parent.spawn((
                            Sprite::from_image(static_sprite_assets.foundation_1.clone()),
                            Transform::from_translation(Vec3::new(
                                (message.x * ground.grid_size) as f32,
                                (message.y * ground.grid_size) as f32,
                                0.51,
                            )),
                            Name::new("foundation tile"),
                            GroundStruct {
                                x: message.x,
                                y: message.y,
                            },
                            FoundationTile {
                                x: message.x,
                                y: message.y,
                            },
                        ));
                    },
                );
            }
        }
    }
}
