use crate::player::Player;
use bevy::color::palettes::basic::*;
use bevy::prelude::*;
use bevy::ui::Val;
use bevy_color::{Color, Srgba};
use lom_game::GameState;
use lom_ui::game::UIGamePlay;
use std::fmt;

const ALIGN_ITEMS_COLOR: Color = Color::srgb(1., 0.066, 0.349);
const JUSTIFY_CONTENT_COLOR: Color = Color::srgb(0.102, 0.522, 1.);
const MARGIN: Val = Val::Px(12.);

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Reflect)]
pub enum Tool {
    None,
    #[default]
    Shovel,
    Axe,
    Hammer,
    Pickaxe,
    WateringCan,
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tool::Shovel => write!(f, "Shovel"),
            Tool::Axe => write!(f, "Axe"),
            Tool::Hammer => write!(f, "Hammer"),
            Tool::Pickaxe => write!(f, "Pickaxe"),
            Tool::WateringCan => write!(f, "Wayering Can"),
            Tool::None => write!(f, "No Tool"),
        }
    }
}

#[derive(Component)]
pub struct UIToolChoose;

fn spawn_tool_selection_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                            spawn_tool_selection_ui_item(
                                builder,
                                font.clone(),
                                WHITE.into(),
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
    // mut text_query: Query<&mut Text>,
    mut player: Query<&mut Player>,
) {
    for (_, interaction, mut background_color, children, tool, ui_tool_choose) in
        &mut interaction_query
    {
        if player.single().is_err() {
            return;
        }

        // let mut text = text_query.get_mut(children[0]).unwrap();
        let mut player = player.single_mut().unwrap();

        match *interaction {
            Interaction::Pressed => {
                if player.try_choosing_tool(tool.clone()) {
                    *background_color = BackgroundColor::from(GREEN);
                }
            }
            Interaction::Hovered => {
                if player.can_choose_tool(tool.clone()) {
                    *background_color = BackgroundColor::from(RED);
                }
            }
            Interaction::None => {
                if player.tool == tool.clone() {
                    *background_color = BackgroundColor::from(WHITE);
                } else {
                    *background_color = BackgroundColor::from(JUSTIFY_CONTENT_COLOR);
                }
            }
        }
    }
}

pub struct ToolUIPlugin;

impl Plugin for ToolUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GamePlay), spawn_tool_selection_ui)
            .add_systems(
                Update,
                (tool_selection_system).run_if(in_state(GameState::GamePlay)),
            );
    }
}
