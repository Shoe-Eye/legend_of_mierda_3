use bevy::prelude::*;
use lom_assets::{
    loading::{
        CharacterSpritesheets, N_FRAMES_USE_TOOL_SHEET_1, N_FRAMES_USE_TOOL_SHEET_2, N_FRAMES_WALK,
        SHEET_1_COLUMNS, SHEET_2_COLUMNS,
    },
    sprites::*,
};

use crate::{
    player::{Player, PlayerToolUseEvent},
    tools::Tool,
};

pub fn animate_character_sprtire(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut CharacterAnimation,
        &mut AnimationTimer,
        &mut Sprite,
        &mut AnimatedCharacterSprite,
        &Player,
    )>,
    spritesheets: Res<CharacterSpritesheets>,
    mut mw_player_use_tool: MessageWriter<PlayerToolUseEvent>,
) {
    for (
        entity,
        mut character_animation,
        mut timer,
        mut sprite,
        animated_character_sprite,
        player,
    ) in &mut query
    {
        timer.tick(time.delta());

        if animated_character_sprite.animated_character_type == AnimatedCharacterType::NotAnimated {
            continue;
        }

        let tool = match player.tool {
            Tool::Axe => Some(Tool::Axe),
            Tool::Hammer => Some(Tool::Hammer),
            Tool::Pickaxe => Some(Tool::Pickaxe),
            _ => None,
        };

        let mut indices = get_animation_indices(
            character_animation.animation_type,
            character_animation.direction,
            tool,
        );

        if timer.just_finished() {
            let current_index = sprite.texture_atlas.as_ref().map(|t| t.index).unwrap_or(0);
            let new_index = if (current_index >= indices.last) || (current_index < indices.first) {
                if character_animation.animation_type == AnimationType::UseTool
                    && (current_index >= indices.last)
                {
                    character_animation.animation_type = AnimationType::Stand;

                    match player.tool {
                        Tool::Axe => {
                            sprite.image = spritesheets.gennadij_axe.clone();
                        }
                        Tool::Hammer => {
                            sprite.image = spritesheets.gennadij_hammer.clone();
                        }
                        Tool::Pickaxe => {
                            sprite.image = spritesheets.gennadij_pickaxe.clone();
                        }
                        _ => {}
                    }

                    sprite.texture_atlas = Some(TextureAtlas {
                        index: indices.first,
                        layout: spritesheets.normal_character_atlas_layout.clone(),
                    });

                    mw_player_use_tool.write(PlayerToolUseEvent {
                        entity: entity.clone(),
                        tool: player.tool,
                    });
                }

                character_animation.state = AnimationState::Normal;

                if character_animation.animation_type == AnimationType::Stand {
                    indices = get_animation_indices(
                        character_animation.animation_type,
                        character_animation.direction,
                        tool,
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

pub fn get_animation_indices(
    animation_type: AnimationType,
    animation_direction: AnimationDirection,
    tool: Option<Tool>,
) -> AnimationIndices {
    let mut first = 0;
    let mut last = 0;

    // Walk
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Right {
        first = SHEET_1_COLUMNS * 11 + 1;
        last = SHEET_1_COLUMNS * 11 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Left {
        first = SHEET_1_COLUMNS * 9 + 1;
        last = SHEET_1_COLUMNS * 9 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Up {
        first = SHEET_1_COLUMNS * 8 + 1;
        last = SHEET_1_COLUMNS * 8 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Down {
        first = SHEET_1_COLUMNS * 10 + 1;
        last = SHEET_1_COLUMNS * 10 + N_FRAMES_WALK;
    }

    // Stand
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Right {
        first = SHEET_1_COLUMNS * 11;
        last = first;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Left {
        first = SHEET_1_COLUMNS * 9;
        last = first;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Up {
        first = SHEET_1_COLUMNS * 8;
        last = first;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Down {
        first = SHEET_1_COLUMNS * 10;
        last = first;
    }

    // UseTool
    if let Some(_) = tool {
        if animation_type == AnimationType::UseTool
            && animation_direction == AnimationDirection::Right
        {
            first = SHEET_2_COLUMNS * 3;
            last = SHEET_2_COLUMNS * 3 + N_FRAMES_USE_TOOL_SHEET_2;
        }
        if animation_type == AnimationType::UseTool
            && animation_direction == AnimationDirection::Left
        {
            first = SHEET_2_COLUMNS * 1;
            last = SHEET_2_COLUMNS * 1 + N_FRAMES_USE_TOOL_SHEET_2;
        }
        if animation_type == AnimationType::UseTool && animation_direction == AnimationDirection::Up
        {
            first = SHEET_2_COLUMNS * 0;
            last = SHEET_2_COLUMNS * 0 + N_FRAMES_USE_TOOL_SHEET_2;
        }
        if animation_type == AnimationType::UseTool
            && animation_direction == AnimationDirection::Down
        {
            first = SHEET_2_COLUMNS * 2;
            last = SHEET_2_COLUMNS * 2 + N_FRAMES_USE_TOOL_SHEET_2;
        }
    } else {
        if animation_type == AnimationType::UseTool
            && animation_direction == AnimationDirection::Right
        {
            first = SHEET_1_COLUMNS * 7;
            last = SHEET_1_COLUMNS * 7 + N_FRAMES_USE_TOOL_SHEET_1;
        }
        if animation_type == AnimationType::UseTool
            && animation_direction == AnimationDirection::Left
        {
            first = SHEET_1_COLUMNS * 5;
            last = SHEET_1_COLUMNS * 5 + N_FRAMES_USE_TOOL_SHEET_1;
        }
        if animation_type == AnimationType::UseTool && animation_direction == AnimationDirection::Up
        {
            first = SHEET_1_COLUMNS * 4;
            last = SHEET_1_COLUMNS * 4 + N_FRAMES_USE_TOOL_SHEET_1;
        }
        if animation_type == AnimationType::UseTool
            && animation_direction == AnimationDirection::Down
        {
            first = SHEET_1_COLUMNS * 6;
            last = SHEET_1_COLUMNS * 6 + N_FRAMES_USE_TOOL_SHEET_1;
        }
    }

    AnimationIndices { first, last }
}
