use nwn_voiceset;
use nwn_voiceset::database;
mod helpers;

use database::voiceset_soundset as vs;

#[test]
fn insert_voiceset_soundset() {
    let conn = database::connect();
    let voiceset = helpers::create::NewVoiceset::new(&conn);
    let soundset = helpers::create::NewSoundset::new(&conn);

    let inserted = vs::insert::one(&conn, voiceset.id, soundset.id).unwrap();
    vs::delete::one(&conn, inserted.id).unwrap();
}