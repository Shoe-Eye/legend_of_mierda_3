use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::prelude::{Audio, AudioControl};
use lom_assets::loading::FontAssets;

use crate::voice::{DialogueVoice, voice_jitter, voice_rate_for};
use crate::{ActiveDialogue, AdvanceDialogue, DialogueSettings, EndNPCDialogue, StartNPCDialogue};

const BUBBLE_BACKGROUND: Color = Color::srgb(0.07, 0.08, 0.24);
const BUBBLE_BORDER: Color = Color::srgb(0.95, 0.95, 0.95);
const SPEAKER_COLOR: Color = Color::srgb(1.0, 0.85, 0.3);
const TEXT_COLOR: Color = Color::WHITE;
const BUBBLE_MAX_WIDTH: f32 = 320.0;
const BUBBLE_BORDER_WIDTH: f32 = 3.0;
const BUBBLE_PADDING_X: f32 = 10.0;
const TEXT_FONT_SIZE: f32 = 16.0;
/// PixeloidMono is monospace: every glyph advances 1224 / 1836 em.
const GLYPH_ADVANCE_EM: f32 = 1224.0 / 1836.0;
/// World-space offset above the NPC's origin where the bubble tail points.
const BUBBLE_WORLD_OFFSET: Vec3 = Vec3::new(0.0, 18.0, 0.0);
/// Pixel triangle under the bubble: row widths, top to bottom.
const TAIL_ROWS: [f32; 4] = [14.0, 10.0, 6.0, 2.0];
const TAIL_ROW_HEIGHT: f32 = 2.0;
const SCREEN_MARGIN: f32 = 8.0;

// ----------
// Components
// ----------

/// Root UI node of the speech bubble; holds the dialogue state.
#[derive(Component)]
pub struct UIDialogue {
    pub npc: Entity,
    pub lines: Vec<String>,
    pub line: usize,
    /// Number of characters of the current line that are visible.
    pub revealed: usize,
    /// Fractional characters accumulated by the typewriter. Goes negative during pauses.
    pub reveal_budget: f32,
    pub voice_rate: f64,
}

impl UIDialogue {
    fn current_line(&self) -> &str {
        &self.lines[self.line]
    }

    fn line_len(&self) -> usize {
        self.current_line().chars().count()
    }

    fn line_complete(&self) -> bool {
        self.revealed >= self.line_len()
    }
}

#[derive(Component)]
pub struct UIDialogueRevealedText;

/// Not-yet-revealed part of the line, drawn transparent so word wrapping
/// doesn't jump around while letters appear.
#[derive(Component)]
pub struct UIDialogueHiddenText;

/// Text node holding the line. Gets an explicit width so the layout measures
/// the wrapped height correctly (with only a max width the text overflowed).
#[derive(Component)]
pub struct UIDialogueBody;

#[derive(Component)]
pub struct UIDialogueTail;

#[derive(Component)]
pub struct UIDialogueAdvanceIndicator;

// -------
// Systems
// -------

pub fn handle_start_dialogue(
    mut commands: Commands,
    mut mr_start_dialogue: MessageReader<StartNPCDialogue>,
    mut active: ResMut<ActiveDialogue>,
    settings: Res<DialogueSettings>,
    font_assets: Res<FontAssets>,
) {
    for event in mr_start_dialogue.read() {
        if active.is_active() || event.lines.is_empty() {
            continue;
        }

        active.npc = Some(event.entity);

        let dialogue = UIDialogue {
            npc: event.entity,
            lines: event.lines.clone(),
            line: 0,
            revealed: 0,
            reveal_budget: 0.0,
            voice_rate: voice_rate_for(&event.speaker),
        };
        let revealed = if settings.typewriter {
            0
        } else {
            dialogue.line_len()
        };

        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    min_width: Val::Px(120.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::axes(Val::Px(BUBBLE_PADDING_X), Val::Px(8.0)),
                    border: UiRect::all(Val::Px(BUBBLE_BORDER_WIDTH)),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    row_gap: Val::Px(4.0),
                    ..default()
                },
                BackgroundColor(BUBBLE_BACKGROUND),
                BorderColor::all(BUBBLE_BORDER),
                // hidden until layout has computed its size and it's been positioned
                Visibility::Hidden,
                ZIndex(100),
                UIDialogue {
                    revealed,
                    ..dialogue
                },
                Name::new("dialogue bubble"),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(event.speaker.clone()),
                    TextFont {
                        font: font_assets.pixeloid_mono.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(SPEAKER_COLOR),
                ));

                parent
                    .spawn((
                        Text::default(),
                        TextFont {
                            font: font_assets.pixeloid_mono.clone(),
                            font_size: TEXT_FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            width: Val::Px(text_width_for(&event.lines[0])),
                            margin: UiRect::bottom(Val::Px(6.0)),
                            ..default()
                        },
                        UIDialogueBody,
                    ))
                    .with_children(|text| {
                        text.spawn((
                            TextSpan::default(),
                            TextFont {
                                font: font_assets.pixeloid_mono.clone(),
                                font_size: TEXT_FONT_SIZE,
                                ..default()
                            },
                            TextColor(TEXT_COLOR),
                            UIDialogueRevealedText,
                        ));
                        text.spawn((
                            TextSpan::default(),
                            TextFont {
                                font: font_assets.pixeloid_mono.clone(),
                                font_size: TEXT_FONT_SIZE,
                                ..default()
                            },
                            TextColor(Color::NONE),
                            UIDialogueHiddenText,
                        ));
                    });

                // blinking "press E" marker in the bottom-right corner
                parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(6.0),
                        bottom: Val::Px(4.0),
                        width: Val::Px(6.0),
                        height: Val::Px(6.0),
                        ..default()
                    },
                    BackgroundColor(SPEAKER_COLOR),
                    Visibility::Hidden,
                    UIDialogueAdvanceIndicator,
                ));

                // stepped pixel tail pointing down at the NPC
                parent
                    .spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(100.0),
                            // absolute offsets are measured from the padding box, skip the bottom border
                            margin: UiRect::top(Val::Px(BUBBLE_BORDER_WIDTH)),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: Val::Px(TAIL_ROWS[0]),
                            ..default()
                        },
                        UIDialogueTail,
                    ))
                    .with_children(|tail| {
                        for width in TAIL_ROWS {
                            tail.spawn((
                                Node {
                                    width: Val::Px(width),
                                    height: Val::Px(TAIL_ROW_HEIGHT),
                                    ..default()
                                },
                                BackgroundColor(BUBBLE_BORDER),
                            ));
                        }
                    });
            });
    }
}

