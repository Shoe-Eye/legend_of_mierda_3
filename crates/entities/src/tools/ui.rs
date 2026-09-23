use crate::player::Player;
use crate::tools::actions::Action;
use crate::tools::{ChooseAction, ChooseTool, Tool};
use bevy::color::palettes::basic::*;
use bevy::prelude::*;
use bevy::ui::Val;
use bevy_color::Color;
use lom_game::{GameMode, GameWorldState};
use lom_ui::game::UIGamePlay;

const JUSTIFY_CONTENT_COLOR: Color = Color::srgb(0.102, 0.522, 1.);
const MARGIN: Val = Val::Px(12.);

#[derive(Component)]
pub struct UIToolChoose;

#[derive(Component)]
pub struct UIToolChooseMenu;

#[derive(Component)]
pub struct UIToolActionMenu;

#[derive(Component)]
pub struct UIToolActionChoose;

#[derive(Component)]
pub struct UIToolHovering;

#[derive(Component)]
pub struct UIToolActionHovering;

#[derive(Component)]
pub struct UIPlayerHealth;

#[derive(Component)]
pub struct UIGameWorldEpoch;

fn draw_spawn_tool_selection_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            UIToolChooseMenu,
            Name::from("tool_choose::ui"),
        ))
        .with_children(|builder| {
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
                            spawn_tool_selection_ui_item(
                                builder,
                                font.clone(),
                                WHITE.into(),
                                px(3.0).top(),
                                Tool::NoTool,
                            );
                            spawn_tool_selection_ui_item(
                                builder,
                                font.clone(),
                                JUSTIFY_CONTENT_COLOR,
                                px(3.0).top(),
                                Tool::Shovel,
                            );
                            spawn_tool_selection_ui_item(
                                builder,
                                font.clone(),
                                JUSTIFY_CONTENT_COLOR,
                                px(3.0).top(),
                                Tool::Axe,
                            );
                            spawn_tool_selection_ui_item(
                                builder,
                                font.clone(),
                                JUSTIFY_CONTENT_COLOR,
                                px(3.0).top(),
                                Tool::Hammer,
                            );
                            spawn_tool_selection_ui_item(
                                builder,
                                font.clone(),
                                JUSTIFY_CONTENT_COLOR,
                                px(3.0).top(),
                                Tool::Pickaxe,
                            );
                            spawn_tool_selection_ui_item(
                                builder,
                                font.clone(),
                                JUSTIFY_CONTENT_COLOR,
                                px(3.0).top(),
                                Tool::WateringCan,
                            );
                        });
                });
        });
}

fn draw_player_status_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                bottom: Val::Px(64.0),
                padding: MARGIN.all(),
                row_gap: MARGIN,
                ..Default::default()
            },
            UIGamePlay,
            UIToolChooseMenu,
            Name::from("player_status::ui"),
        ))
        .with_children(|builder| {
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
                            builder.spawn((
                                UIPlayerHealth,
                                Text::new(format!("Gennadiy Health: 100")),
                                TextFont::from(font),
                                TextColor::BLACK,
                            ));
                        });
                });
        });
}

fn draw_epoch_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/PixeloidMono-d94EV.ttf");
    commands
        .spawn((
            Node {
                // fill the entire window
                width: percent(100),
                height: px(50),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                padding: MARGIN.all(),
                row_gap: MARGIN,
                ..Default::default()
            },
            UIGamePlay,
            UIToolChooseMenu,
            Name::from("game_state::ui"),
        ))
        .with_children(|builder| {
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
                            align_items: AlignItems::Start,
                            justify_content: JustifyContent::FlexStart,
                            width: percent(100),
                            height: percent(100),
                            ..default()
                        },))
                        .with_children(|builder| {
                            builder.spawn((
                                UIGameWorldEpoch,
                                Text::new(format!("Game Epoch: 0")),
                                TextFont::from(font),
                                TextColor::WHITE,
                            ));
                        });
                });
        });
}

pub fn update_epoch_ui(
    mut q_game_world_epoch_ui: Query<(Entity, &mut Text, &UIGameWorldEpoch)>,
    game_world_state: Res<GameWorldState>,
) {
    for (_, mut text, _) in q_game_world_epoch_ui.iter_mut() {
        *text = Text::new(format!("GameWorld Epoch: {}", game_world_state.epoch));
    }
}

fn draw_tool_action_ui_item(
    builder: &mut ChildSpawnerCommands,
    font: Handle<Font>,
    background_color: Color,
    margin: UiRect,
    action: Action,
) {
    builder
        .spawn((
            Node {
                margin,
                padding: UiRect::axes(px(5), px(1)),
                ..default()
            },
            BackgroundColor(background_color),
            UIToolActionChoose,
            action,
            Interaction::default(),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(format!("{}", action)),
                TextFont::from(font),
                TextColor::BLACK,
            ));
        });
}

