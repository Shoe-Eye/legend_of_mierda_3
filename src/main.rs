#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use bevy::camera::ScalingMode;
use bevy::window::*;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_defer::AsyncPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_obj::ObjPlugin;
use bevy_rapier2d::prelude::*;
use bevy_scene_hook::HookPlugin;
use bevy_tweening::TweeningPlugin;
use lom_assets::loading::LoadingPlugin;
use lom_assets::sprites::flash_sprite;
use lom_audio::InternalAudioPlugin;
use lom_cutscene::CutscenePlugin;
use lom_entities as entities;
use lom_entities::gameplay::gameover::GameOverPlugin;
use lom_entities::gameplay::GameplayPlugin;
use lom_entities::sprites;
use lom_game::GameState;
use lom_ldtk::ldtk::{self, LevelChangeEvent};
use lom_ldtk::ldtk::{WallBundle, LEVEL_1_IID};
use lom_splashscreen::SplashscreenPlugin;
use lom_ui::game::GameUIPlugin;
use lom_ui::menu::MainMenuPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1680, 1280).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(AsyncPlugin::default_settings())
    .add_plugins(AudioPlugin)
    .add_plugins(LdtkPlugin)
    .add_plugins(ObjPlugin)
    .insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        set_clear_color: SetClearColor::FromLevelBackground,
        ..Default::default()
    })
    .add_plugins((HookPlugin, TweeningPlugin))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .init_state::<GameState>()
    .add_plugins((
        LoadingPlugin,
        MainMenuPlugin,
        GameUIPlugin,
        CutscenePlugin,
        LegendOfMierda3Plugin,
        GameOverPlugin,
        SplashscreenPlugin,
    ))
    .add_plugins(InternalAudioPlugin)
    .add_plugins(EguiPlugin::default())
    .add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
    )
    .insert_resource(LevelSelection::iid(LEVEL_1_IID))
    .register_ldtk_int_cell::<WallBundle>(1)
    .add_systems(Startup, (load_mesh));

    app.run();
}

pub struct LegendOfMierda3Plugin;

impl Plugin for LegendOfMierda3Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            lom_ldtk::LomLdtkPlugin,
            entities::EntitiesPlugin,
            GameplayPlugin,
        ))
        .add_systems(OnEnter(GameState::GamePlay), ldtk::spawn_game_world)
        .add_systems(OnExit(GameState::GamePlay), ldtk::despawn_game_world)
        .add_systems(
            Update,
            (sprites::animate_character_sprtire, flash_sprite)
                .run_if(in_state(GameState::GamePlay)),
        )
        .add_systems(Startup, (setup_cameras))
        .add_message::<LevelChangeEvent>();
    }
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            order: 2,
            ..default()
        },
        // Projection::from(OrthographicProjection {
        //     // scaling_mode: ScalingMode::FixedVertical {
        //     //     viewport_height: 38.0,
        //     // },
        //     ..OrthographicProjection::default_3d()
        // }),
    ));
}

fn load_mesh(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a spinning cube
    commands.spawn((
        Mesh3d(asset_server.load("models/turrets/simple.obj")),
        MeshMaterial3d(materials.add(StandardMaterial {
            // base_color_texture: Some(asset_server.load("cube.png")),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_x(FRAC_PI_4)),
    ));

    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 4.0, 3.0)));
}
