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
        Commands<'w, 's>,
        MessageReader<'w, 's, SpawnEnemyEvent>,
        Res<'w, LevelSelection>,
        Query<'w, 's, (Entity, &'w LevelIid)>,
        Query<'w, 's, &'w LdtkProjectHandle>,
        Res<'w, Assets<LdtkProject>>,
        Query<'w, 's, (Entity, &'w ChildOf, &'w Enemy, &'w Transform)>,
        Query<'w, 's, (&'w Player, &'w Transform)>,
    );

    let mut state: SystemState<Params> = SystemState::new(world);

    let events_read = {
        let (_, ev_spawn_enemy, _, _, _, _, _, _) = state.get(world);
        let spawns: Vec<SpawnEnemyEvent> = ev_spawn_enemy.read().cloned().collect();
        spawns
    };

    if events_read.is_empty() {
        return;
    }

    let mut rng = ThreadRng::default();

    for ev_spawn in events_read {
        let (mut commands, _, level_selection, levels, projects, project_assets, enemies, q_player_query) = state.get_mut(world);

        if let Some((_, player_transform)) = q_player_query.iter().next() {
            let player_translation = player_transform.translation;

            for (_, level_iid) in levels.iter() {
                let project = match project_assets.get(projects.iter().next().unwrap()) {
                    Some(p) => p,
                    None => continue,
                };
                let level = match project.get_raw_level_by_iid(level_iid.get()) {
                    Some(l) => l,
                    None => continue,
                };

                if !level_selection.is_match(&LevelIndices { level: 0, ..default() }, level) {
                    continue;
                }

                let max_level_dimension = level.px_wid.max(level.px_hei) as f32;

                for _ in 0..ev_spawn.count {
                    let (dummy_entity, parent_entity, enemy_transform) = {
                        let mut found: Option<(Entity, Entity, Transform)> = None;
                        for (e, child_of, enemy, transform) in enemies.iter() {
                            if enemy.is_dummy && enemy.enemy_type == ev_spawn.enemy_type {
                                found = Some((e, child_of.parent(), *transform));
                                break;
                            }
                        }
                        match found {
                            Some(f) => f,
                            None => continue,
                        }
                    };

                    let mut enemy_position = player_translation;
                    while (player_translation - enemy_position).length()
                        < max_level_dimension / 2.0
                        || enemy_position.x < 24.0
                        || enemy_position.x > (level.px_wid as f32) - 24.0
                        || enemy_position.y < 24.0
                        || enemy_position.y > (level.px_hei as f32) - 24.0
                    {
                        let r = rng.random_range(0.0..1000.0);
                        let angle = rng.random_range(0.0..std::f32::consts::TAU);
                        enemy_position = player_translation
                            + Vec3::new(r * angle.sin(), r * angle.cos(), 0.0);
                    }

                    let new_entity = commands.spawn_empty().id();
                    let transform = Transform::from_translation(enemy_position).with_scale(Vec3::ONE * 0.5);

                    commands.entity(parent_entity).add_child(new_entity);
                    commands.entity(new_entity).insert(Enemy {
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
                    });
                    commands.entity(new_entity).insert(transform);

                    EntityCloner::build_opt_out(world).clone_entity(dummy_entity, new_entity);
                }
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
