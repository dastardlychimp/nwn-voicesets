use serde::{Serialize, Deserialize};
use super::key::{
    generate,
    HasKey
};

type Key = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Voiceset
{
    pub key: Key,
    pub name: String,
    pub keys_soundset: Vec<Key>,
}

impl Voiceset
{
    pub fn new<S: Into<String>>(name: S)
        -> Self
    {
        Voiceset {
            key: generate(),
            name: name.into(),
            keys_soundset: Vec::new(),
        }
    }
}

impl HasKey for Voiceset
{
    fn key(&self)
        -> &str
    {
        &self.key
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Soundset
{
    pub key: Key,
    pub name: String,
    pub gender: u32,
    pub soundset_type: u32,
    pub keys_sound: Vec<Key>,
}

impl Soundset
{
    pub fn new<S: Into<String>>(
        name: S,
        gender: u32,
        soundset_type: u32,
    )
        -> Self
    {
        Soundset {
            key: generate(),
            name: name.into(),
            gender: gender,
            soundset_type: soundset_type,
            keys_sound: Vec::new(),
        }
    }
}

impl HasKey for Soundset
{
    fn key(&self)
        -> &str
    {
        &self.key
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sound
{
    pub key: Key,
    pub idx_soundset: usize,
    pub name: String,
    pub transcription: String,
    pub sound_data: Vec<u8>,
}

impl Sound
{
    pub fn new<S, B>(
        name: S,
        idx_soundset: usize,
        transcription: S,
        sound_data: B,
    )
        -> Self
        where
            S: Into<String>,
            B: Into<Vec<u8>>
    {
        Sound {
            key: generate(),
            name: name.into(),
            idx_soundset: idx_soundset,
            transcription: transcription.into(),
            sound_data: sound_data.into(),
        }
    }
}

impl HasKey for Sound
{
    fn key(&self)
        -> &str
    {
        &self.key
    }
}
