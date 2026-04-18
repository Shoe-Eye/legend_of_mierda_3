use crate::player::Player;
use bevy::prelude::*;
use bevy_ecs::entity::EntityCloner;
use bevy_ecs::system::SystemState;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use lom_assets::load_texture_atlas_layout;
use lom_assets::sprites::{BIBORAN_ASSET_SHEET, PIZZA_ASSET_SHEET};
use lom_ldtk::physics::ColliderBundle;
use rand::rngs::ThreadRng;
use rand::RngExt;

#[derive(Clone, Copy, PartialEq, Debug, Default, Component, Reflect)]
pub enum ItemType {
    #[default]
    Pizza,
    Biboran,
}

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Item {
    pub is_dummy: bool,
    pub item_type: ItemType,
}

#[derive(Clone, Default, Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub collider_bundle: ColliderBundle,
    pub sensor: Sensor,
}

pub fn create_item_bundle(
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlasLayout>,
    is_dummy: bool,
    item_type: ItemType,
) -> ItemBundle {
    let rotation_constraints = LockedAxes::ROTATION_LOCKED;

    let collider_bundle = match item_type {
        ItemType::Pizza => ColliderBundle {
            collider: Collider::cuboid(8., 8.),
            rigid_body: RigidBody::Dynamic,
            friction: Friction {
                coefficient: 20.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            rotation_constraints,
            ..Default::default()
        },
        ItemType::Biboran => ColliderBundle {
            collider: Collider::cuboid(8., 16.),
            rigid_body: RigidBody::Dynamic,
            friction: Friction {
                coefficient: 20.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            rotation_constraints,
            ..Default::default()
        },
    };

    let _atlas_handle = match item_type {
        ItemType::Pizza => load_texture_atlas_layout(
            PIZZA_ASSET_SHEET.to_string(),
            asset_server,
            1,
            1,
            Vec2::ONE * 16.,
        ),
        ItemType::Biboran => load_texture_atlas_layout(
            BIBORAN_ASSET_SHEET.to_string(),
            asset_server,
            1,
            1,
            Vec2::ONE * 32.,
        ),
    };

    ItemBundle {
        collider_bundle,
        item: Item {
            is_dummy,
            item_type,
        },
        sensor: Sensor {},
    }
}

// ------
// Events
// ------

#[derive(Message, Clone)]
pub struct ItemStepOverEvent {
    pub entity: Entity,
    pub item_type: ItemType,
}

#[derive(Message, Clone)]
pub struct SpawnItemEvent {
    pub count: u32,
    pub item_type: ItemType,
}

// --------------
// Event Handlers
// --------------

pub fn event_spawn_item(world: &mut World) {
    type Params<'w, 's> = (
        MessageReader<'w, 's, SpawnItemEvent>,
        Res<'w, LevelSelection>,
        Query<'w, 's, (Entity, &'static LevelIid)>,
        Query<'w, 's, &'static LdtkProjectHandle>,
        Res<'w, Assets<LdtkProject>>,
        Query<'w, 's, (Entity, &'static ChildOf, &'static Item)>,
        Query<'w, 's, (&'static Player, &'static Transform)>,
    );

    let mut state: SystemState<Params> = SystemState::new(world);

    // 1. Drain events into owned data
    let events: Vec<SpawnItemEvent> = {
        let (mut ev, ..) = state.get_mut(world);
        ev.read().cloned().collect()
    };

    if events.is_empty() {
        return;
    }

    let mut rng = ThreadRng::default();

    for ev_spawn in &events {
        // 2. Read all query data into owned values, drop borrow completely
        let (player_translation, level_bounds, dummy_candidates) = {
            let (_, level_selection, levels, projects, project_assets, q_items, players) =
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

            let level_bounds: Vec<(i32, i32)> = levels
                .iter()
                .filter_map(|(_, iid)| {
                    let level = project.get_raw_level_by_iid(iid.get())?;
                    if level_selection.is_match(
                        &LevelIndices {
                            level: 0,
                            ..default()
                        },
                        level,
                    ) {
                        Some((level.px_wid, level.px_hei))
                    } else {
                        None
                    }
                })
                .collect();

            let dummy_candidates: Vec<(Entity, Entity)> = q_items
                .iter()
                .filter(|(_, _, item)| item.is_dummy && item.item_type == ev_spawn.item_type)
                .map(|(e, child_of, _)| (e, child_of.parent()))
                .collect();

            (player_translation, level_bounds, dummy_candidates)
        }; // <-- ALL state borrows dropped here

        for (px_wid, px_hei) in &level_bounds {
            let px_wid = *px_wid;
            let px_hei = *px_hei;
            let max_level_dimension = px_wid.max(px_hei) as f32;

            for _ in 0..ev_spawn.count {
                let (item_entity, parent_entity) = match dummy_candidates.first() {
                    Some(&d) => d,
                    None => continue,
                };

                // Generate position
                let mut item_position = player_translation;
                while (player_translation - item_position).length() < max_level_dimension / 3.0
                    || item_position.x < 24.0
                    || item_position.x > px_wid as f32 - 24.0
                    || item_position.y < 24.0
                    || item_position.y > px_hei as f32 - 24.0
                {
                    let r = rng.random_range(0.0..1000.0);
                    let angle = rng.random_range(0.0..std::f32::consts::TAU);
                    item_position =
                        player_translation + Vec3::new(r * angle.sin(), r * angle.cos(), 0.0);
                }

                // 3. Spawn directly on world — entity is real immediately, no Commands needed
                let new_entity = world
                    .spawn((
                        Item {
                            is_dummy: false,
                            item_type: ev_spawn.item_type,
                        },
                        Transform::from_translation(item_position).with_scale(Vec3::ONE * 0.5),
                    ))
                    .id();

                world.entity_mut(parent_entity).add_child(new_entity);

                // 4. No borrow conflict — world is fully free here
                EntityCloner::build_opt_out(world).clone_entity(item_entity, new_entity);
            }
        }
    }
}

// -------
// Physics
// -------

pub fn handle_player_item_collision(
    mut collision_events: MessageReader<CollisionEvent>,
    mut q_items: Query<(Entity, &Item)>,
    q_player: Query<(Entity, &mut Player)>,
    mut ev_item_step_over: MessageWriter<ItemStepOverEvent>,
) {
    for (player_entity, _) in q_player.iter() {
        for event in collision_events.read() {
            for (e_item, item) in q_items.iter_mut() {
                if let CollisionEvent::Started(e1, e2, _) = event {
                    if e1.index() == e_item.index() && e2.index() == player_entity.index() {
                        ev_item_step_over.write(ItemStepOverEvent {
                            entity: e_item,
                            item_type: item.item_type,
                        });

                        return;
                    }

                    if e2.index() == e_item.index() && e1.index() == player_entity.index() {
                        ev_item_step_over.write(ItemStepOverEvent {
                            entity: e_item,
                            item_type: item.item_type,
                        });

                        return;
                    }
                }
            }
        }
    }
}

// ------
// Plugin
// ------

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnItemEvent>()
            .add_message::<ItemStepOverEvent>()
            .add_systems(Update, (handle_player_item_collision, event_spawn_item));
    }
}
