use std::collections::HashMap;

use eyre::Result;
use macroquad::audio;
use strum::{EnumIter, IntoEnumIterator};

async fn load_sound(path: &str) -> audio::Sound {
    audio::load_sound(path).await.expect("valid sound")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, strum::Display, EnumIter, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum Sound {
    Click,
    Win,
    Loss,
    Button,
    Mode,
}

/// Loads sounds - expects a file in the form "../assets/enum.wav"
pub struct Sounds {
    sounds: HashMap<Sound, audio::Sound>,
}

impl Sounds {
    pub async fn try_new() -> Result<Self> {
        let mut sounds = HashMap::new();
        for sound in Sound::iter() {
            sounds.insert(
                sound,
                load_sound(dbg!(&format!("assets/{}.wav", sound))).await,
            );
        }
        Ok(Self { sounds })
    }

    pub fn sound(&self, sound: Sound) -> &audio::Sound {
        &self.sounds[&sound]
    }
}
