macro_rules! impl_operations {
    
    ($model:ident, $model_mod_name:ident, $model_key:expr) => {
        pub mod $model_mod_name
        {
            use crate::repo::operations::*;
            use crate::repo::Repo;
            use super::$model;

            type Key = String;
            type KeyRef = str;

            struct ModelDB {}
            impl Operations for ModelDB
            {
                type Model = $model;
                const KEY: &'static str = $model_key;
            }
            
            pub mod select
            {
                use super::*;

                pub fn all(repo: &mut Repo)
                    -> Vec<$model>
                {
                    <ModelDB as OperationsSelect>::all(repo)
                }
            
                pub fn one<K>(repo: &mut Repo, key: K)
                    -> $model
                    where K: AsRef<KeyRef>
                {
                    <ModelDB as OperationsSelect>::one(repo, key)
                }
            
                pub fn count(repo: &mut Repo)
                    -> usize
                {
                    <ModelDB as OperationsCount>::count(repo)
                }

                pub fn many<K>(repo: &mut Repo, keys: &Vec<K>)
                    -> Vec<$model>
                    where K: AsRef<KeyRef>
                {
                    <ModelDB as OperationsSelect>::many(repo, keys)
                }
            }

            pub mod insert
            {
                use super::*;
            
                pub fn one(repo: &mut Repo, voiceset: $model)
                    -> String
                {
                    <ModelDB as OperationsInsert>::one(repo, voiceset)
                }
            }

            pub mod delete {
                use super::*;
            
                pub fn one(repo: &mut Repo, key: Key)
                    -> bool
                {
                    <ModelDB as OperationsDelete>::one(repo, key)
                }
            }            
        }
        
    };
}