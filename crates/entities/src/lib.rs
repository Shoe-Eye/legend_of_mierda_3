use bevy::prelude::*;

pub mod characters;
pub mod controls;
pub mod gameplay;
pub mod items;
pub mod level_objects;
pub mod player;
pub mod text_indicator;
pub mod weapons;

pub use lom_assets::loading;
pub use lom_assets::sprites;
pub use lom_assets::{load_texture_atlas, load_texture_atlas_layout};
pub use lom_ldtk::physics;
pub use lom_game::GameState;
pub use lom_ui as ui;
pub use gameplay::gameover::{GameOverEvent, GameWinEvent};

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            characters::CharactersPlugin,
            player::PlayerPlugin,
            items::ItemsPlugin,
            weapons::WeaponsPlugin,
            text_indicator::TextIndicatorPlugin,
            level_objects::light::LightPlugin,
        ));
    }
}