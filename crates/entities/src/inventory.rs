use bevy::prelude::*;
use bevy::ui::Val;
use lom_assets::loading::{StaticSpriteAssets, StaticSpriteAtlasLayouts};
use lom_game::GameMode;
use lom_ui::game::UIGamePlay;

use crate::level::plant::PlantType;
use crate::player::{Player, PlayerInventory};

const HOTBAR_BACKGROUND: Color = Color::srgba(0.15, 0.12, 0.10, 0.85);
const SLOT_BACKGROUND: Color = Color::srgba(0.0, 0.0, 0.0, 0.45);
const SLOT_BORDER: Color = Color::srgba(0.75, 0.75, 0.75, 1.0);

#[derive(Component)]
pub struct InventoryUI;

#[derive(Component)]
pub struct InventorySlot;

pub fn count_plants(inventory: &PlayerInventory) -> Vec<(PlantType, usize)> {
    let mut counts: Vec<(PlantType, usize)> = Vec::new();

    for plant in &inventory.plants {
        match counts.iter_mut().find(|(plant_type, _)| *plant_type == plant.plant_type) {
            Some((_, count)) => *count += 1,
            None => counts.push((plant.plant_type, 1)),
        }
    }

    counts
}

pub fn plant_icon_index(plant_type: PlantType) -> usize {
    match plant_type {
        PlantType::Watermelon => 5,
    }
}

fn spawn_inventory_slot(
    builder: &mut ChildSpawnerCommands,
    plant_type: PlantType,
    count: usize,
    asset_server: &Res<AssetServer>,
    static_sprite_assets: &Res<StaticSpriteAssets>,
    sprite_layouts: &Res<StaticSpriteAtlasLayouts>,
) {
    let font = asset_server.load("fonts/PixeloidMono-d94EV.ttf");

    builder
        .spawn((
            Node {
                width: Val::Px(56.0),
                height: Val::Px(56.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(SLOT_BACKGROUND),
            BorderColor::all(SLOT_BORDER),
            InventorySlot,
            Name::new("inventory_slot"),
        ))
        .with_children(|builder| {
            builder.spawn((
                Node {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    ..default()
                },
                ImageNode {
                    image_mode: NodeImageMode::Stretch,
                    ..ImageNode::from_atlas_image(
                        static_sprite_assets.watermelon.clone(),
                        TextureAtlas {
                            index: plant_icon_index(plant_type),
                            layout: sprite_layouts.watermelon.clone(),
                        },
                    )
                },
                Name::new("inventory_icon"),
            ));

            builder.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(2.0),
                    right: Val::Px(4.0),
                    ..default()
                },
                Text::new(format!("{}", count)),
                TextFont::from(font),
                TextColor::WHITE,
                Name::new("inventory_count"),
            ));
        });
}

pub fn spawn_inventory_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            bottom: Val::Px(120.0),
            column_gap: Val::Px(6.0),
            padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(HOTBAR_BACKGROUND),
        UIGamePlay,
        InventoryUI,
        Name::from("inventory::ui"),
    ));
}

pub fn update_inventory_ui(
    mut commands: Commands,
    mut previous: Local<Option<Vec<(PlantType, usize)>>>,
    q_player: Query<&Player>,
    q_inventory_ui: Query<Entity, With<InventoryUI>>,
    q_inventory_slots: Query<Entity, (With<InventorySlot>, Without<InventoryUI>)>,
    asset_server: Res<AssetServer>,
    static_sprite_assets: Res<StaticSpriteAssets>,
    sprite_layouts: Res<StaticSpriteAtlasLayouts>,
) {
    let Ok(player) = q_player.single() else {
        return;
    };

    let current = count_plants(&player.inventory);

    if previous.as_ref() == Some(&current) {
        return;
    }

    *previous = Some(current.clone());

    let Ok(inventory_ui_entity) = q_inventory_ui.single() else {
        return;
    };

    for slot_entity in q_inventory_slots.iter() {
        commands.entity(slot_entity).despawn();
    }

    commands
        .entity(inventory_ui_entity)
        .with_children(|builder| {
            for (plant_type, count) in current {
                spawn_inventory_slot(
                    builder,
                    plant_type,
                    count,
                    &asset_server,
                    &static_sprite_assets,
                    &sprite_layouts,
                );
            }
        });
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMode::GamePlay), spawn_inventory_ui)
            .add_systems(
                Update,
                update_inventory_ui.run_if(in_state(GameMode::GamePlay)),
            );
    }
}