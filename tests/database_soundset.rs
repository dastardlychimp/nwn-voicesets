use nwn_voiceset;
use nwn_voiceset::database;

#[test]
fn soundset_insert_select_update_and_delete() {
    let conn = database::connect();

    let rows1 = database::soundset::select::all(&conn).unwrap();

    let mut soundset = database::soundset::insert::one(&conn, "Soundset 1", 0, 0).unwrap();

    let rows2 = database::soundset::select::all(&conn).unwrap();

    assert_eq!(rows2.len(), rows1.len() + 1);
    assert_eq!("Soundset 1", &rows2.last().unwrap().name);

    soundset.name = "Renamed Soundset 3".to_string();

    let soundset = database::soundset::update::one(&conn, &soundset).unwrap();

    let rows3 = database::soundset::select::all(&conn).unwrap();

    assert_eq!(rows2.len(), rows3.len());
    assert_eq!("Renamed Soundset 3", &rows3.last().unwrap().name);

    let rows_affected = database::soundset::delete::one(&conn, soundset.id).unwrap();

    assert_eq!(1, rows_affected);
    
    let rows4 = database::soundset::select::all(&conn).unwrap();

    assert_eq!(rows4.len(), rows1.len());
}