use std::f32::consts::FRAC_PI_4;
use std::time::Duration;

use crate::characters::enemy::{Enemy, EnemyHitEvent};
use crate::level::ground::Ground;
use crate::level::turret::{Turret, TurretModel};
use crate::physics::ColliderBundle;
use crate::{loading::StaticSpriteAssets, GameState};

use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::*;

// ----------
// Components
// ----------

#[derive(Component, Clone, Copy, Default)]
pub struct Speargun;

#[derive(Component, Clone, Copy, Default)]
pub struct SpeargunArrow;

#[derive(Component, Clone, Copy, Default)]
pub struct SpeargunArrowTrail;

// ------
// Events
// ------

#[derive(Message)]
pub struct SpeargunShootEvent {
    pub entity: Entity,
}

// -------
// Bundles
// -------

#[derive(Clone, Default, Bundle)]
pub struct SpeargunArrowBundle {
    pub sprite: Sprite,
    pub speargun_arrow: SpeargunArrow,
    pub timer_despawn: SpeargunArrowDespawnTimer,
    pub timer_trail_spawn: SpeargunTrailSpawnTimer,
    pub collider_bundle: ColliderBundle,
    pub active_events: ActiveEvents,
}

#[derive(Clone, Default, Bundle)]
pub struct SpeargunArrowTrailBundle {
    pub sprite: Sprite,
    pub speargun_arrow_trail: SpeargunArrowTrail,
    pub timer_despawn: SpeargunArrowTrailDespawnTimer,
}

// ---------
// Resources
// ---------

#[derive(Resource, Default, Clone, Component)]
pub struct SpeargunTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct SpeargunTrailSpawnTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct SpeargunArrowDespawnTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct SpeargunArrowTrailDespawnTimer(pub Timer);

// -------
// Systems
// -------

const TRAIL_TIMER_SPAWN_MILLIS: u64 = 10;
const ARROW_VELOCITY: f32 = 700.0;

fn handle_speargun_attack_event(
    mut commands: Commands,
    q_spearguns: Query<(Entity, &ChildOf, &Speargun, &Turret)>,
    q_turret_models: Query<(Entity, &TurretModel)>,
    q_ground: Query<(Entity, &Ground)>,
    mut mr_speargun_attack: MessageReader<SpeargunShootEvent>,
    static_sprite_assets: Res<StaticSpriteAssets>,
) {
    for attack in mr_speargun_attack.read() {
        if let Ok((entity, child_of, _speargun, turret)) = q_spearguns.get(attack.entity) {
            if let Some((_, model)) = q_turret_models
                .iter()
                .find(|(_, model)| model.game_entity == entity)
            {
                if let Some((ground_entity, ground)) = q_ground.iter().next() {
                    commands.entity(child_of.0).with_children(|parent| {
                        let timer_despawn = SpeargunArrowDespawnTimer(Timer::new(
                            Duration::from_secs_f32(1.0),
                            TimerMode::Repeating,
                        ));

                        let timer_trail_spawn = SpeargunTrailSpawnTimer(Timer::new(
                            Duration::from_millis(TRAIL_TIMER_SPAWN_MILLIS),
                            TimerMode::Repeating,
                        ));

                        let z_rot = model.angle;

                        parent.spawn((
                            Transform::from_translation(Vec3::new(
                                ((turret.x) * ground.grid_size) as f32 + 16. * 2.,
                                ((turret.y) * ground.grid_size) as f32 + 16. * 2.,
                                0.52,
                            ))
                            .with_rotation(Quat::from_rotation_z(z_rot)),
                            SpeargunArrowBundle {
                                speargun_arrow: SpeargunArrow,
                                active_events: ActiveEvents::COLLISION_EVENTS,
                                timer_despawn,
                                timer_trail_spawn,
                                collider_bundle: ColliderBundle {
                                    collider: Collider::cuboid(20., 5.),
                                    rigid_body: RigidBody::Dynamic,
                                    friction: Friction {
                                        coefficient: 0.0,
                                        combine_rule: CoefficientCombineRule::Min,
                                    },
                                    density: ColliderMassProperties::Density(0.0),
                                    rotation_constraints: LockedAxes::ROTATION_LOCKED_X,
                                    velocity: Velocity {
                                        linear: ARROW_VELOCITY
                                            * Vec2 {
                                                x: z_rot.cos(),
                                                y: z_rot.sin(),
                                            },
                                        angular: 0.0,
                                    },
                                    ..default()
                                },
                                sprite: Sprite::from_image(
                                    static_sprite_assets.speargun_arrow.clone(),
                                ),
                            },
                            Name::new("weapon speargun arrow"),
                            ZIndex(202),
                        ));
                    });
                }
            }
        }
    }
}

