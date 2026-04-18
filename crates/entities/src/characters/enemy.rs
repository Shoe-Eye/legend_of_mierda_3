use crate::gameplay::scoring::Score;
use bevy::prelude::*;
use bevy_ecs::entity::EntityCloner;
use bevy_ecs::system::SystemState;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;
use lom_assets::sprites::*;
use lom_assets::{load_texture_atlas_layout, loading::AudioAssets};
use lom_game::GameState;
use lom_ldtk::physics::ColliderBundle;
use pecs::prelude::*;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use rand::RngExt;

use crate::player::Player;
use crate::text_indicator::SpawnTextIndicatorEvent;

// ----------
// Components
// ----------

#[derive(Component, Clone, Default, Reflect)]
pub struct DirectionUpdateTime {
    pub timer: Timer,
}

// --------
// Entities
// --------

#[derive(Clone, Copy, PartialEq, Debug, Default, Component, Reflect)]
pub enum EnemyType {
    #[default]
    Mierda,
    Pendejo,
    Psychiatrist1,
    Psychiatrist2,
}

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub move_direction: Vec2,
    pub health: u16,
    pub hit_at: Option<Timer>,
    pub is_dummy: bool,
    pub marked_for_despawn: bool,
}

#[derive(Default, Bundle, Clone)]
pub struct EnemyBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub character_animation: CharacterAnimation,
    pub animation_timer: AnimationTimer,
    pub enemy: Enemy,
    pub collider_bundle: ColliderBundle,
    pub active_events: ActiveEvents,
    pub direction_update_time: DirectionUpdateTime,
    pub animated_character_sprite: AnimatedCharacterSprite,
}

// ----
// LDTK
// ----

pub fn create_enemy_bundle(
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlasLayout>,
    is_dummy: bool,
    enemy_type: EnemyType,
) -> EnemyBundle {
    let rotation_constraints = LockedAxes::ROTATION_LOCKED;

    let collider_bundle = ColliderBundle {
        collider: Collider::cuboid(8., 26.),
        rigid_body: RigidBody::Dynamic,
        friction: Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        rotation_constraints,
        ..Default::default()
    };

    let (atlas_handle, spritesheet_type) = match enemy_type {
        EnemyType::Mierda => (
            load_texture_atlas_layout(
                MIERDA_ASSET_SHEET.to_string(),
                asset_server,
                5,
                1,
                Vec2::ONE * 16.,
            ),
            AnimatedCharacterType::NotAnimated,
        ),
        EnemyType::Pendejo => {
            let mut rng = ThreadRng::default();
            let (spritesheet_path, spritesheet_type) =
                PENDEJO_SPRITE_SHEETS.choose(&mut rng).unwrap();

            (
                load_texture_atlas_layout(
                    spritesheet_path.to_string(),
                    asset_server,
                    SHEET_1_COLUMNS as usize,
                    SHEET_1_ROWS as usize,
                    Vec2::ONE * 64.,
                ),
                *spritesheet_type,
            )
        }
        EnemyType::Psychiatrist1 => (
            load_texture_atlas_layout(
                PSYCHIATRIST_1_ASSET_SHEET.to_string(),
                asset_server,
                1,
                1,
                128. * Vec2::ONE,
            ),
            AnimatedCharacterType::NotAnimated,
        ),
        EnemyType::Psychiatrist2 => (
            load_texture_atlas_layout(
                PSYCHIATRIST_2_ASSET_SHEET.to_string(),
                asset_server,
                1,
                1,
                128. * Vec2::ONE,
            ),
            AnimatedCharacterType::NotAnimated,
        ),
    };

    let mut rng = ThreadRng::default();
    let enemy = Enemy {
        health: 100,
        enemy_type,
        move_direction: Vec2 {
            x: rng.random::<f32>() * 2.0 - 1.0,
            y: rng.random::<f32>() * 2.0 - 1.0,
        }
        .normalize(),
        hit_at: None,
        is_dummy,
        marked_for_despawn: false,
    };

    EnemyBundle {
        sprite: Sprite { ..default() },
        transform: Transform::default(),
        character_animation: CharacterAnimation {
            state: AnimationState::default(),
            direction: AnimationDirection::Right,
            animation_type: AnimationType::Walk,
        },
        animation_timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        collider_bundle,
        active_events: ActiveEvents::COLLISION_EVENTS,
        enemy,
        direction_update_time: DirectionUpdateTime {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        },
        animated_character_sprite: AnimatedCharacterSprite {
            animated_character_type: spritesheet_type,
        },
    }
}