fn spawn_tool_selection_ui_item(
    builder: &mut ChildSpawnerCommands,
    font: Handle<Font>,
    background_color: Color,
    margin: UiRect,
    tool: Tool,
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
            tool,
            Interaction::default(),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(format!("{}", tool)),
                TextFont::from(font),
                TextColor::BLACK,
            ));
        });
}

fn tool_selection_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &Tool,
            &UIToolChoose,
        ),
        Changed<Interaction>,
    >,
    mut player: Query<&mut Player>,

    mut ew_choose_tool: MessageWriter<ChooseTool>,
) {
    for (entity, interaction, mut background_color, _children, tool, _ui_tool_choose) in
        &mut interaction_query
    {
        if player.single().is_err() {
            return;
        }

        let player = player.single_mut().unwrap();

        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor::from(GREEN);
                ew_choose_tool.write(ChooseTool { tool: tool.clone() });
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor::from(RED);
                commands.entity(entity).insert(UIToolHovering {});
            }
            Interaction::None => {
                if player.tool == tool.clone() {
                    *background_color = BackgroundColor::from(WHITE);
                } else {
                    *background_color = BackgroundColor::from(JUSTIFY_CONTENT_COLOR);
                }

                commands.entity(entity).remove::<UIToolHovering>();
            }
        }
    }
}

fn tool_action_selection_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &Action,
            &UIToolActionChoose,
        ),
        Changed<Interaction>,
    >,
    mut player: Query<&mut Player>,
    mut ew_choose_action: MessageWriter<ChooseAction>,
) {
    for (entity, interaction, mut background_color, _children, action, _ui_tool_choose) in
        &mut interaction_query
    {
        if player.single().is_err() {
            return;
        }

        let player = player.single_mut().unwrap();

        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor::from(GREEN);
                ew_choose_action.write(ChooseAction {
                    action: action.clone(),
                });
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor::from(RED);
                commands.entity(entity).insert(UIToolHovering {});
            }
            Interaction::None => {
                if player.action == Some(action.clone()) {
                    *background_color = BackgroundColor::from(WHITE);
                } else {
                    *background_color = BackgroundColor::from(JUSTIFY_CONTENT_COLOR);
                }

                commands.entity(entity).remove::<UIToolHovering>();
            }
        }
    }
}

fn tool_highlight_system(
    mut interaction_query: Query<(Entity, &mut BackgroundColor, &Tool), Without<UIToolHovering>>,
    player: Query<&Player>,
) {
    if player.single().is_err() {
        return;
    }

    let player = player.single().unwrap();

    for (_, mut background_color, tool) in interaction_query.iter_mut() {
        if tool.clone() != player.tool {
            *background_color = BackgroundColor::from(JUSTIFY_CONTENT_COLOR);
        }
    }
}

fn tool_action_highlight_system(
    mut interaction_query: Query<
        (Entity, &mut BackgroundColor, &Action),
        Without<UIToolActionHovering>,
    >,
    player: Query<&Player>,
) {
    if player.single().is_err() {
        return;
    }

    let player = player.single().unwrap();

    if player.action.is_none() {
        return;
    }

    for (_, mut background_color, action) in interaction_query.iter_mut() {
        if action.clone() != player.action.unwrap() {
            *background_color = BackgroundColor::from(JUSTIFY_CONTENT_COLOR);
        }
    }
}

pub(crate) fn spawn_tool_action_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    actions: Vec<Action>,

    q_action_menus: Query<(Entity, &UIToolActionMenu)>,
) {
    for (entity, _) in q_action_menus.iter() {
        commands.entity(entity).despawn();
    }

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
                top: Val::Px(16.0),
                padding: MARGIN.all(),
                row_gap: MARGIN,
                ..Default::default()
            },
            UIGamePlay,
            UIToolActionMenu,
            Name::from("tool_action::ui"),
        ))
        .with_children(|builder| {
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
                            actions.iter().for_each(|action| {
                                draw_tool_action_ui_item(
                                    builder,
                                    font.clone(),
                                    WHITE.into(),
                                    px(3.0).top(),
                                    action.clone(),
                                );
                            });
                        });
                });
        });
}

pub struct ToolUIPlugin;

impl Plugin for ToolUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameMode::GamePlay),
            (
                draw_spawn_tool_selection_ui,
                draw_player_status_ui,
                draw_epoch_ui,
            ),
        )
        .add_systems(
            Update,
            (
                tool_selection_system,
                tool_highlight_system,
                tool_action_selection_system,
                tool_action_highlight_system,
            )
                .chain()
                .run_if(in_state(GameMode::GamePlay)),
        )
        .add_systems(
            Update,
            (update_epoch_ui,).run_if(in_state(GameMode::GamePlay)),
        );
    }
}
