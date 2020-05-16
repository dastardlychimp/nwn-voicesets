use super::Repo;
use super::models::{
    Soundset,
};
use super::operations::*;
use super::key;

pub mod select
{
    use super::*;

    pub fn all(repo: &mut Repo)
        -> Vec<Soundset>
    {
        <SoundsetDB as OperationsSelect>::all(repo)
    }

    pub fn one<K: AsRef<str>>(repo: &mut Repo, key: K)
        -> Soundset
    {
        <SoundsetDB as OperationsSelect>::one(repo, key)
    }

    pub fn count(repo: &mut Repo)
        -> usize
    {
        <SoundsetDB as OperationsCount>::count(repo)
    }
}


pub mod insert
{
    use super::*;

    pub fn one(
        repo: &mut Repo,
        soundset: Soundset
    )
        -> String
    {
        <SoundsetDB as OperationsInsert>::one(repo, soundset)
    }
}

pub mod delete {
    use super::*;

    pub fn one<K: Into<String>>(repo: &mut Repo, key: K)
        -> bool
    {
        <SoundsetDB as OperationsDelete>::one(repo, key)
    }
}

struct SoundsetDB {}

impl Operations for SoundsetDB {
    type Model = Soundset;
    const KEY: &'static str = "sound_keys";
}