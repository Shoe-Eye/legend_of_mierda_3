use bevy::color::palettes::basic::*;
use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy::ui::Val;
use bevy_color::{Color, Srgba};
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
    // commands
    //     .spawn((
    //         Node {
    //             width: Val::Percent(100.0),
    //             position_type: PositionType::Absolute,
    //             justify_content: JustifyContent::Center,
    //             bottom: Val::Px(0.0),
    //             align_items: AlignItems::FlexStart,
    //             ..default()
    //         },
    //         // BackgroundColor(Color::Srgba(Srgba::NONE)),
    //         UIGamePlay,
    //         Name::new("ui face"),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn((
    //             Node {
    //                 width: Val::Px(125.0),
    //                 height: Val::Px(125.0),
    //                 margin: UiRect::top(Val::VMin(5.)),
    //                 ..default()
    //             },
    //             // BackgroundColor(Color::Srgba(Srgba::WHITE)),
    //             ImageNode::new(asset_server.load("avatars/alextime.png")),
    //         ));
    //     });
    // commands
    //     .spawn((
    //         Node {
    //             width: Val::Percent(50.0),
    //             position_type: PositionType::Absolute,
    //             justify_content: JustifyContent::FlexStart,
    //             bottom: Val::Px(35.0),
    //             left: Val::Px(20.0),
    //             padding: UiRect {
    //                 right: Val::Px(15.0),
    //                 ..default()
    //             },
    //             align_items: AlignItems::FlexStart,
    //             ..default()
    //         },
    //         // BackgroundColor(Color::Srgba(Srgba::NONE)),
    //         UIGamePlay,
    //         Name::new("ui healthbar"),
    //     ))
    //     .with_children(|parent| {
    //         parent
    //             .spawn((
    //                 Node {
    //                     width: Val::Percent(100.0),
    //                     height: Val::Px(20.0),
    //                     margin: UiRect::top(Val::VMin(5.)),
    //                     ..default()
    //                 },
    //                 BackgroundColor(Color::Srgba(Srgba::RED)),
    //             ))
    //             .insert(UIPlayerHealth);
    //     });

    // commands
    //     .spawn((
    //         Node {
    //             position_type: PositionType::Absolute,
    //             justify_content: JustifyContent::FlexStart,
    //             bottom: Val::Px(25.0),
    //             right: Val::Px(5.0),
    //             padding: UiRect {
    //                 right: Val::Px(15.0),
    //                 ..default()
    //             },
    //             align_items: AlignItems::FlexStart,
    //             ..default()
    //         },
    //         // BackgroundColor(Color::Srgba(Srgba::NONE)),
    //         UIGamePlay,
    //         Name::new("Weapon gun image"),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn((
    //             Node {
    //                 width: Val::Px(160.0),
    //                 height: Val::Px(22.5),
    //                 ..default()
    //             },
    //             // BackgroundColor(Color::Srgba(Srgba::WHITE)),
    //             ImageNode::new(asset_server.load("sprites/speargun.png")),
    //         ));
    //     });

    // commands
    //     .spawn((
    //         Node {
    //             position_type: PositionType::Absolute,
    //             justify_content: JustifyContent::FlexStart,
    //             bottom: Val::Px(15.0),
    //             right: Val::Px(5.0),
    //             padding: UiRect {
    //                 right: Val::Px(15.0),
    //                 ..default()
    //             },
    //             align_items: AlignItems::FlexStart,
    //             ..default()
    //         },
    //         // BackgroundColor(Color::Srgba(Srgba::NONE)),
    //         UIGamePlay,
    //         Name::new("Weapon name"),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn((
    //             Text::new("SPEARGUN"),
    //             TextFont {
    //                 font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
    //                 font_size: 20.0,
    //                 ..default()
    //             },
    //             TextColor(Color::Srgba(Srgba::WHITE)),
    //             UIWeaponName,
    //         ));
    //     });

    // commands
    //     .spawn((
    //         Node {
    //             position_type: PositionType::Absolute,
    //             justify_content: JustifyContent::FlexStart,
    //             bottom: Val::Px(100.0),
    //             right: Val::Px(5.0),
    //             padding: UiRect {
    //                 right: Val::Px(15.0),
    //                 ..default()
    //             },
    //             align_items: AlignItems::FlexStart,
    //             ..default()
    //         },
    //         // BackgroundColor(Color::Srgba(Srgba::NONE)),
    //         UIGamePlay,
    //         Name::new("Weapon machete image"),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn((
    //             Node {
    //                 width: Val::Px(160.0),
    //                 height: Val::Px(22.5),
    //                 ..default()
    //             },
    //             // // BackgroundColor(Color::Srgba(Srgba::WHITE)),
    //             ImageNode::new(asset_server.load("sprites/machete.png")),
    //         ));
    //     });

    // commands
    //     .spawn((
    //         Node {
    //             position_type: PositionType::Absolute,
    //             justify_content: JustifyContent::FlexStart,
    //             bottom: Val::Px(85.0),
    //             right: Val::Px(5.0),
    //             padding: UiRect {
    //                 right: Val::Px(15.0),
    //                 ..default()
    //             },
    //             align_items: AlignItems::FlexStart,
    //             ..default()
    //         },
    //         // // BackgroundColor(Color::Srgba(Srgba::NONE)),
    //         UIGamePlay,
    //         Name::new("Weapon name"),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn((
    //             Text::new("MACHETE"),
    //             TextFont {
    //                 font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
    //                 font_size: 20.0,
    //                 ..default()
    //             },
    //             TextColor(Color::Srgba(Srgba::WHITE)),
    //             UIWeaponName,
    //         ));
    //     });

    // commands
    //     .spawn((
    //         Node {
    //             width: Val::Percent(100.0),
    //             position_type: PositionType::Absolute,
    //             justify_content: JustifyContent::FlexStart,
    //             top: Val::Px(20.0),
    //             left: Val::Px(20.0),
    //             align_items: AlignItems::FlexStart,
    //             ..default()
    //         },
    //         // // BackgroundColor(Color::Srgba(Srgba::NONE)),
    //         UIGamePlay,
    //         Name::new("Wave Text"),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn((
    //             Text::new("ui wave text"),
    //             TextFont {
    //                 font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
    //                 font_size: 20.0,
    //                 ..default()
    //             },
    //             TextColor(Color::Srgba(Srgba::WHITE)),
    //             UIGameplayWave,
    //         ));
    //     });

    // commands
    //     .spawn((
    //         Node {
    //             width: Val::Percent(100.0),
    //             position_type: PositionType::Absolute,
    //             justify_content: JustifyContent::FlexEnd,
    //             top: Val::Px(20.0),
    //             right: Val::Px(20.0),
    //             align_items: AlignItems::FlexStart,
    //             ..default()
    //         },
    //         // // BackgroundColor(Color::Srgba(Srgba::NONE)),
    //         UIGamePlay,
    //         Name::new("Wave Text"),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn((
    //             Text::new("SCORE: 0"),
    //             TextFont {
    //                 font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
    //                 font_size: 30.0,
    //                 ..default()
    //             },
    //             TextColor(Color::Srgba(Srgba::WHITE)),
    //             UIHighscore,
    //         ));
    //     });
}

