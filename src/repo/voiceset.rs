use super::Repo;
use super::models::{
    Voiceset,
};
use super::operations::*;

pub mod select
{
    use super::*;

    pub fn all(repo: &mut Repo)
        -> Vec<Voiceset>
    {
        <VoicesetDB as OperationsSelect>::all(repo)
    }

    pub fn one<K: AsRef<str>>(repo: &mut Repo, key: K)
        -> Voiceset
    {
        <VoicesetDB as OperationsSelect>::one(repo, key)
    }

    pub fn count(repo: &mut Repo)
        -> usize
    {
        <VoicesetDB as OperationsCount>::count(repo)
    }
}


pub mod insert
{
    use super::*;

    pub fn one(repo: &mut Repo, voiceset: Voiceset)
        -> String
    {
        <VoicesetDB as OperationsInsert>::one(repo, voiceset)
    }
}

pub mod delete {
    use super::*;

    pub fn one(repo: &mut Repo, key: String)
        -> bool
    {
        <VoicesetDB as OperationsDelete>::one(repo, key)
    }
}

pub struct VoicesetDB {}

impl Operations for VoicesetDB {
    type Model = Voiceset;
    const KEY: &'static str = "voiceset_keys";
}