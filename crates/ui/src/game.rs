use bevy::prelude::*;
use bevy::ui::Val;
use bevy_color::{Color, Srgba};

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

pub(crate) fn despawn_ui(mut commands: Commands, query: Query<Entity, With<UIGamePlay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn draw_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                bottom: Val::Px(0.0),
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba::NONE)),
            UIGamePlay,
            Name::new("ui face"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(125.0),
                    height: Val::Px(125.0),
                    margin: UiRect::top(Val::VMin(5.)),
                    ..default()
                },
                BackgroundColor(Color::Srgba(Srgba::WHITE)),
                ImageNode::new(asset_server.load("avatars/alextime.png")),
            ));
        });
    commands
        .spawn((
            Node {
                width: Val::Percent(50.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                bottom: Val::Px(35.0),
                left: Val::Px(20.0),
                padding: UiRect {
                    right: Val::Px(15.0),
                    ..default()
                },
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba::NONE)),
            UIGamePlay,
            Name::new("ui healthbar"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(20.0),
                        margin: UiRect::top(Val::VMin(5.)),
                        ..default()
                    },
                    BackgroundColor(Color::Srgba(Srgba::RED)),
                ))
                .insert(UIPlayerHealth);
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                bottom: Val::Px(25.0),
                right: Val::Px(5.0),
                padding: UiRect {
                    right: Val::Px(15.0),
                    ..default()
                },
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba::NONE)),
            UIGamePlay,
            Name::new("Weapon gun image"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(160.0),
                    height: Val::Px(22.5),
                    ..default()
                },
                BackgroundColor(Color::Srgba(Srgba::WHITE)),
                ImageNode::new(asset_server.load("sprites/speargun.png")),
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                bottom: Val::Px(15.0),
                right: Val::Px(5.0),
                padding: UiRect {
                    right: Val::Px(15.0),
                    ..default()
                },
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba::NONE)),
            UIGamePlay,
            Name::new("Weapon name"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("SPEARGUN"),
                TextFont {
                    font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::Srgba(Srgba::WHITE)),
                UIWeaponName,
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                bottom: Val::Px(100.0),
                right: Val::Px(5.0),
                padding: UiRect {
                    right: Val::Px(15.0),
                    ..default()
                },
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba::NONE)),
            UIGamePlay,
            Name::new("Weapon machete  image"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Px(160.0),
                    height: Val::Px(22.5),
                    ..default()
                },
                BackgroundColor(Color::Srgba(Srgba::WHITE)),
                ImageNode::new(asset_server.load("sprites/machete.png")),
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                bottom: Val::Px(85.0),
                right: Val::Px(5.0),
                padding: UiRect {
                    right: Val::Px(15.0),
                    ..default()
                },
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba::NONE)),
            UIGamePlay,
            Name::new("Weapon name"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("MACHETE"),
                TextFont {
                    font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::Srgba(Srgba::WHITE)),
                UIWeaponName,
            ));
        });

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba::NONE)),
            UIGamePlay,
            Name::new("Wave Text"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("ui wave text"),
                TextFont {
                    font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::Srgba(Srgba::WHITE)),
                UIGameplayWave,
            ));
        });

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexEnd,
                top: Val::Px(20.0),
                right: Val::Px(20.0),
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba::NONE)),
            UIGamePlay,
            Name::new("Wave Text"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("SCORE: 0"),
                TextFont {
                    font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::Srgba(Srgba::WHITE)),
                UIHighscore,
            ));
        });
}