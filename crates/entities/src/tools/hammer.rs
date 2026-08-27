use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use lom_assets::{loading::StaticSpriteTextureAtlasLayoutAssets, StaticSpriteAssets};
use lom_game::GameState;

use crate::{
    level::{
        fence::{get_sprite_index, FenceTile},
        ground::Ground,
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
    mut commands: Commands,
    mut mr: MessageReader<PlayerToolUseEvent>,
    q_ground: Query<(Entity, &Ground)>,
    q_fence_tiles: Query<(Entity, &ChildOf, &FenceTile)>,
    q_tool_pointer_tiles: Query<(Entity, &ToolPointerTile)>,
    static_sprite_assets: Res<StaticSpriteAssets>,
    static_sprite_atlas_assets: Res<StaticSpriteTextureAtlasLayoutAssets>,
) {
    for message in mr.read() {
        if message.action.is_none() {
            continue;
        }

        let action = message.action.unwrap();

        if !(message.tool == Tool::Hammer && action == Action::Fence) {
            continue;
        }

        let fences: Vec<FenceTile> = q_fence_tiles
            .iter()
            .map(|(_, _, fence)| fence.clone())
            .collect();

        if let Some((_, tool_pointer_tile)) = q_tool_pointer_tiles.iter().next() {
            if let Some((ground_entity, ground)) = q_ground.iter().next() {
                if q_fence_tiles
                    .iter()
                    .filter(|(_, parent, tile)| {
                        parent.parent() == ground_entity
                            && tile.x == tool_pointer_tile.x
                            && tile.y == tool_pointer_tile.y
                    })
                    .count()
                    == 0
                {
                    commands.entity(ground_entity).with_children(
                        |parent: &mut bevy_ecs::relationship::RelatedSpawnerCommands<
                            '_,
                            ChildOf,
                        >| {
                            let fence = FenceTile {
                                x: tool_pointer_tile.x,
                                y: tool_pointer_tile.y,
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
                                    (tool_pointer_tile.x * ground.grid_size) as f32,
                                    (tool_pointer_tile.y * ground.grid_size) as f32,
                                    0.51,
                                )),
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
                                && tile.x == tool_pointer_tile.x
                                && tile.y == tool_pointer_tile.y
                        })
                        .next()
                        .unwrap();

                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
