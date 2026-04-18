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
        Commands<'w, 's>,
        MessageReader<'w, 's, SpawnItemEvent>,
        Res<'w, LevelSelection>,
        Query<'w, 's, (Entity, &'w LevelIid)>,
        Query<'w, 's, &'w LdtkProjectHandle>,
        Res<'w, Assets<LdtkProject>>,
        Query<'w, 's, (Entity, &'w ChildOf, &'w Item)>,
        Query<'w, 's, (&'w Player, &'w Transform)>,
    );

    let mut state: SystemState<Params> = SystemState::new(world);

    let events_read = {
        let (_, ev_spawn_item, _, _, _, _, _, _) = state.get(world);
        let spawns: Vec<SpawnItemEvent> = ev_spawn_item.read().cloned().collect();
        spawns
    };

    if events_read.is_empty() {
        return;
    }

    let mut rng = ThreadRng::default();

    for ev_spawn in events_read {
        let (mut commands, _, level_selection, levels, projects, project_assets, q_items, q_player_query) = state.get_mut(world);

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
                    let (item_entity, item_parent, item_transform) = {
                        let mut found: Option<(Entity, Entity, Transform)> = None;
                        for (e, child_of, item, transform) in q_items.iter() {
                            if item.is_dummy && item.item_type == ev_spawn.item_type {
                                found = Some((e, child_of.parent(), *transform));
                                break;
                            }
                        }
                        match found {
                            Some(f) => f,
                            None => continue,
                        }
                    };

                    let mut item_position = player_translation;
                    while (player_translation - item_position).length()
                        < max_level_dimension / 3.0
                        || item_position.x < 24.0
                        || item_position.x > (level.px_wid as f32) - 24.0
                        || item_position.y < 24.0
                        || item_position.y > (level.px_hei as f32) - 24.0
                    {
                        let r = rng.random_range(0.0..1000.0);
                        let angle = rng.random_range(0.0..std::f32::consts::TAU);
                        item_position = player_translation
                            + Vec3::new(r * angle.sin(), r * angle.cos(), 0.0);
                    }

                    let new_entity = commands.spawn_empty().id();
                    let transform = Transform::from_translation(item_position).with_scale(Vec3::ONE * 0.5);

                    commands.entity(item_parent).add_child(new_entity);
                    commands.entity(new_entity).insert(Item {
                        is_dummy: false,
                        item_type: ev_spawn.item_type,
                    });
                    commands.entity(new_entity).insert(transform);

                    EntityCloner::build_opt_out(world).clone_entity(item_entity, new_entity);
                }
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
