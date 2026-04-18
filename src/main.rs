#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::window::WindowResolution;
use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PresentMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_scene_hook::HookPlugin;
use bevy_tweening::TweeningPlugin;
use pecs::prelude::*;
use lom_entities as entities;
use lom_splashscreen::SplashscreenPlugin;
use lom_entities::gameplay::GameplayPlugin;
use lom_entities::sprites;
use lom_ldtk::ldtk;
use lom_game::GameState;
use lom_cutscene::CutscenePlugin;
use lom_audio::InternalAudioPlugin;
use lom_assets::loading::LoadingPlugin;
use lom_ui::menu::MenuPlugin;
use lom_ldtk::ldtk::{LEVEL_1_IID, WallBundle};

fn main() {
    let mut app = App::new();

    app.init_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Legend of Mierda".into(),
                        resolution: WindowResolution::new(700, 700),
                        present_mode: PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(AudioPlugin)
        .add_plugins((HookPlugin, PecsPlugin, TweeningPlugin))
        .add_plugins((
            LoadingPlugin,
            MenuPlugin,
            CutscenePlugin,
            LegendOfMierdaPlugin,
        ))
        .add_plugins(InternalAudioPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(LdtkPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(LevelSelection::iid(LEVEL_1_IID))
        .register_ldtk_int_cell::<WallBundle>(1);

    app.run();
}

pub struct LegendOfMierdaPlugin;

impl Plugin for LegendOfMierdaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            entities::EntitiesPlugin,
            GameplayPlugin,
            SplashscreenPlugin,
        ))
        .add_systems(
            OnEnter(GameState::GamePlay),
            ldtk::spawn_game_world,
        )
        .add_systems(
            OnExit(GameState::GamePlay),
            ldtk::despawn_game_world,
        )
        .add_systems(
            Update,
            (
                ldtk::spawn_wall_collision,
                ldtk::camera_fit_inside_current_level,
                ldtk::update_level_selection,
            )
                .run_if(in_state(GameState::GamePlay)),
        )
        .add_systems(
            Update,
            (ldtk::hide_dummy_entities, ldtk::fix_missing_ldtk_entities)
                .run_if(in_state(GameState::GamePlay)),
        )
        .add_systems(
            Update,
            (sprites::animate_player_sprite, sprites::flash_sprite)
                .run_if(in_state(GameState::GamePlay)),
        );
    }
}