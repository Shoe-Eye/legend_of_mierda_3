use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use lom_assets::loading::GENNADIJ_ASSET_SHEET;
use lom_assets::loading::NPC_1_ASSET_SHEET_SHEET;
use lom_assets::loading::SHEET_1_COLUMNS;
use lom_assets::loading::SHEET_1_ROWS;

use lom_assets::load_texture_atlas;
use lom_assets::sprites::*;
use lom_game::GameMode;
use lom_ldtk::physics::ColliderBundle;

// --------
// Entities
// --------

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct NPC {
    pub health: u16,
    pub name: String,
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
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
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
            .expect("expected entity t1o have non-nullable npc_name string field");

        NPCBundle {
            character_animation: CharacterAnimation { ..default() },
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            collider_bundle,
            active_events: ActiveEvents::COLLISION_EVENTS,
            npc: NPC {
                health: 100,
                name: npc_name.clone(),
            },
            animated_character_sprite: AnimatedCharacterSprite {
                animated_character_type: AnimatedCharacterType::Player,
            },
            name: Name::new("Player"),
            sprite: Sprite::from_atlas_image(
                image,
                TextureAtlas {
                    layout: layout,
                    index: 0,
                },
            ),
        }
    }
}

// ------
// Plugin
// ------

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<NPCBundle>("NPC");
        // Event Handlers
        // .add_systems(
        //     Update,
        //     (
        //         handle_player_enemy_collisions.run_if(in_state(GameMode::GamePlay)),
        //         // event_player_attack.run_if(in_state(GameState::GamePlay)),
        //         event_player_hit.run_if(in_state(GameMode::GamePlay)),
        //     ),
        // )
        // .add_systems(
        //     Update,
        //     adjust_healthbar.run_if(in_state(GameMode::GamePlay)),
        // );
    }
}
