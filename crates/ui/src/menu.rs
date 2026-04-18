use bevy::prelude::*;
use lom_assets::loading::CutsceneAssets;
use lom_assets::loading::FontAssets;
use lom_game::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Menu),
            |mut commands: Commands, q_menu_components: Query<(Entity, &Menu)>| {
                for (e, _) in q_menu_components.iter() {
                    commands.entity(e).despawn();
                }
            },
        )
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::WHITE,
            hovered: Color::BLACK,
        }
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct StartGameButton;

fn setup_menu(
    mut commands: Commands,
    cutscene_assets: Res<CutsceneAssets>,
    font_assets: Res<FontAssets>,
) {
    info!("menu");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Menu,
            Name::new("cutscene image container"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                ImageNode::new(cutscene_assets.main_menu.clone()),
            ));
        });

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                margin: UiRect::right(Val::Percent(20.0)),
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    Node {
                        width: Val::Px(318.0),
                        height: Val::Px(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba_u8(0, 0, 0, 0)),
                    Button,
                    button_colors,
                    ChangeState(GameState::Cutscene),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("START"),
                        StartGameButton,
                        TextFont {
                            font: font_assets.pixeloid_mono.clone(),
                            font_size: 100.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

#[derive(Component)]
pub struct ChangeState(pub GameState);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &ButtonColors, Option<&ChangeState>),
        (Changed<Interaction>, With<Button>),
    >,
    mut start_game_button: Query<&mut TextColor, With<StartGameButton>>,
) {
    for (interaction, button_colors, change_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                }
            }
            Interaction::Hovered => {
                for mut text_color in start_game_button.iter_mut() {
                    text_color.0 = button_colors.hovered;
                }
            }
            Interaction::None => {
                for mut text_color in start_game_button.iter_mut() {
                    text_color.0 = button_colors.normal;
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