pub fn handle_advance_dialogue(
    mut commands: Commands,
    mut mr_advance: MessageReader<AdvanceDialogue>,
    mut mw_end: MessageWriter<EndNPCDialogue>,
    mut active: ResMut<ActiveDialogue>,
    settings: Res<DialogueSettings>,
    mut q_dialogue: Query<(Entity, &mut UIDialogue)>,
) {
    // multiple presses in one frame count as one
    if mr_advance.read().count() == 0 {
        return;
    }

    for (entity, mut dialogue) in q_dialogue.iter_mut() {
        if !dialogue.line_complete() {
            // Ace Attorney style: first press skips the typing
            dialogue.revealed = dialogue.line_len();
            continue;
        }

        if dialogue.line + 1 < dialogue.lines.len() {
            dialogue.line += 1;
            dialogue.reveal_budget = 0.0;
            dialogue.revealed = if settings.typewriter {
                0
            } else {
                dialogue.line_len()
            };
        } else {
            commands.entity(entity).despawn();
            active.npc = None;
            mw_end.write(EndNPCDialogue {
                entity: dialogue.npc,
            });
        }
    }
}

pub fn typewriter_tick(
    time: Res<Time>,
    settings: Res<DialogueSettings>,
    audio: Res<Audio>,
    voice: Option<Res<DialogueVoice>>,
    mut q_dialogue: Query<&mut UIDialogue>,
) {
    for mut dialogue in q_dialogue.iter_mut() {
        if dialogue.line_complete() {
            continue;
        }

        dialogue.reveal_budget += time.delta_secs() * settings.chars_per_second;

        let mut play_blip = false;
        while dialogue.reveal_budget >= 1.0 && !dialogue.line_complete() {
            let c = dialogue
                .current_line()
                .chars()
                .nth(dialogue.revealed)
                .unwrap_or(' ');
            dialogue.revealed += 1;
            dialogue.reveal_budget -= 1.0;

            if !c.is_whitespace() && dialogue.revealed % settings.voice_every_n_chars.max(1) == 0 {
                play_blip = true;
            }

            let pause = match c {
                '.' | '!' | '?' => settings.sentence_pause,
                ',' | ';' | ':' => settings.comma_pause,
                _ => 0.0,
            };
            if pause > 0.0 {
                dialogue.reveal_budget -= pause * settings.chars_per_second;
            }
        }

        // at most one blip per frame so they don't stack into noise
        if play_blip && settings.voice {
            if let Some(voice) = voice.as_ref() {
                audio
                    .play(voice.0.clone())
                    .with_playback_rate(dialogue.voice_rate * voice_jitter(dialogue.revealed))
                    .with_volume(settings.voice_volume_db);
            }
        }
    }
}

/// Width of the text node for `line`: the whole line on one row if it fits,
/// otherwise the max bubble width (the text then wraps inside it).
fn text_width_for(line: &str) -> f32 {
    let max_width = BUBBLE_MAX_WIDTH - 2.0 * (BUBBLE_PADDING_X + BUBBLE_BORDER_WIDTH);
    let glyph_width = TEXT_FONT_SIZE * GLYPH_ADVANCE_EM;
    // +1px so float rounding never wraps the last glyph of a line that fits
    (line.chars().count() as f32 * glyph_width + 1.0).min(max_width)
}

