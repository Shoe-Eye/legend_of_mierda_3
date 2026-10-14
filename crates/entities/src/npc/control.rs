use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use lom_assets::loading::CharacterSpritesheets;
use lom_assets::sprites::*;

use crate::npc::NPC;

#[derive(Message, Copy, Clone, Reflect, Debug, PartialEq, Eq)]
pub struct NPCControlMessage {
    pub go_up: bool,
    pub go_down: bool,
    pub go_left: bool,
    pub go_right: bool,
    pub idle: bool,
    pub entity: Entity,
}

pub fn handle_npc_control(
    mut ev_control: MessageReader<NPCControlMessage>,
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            &mut CharacterAnimation,
            &mut Sprite,
            &NPC,
        ),
        With<NPC>,
    >,
    spritesheets: Res<CharacterSpritesheets>,
) {
    for control in ev_control.read() {
        for (entity, mut velocity, mut char_animation, mut sprite, _npc) in &mut query {
            if entity != control.entity {
                continue;
            }

            let right = if control.go_right { 1. } else { 0. };
            let left = if control.go_left { 1. } else { 0. };
            let up = if control.go_up { 1. } else { 0. };
            let down = if control.go_down { 1. } else { 0. };

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
