use nwn_voiceset;
use nwn_voiceset::database;
mod helpers;

use database::sound::select::all as select_all;
use database::sound::insert::one as insert;

use diesel::result::{Error as DieselError, DatabaseErrorKind};

#[test]
fn insert_sound_with_non_existant_soundset() {
    let conn = database::connect();

    let sound_data = helpers::file::read_file_to_vec("./tests/samples/organfinale.wav").unwrap();

    let e = insert(
        &conn,
        684684654,
        0,
        "Organ Finale Attack",
        Some("Can you hear the organ playing?"),
        sound_data
    ).unwrap_err();

    match e {
        DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _) => assert!(true),
        _ => assert!(false),
    }
}


#[test]
fn insert_sound() {
    let conn = database::connect();

    let rows1 = select_all(&conn).unwrap();

    let soundset = helpers::create::NewSoundset::new(&conn);

    let sound_data = helpers::file::read_file_to_vec("./tests/samples/organfinale.wav").unwrap();
    
    insert(
        &conn,
        soundset.id,
        0,
        "Organ Finale Attack",
        Some("Can you hear the organ playing?"),
        sound_data
    ).unwrap();

    let rows2 = select_all(&conn).unwrap();

    assert_eq!(rows2.len(), rows1.len() + 1);

}