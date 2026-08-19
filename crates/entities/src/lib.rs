use bevy::prelude::*;

pub mod characters;
pub mod controls;
pub mod gameplay;
pub mod items;
pub mod ldtk;
pub mod level_objects;
pub mod player;
pub mod text_indicator;
pub mod weapons;

pub use gameplay::gameover::{GameOverEvent, GameWinEvent};
pub use lom_assets::loading;
pub use lom_assets::sprites;
pub use lom_assets::{load_texture_atlas, load_texture_atlas_layout};
pub use lom_game::GameState;
pub use lom_ldtk::physics;
pub use lom_ui as ui;

use crate::controls::ControlEvent;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            characters::CharactersPlugin,
            player::PlayerPlugin,
            items::ItemsPlugin,
            // weapons::WeaponsPlugin,
            text_indicator::TextIndicatorPlugin,
            level_objects::light::LightPlugin,
        ))
        .add_message::<GameOverEvent>()
        .add_message::<GameWinEvent>()
        .add_message::<ControlEvent>()
        .add_systems(
            Update,
            (controls::keyboard_controls, controls::control_character)
                .run_if(in_state(GameState::GamePlay)),
        )
        .add_systems(
            Update,
            (ldtk::hide_dummy_entities, ldtk::fix_missing_ldtk_entities)
                .run_if(in_state(GameState::GamePlay)),
        );
    }
}
