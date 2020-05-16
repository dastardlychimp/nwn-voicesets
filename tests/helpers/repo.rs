use nwn_voiceset::repo;

use std::path::PathBuf;

pub fn connect() -> repo::Repo
{
    let test_repo_path = PathBuf::from("./tests/outputs/repo");
    repo::Repo::new(test_repo_path)
}