use bevy::prelude::*;
use lom_assets::loading::{StaticSpriteAtlasLayouts, StaticSpriteTextureAtlasLayoutAssets};
use lom_assets::StaticSpriteAssets;

use crate::level::ground::Watermelon;
use crate::level::ground::{Ground, GroundTile};
use crate::level::{BuildFoundation, PlantWatermelon};

pub fn handle_plant_watermelon(
    mut commands: Commands,
    mut mr_build_foundation: MessageReader<PlantWatermelon>,
    q_ground: Query<(Entity, &Ground)>,
    q_ground_tiles: Query<(Entity, &ChildOf, &GroundTile)>,
    q_watermelons: Query<(Entity, &ChildOf, &Watermelon)>,
    static_sprite_assets: Res<StaticSpriteAssets>,
    sprite_layouts: Res<StaticSpriteAtlasLayouts>,
) {
    for message in mr_build_foundation.read() {
        if let Some((ground_entity, ground)) = q_ground.iter().next() {
            let watermelon_does_not_exist = q_watermelons
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

            // let watermelon_does_exist = !watermelon_does_no_exist;

            if watermelon_does_not_exist && ground_digged {
                commands.entity(ground_entity).with_children(
                    |parent: &mut bevy_ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                        let mut sprite =
                            Sprite::from_image(static_sprite_assets.watermelon.clone());
                        sprite.texture_atlas = Some(TextureAtlas {
                            index: 5,
                            layout: sprite_layouts.watermelon.clone(),
                        });

                        parent.spawn((
                            sprite,
                            Transform::from_translation(Vec3::new(
                                (message.x * ground.grid_size) as f32,
                                (message.y * ground.grid_size) as f32,
                                0.51,
                            )),
                            Name::new("watermelon"),
                            Watermelon {
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
