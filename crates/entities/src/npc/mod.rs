pub mod ai;
pub mod control;
pub mod ui;

use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use lom_assets::loading::NPC_1_ASSET_SHEET_SHEET;
use lom_assets::loading::SHEET_1_COLUMNS;
use lom_assets::loading::SHEET_1_ROWS;

use lom_assets::load_texture_atlas;
use lom_assets::sprites::*;
use lom_game::GameMode;
use lom_ldtk::physics::ColliderBundle;
use rand::rngs::ThreadRng;
use rand::RngExt;

use crate::npc::ai::npc_ai;
use crate::npc::ai::BehaviourTimer;
use crate::npc::control::handle_npc_control;
use crate::npc::control::NPCControlMessage;
use crate::npc::ui::handle_despawn_npc_name_label;
use crate::npc::ui::handle_player_npc_proximity;
use crate::npc::ui::handle_spawn_npc_name_label;
use crate::npc::ui::{DespawnNPCNameLabel, SpawnNPCNameLabel};

// --------
// Entities
// --------

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct NPC {
    pub health: u16,
    pub name: String,
    pub is_ai_driven: bool,
}

#[derive(Default, Bundle)]
pub struct NPCBundle {
    pub character_animation: CharacterAnimation,
    pub animation_timer: AnimationTimer,
    pub npc: NPC,
    pub animated_character_sprite: AnimatedCharacterSprite,
    pub collider_bundle: ColliderBundle,
    pub active_events: ActiveEvents,
    pub name: Name,
    pub sprite: Sprite,
    pub behaviour_timer: BehaviourTimer,
    //
}

// ----
// LDTK
// ----

impl LdtkEntity for NPCBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlasses: &mut Assets<TextureAtlasLayout>,
    ) -> NPCBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        let collider_bundle = ColliderBundle {
            collider: Collider::cuboid(8., 8.),
            rigid_body: RigidBody::Dynamic,
            rotation_constraints,
            density: ColliderMassProperties::Mass(300.0),
            ..Default::default()
        };

        let layout = load_texture_atlas(
            SHEET_1_COLUMNS as u32,
            SHEET_1_ROWS as u32,
            Vec2::ONE * 64.,
            texture_atlasses,
        );
        let image = asset_server.load(NPC_1_ASSET_SHEET_SHEET.to_string());

        let npc_name = entity_instance
            .get_string_field("Name")
            .expect("expected entity t1o have non-nullable Name string field");

        let is_ai_driven = entity_instance
            .get_bool_field("IsAIDriven")
            .expect("expected entity to have non-nullable IsAIDriven string field");

        let mut rng = ThreadRng::default();
        let behaviour_timer_duration_sec = rng.random_range(1..10) as u64;

        NPCBundle {
            character_animation: CharacterAnimation { ..default() },
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            collider_bundle,
            active_events: ActiveEvents::COLLISION_EVENTS,
            npc: NPC {
                health: 100,
                name: npc_name.clone(),
                is_ai_driven: *is_ai_driven,
            },
            animated_character_sprite: AnimatedCharacterSprite {
                animated_character_type: AnimatedCharacterType::Player,
            },
            name: Name::new("NPC"),
            sprite: Sprite::from_atlas_image(
                image,
                TextureAtlas {
                    layout: layout,
                    index: 0,
                },
            ),
            behaviour_timer: BehaviourTimer(Timer::new(
                Duration::from_secs(behaviour_timer_duration_sec),
                TimerMode::Repeating,
            )),
        }
    }
}

// ------
// Plugin
// ------

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<NPCBundle>("NPC")
            .add_message::<NPCControlMessage>()
            .add_message::<SpawnNPCNameLabel>()
            .add_message::<DespawnNPCNameLabel>()
            // Event Handlers
            .add_systems(
                Update,
                (
                    npc_ai,
                    handle_npc_control,
                    handle_despawn_npc_name_label,
                    handle_spawn_npc_name_label,
                    handle_player_npc_proximity,
                )
                    .chain()
                    .run_if(in_state(GameMode::GamePlay)),
            );
    }
}
