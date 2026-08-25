use bevy::prelude::*;

#[derive(Copy, Clone, Reflect, Default, Debug, PartialEq, Eq)]
pub enum AnimatedCharacterType {
    #[default]
    Player,
    NotAnimated,
}

#[derive(Copy, Clone, Component, Reflect, Default)]
pub struct AnimatedCharacterSprite {
    pub animated_character_type: AnimatedCharacterType,
}

#[allow(dead_code)]
#[derive(Component, Clone, Default, Debug, Reflect)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Clone, Default, Debug, Reflect, PartialEq, Eq)]
pub enum AnimationState {
    #[default]
    Normal,
    ToolUse,
}

#[derive(Clone, Default, Copy, PartialEq, Debug, Reflect)]
pub enum AnimationDirection {
    #[default]
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Default, Copy, PartialEq, Debug, Reflect)]
pub enum AnimationType {
    Walk,
    #[default]
    Stand,
    UseTool,
}

#[derive(Component, Clone, Default, Debug, Reflect)]
pub struct CharacterAnimation {
    pub state: AnimationState,
    pub direction: AnimationDirection,
    pub animation_type: AnimationType,
}

#[derive(Component, Reflect)]
pub struct FlashingTimer {
    pub timer: Timer,
}

#[derive(Component, Deref, DerefMut, Clone, Default, Reflect)]
pub struct AnimationTimer(pub Timer);

pub fn flash_sprite(
    mut commands: Commands,
    mut flashing_query: Query<(&mut FlashingTimer, Entity, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut timer, timer_e, mut timer_sprite) in flashing_query.iter_mut() {
        timer_sprite.color = Color::srgba(1.0, 0.0, 0.0, 0.5);

        timer.timer.tick(time.delta());

        if timer.timer.just_finished() {
            timer_sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            if commands.get_entity(timer_e).is_err() {
                continue;
            }
            commands.entity(timer_e).remove::<FlashingTimer>();
        }
    }
}
