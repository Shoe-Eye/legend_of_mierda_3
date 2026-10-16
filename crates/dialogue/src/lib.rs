mod ui;
mod voice;

use bevy::prelude::*;
use lom_game::GameMode;

use crate::ui::*;
use crate::voice::*;

// --------
// Messages
// --------

/// Request to open a dialogue bubble above `entity`.
/// Ignored if another dialogue is already running or `lines` is empty.
#[derive(Message, Clone)]
pub struct StartNPCDialogue {
    pub entity: Entity,
    pub speaker: String,
    pub lines: Vec<String>,
}

/// Player pressed the interact key while a dialogue is open.
/// Completes the current line if it is still being typed, otherwise moves to the next one.
#[derive(Message, Clone, Copy)]
pub struct AdvanceDialogue;

/// Sent when the last line was dismissed (or the speaker disappeared).
#[derive(Message, Clone, Copy)]
pub struct EndNPCDialogue {
    pub entity: Entity,
}

// ---------
// Resources
// ---------

/// Which NPC is currently talking, if any. Gameplay systems use it to
/// freeze the player / NPC and to route the interact key.
#[derive(Resource, Default, Debug)]
pub struct ActiveDialogue {
    pub npc: Option<Entity>,
}

impl ActiveDialogue {
    pub fn is_active(&self) -> bool {
        self.npc.is_some()
    }
}

#[derive(Resource, Clone, Debug)]
pub struct DialogueSettings {
    /// Reveal lines letter by letter. When false the whole line is shown at once.
    pub typewriter: bool,
    pub chars_per_second: f32,
    /// Extra pause (seconds) after sentence punctuation (. ! ?).
    pub sentence_pause: f32,
    /// Extra pause (seconds) after , ; :
    pub comma_pause: f32,
    /// Play a voice blip while typing.
    pub voice: bool,
    /// Play a blip every N revealed (non-whitespace) characters.
    pub voice_every_n_chars: usize,
    pub voice_volume_db: f32,
}

impl Default for DialogueSettings {
    fn default() -> Self {
        Self {
            typewriter: true,
            chars_per_second: 35.0,
            sentence_pause: 0.3,
            comma_pause: 0.12,
            voice: true,
            voice_every_n_chars: 2,
            voice_volume_db: -8.0,
        }
    }
}

// ------
// Plugin
// ------

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveDialogue>()
            .init_resource::<DialogueSettings>()
            .add_message::<StartNPCDialogue>()
            .add_message::<AdvanceDialogue>()
            .add_message::<EndNPCDialogue>()
            .add_systems(Startup, setup_dialogue_voice)
            .add_systems(
                Update,
                (
                    handle_start_dialogue,
                    handle_advance_dialogue,
                    typewriter_tick,
                    update_dialogue_text,
                    position_dialogue_bubble,
                    blink_advance_indicator,
                )
                    .chain()
                    .run_if(in_state(GameMode::GamePlay)),
            )
            .add_systems(OnExit(GameMode::GamePlay), cleanup_dialogue);
    }
}
