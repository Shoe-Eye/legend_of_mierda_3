use std::time::Duration;

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

pub fn gameworld_time_tick() {}

#[derive(Resource, Clone, DerefMut, Deref)]
pub struct GameWorldTimer(pub Timer);

#[derive(Resource, Clone)]
pub struct GameWorldState {
    pub epoch: usize,
    pub is_paused: bool,
}

pub fn handle_game_world_timer(
    mut game_world_timer: ResMut<GameWorldTimer>,
    mut game_world_state: ResMut<GameWorldState>,
    time: Res<Time>,
) {
    if game_world_state.is_paused {
        return;
    }

    game_world_timer.tick(time.delta());
    if !game_world_timer.just_finished() {
        return;
    }

    game_world_state.epoch += 1;
}

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameWorldTimer(Timer::new(
            Duration::from_secs(1),
            TimerMode::Repeating,
        )))
        .init_state::<GameState>()
        .insert_resource(GameWorldState {
            epoch: 10,
            is_paused: false,
        });

        app.add_systems(
            Update,
            (handle_game_world_timer).run_if(in_state(GameState::GamePlay)),
        );
    }
}
