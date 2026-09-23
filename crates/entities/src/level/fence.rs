use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use lom_assets::loading::StaticSpriteTextureAtlasLayoutAssets;
use lom_assets::StaticSpriteAssets;
use lom_game::GameState;

use crate::level::{ground::Ground, BuildFence, Building};

#[derive(Component, Clone, Copy)]
pub struct FenceTile {
    pub x: u32,
    pub y: u32,
}

pub fn get_sprite_index(tile: FenceTile, all_tiles: Vec<FenceTile>) -> usize {
    let exists_left = all_tiles
        .iter()
        .filter(|t| t.x == (tile.x - 1) && t.y == tile.y)
        .count()
        > 0;

    let exists_right = all_tiles
        .iter()
        .filter(|t| t.x == (tile.x + 1) && t.y == tile.y)
        .count()
        > 0;

    let exists_top = all_tiles
        .iter()
        .filter(|t| t.x == tile.x && t.y == (tile.y + 1))
        .count()
        > 0;

    let exists_bottom = all_tiles
        .iter()
        .filter(|t| {
            if tile.y > 0 {
                return t.x == tile.x && t.y == (tile.y - 1);
            } else {
                return false;
            }
        })
        .count()
        > 0;

    if exists_left && exists_right && exists_top && exists_bottom {
        return 7;
    }

    if !exists_left && !exists_right && !exists_top && !exists_bottom {
        return 3;
    }

    if !exists_left && exists_right && exists_bottom {
        return 0;
    }

    if !exists_left && exists_right && !exists_bottom {
        return 6;
    }

    // ---

    if exists_left && !exists_right && exists_bottom {
        return 2;
    }

    if exists_left && !exists_right && !exists_bottom {
        return 8;
    }

    if exists_bottom && exists_top {
        return 3;
    }

    if exists_left && exists_right {
        return 7;
    }

    if !exists_left && !exists_right && exists_bottom {
        return 3;
    }

    if !exists_left && !exists_right && exists_top {
        return 3;
    }

    return 0;
}

pub fn adjust_fence_sprites(
    mut q_fences: ParamSet<(
        Query<(&mut Sprite, &FenceTile)>,
        Query<(&mut Sprite, &FenceTile), Added<FenceTile>>,
    )>,
) {
    let is_fence_built = q_fences.p1().iter().count() > 0;
    if !is_fence_built {
        return;
    }

    let fences: Vec<FenceTile> = q_fences
        .p0()
        .iter()
        .map(|(_, fence)| fence.clone())
        .collect();

    for (mut sprite, fence) in q_fences.p0().iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = get_sprite_index(fence.clone(), fences.clone());
        }
    }
}

pub fn handle_build_fence(
    mut commands: Commands,
    mut mr: MessageReader<BuildFence>,
    q_ground: Query<(Entity, &Ground)>,
    q_fence_tiles: Query<(Entity, &ChildOf, &FenceTile)>,
    static_sprite_assets: Res<StaticSpriteAssets>,
    static_sprite_atlas_assets: Res<StaticSpriteTextureAtlasLayoutAssets>,
) {
    for message in mr.read() {
        let fences: Vec<FenceTile> = q_fence_tiles
            .iter()
            .map(|(_, _, fence)| fence.clone())
            .collect();

        if let Some((ground_entity, ground)) = q_ground.iter().next() {
            if q_fence_tiles
                .iter()
                .filter(|(_, parent, tile)| {
                    parent.parent() == ground_entity && tile.x == message.x && tile.y == message.y
                })
                .count()
                == 0
            {
                commands.entity(ground_entity).with_children(
                    |parent: &mut bevy_ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                        let fence = FenceTile {
                            x: message.x,
                            y: message.y,
                        };

                        parent.spawn((
                            Sprite::from_atlas_image(
                                static_sprite_assets.fence_sheet.clone(),
                                TextureAtlas {
                                    layout: static_sprite_atlas_assets
                                        .fence_sheet_texture_layout
                                        .clone(),
                                    index: get_sprite_index(fence.clone(), fences.clone()),
                                },
                            ),
                            Transform::from_translation(Vec3::new(
                                (message.x * ground.grid_size) as f32,
                                (message.y * ground.grid_size) as f32,
                                0.51,
                            )),
                            Building {
                                x: message.x,
                                y: message.y,
                                width: 1,
                                height: 1,
                            },
                            Name::new("fence tile"),
                            fence,
                            Collider::cuboid(16., 16.),
                            Friction::new(1.0),
                            ActiveEvents::COLLISION_EVENTS,
                        ));
                    },
                );
            } else {
                let (entity, _, _) = q_fence_tiles
                    .iter()
                    .filter(|(_, parent, tile)| {
                        parent.parent() == ground_entity
                            && tile.x == message.x
                            && tile.y == message.y
                    })
                    .next()
                    .unwrap();

                commands.entity(entity).despawn();
            }
        }
    }
}

pub struct FencePlugin;

impl Plugin for FencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (adjust_fence_sprites, handle_build_fence).run_if(in_state(GameState::GamePlay)),
        );
    }
}
