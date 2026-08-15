use bevy::prelude::*;
use bevy_defer::{AsyncCommandsExtension, AsyncWorld};
use bevy_rapier2d::prelude::*;

use lom_assets::loading::CharacterSpritesheets;
use lom_assets::sprites::*;

use crate::player::{Player, PlayerAttackEvent};

#[derive(Message, Copy, Clone, Reflect, Debug, PartialEq, Eq, Default)]
pub struct ControlEvent {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub attack: bool,
}

pub fn control_character(
    mut commands: Commands,
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
    mut mw_player_attack: MessageWriter<PlayerAttackEvent>,
) {
    for control in ev_control.read() {
        for (entity, mut velocity, mut char_animation, mut sprite, _player) in &mut query {
            if control.attack {
                char_animation.animation_type = AnimationType::Attack;

                let indices =
                    get_animation_indices(char_animation.animation_type, char_animation.direction);
                if let Some(ref mut atlas) = sprite.texture_atlas {
                    atlas.index = indices.first;
                    atlas.layout = spritesheets.player_atlas_2.clone();
                }
                sprite.image = spritesheets.player_sprite_2.clone();

                let en = entity.clone();

                commands.spawn_task(move || async move {
                    AsyncWorld.sleep(0.3).await;
                    // let _ = AsyncWorld.write_message(PlayerAttackEvent { entity: en });
                    Ok(())
                });
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

                if char_animation.animation_type != AnimationType::Attack {
                    if char_animation.animation_type != AnimationType::Walk {
                        if let Some(ref mut atlas) = sprite.texture_atlas {
                            atlas.layout = spritesheets.player_atlas_1.clone();
                        }
                    }

                    if linear_norm == 0.0 {
                        char_animation.animation_type = AnimationType::Stand;
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

    ev_control.write(control);
}
