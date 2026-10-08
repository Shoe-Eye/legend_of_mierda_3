use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
pub use lom_game::GameMode;

use crate::ldtk::WallBundle;

pub mod ldtk;
pub mod physics;

pub struct LomLdtkPlugin;

impl Plugin for LomLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                ldtk::spawn_wall_collision,
                ldtk::camera_fit_inside_current_level,
                ldtk::update_level_selection,
            )
                .run_if(in_state(GameMode::GamePlay)),
        )
        .add_systems(Startup, physics::setup_gravity)
        .register_ldtk_int_cell::<WallBundle>(1);
    }
}
