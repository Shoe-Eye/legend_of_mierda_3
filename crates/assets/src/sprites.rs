use bevy::prelude::*;

pub const SHEET_1_COLUMNS: usize = 13;
pub const SHEET_1_ROWS: usize = 54;
pub const SHEET_2_COLUMNS: usize = 6;
pub const SHEET_2_ROWS: usize = 4;
pub const N_FRAMES_WALK: usize = 8;
pub const N_FRAMES_USE_TOOL: usize = 7;

pub const GENNADIJ_ASSET_SHEET: &str = "sprites/gennadij.png";
pub const GENNADIJ_AXE_ASSET_SHEET: &str = "sprites/gennadij_axe.png";
pub const GENNADIJ_HAMMER_ASSET_SHEET: &str = "sprites/gennadij_hammer.png";
pub const GENNADIJ_PICKAXE_ASSET_SHEET: &str = "sprites/gennadij_pickaxe.png";
pub const GENNADIJ_SHOVEL_ASSET_SHEET: &str = "sprites/gennadij_shovel.png";
pub const GENNADIJ_WATERING_CAN_ASSET_SHEET: &str = "sprites/gennadij_watering_can.png";
pub const MIERDA_ASSET_SHEET: &str = "sprites/mierda.png";

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

#[allow(clippy::erasing_op)]
pub fn get_animation_indices(
    animation_type: AnimationType,
    animation_direction: AnimationDirection,
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
    if animation_type == AnimationType::UseTool && animation_direction == AnimationDirection::Right
    {
        first = SHEET_1_COLUMNS * 7;
        last = SHEET_1_COLUMNS * 7 + N_FRAMES_USE_TOOL;
    }
    if animation_type == AnimationType::UseTool && animation_direction == AnimationDirection::Left {
        first = SHEET_1_COLUMNS * 5;
        last = SHEET_1_COLUMNS * 5 + N_FRAMES_USE_TOOL;
    }
    if animation_type == AnimationType::UseTool && animation_direction == AnimationDirection::Up {
        first = SHEET_1_COLUMNS * 4;
        last = SHEET_1_COLUMNS * 4 + N_FRAMES_USE_TOOL;
    }
    if animation_type == AnimationType::UseTool && animation_direction == AnimationDirection::Down {
        first = SHEET_1_COLUMNS * 6;
        last = SHEET_1_COLUMNS * 6 + N_FRAMES_USE_TOOL;
    }

    AnimationIndices { first, last }
}

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
