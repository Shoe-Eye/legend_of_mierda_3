use bevy::prelude::*;
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Friction};
use lom_assets::loading::{StaticSpriteAtlasLayouts, StaticSpriteTextureAtlasLayoutAssets};
use lom_assets::StaticSpriteAssets;
use lom_game::GameWorldState;

use crate::level::ground::{Ground, GroundTile};
use crate::level::PlantWatermelon;

const N_WATERMELON_GROWTH_STAGES: usize = 4;

#[derive(Component, Clone, Copy)]
pub struct Watermelon {
    pub x: u32,
    pub y: u32,
    pub epoch_planted: usize,
    pub growth_stage: usize,
}

pub fn handle_plant_watermelon(
    mut commands: Commands,
    mut mr_build_foundation: MessageReader<PlantWatermelon>,
    q_ground: Query<(Entity, &Ground)>,
    q_ground_tiles: Query<(Entity, &ChildOf, &GroundTile)>,
    q_watermelons: Query<(Entity, &ChildOf, &Watermelon)>,
    static_sprite_assets: Res<StaticSpriteAssets>,
    sprite_layouts: Res<StaticSpriteAtlasLayouts>,
    game_world_state: Res<GameWorldState>,
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
                            index: 1,
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
                                epoch_planted: game_world_state.epoch,
                                growth_stage: 1,
                            },
                            Collider::cuboid(16., 16.),
                            Friction::new(1.0),
                            ActiveEvents::COLLISION_EVENTS,
                        ));
                    },
                );
            }
        }
    }
}

pub fn handle_watermelons_growth(
    mut q_watermelons: Query<(Entity, &mut Sprite, &mut Watermelon)>,
    game_world_state: Res<GameWorldState>,
) {
    for (_, mut sprite, mut watermelon) in q_watermelons.iter_mut() {
        watermelon.growth_stage =
            (game_world_state.epoch - watermelon.epoch_planted).min(N_WATERMELON_GROWTH_STAGES);

        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = watermelon.growth_stage + 1;
        }
    }
}
