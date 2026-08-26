use bevy::prelude::*;
use lom_game::GameState;

#[derive(Component)]
pub struct UIPlayerHealth;

#[derive(Component)]
pub struct UIGameOver;

#[derive(Component)]
pub struct UIGameplayWave;

#[derive(Component)]
pub struct UIWeaponName;

#[derive(Component)]
pub struct UIHighscore;

#[derive(Component)]
pub struct UIGamePlay;

#[derive(Component)]
pub struct UIToolChoose;

pub(crate) fn despawn_ui(mut commands: Commands, query: Query<Entity, With<UIGamePlay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn draw_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("here");

    // let font = asset_server.load("fonts/PixeloidMono-d94EV.ttf");
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                right: Val::Px(250.0),
                ..Default::default()
            },
            Name::from("logo::ui"),
        ))
        .with_children(|builder| {
            let mut transform = UiTransform::IDENTITY;
            transform.scale = Vec2::ONE * 5.0;

            builder.spawn((
                ImageNode::new(asset_server.load("sprites/logo.png")),
                transform,
            ));
        });
}

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GamePlay), draw_ui)
            .add_systems(OnExit(GameState::GamePlay), despawn_ui);
    }
}
