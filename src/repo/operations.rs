use super::Repo;
use super::key::HasKey;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait Operations
{
    type Model: Serialize + DeserializeOwned + HasKey;
    const KEY: &'static str;
}

pub trait OperationsSelect : Operations
{
    fn all(repo: &mut Repo)
        -> Vec<Self::Model>
    {
        all_keys(repo, Self::KEY)
            .into_iter()
            .map(|k| get_key(repo, &k, None))
            .collect()
    }

    fn many<K>(repo: &mut Repo, keys: &Vec<K>)
        -> Vec<Self::Model>
        where K: AsRef<str>
    {
        keys
            .iter()
            .map(|k| {
                <Self as OperationsSelect>::one(repo, k)
            })
            .collect()
    }

    fn one<K: AsRef<str>>(repo: &mut Repo, key: K)
        -> Self::Model
    {
        get_key(repo, key.as_ref(), None)
    }
}

pub trait OperationsInsert : Operations
{
    fn one(repo: &mut Repo, value: Self::Model)
        -> String
    {
        let key = value.key().to_owned();

        let mut keys = all_keys(repo, Self::KEY);

        keys.push(key.clone());

        {
            let mut o_value = repo.0.insert(key.clone());
            o_value.serialize(&value).unwrap();
        }

        {
            let mut o_keys = repo.0.insert(Self::KEY.to_owned());
            o_keys.serialize(&keys).unwrap();
        }

        key
    }
}

pub trait OperationsDelete : Operations
{
    fn one<K: Into<String>>(repo: &mut Repo, key: K)
        -> bool
    {
        let key = key.into();

        let keys = all_keys(repo, Self::KEY)
            .into_iter()
            .filter(|k| *k != key)
            .collect::<Vec<String>>();
        
        {
            let mut o_keys = repo.0.insert(Self::KEY.to_owned());
            o_keys.serialize(&keys).unwrap();
        }

        repo.0.remove(&key)
    }
}

pub trait OperationsCount : Operations
{
    fn count(repo: &mut Repo)
        -> usize
    {
        all_keys(repo, Self::KEY).len()
    }
}

impl<T> OperationsSelect for T where T : Operations {}
impl<T> OperationsInsert for T where T : Operations {}
impl<T> OperationsDelete for T where T : Operations {}
impl<T> OperationsCount for T where T : Operations {}

fn all_keys(repo: &mut Repo, key: &str)
    -> Vec<String>
{
    repo.0.get(key)
        .map(|mut o| o.deserialize::<Vec<String>>()
            .expect("Failed to deserialize keys.")
        )
        .unwrap_or_else(Vec::new)
}

fn get_key<M: DeserializeOwned>(repo: &mut Repo, key: &str, default: Option<M>)
    -> M
{
    repo.0
        .get(key)
        .map(|mut o| o
            .deserialize::<M>()
            .expect(&format!("Could not deserialize into: {}", key))
        )
        .or(default)
        .expect(&format!("Failed to retrieve key: {}", key))
}