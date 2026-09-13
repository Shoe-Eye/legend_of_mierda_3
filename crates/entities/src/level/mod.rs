use bevy::prelude::*;
use lom_game::GameState;

pub mod fence;
pub mod foundation;
pub mod ground;
pub mod trail;
pub mod turret;

pub struct LevelPlugin;

#[derive(Debug, Clone, Copy, Reflect, Component)]
pub struct Building {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, Reflect, Component)]
pub struct GroundStruct {
    pub x: u32,
    pub y: u32,
}

#[derive(Message, Clone, Copy)]
pub struct BuildFoundationMessage {
    pub x: u32,
    pub y: u32,
}

#[derive(Message, Clone, Copy)]
pub struct BuildTrailMessage {
    pub x: u32,
    pub y: u32,
}

#[derive(Message, Clone, Copy)]
pub struct DigGroundMessage {
    pub x: u32,
    pub y: u32,
}

#[derive(Message, Clone, Copy)]
pub struct BuildFenceMessage {
    pub x: u32,
    pub y: u32,
}

#[derive(Message, Clone, Copy)]
pub struct BuildTurret {
    pub x: u32,
    pub y: u32,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ground::GroundPlugin,
            fence::FencePlugin,
            turret::TurretPlugin,
        ))
        .add_systems(
            Update,
            (
                foundation::handle_build_foundation,
                trail::handle_build_trail,
                ground::handle_build_ground,
                turret::handle_build_turret,
                turret::handle_turret_rotation,
            )
                .run_if(in_state(GameState::GamePlay)),
        )
        .add_message::<BuildFoundationMessage>()
        .add_message::<BuildFenceMessage>()
        .add_message::<BuildTrailMessage>()
        .add_message::<BuildTurret>()
        .add_message::<DigGroundMessage>();
    }
}
