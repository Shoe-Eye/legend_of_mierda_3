use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use lom_assets::loading::{AudioAssets, FontAssets};
use lom_game::GameMode;
use lom_ui::game::UIGameOver;

#[derive(Message, Clone)]
pub struct GameOverEvent;

#[derive(Message, Clone)]
pub struct GameWinEvent;

#[derive(Component)]
pub struct UIGameOverButton;

#[derive(Component)]
pub struct UIGameOverText;

pub fn handle_game_over(
    mut ev_game_over: MessageReader<GameOverEvent>,
    mut q_ui_game_over: Query<(&mut Visibility, &UIGameOver)>,
    mut next_state: ResMut<NextState<GameMode>>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut text_query: Query<(&mut Text, &UIGameOverText)>,
) {
    for _ in ev_game_over.read() {
        for (mut visibility, _) in q_ui_game_over.iter_mut() {
            *visibility = Visibility::Visible;
        }

        for (mut text, _) in text_query.iter_mut() {
            text.0 = "JUEGO PERDIDO".to_string();
        }

        // audio.play(audio_assets.game_over.clone());

        next_state.set(GameMode::GameOver);
    }
}

pub fn handle_game_win(
    mut ev_game_over: MessageReader<GameWinEvent>,
    mut q_ui_game_over: Query<(&mut Visibility, &UIGameOver)>,
    mut next_state: ResMut<NextState<GameMode>>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut text_query: Query<(&mut Text, &UIGameOverText)>,
) {
    for _ in ev_game_over.read() {
        for (mut visibility, _) in q_ui_game_over.iter_mut() {
            *visibility = Visibility::Visible;
        }

        for (mut text, _) in text_query.iter_mut() {
            text.0 = "YOU WON".to_string();
        }

        // audio.play(audio_assets.victory.clone());

        next_state.set(GameMode::GameOver);
    }
}

#[allow(dead_code)]
pub fn despawn_ui(mut commands: Commands, query: Query<Entity, With<UIGameOver>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[allow(dead_code)]
pub fn draw_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                ..default()
            },
            UIGameOver,
            Name::new("ui game over"),
        ))
        .with_children(|parent| {
            parent.spawn((Text::from("  JUEGO\nTERMINADO"), UIGameOverText));

            parent
                .spawn((
                    Node {
                        width: Val::Px(318.0),
                        height: Val::Px(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(100.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba_u8(0, 0, 0, 255)),
                    Interaction::None,
                    Button,
                    UIGameOverButton,
                ))
                .with_children(|button| {
                    button.spawn((Text::from("REINICIAR"),));
                });
        });
}

fn handle_start_over(
    mut next_state: ResMut<NextState<GameMode>>,
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (_, interaction) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                next_state.set(GameMode::GamePlay);
            }
            _ => {}
        }
    }
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMode::GameOver), draw_ui)
            .add_systems(OnExit(GameMode::GameOver), despawn_ui)
            .add_systems(
                Update,
                handle_start_over.run_if(in_state(GameMode::GameOver)),
            )
            .add_systems(
                Update,
                (
                    handle_game_over.run_if(in_state(GameMode::GamePlay)),
                    handle_game_win.run_if(in_state(GameMode::GamePlay)),
                ),
            );
    }
}
