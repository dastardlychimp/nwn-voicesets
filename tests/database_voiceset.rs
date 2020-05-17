use nwn_voiceset;
use nwn_voiceset::repo;
use repo::models::Voiceset;
mod helpers;


#[test]
fn repo_insert_voiceset()
{
    let name = "My voiceset: Blah";
    let mut conn = helpers::repo::connect();
    let voiceset = Voiceset::new(name);

    let count = repo::voiceset::select::count(&mut conn);
    let voiceset_key = repo::voiceset::insert::one(&mut conn, &voiceset);
    let voiceset = repo::voiceset::select::one(&mut conn, &voiceset_key);

    assert_eq!(voiceset.name, name.to_string());
    assert_eq!(count + 1, repo::voiceset::select::count(&mut conn));

    repo::voiceset::delete::one(&mut conn, voiceset_key);

    assert_eq!(count, repo::voiceset::select::count(&mut conn));
}