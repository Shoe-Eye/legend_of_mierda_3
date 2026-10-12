use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use lom_assets::loading::CharacterSpritesheets;
use lom_assets::loading::NPC_1_ASSET_SHEET_SHEET;
use lom_assets::loading::SHEET_1_COLUMNS;
use lom_assets::loading::SHEET_1_ROWS;

use lom_assets::load_texture_atlas;
use lom_assets::sprites::*;
use lom_game::GameMode;
use lom_ldtk::physics::ColliderBundle;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use rand::RngExt;

// --------
// Entities
// --------

#[derive(Component, Deref, DerefMut, Clone, Default, Reflect)]
pub struct BehaviourTimer(pub Timer);

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
            .expect("expected entity t1o have non-nullable npc_name string field");

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

#[derive(Message, Copy, Clone, Reflect, Debug, PartialEq, Eq)]
pub struct NPCControlMessage {
    pub go_up: bool,
    pub go_down: bool,
    pub go_left: bool,
    pub go_right: bool,
    pub idle: bool,
    pub entity: Entity,
}

pub fn handle_npc_control(
    mut ev_control: MessageReader<NPCControlMessage>,
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            &mut CharacterAnimation,
            &mut Sprite,
            &NPC,
        ),
        With<NPC>,
    >,
    spritesheets: Res<CharacterSpritesheets>,
) {
    for control in ev_control.read() {
        for (entity, mut velocity, mut char_animation, mut sprite, _npc) in &mut query {
            if entity != control.entity {
                continue;
            }

            let right = if control.go_right { 1. } else { 0. };
            let left = if control.go_left { 1. } else { 0. };
            let up = if control.go_up { 1. } else { 0. };
            let down = if control.go_down { 1. } else { 0. };

            velocity.linear.x = right - left;
            velocity.linear.y = up - down;

            velocity.linear = velocity.linear.normalize_or_zero() * 100.;

            let linear_norm = velocity.linear.distance(Vec2::ZERO);

            if char_animation.animation_type == AnimationType::Walk {
                if velocity.linear.x > 0. {
                    char_animation.direction = AnimationDirection::Right;
                } else if velocity.linear.x < 0. {
                    char_animation.direction = AnimationDirection::Left;
                } else if velocity.linear.y > 0. {
                    char_animation.direction = AnimationDirection::Up;
                } else if velocity.linear.y < 0. {
                    char_animation.direction = AnimationDirection::Down;
                }
            }

            if char_animation.animation_type != AnimationType::UseTool {
                if char_animation.animation_type != AnimationType::Walk {
                    if let Some(ref mut atlas) = sprite.texture_atlas {
                        atlas.layout = spritesheets.normal_character_atlas_layout.clone();
                    }
                }

                if linear_norm == 0.0 {
                    char_animation.animation_type = AnimationType::Stand;
                    char_animation.state = AnimationState::Normal;
                } else {
                    char_animation.animation_type = AnimationType::Walk;
                }
            }
        }
    }
}

pub fn npc_ai(
    mut q_npc: Query<(Entity, &mut NPC, &mut BehaviourTimer)>,
    time: Res<Time>,
    mut mw_control_npc: MessageWriter<NPCControlMessage>,
) {
    for (entity, mut npc, mut timer) in q_npc.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            let npc_behaviour_options = vec![
                NPCControlMessage {
                    go_up: true,
                    go_down: false,
                    go_left: false,
                    go_right: false,
                    idle: false,
                    entity: entity.clone(),
                },
                NPCControlMessage {
                    go_down: true,
                    go_up: false,
                    go_left: false,
                    go_right: false,
                    idle: false,
                    entity: entity.clone(),
                },
                NPCControlMessage {
                    go_left: true,
                    go_up: false,
                    go_down: false,
                    go_right: false,
                    idle: false,
                    entity: entity.clone(),
                },
                NPCControlMessage {
                    go_right: true,
                    go_up: false,
                    go_down: false,
                    go_left: false,
                    idle: false,
                    entity: entity.clone(),
                },
                NPCControlMessage {
                    idle: true,
                    go_up: false,
                    go_down: false,
                    go_left: false,
                    go_right: false,
                    entity: entity.clone(),
                },
            ];

            let mut rng = ThreadRng::default();

            let behaviour = npc_behaviour_options.choose(&mut rng).unwrap();
            mw_control_npc.write(*behaviour);
        }
    }
}

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<NPCBundle>("NPC")
            .add_message::<NPCControlMessage>()
            // Event Handlers
            .add_systems(
                Update,
                (npc_ai, handle_npc_control)
                    .chain()
                    .run_if(in_state(GameMode::GamePlay)),
            );
    }
}
