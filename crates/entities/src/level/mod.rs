use bevy::prelude::*;
use lom_game::GameState;

pub mod fence;
pub mod foundation;
pub mod ground;
pub mod trail;
pub mod turret;
pub mod watermelon;

pub struct LevelPlugin;

#[derive(Debug, Clone, Copy, Reflect, Component)]
pub struct Building {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Message, Clone, Copy)]
pub struct BuildFoundation {
    pub x: u32,
    pub y: u32,
}

#[derive(Message, Clone, Copy)]
pub struct BuildTrail {
    pub x: u32,
    pub y: u32,
}

#[derive(Message, Clone, Copy)]
pub struct DigGround {
    pub x: u32,
    pub y: u32,
}

#[derive(Message, Clone, Copy)]
pub struct BuildFence {
    pub x: u32,
    pub y: u32,
}

#[derive(Message, Clone, Copy)]
pub struct BuildTurret {
    pub x: u32,
    pub y: u32,
}

#[derive(Message, Clone, Copy)]
pub struct PlantWatermelon {
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
                watermelon::handle_plant_watermelon,
            )
                .run_if(in_state(GameState::GamePlay)),
        )
        .add_message::<BuildFoundation>()
        .add_message::<BuildFence>()
        .add_message::<BuildTrail>()
        .add_message::<BuildTurret>()
        .add_message::<PlantWatermelon>()
        .add_message::<DigGround>();
    }
}