const ALIGN_ITEMS_COLOR: Color = Color::srgb(1., 0.066, 0.349);
const JUSTIFY_CONTENT_COLOR: Color = Color::srgb(0.102, 0.522, 1.);
const MARGIN: Val = Val::Px(12.);

fn spawn_tool_selection(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/PixeloidMono-d94EV.ttf");
    commands
        .spawn((
            Node {
                // fill the entire window
                width: percent(100),
                height: px(50),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                bottom: Val::Px(16.0),
                padding: MARGIN.all(),
                row_gap: MARGIN,
                ..Default::default()
            },
            UIGamePlay,
            Name::from("tool_choose::ui"),
        ))
        .with_children(|builder| {
            // spawn the key
            builder.spawn(Node {
                flex_direction: FlexDirection::Row,
                ..default()
            });

            builder
                .spawn((Node {
                    width: percent(100),
                    height: percent(100),
                    flex_direction: FlexDirection::Column,
                    row_gap: MARGIN,
                    ..default()
                },))
                .with_children(|builder| {
                    builder
                        .spawn((Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: percent(100),
                            height: percent(100),
                            ..default()
                        },))
                        .with_children(|builder| {
                            spawn_tool_choose_item(
                                builder,
                                font.clone(),
                                JUSTIFY_CONTENT_COLOR,
                                px(3.0).top(),
                                "Shovel",
                            );
                            spawn_tool_choose_item(
                                builder,
                                font.clone(),
                                JUSTIFY_CONTENT_COLOR,
                                px(3.0).top(),
                                "Axe",
                            );
                            spawn_tool_choose_item(
                                builder,
                                font.clone(),
                                JUSTIFY_CONTENT_COLOR,
                                px(3.0).top(),
                                "Hammer",
                            );
                            spawn_tool_choose_item(
                                builder,
                                font.clone(),
                                JUSTIFY_CONTENT_COLOR,
                                px(3.0).top(),
                                "Pickaxe",
                            );
                            spawn_tool_choose_item(
                                builder,
                                font.clone(),
                                JUSTIFY_CONTENT_COLOR,
                                px(3.0).top(),
                                "Watering Can",
                            );
                        });
                });
        });
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn spawn_tool_choose_item(
    builder: &mut ChildSpawnerCommands,
    font: Handle<Font>,
    background_color: Color,
    margin: UiRect,
    text: &str,
) {
    builder
        .spawn((
            Node {
                margin,
                padding: UiRect::axes(px(5), px(1)),
                ..default()
            },
            BackgroundColor(background_color),
            UIToolChoose,
        ))
        .with_children(|builder| {
            builder.spawn((Text::new(text), TextFont::from(font), TextColor::BLACK));
        });
}

fn tool_selection_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &UIToolChoose, &Children),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for (entity, interaction, ui_tool_choose, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        println!("tool ui iteraction");

        match *interaction {
            Interaction::Pressed => {
                // input_focus.set(entity);
                **text = "Press".to_string();
                // *color = PRESSED_BUTTON.into();
                // *border_color = BorderColor::all(RED);

                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                // button.set_changed();
            }
            Interaction::Hovered => {
                // input_focus.set(entity);
                // UIToolChooseall(Color::WHITE);
                // button.set_changed();
            }
            Interaction::None => {
                // input_focus.clear();
                **text = "Button".to_string();
                // *color = NORMAL_BUTTON.into();
                // *border_color = BorderColor::all(Color::BLACK);
            }
        }
    }
}

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GamePlay), draw_ui)
            .add_systems(OnEnter(GameState::GamePlay), spawn_tool_selection)
            .add_systems(OnExit(GameState::GamePlay), despawn_ui)
            .add_systems(
                Update,
                (tool_selection_system).run_if(in_state(GameState::GamePlay)),
            );
    }
}
