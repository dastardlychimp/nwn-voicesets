use acid_store::repo::{
    ObjectRepository,
    RepositoryConfig,
    Compression,
    LockStrategy,
    OpenRepo,
};

use acid_store::store::{
    DirectoryStore,
    OpenStore,
    OpenOption,
};

use std::path::PathBuf;

pub mod key;
pub mod models;
pub mod operations;
#[macro_use] mod operations_macro;

use models::*;

pub struct Repo(ObjectRepository<String, DirectoryStore>);

impl Repo
{
    pub fn new(path: PathBuf)
        -> Self
    {
        let mut config = RepositoryConfig::default();
        config.compression = Compression::Lz4 { level: 1 };

        let mut options = OpenOption::empty();
        options.insert(OpenOption::CREATE);

        let store = DirectoryStore::open(path, options)
            .unwrap();
        
        let repository = ObjectRepository::create_repo(
            store,
            config,
            LockStrategy::Abort,
            None,
        ).unwrap();

        Repo(repository)
    }
}

impl_operations!(Soundset, soundset, "soundset_keys");
impl_operations!(Sound, sound, "sound_keys");
impl_operations!(Voiceset, voiceset, "voiceset_keys");