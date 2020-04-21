use nwn_voiceset;
use nwn_voiceset::database;
mod helpers;

#[test]
fn insert_voiceset() {
    let conn = database::connect();
    let _voiceset = helpers::create::NewVoiceset::new(&conn);
}