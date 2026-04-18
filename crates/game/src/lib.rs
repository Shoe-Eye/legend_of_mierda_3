use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Splash,
    Menu,
    Cutscene,
    GamePlay,
    GameOver,
}
