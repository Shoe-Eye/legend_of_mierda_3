use bevy::prelude::*;

pub mod ground;
pub mod house;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ground::GroundPlugin, house::HousePlugin));
    }
}