pub fn update_dialogue_text(
    q_dialogue: Query<&UIDialogue, Changed<UIDialogue>>,
    mut q_revealed: Query<
        &mut TextSpan,
        (With<UIDialogueRevealedText>, Without<UIDialogueHiddenText>),
    >,
    mut q_hidden: Query<
        &mut TextSpan,
        (With<UIDialogueHiddenText>, Without<UIDialogueRevealedText>),
    >,
    mut q_body: Query<&mut Node, With<UIDialogueBody>>,
) {
    for dialogue in q_dialogue.iter() {
        let line = dialogue.current_line();

        let width = Val::Px(text_width_for(line));
        for mut node in q_body.iter_mut() {
            if node.width != width {
                node.width = width;
            }
        }

        let split = line
            .char_indices()
            .nth(dialogue.revealed)
            .map(|(i, _)| i)
            .unwrap_or(line.len());
        let (shown, hidden) = line.split_at(split);

        for mut span in q_revealed.iter_mut() {
            if span.0 != shown {
                span.0 = shown.to_string();
            }
        }
        for mut span in q_hidden.iter_mut() {
            if span.0 != hidden {
                span.0 = hidden.to_string();
            }
        }
    }
}

pub fn position_dialogue_bubble(
    mut commands: Commands,
    mut active: ResMut<ActiveDialogue>,
    mut mw_end: MessageWriter<EndNPCDialogue>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_targets: Query<&GlobalTransform>,
    mut q_dialogue: Query<(
        Entity,
        &UIDialogue,
        &mut Node,
        &ComputedNode,
        &mut Visibility,
    )>,
    mut q_tail: Query<&mut Node, (With<UIDialogueTail>, Without<UIDialogue>)>,
) {
    let Some((camera, camera_transform)) = q_camera.iter().next() else {
        return;
    };
    let Some(window) = q_window.iter().next() else {
        return;
    };

    for (entity, dialogue, mut node, computed, mut visibility) in q_dialogue.iter_mut() {
        // speaker is gone (despawned / level changed): close the bubble
        let Ok(npc_transform) = q_targets.get(dialogue.npc) else {
            commands.entity(entity).despawn();
            active.npc = None;
            mw_end.write(EndNPCDialogue {
                entity: dialogue.npc,
            });
            continue;
        };

        let Ok(anchor) = camera.world_to_viewport(
            camera_transform,
            npc_transform.translation() + BUBBLE_WORLD_OFFSET,
        ) else {
            *visibility = Visibility::Hidden;
            continue;
        };

        let size = computed.size() * computed.inverse_scale_factor();
        if size.x <= 0.0 || size.y <= 0.0 {
            // layout not computed yet
            continue;
        }

        let tail_height = TAIL_ROWS.len() as f32 * TAIL_ROW_HEIGHT;
        let max_left = (window.width() - size.x - SCREEN_MARGIN).max(SCREEN_MARGIN);
        let max_top = (window.height() - size.y - SCREEN_MARGIN).max(SCREEN_MARGIN);
        let left = (anchor.x - size.x / 2.0).clamp(SCREEN_MARGIN, max_left);
        let top = (anchor.y - size.y - tail_height).clamp(SCREEN_MARGIN, max_top);

        node.left = Val::Px(left.round());
        node.top = Val::Px(top.round());

        // keep the tail pointing at the NPC even when the bubble is clamped to the screen edge
        let inner_width = size.x - 2.0 * BUBBLE_BORDER_WIDTH;
        let tail_left = (anchor.x - left - BUBBLE_BORDER_WIDTH - TAIL_ROWS[0] / 2.0)
            .clamp(4.0, (inner_width - TAIL_ROWS[0] - 4.0).max(4.0));
        for mut tail_node in q_tail.iter_mut() {
            tail_node.left = Val::Px(tail_left.round());
        }

        *visibility = Visibility::Inherited;
    }
}

pub fn blink_advance_indicator(
    time: Res<Time>,
    q_dialogue: Query<&UIDialogue>,
    mut q_indicator: Query<&mut Visibility, With<UIDialogueAdvanceIndicator>>,
) {
    let Some(dialogue) = q_dialogue.iter().next() else {
        return;
    };

    let blink_on = (time.elapsed_secs() * 3.0) as u32 % 2 == 0;
    let target = if dialogue.line_complete() && blink_on {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    for mut visibility in q_indicator.iter_mut() {
        visibility.set_if_neq(target);
    }
}

pub fn cleanup_dialogue(
    mut commands: Commands,
    mut active: ResMut<ActiveDialogue>,
    q_dialogue: Query<Entity, With<UIDialogue>>,
) {
    for entity in q_dialogue.iter() {
        commands.entity(entity).despawn();
    }
    active.npc = None;
}
