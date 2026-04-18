use bevy::{image::TextureAtlasLayout, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::sprites::*;
use lom_game::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Splash)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<AvatarAssets>()
                .load_collection::<CutsceneAssets>()
                .load_collection::<SceneAssets>()
                .load_collection::<AnimationAssets>()
                .load_collection::<StaticSpriteAssets>(),
        );

        app.init_resource::<FontAssets>();
        app.init_resource::<MaterialAssets>();
        app.init_resource::<MeshAssets>();
        app.init_resource::<CharacterSpritesheets>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/biboran.ogg")]
    pub biboran: Handle<AudioSource>,
    #[asset(path = "audio/mierda.ogg")]
    pub mierda: Handle<AudioSource>,
    #[asset(path = "audio/slash.ogg")]
    pub slash: Handle<AudioSource>,
    #[asset(path = "audio/hit.ogg")]
    pub hit: Handle<AudioSource>,
    #[asset(path = "audio/hurt.ogg")]
    pub hurt: Handle<AudioSource>,
    #[asset(path = "audio/gameover.ogg")]
    pub gameover: Handle<AudioSource>,
    #[asset(path = "audio/mexico.ogg")]
    pub mexico: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    #[asset(path = "models/biboran.glb#Scene0")]
    pub biboran: Handle<Scene>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct AnimationAssets {
    #[asset(path = "models/biboran.glb#Animation0")]
    pub biboran: Handle<AnimationClip>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    pub pixeloid_mono: Handle<Font>,
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();

        FontAssets {
            pixeloid_mono: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct StaticSpriteAssets {
    #[asset(path = "sprites/arrow.png")]
    pub arrow: Handle<Image>,
    #[asset(path = "sprites/speargun-wide.png")]
    pub speargun: Handle<Image>,
    #[asset(path = "sprites/speargun-arrow.png")]
    pub speargun_arrow: Handle<Image>,
    #[asset(path = "sprites/pill.png")]
    pub pill: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AvatarAssets {
    #[asset(path = "avatars/alextime.png")]
    pub alextime: Handle<Image>,
    #[asset(path = "avatars/gennadiy.png")]
    pub gennadiy: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct CutsceneAssets {
    #[asset(path = "cutscenes/phone-call-1.png")]
    pub phone_call_1: Handle<Image>,
    #[asset(path = "cutscenes/main-menu.png")]
    pub main_menu: Handle<Image>,
    #[asset(path = "cutscenes/splash.png")]
    pub splash: Handle<Image>,
}

#[derive(Resource)]
pub struct MeshAssets {}

impl FromWorld for MeshAssets {
    fn from_world(_world: &mut World) -> Self {
        Self {}
    }
}

#[derive(Resource)]
pub struct MaterialAssets {
    pub black: Handle<StandardMaterial>,
    pub white: Handle<StandardMaterial>,
    pub yellow: Handle<StandardMaterial>,
    pub blue: Handle<StandardMaterial>,
    pub red: Handle<StandardMaterial>,
    pub transparent_white: Handle<StandardMaterial>,
    pub transparent_black: Handle<StandardMaterial>,
}

impl FromWorld for MaterialAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials_asset = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        MaterialAssets {
            black: materials_asset.add(bevy::prelude::Color::srgb(0., 0.1, 0.1)),
            white: materials_asset.add(bevy::prelude::Color::srgb(1., 0.9, 0.9)),
            red: materials_asset.add(bevy::prelude::Color::srgba(1., 0.1, 0.1, 0.5)),
            yellow: materials_asset.add(bevy::prelude::Color::srgb(1.0, 1.0, 0.0)),
            blue: materials_asset.add(bevy::prelude::Color::srgb(0., 0., 1.)),
            transparent_white: materials_asset.add(bevy::prelude::Color::srgba(1., 0.9, 0.9, 0.5)),
            transparent_black: materials_asset.add(bevy::prelude::Color::srgba(0., 0.1, 0.1, 0.5)),
        }
    }
}

#[derive(Resource)]
pub struct CharacterSpritesheets {
    pub player_atlas_1: Handle<TextureAtlasLayout>,
    pub player_atlas_2: Handle<TextureAtlasLayout>,
    pub mierda_atlas: Handle<TextureAtlasLayout>,
    pub pendejo_atlas_1: Handle<TextureAtlasLayout>,
    pub pendejo_atlas_2: Handle<TextureAtlasLayout>,
}

impl FromWorld for CharacterSpritesheets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap().clone();

        let mut layouts = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();

        let player_atlas_1 = layouts.add(load_texture_atlas_layout(
            PLAYER_ASSET_SHEET_1.to_string(),
            &asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            Vec2::ONE * 64.,
        ));

        let pendejo_atlas_1 = layouts.add(load_texture_atlas_layout(
            PENDEJO_SPRITE_SHEETS[0].0.to_string(),
            &asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            Vec2::ONE * 64.,
        ));

        let pendejo_atlas_2 = layouts.add(load_texture_atlas_layout(
            PENDEJO_SPRITE_SHEETS[1].0.to_string(),
            &asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            Vec2::ONE * 64.,
        ));

        let player_atlas_2 = layouts.add(load_texture_atlas_layout(
            PLAYER_ASSET_SHEET_2.to_string(),
            &asset_server,
            SHEET_2_COLUMNS,
            SHEET_2_ROWS,
            Vec2::ONE * 64. * 3.,
        ));

        let mierda_atlas = layouts.add(load_texture_atlas_layout(
            MIERDA_ASSET_SHEET.to_string(),
            &asset_server,
            5,
            1,
            Vec2::ONE * 16.0,
        ));

        CharacterSpritesheets {
            player_atlas_1,
            player_atlas_2,
            mierda_atlas,
            pendejo_atlas_1,
            pendejo_atlas_2,
        }
    }
}

pub fn load_texture_atlas_layout(
    path: String,
    asset_server: &AssetServer,
    sheet_columns: usize,
    sheet_rows: usize,
    sprite_size: Vec2,
) -> TextureAtlasLayout {
    let _texture_handle: Handle<Image> = asset_server.load(path);

    TextureAtlasLayout::from_grid(
        UVec2::splat(sprite_size.x as u32),
        sheet_columns as u32,
        sheet_rows as u32,
        Some(UVec2::ZERO),
        Some(UVec2::ZERO),
    )
}
