use bevy::prelude::*;

pub mod fence;
pub mod ground;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ground::GroundPlugin, fence::FencePlugin));
    }
}
