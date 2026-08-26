use bevy::prelude::*;
use bevy_defer::{AsyncCommandsExtension, AsyncWorld};
use bevy_rapier2d::prelude::*;

use lom_assets::loading::CharacterSpritesheets;
use lom_assets::sprites::*;

use crate::{
    player::{Player, PlayerToolUseEvent},
    sprites::get_animation_indices,
    tools::Tool::{self, Axe, Hammer, Pickaxe},
};

#[derive(Message, Copy, Clone, Reflect, Debug, PartialEq, Eq, Default)]
pub struct ControlEvent {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub use_tool: bool,
}

pub fn control_character(
    mut ev_control: MessageReader<ControlEvent>,
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            &mut CharacterAnimation,
            &mut Sprite,
            &Player,
        ),
        With<Player>,
    >,
    spritesheets: Res<CharacterSpritesheets>,
) {
    for control in ev_control.read() {
        for (_, mut velocity, mut char_animation, mut sprite, player) in &mut query {
            if char_animation.state == AnimationState::ToolUse {
                return;
            }

            if control.use_tool && player.tool != Tool::None && player.action != None {
                char_animation.animation_type = AnimationType::UseTool;
                char_animation.state = AnimationState::ToolUse;

                velocity.linear = Vec2::ZERO;

                match player.tool.clone() {
                    Axe => {
                        let indices = get_animation_indices(
                            char_animation.animation_type,
                            char_animation.direction,
                            Some(player.tool.clone()),
                        );
                        sprite.image = spritesheets.gennadij_axe_use.clone();
                        sprite.texture_atlas = Some(TextureAtlas {
                            index: indices.first,
                            layout: spritesheets.tool_use_character_atlas_layout.clone(),
                        });
                    }
                    Hammer => {
                        let indices = get_animation_indices(
                            char_animation.animation_type,
                            char_animation.direction,
                            Some(player.tool.clone()),
                        );
                        sprite.image = spritesheets.gennadij_hammer_use.clone();
                        sprite.texture_atlas = Some(TextureAtlas {
                            index: indices.first,
                            layout: spritesheets.tool_use_character_atlas_layout.clone(),
                        });
                    }
                    Pickaxe => {
                        let indices = get_animation_indices(
                            char_animation.animation_type,
                            char_animation.direction,
                            Some(player.tool.clone()),
                        );
                        sprite.image = spritesheets.gennadij_pickaxe_use.clone();
                        sprite.texture_atlas = Some(TextureAtlas {
                            index: indices.first,
                            layout: spritesheets.tool_use_character_atlas_layout.clone(),
                        });
                    }
                    _ => {
                        let indices = get_animation_indices(
                            char_animation.animation_type,
                            char_animation.direction,
                            None,
                        );
                        if let Some(ref mut texture_atlas) = sprite.texture_atlas {
                            texture_atlas.index = indices.first;
                        }
                    }
                }
            } else {
                let right = if control.right { 1. } else { 0. };
                let left = if control.left { 1. } else { 0. };
                let up = if control.up { 1. } else { 0. };
                let down = if control.down { 1. } else { 0. };

                velocity.linear.x = right - left;
                velocity.linear.y = up - down;

                velocity.linear = velocity.linear.normalize_or_zero() * 100.;

                let linear_norm = velocity.linear.distance(Vec2::ZERO);

                if char_animation.animation_type == AnimationType::Walk {
                    if velocity.linear.x > 0. {
                        char_animation.direction = AnimationDirection::Right;
                    } else if velocity.linear.x < 0. {
                        char_animation.direction = AnimationDirection::Left;
                    } else if velocity.linear.y > 0. {
                        char_animation.direction = AnimationDirection::Up;
                    } else if velocity.linear.y < 0. {
                        char_animation.direction = AnimationDirection::Down;
                    }
                }

                if char_animation.animation_type != AnimationType::UseTool {
                    if char_animation.animation_type != AnimationType::Walk {
                        if let Some(ref mut atlas) = sprite.texture_atlas {
                            atlas.layout = spritesheets.normal_character_atlas_layout.clone();
                        }
                    }

                    if linear_norm == 0.0 {
                        char_animation.animation_type = AnimationType::Stand;
                        char_animation.state = AnimationState::Normal;
                    } else {
                        char_animation.animation_type = AnimationType::Walk;
                    }
                }
            }
        }
    }
}

pub fn keyboard_controls(
    input: Res<ButtonInput<KeyCode>>,
    mut ev_control: MessageWriter<ControlEvent>,
) {
    let mut control = ControlEvent { ..default() };

    control.right = input.pressed(KeyCode::KeyD);
    control.left = input.pressed(KeyCode::KeyA);
    control.up = input.pressed(KeyCode::KeyW);
    control.down = input.pressed(KeyCode::KeyS);
    control.use_tool = input.pressed(KeyCode::Space);

    if control.use_tool {
        control.right = false;
        control.left = false;
        control.up = false;
        control.down = false;
    }

    ev_control.write(control);
}