const TRAIL_TIMER_DE_SPAWN_MILLIS: u64 = 500;

fn handle_arrow_timers(
    mut commands: Commands,
    mut q_speargun: Query<(
        Entity,
        &ChildOf,
        &Transform,
        &mut SpeargunArrowDespawnTimer,
        &mut SpeargunTrailSpawnTimer,
        &SpeargunArrow,
    )>,

    static_sprite_assets: Res<StaticSpriteAssets>,
    time: Res<Time>,
) {
    for (entity, parent, transform, mut timer_despawn, mut timer_trail, _) in q_speargun.iter_mut()
    {
        timer_despawn.0.tick(time.delta());
        timer_trail.0.tick(time.delta());
        if timer_despawn.0.just_finished() {
            commands.entity(entity).despawn();
        }

        if timer_trail.0.just_finished() {
            let timer_despawn = SpeargunArrowTrailDespawnTimer(Timer::new(
                Duration::from_millis(TRAIL_TIMER_DE_SPAWN_MILLIS),
                TimerMode::Once,
            ));

            // commands.entity(parent.0).with_children(|parent| {
            //     parent.spawn((
            //         SpeargunArrowTrailBundle {
            //             sprite: Sprite {
            //                 image: static_sprite_assets.speargun_arrow.clone(),
            //                 color: Color::srgba(0.3, 0.0, 0.0, 0.5),
            //                 ..default()
            //             },
            //             speargun_arrow_trail: SpeargunArrowTrail,
            //             timer_despawn,
            //         },
            //         ZIndex(105),
            //         Name::new("speargun arrow trail"),
            //     ));
            // });
        }
    }
}

fn handle_trail_timers(
    mut commands: Commands,
    mut q_arrow_trails: Query<(Entity, &mut Sprite, &mut SpeargunArrowTrailDespawnTimer)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut timer) in q_arrow_trails.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
        }

        let opacity = (1.0 - timer.0.fraction()) * 0.5;
        sprite.color = Color::srgba(0.8, 0.0, 0.0, opacity);
    }
}

// -------
// Physics
// -------

pub fn handle_arrow_enemy_collisions(
    mut collision_events: MessageReader<CollisionEvent>,
    q_enemies: Query<(Entity, &Enemy)>,
    q_arrows: Query<(Entity, &SpeargunArrow)>,
    mut ev_enemy_hit: MessageWriter<EnemyHitEvent>,
) {
    for event in collision_events.read() {
        // println!("collision event: {:?}", event);
        if let CollisionEvent::Started(e1, e2, _) = event {
            let contact_1_enemy = q_enemies.get(*e1);
            let contact_2_enemy = q_enemies.get(*e2);
            let is_enemy_contact = contact_2_enemy.is_ok() || contact_1_enemy.is_ok();

            let contact_1_arrow = q_arrows.get(*e1);
            let contact_2_arrow = q_arrows.get(*e2);
            let is_arrow_contact = contact_1_arrow.is_ok() || contact_2_arrow.is_ok();

            if !(is_enemy_contact && is_arrow_contact) {
                continue;
            }

            let enemy_entity = match contact_1_enemy.is_ok() {
                true => contact_1_enemy.unwrap().0,
                false => contact_2_enemy.unwrap().0,
            };

            ev_enemy_hit.write(EnemyHitEvent {
                entity: enemy_entity,
                damage: 50,
            });
        }
    }
}

// ------
// Plugin
// ------

pub struct SpeargunPlugin;

impl Plugin for SpeargunPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpeargunTimer>()
            // Event Handlers
            .add_systems(
                Update,
                (
                    handle_speargun_attack_event,
                    handle_arrow_timers,
                    // handle_trail_timers,
                    handle_arrow_enemy_collisions,
                )
                    .run_if(in_state(GameState::GamePlay)),
            )
            .add_message::<SpeargunShootEvent>();
    }
}
