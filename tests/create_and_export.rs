use nwn_voiceset;
use nwn_voiceset::export_voiceset;
use nwn_voiceset::database;

mod helpers;

use database::voiceset_soundset as dbvs;
use database::sound as dbs;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;

#[test]
fn export() {
    let conn = database::connect();
    let voiceset = helpers::create::NewVoiceset::new(&conn);
    let soundset = helpers::create::NewSoundset::new(&conn);

    let vs_row = dbvs::insert::one(&conn, voiceset.id, soundset.id).unwrap();

    let create_sound = |path, name, text, idx| {
        let sound_data = helpers::file::read_file_to_vec(path).unwrap();

        let sound = dbs::insert::one(
            &conn,
            soundset.id,
            idx,
            name,
            Some(text),
            sound_data
        ).unwrap();
    };

    create_sound(
        "./tests/samples/organfinale.wav",
        "Organ Finale Attack",
        "Can you hear the organ playing?",
        0
    );

    create_sound(
        "./tests/samples/vs_fEdwinax_help.wav",
        "fEdwinaxHelp",
        "I can see I am going to get no help from the likes of you.",
        4
    );

    let output_path = Path::new("./tests/outputs/");

    export_voiceset(voiceset.id, output_path).unwrap();
    let f = File::open(Path::new("./tests/outputs/Voiceset (Will be dropped)/dialog.tlk")).unwrap();
    let mut reader = BufReader::new(f);
    let tlk = nwn_files::TlkFile::parse_from(&mut reader, false).unwrap();
    dbg!("{:?}", &tlk.entries[tlk.entries.len()-5..]);
}
