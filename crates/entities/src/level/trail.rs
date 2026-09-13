use bevy::prelude::*;
use bevy_color::palettes::css::DARK_GREEN;

use crate::level::ground::{FoundationTile, TrailTile};
use crate::level::BuildTrailMessage;
use crate::level::{ground::Ground, GroundStruct};

pub fn handle_build_trail(
    mut commands: Commands,
    mut mr_build_trail: MessageReader<BuildTrailMessage>,
    q_ground: Query<(Entity, &Ground)>,
    q_trail_tiles: Query<(Entity, &ChildOf, &TrailTile)>,
    q_foundation_tiles: Query<(Entity, &ChildOf, &FoundationTile)>,
) {
    for message in mr_build_trail.read() {
        if let Some((ground_entity, ground)) = q_ground.iter().next() {
            let foundation_does_exist = q_foundation_tiles
                .iter()
                .filter(|(_, parent, tile)| {
                    parent.parent() == ground_entity && tile.x == message.x && tile.y == message.y
                })
                .count()
                != 0;

            if foundation_does_exist {
                commands.entity(ground_entity).with_children(
                    |parent: &mut bevy_ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                        parent.spawn((
                            Sprite::from_color(DARK_GREEN, Vec2::new(16.0, 16.0)),
                            Transform::from_translation(Vec3::new(
                                (message.x * ground.grid_size) as f32,
                                (message.y * ground.grid_size) as f32,
                                0.51,
                            )),
                            Name::new("trail tile"),
                            GroundStruct {
                                x: message.x,
                                y: message.y,
                            },
                            TrailTile {
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
