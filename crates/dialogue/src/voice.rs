use std::f32::consts::TAU;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_kira_audio::prelude::{Frame, StaticSoundData, StaticSoundSettings};

/// Short procedurally generated "blip" played while text is being typed,
/// Ace Attorney style. Pitch is varied per speaker via playback rate.
#[derive(Resource)]
pub struct DialogueVoice(pub Handle<AudioSource>);

const SAMPLE_RATE: u32 = 44_100;
const BLIP_FREQUENCY: f32 = 330.0;
const BLIP_SECONDS: f32 = 0.045;

pub fn setup_dialogue_voice(mut commands: Commands, mut sources: ResMut<Assets<AudioSource>>) {
    let num_frames = (SAMPLE_RATE as f32 * BLIP_SECONDS) as usize;
    let attack = (SAMPLE_RATE as f32 * 0.003) as usize;

    let frames: Arc<[Frame]> = (0..num_frames)
        .map(|i| {
            let t = i as f32 / SAMPLE_RATE as f32;
            // square wave with a soft fundamental mixed in so it's less harsh
            let phase = (t * BLIP_FREQUENCY).fract();
            let square = if phase < 0.5 { 1.0 } else { -1.0 };
            let sine = (t * BLIP_FREQUENCY * TAU).sin();
            // quick fade-in to avoid clicks, then linear decay
            let envelope = if i < attack {
                i as f32 / attack as f32
            } else {
                1.0 - (i - attack) as f32 / (num_frames - attack) as f32
            };
            let sample = (0.6 * square + 0.4 * sine) * envelope * 0.3;
            Frame {
                left: sample,
                right: sample,
            }
        })
        .collect();

    let handle = sources.add(AudioSource {
        sound: StaticSoundData {
            sample_rate: SAMPLE_RATE,
            frames,
            settings: StaticSoundSettings::default(),
            slice: None,
        },
    });

    commands.insert_resource(DialogueVoice(handle));
}

/// Stable per-speaker playback rate so every NPC gets its own "voice".
pub fn voice_rate_for(speaker: &str) -> f64 {
    let mut hasher = DefaultHasher::new();
    speaker.hash(&mut hasher);
    let h = hasher.finish();
    0.75 + (h % 1000) as f64 / 1000.0 * 0.75
}

/// Small deterministic jitter so consecutive blips don't sound robotic.
pub fn voice_jitter(char_index: usize) -> f64 {
    const STEPS: [f64; 5] = [1.0, 1.04, 0.97, 1.02, 0.95];
    STEPS[char_index % STEPS.len()]
}
