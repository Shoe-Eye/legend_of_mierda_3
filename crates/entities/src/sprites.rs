use bevy::prelude::*;
use lom_assets::{loading::CharacterSpritesheets, sprites::*};

use crate::player::{Player, PlayerToolUseEvent};

pub fn animate_character_sprtire(
    time: Res<Time>,
    mut query: Query<(
        &mut CharacterAnimation,
        &mut AnimationTimer,
        &mut Sprite,
        &mut AnimatedCharacterSprite,
    )>,
    spritesheets: Res<CharacterSpritesheets>,
    mut mw_player_use_tool: MessageWriter<PlayerToolUseEvent>,
    q_player: Query<(Entity, &Player)>,
) {
    for (mut character_animation, mut timer, mut sprite, animated_character_sprite) in &mut query {
        timer.tick(time.delta());

        if animated_character_sprite.animated_character_type == AnimatedCharacterType::NotAnimated {
            continue;
        }

        let mut indices = get_animation_indices(
            character_animation.animation_type,
            character_animation.direction,
        );

        if timer.just_finished() {
            let current_index = sprite.texture_atlas.as_ref().map(|t| t.index).unwrap_or(0);
            let new_index = if (current_index >= indices.last) || (current_index < indices.first) {
                if character_animation.animation_type == AnimationType::UseTool
                    && (current_index >= indices.last)
                {
                    let spritesheet = match animated_character_sprite.animated_character_type {
                        AnimatedCharacterType::Player => spritesheets.gennadij_no_tool.clone(),
                        _ => panic!("not implemented"),
                    };

                    character_animation.animation_type = AnimationType::Stand;
                    sprite.texture_atlas = Some(TextureAtlas {
                        index: current_index,
                        layout: spritesheets.character_atlas_layout.clone(),
                    });

                    sprite.image = spritesheets.gennadij_no_tool.clone();

                    if character_animation.state == AnimationState::ToolUse {
                        if let Some((entity, player)) = q_player.iter().next() {
                            mw_player_use_tool.write(PlayerToolUseEvent {
                                entity: entity,
                                tool: player.tool,
                            });
                        }
                    }

                    character_animation.state = AnimationState::Normal;
                }

                if character_animation.animation_type == AnimationType::Stand {
                    indices = get_animation_indices(
                        character_animation.animation_type,
                        character_animation.direction,
                    );
                }

                indices.first
            } else {
                current_index + 1
            };

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = new_index;
            }
        }
    }
}