// ------
// Messages
// ------

#[derive(Message, Clone)]
pub struct EnemyHitEvent {
    pub entity: Entity,
    pub damage: u16,
}

#[derive(Message, Clone)]
pub struct SpawnEnemyEvent {
    pub count: u32,
    pub enemy_type: EnemyType,
}

// --------------
// Event Handlers
// --------------

pub fn handle_spawn_enemy(world: &mut World) {
    type Params<'w, 's> = (
        MessageReader<'w, 's, SpawnEnemyEvent>,
        Res<'w, LevelSelection>,
        Query<'w, 's, (Entity, &'static LevelIid)>,
        Query<'w, 's, &'static LdtkProjectHandle>,
        Res<'w, Assets<LdtkProject>>,
        Query<'w, 's, (Entity, &'static ChildOf, &'static Enemy)>,
        Query<'w, 's, (&'static Player, &'static Transform)>,
    );

    let mut state: SystemState<Params> = SystemState::new(world);

    // 1. Drain events into owned data
    let events: Vec<SpawnEnemyEvent> = {
        let (mut ev, ..) = state.get_mut(world);
        ev.read().cloned().collect()
    };

    if events.is_empty() {
        return;
    }

    let mut rng = ThreadRng::default();

    for ev_spawn in &events {
        // 2. Read all query data into owned values (no world borrow held after block)
        let (player_translation, level_bounds, dummy_candidates) = {
            let (_, level_selection, levels, projects, project_assets, enemies, players) =
                state.get(world);

            let player_translation = match players.iter().next() {
                Some((_, t)) => t.translation,
                None => continue,
            };

            let project_handle = match projects.iter().next() {
                Some(h) => h,
                None => continue,
            };
            let project = match project_assets.get(project_handle) {
                Some(p) => p,
                None => continue,
            };

            let level_bounds: Vec<(i32, i32, Entity)> = levels
                .iter()
                .filter_map(|(entity, iid)| {
                    let level = project.get_raw_level_by_iid(iid.get())?;
                    if level_selection.is_match(
                        &LevelIndices {
                            level: 0,
                            ..default()
                        },
                        level,
                    ) {
                        Some((level.px_wid, level.px_hei, entity))
                    } else {
                        None
                    }
                })
                .collect();

            let dummy_candidates: Vec<(Entity, Entity)> = enemies
                .iter()
                .filter(|(_, _, enemy)| enemy.is_dummy && enemy.enemy_type == ev_spawn.enemy_type)
                .map(|(e, child_of, _)| (e, child_of.parent()))
                .collect();

            (player_translation, level_bounds, dummy_candidates)
        }; // <-- ALL state borrows dropped here

        for (px_wid, px_hei, _) in &level_bounds {
            let px_wid = *px_wid;
            let px_hei = *px_hei;
            let max_level_dimension = px_wid.max(px_hei) as f32;

            for _ in 0..ev_spawn.count {
                let (dummy_entity, parent_entity) = match dummy_candidates.first() {
                    Some(&d) => d,
                    None => continue,
                };

                // Generate position
                let mut enemy_position = player_translation;
                while (player_translation - enemy_position).length() < max_level_dimension / 2.0
                    || enemy_position.x < 24.0
                    || enemy_position.x > px_wid as f32 - 24.0
                    || enemy_position.y < 24.0
                    || enemy_position.y > px_hei as f32 - 24.0
                {
                    let r = rng.random_range(0.0..1000.0);
                    let angle = rng.random_range(0.0..std::f32::consts::TAU);
                    enemy_position =
                        player_translation + Vec3::new(r * angle.sin(), r * angle.cos(), 0.0);
                }

                // 3. Spawn directly on world — no Commands needed, no borrow conflict
                let new_entity = world
                    .spawn((
                        Enemy {
                            enemy_type: ev_spawn.enemy_type,
                            is_dummy: false,
                            health: match ev_spawn.enemy_type {
                                EnemyType::Mierda => 50,
                                EnemyType::Pendejo => 100,
                                EnemyType::Psychiatrist1 => 5000,
                                EnemyType::Psychiatrist2 => 5000,
                            },
                            move_direction: Vec2::ZERO,
                            hit_at: None,
                            marked_for_despawn: false,
                        },
                        Transform::from_translation(enemy_position).with_scale(Vec3::ONE * 0.5),
                    ))
                    .id();

                // Add as child of parent
                world.entity_mut(parent_entity).add_child(new_entity);

                // 4. EntityCloner has exclusive world access — no conflict
                EntityCloner::build_opt_out(world).clone_entity(dummy_entity, new_entity);
            }
        }
    }
}

pub fn handle_enemy_hit(
    mut commands: Commands,
    q_player: Query<(&Transform, &Player)>,
    mut enemies: Query<(Entity, &Transform, &mut Velocity, &mut Enemy)>,
    mut ev_enemy_hit: MessageReader<EnemyHitEvent>,
    mut ev_spawn_text_indicator: MessageWriter<SpawnTextIndicatorEvent>,

    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    let mut hit_sound_played = false;

    for event in ev_enemy_hit.read() {
        if commands.get_entity(event.entity).is_err() {
            continue;
        }

        for (player_transform, _) in q_player.iter() {
            let player_position = player_transform.translation;

            let (enemy_entity, mierda_transform, mut enemy_velocity, mut enemy) =
                enemies.get_mut(event.entity).unwrap();
            let enemy_position = mierda_transform.translation;
            let vector_attack = (enemy_position - player_position).normalize();
            enemy_velocity.linvel.x += vector_attack.x * 500.;
            enemy_velocity.linvel.y += vector_attack.y * 500.;

            let damage = match enemy.enemy_type {
                EnemyType::Mierda => (1.0 * event.damage as f32) as u16,
                EnemyType::Pendejo => (0.5 * event.damage as f32) as u16,
                EnemyType::Psychiatrist1 => (1.0 * event.damage as f32) as u16,
                EnemyType::Psychiatrist2 => (1.0 * event.damage as f32) as u16,
            };

            let timer = Timer::new(std::time::Duration::from_millis(200), TimerMode::Once);
            enemy.hit_at = Some(timer.clone());
            enemy.health -= u16::min(damage, enemy.health);

            if !hit_sound_played {
                audio.play(audio_assets.hit.clone()).with_volume(0.05);
                hit_sound_played = true;
            }

            commands.entity(enemy_entity).insert(FlashingTimer {
                timer: timer.clone(),
            });

            ev_spawn_text_indicator.write(SpawnTextIndicatorEvent {
                text: format!("-{}", damage),
                entity: enemy_entity,
            });
        }
    }
}

pub fn despawn_dead_enemies(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Transform, &mut Velocity, &mut Enemy)>,
    mut score: ResMut<Score>,
) {
    for (e, _, _, mut enemy) in enemies.iter_mut() {
        if enemy.health != 0 {
            continue;
        }

        if enemy.marked_for_despawn {
            continue;
        }

        enemy.marked_for_despawn = true;
        score.score += match enemy.enemy_type {
            EnemyType::Mierda => 100,
            EnemyType::Psychiatrist1 => 5000,
            EnemyType::Psychiatrist2 => 5000,
            EnemyType::Pendejo => 50,
        };

        commands
            .promise(|| e)
            .then(asyn!(state => {
                state.asyn().timeout(0.3)
            }))
            .then(asyn!(state, mut commands: Commands => {
                if commands.get_entity(state.value).is_err() {
                    return;
                }
                commands.entity(state.value).despawn();
            }));
    }
}

// ------
// Plugin
// ------

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Event Handlers
            .add_message::<EnemyHitEvent>()
            .add_message::<SpawnEnemyEvent>()
            // Event Handlers
            .add_systems(
                Update,
                handle_enemy_hit.run_if(in_state(GameState::GamePlay)),
            )
            .add_systems(
                Update,
                despawn_dead_enemies.run_if(in_state(GameState::GamePlay)),
            )
            .add_systems(
                Update,
                handle_spawn_enemy.run_if(in_state(GameState::GamePlay)),
            );
    }
}
