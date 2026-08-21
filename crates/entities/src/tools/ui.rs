use crate::player::Player;
use crate::tools::Tool;
use bevy::color::palettes::basic::*;
use bevy::prelude::*;
use bevy::ui::Val;
use bevy_color::Color;
use lom_game::GameState;
use lom_ui::game::UIGamePlay;

const JUSTIFY_CONTENT_COLOR: Color = Color::srgb(0.102, 0.522, 1.);
const MARGIN: Val = Val::Px(12.);

#[derive(Component)]
pub struct UIToolChoose;

#[derive(Component)]
pub struct UIHovering;

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
                                Tool::None,
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
) {
    for (entity, interaction, mut background_color, children, tool, ui_tool_choose) in
        &mut interaction_query
    {
        if player.single().is_err() {
            return;
        }

        let mut player = player.single_mut().unwrap();

        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor::from(GREEN);
                player.choose_tool(tool.clone());
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor::from(RED);
                commands.entity(entity).insert(UIHovering {});
            }
            Interaction::None => {
                if player.tool == tool.clone() {
                    *background_color = BackgroundColor::from(WHITE);
                } else {
                    *background_color = BackgroundColor::from(JUSTIFY_CONTENT_COLOR);
                }

                commands.entity(entity).remove::<UIHovering>();
            }
        }
    }
}

fn tool_highlight_system(
    mut interaction_query: Query<(Entity, &mut BackgroundColor, &Tool), Without<UIHovering>>,
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

pub struct ToolUIPlugin;

impl Plugin for ToolUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GamePlay), spawn_tool_selection_ui)
            .add_systems(
                Update,
                (tool_selection_system, tool_highlight_system)
                    .chain()
                    .run_if(in_state(GameState::GamePlay)),
            );
    }
}
ay)),
            );
    }
}
tate::GamePlay)),
            );
    }
}
