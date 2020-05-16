use acid_store::repo::Key;

use super::Repo;
use super::models::{
    Sound,
};
use super::operations::*;
use super::key;

pub mod select
{
    use super::*;

    pub fn all(repo: &mut Repo)
        -> Vec<Sound>
    {
        <SoundDB as OperationsSelect>::all(repo)
    }

    pub fn one<K: AsRef<str>>(repo: &mut Repo, key: K)
        -> Sound
    {
        <SoundDB as OperationsSelect>::one(repo, key)
    }

    pub fn count(repo: &mut Repo)
        -> usize
    {
        <SoundDB as OperationsCount>::count(repo)
    }
}


pub mod insert
{
    use super::*;

    pub fn one(
        repo: &mut Repo,
        sound: Sound
    )
        -> String
    {
        <SoundDB as OperationsInsert>::one(repo, sound)
    }
}

pub mod delete {
    use super::*;

    pub fn one<K: Into<String>>(repo: &mut Repo, key: K)
        -> bool
    {
        <SoundDB as OperationsDelete>::one(repo, key)
    }
}

struct SoundDB {}

impl Operations for SoundDB {
    type Model = Sound;
    const KEY: &'static str = "sound_keys";
}