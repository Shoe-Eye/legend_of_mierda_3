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
                .continue_to_state(GameState::GamePlay)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<CutsceneAssets>()
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
    #[asset(path = "sprites/earth_1.png")]
    pub earth_1: Handle<Image>,
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
    pub character_atlas_layout: Handle<TextureAtlasLayout>,
    pub gennadij_no_tool: Handle<Image>,
    pub gennadij_axe: Handle<Image>,
    pub gennadij_hammer: Handle<Image>,
    pub gennadij_pickaxe: Handle<Image>,
    pub gennadij_shovel: Handle<Image>,
    pub gennadij_watering_can: Handle<Image>,
}

impl FromWorld for CharacterSpritesheets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap().clone();

        let mut layouts = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();

        let character_atlas_layout = layouts.add(build_texture_atlas_layout(
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            Vec2::ONE * 64.,
        ));

        CharacterSpritesheets {
            character_atlas_layout,
            gennadij_no_tool: asset_server.load(GENNADIJ_ASSET_SHEET.to_string()),
            gennadij_axe: asset_server.load(GENNADIJ_AXE_ASSET_SHEET.to_string()),
            gennadij_hammer: asset_server.load(GENNADIJ_HAMMER_ASSET_SHEET.to_string()),
            gennadij_pickaxe: asset_server.load(GENNADIJ_PICKAXE_ASSET_SHEET.to_string()),
            gennadij_shovel: asset_server.load(GENNADIJ_SHOVEL_ASSET_SHEET.to_string()),
            gennadij_watering_can: asset_server.load(GENNADIJ_WATERING_CAN_ASSET_SHEET.to_string()),
        }
    }
}

pub fn build_texture_atlas_layout(
    sheet_columns: usize,
    sheet_rows: usize,
    sprite_size: Vec2,
) -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(
        UVec2::splat(sprite_size.x as u32),
        sheet_columns as u32,
        sheet_rows as u32,
        Some(UVec2::ZERO),
        Some(UVec2::ZERO),
    )
}
