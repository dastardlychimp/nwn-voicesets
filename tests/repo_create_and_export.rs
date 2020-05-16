use nwn_voiceset::repo;
use repo::models::*;
use repo::export;

mod helpers;

use std::path::PathBuf;

#[test]
fn export() {
    let mut conn = helpers::repo::connect();

    let mut create_sound = |path, name, text, idx| {
        let sound_data = helpers::file::read_file_to_vec(path).unwrap();

        let sound = Sound::new(
            name,
            idx,
            text,
            sound_data
        );

        repo::sound::insert::one(&mut conn, sound)
    };

    let key_s1 = create_sound(
        "./tests/samples/organfinale.wav",
        "Organ Finale Attack",
        "Can you hear the organ playing?",
        0
    );

    let key_s2 = create_sound(
        "./tests/samples/vs_fEdwinax_help.wav",
        "fEdwinaxHelp",
        "I can see I am going to get no help from the likes of you.",
        4
    );

    let mut soundset = Soundset::new(
        "TestSoundset",
        0,
        0,
    );

    soundset.keys_sound.push(key_s1);
    soundset.keys_sound.push(key_s2);

    let key_soundset = repo::soundset::insert::one(&mut conn, soundset);

    let mut voiceset = Voiceset::new(
        "TestVoiceset"
    );

    voiceset.keys_soundset.push(key_soundset);

    let key_voiceset = repo::voiceset::insert::one(&mut conn, voiceset);

    let config_export = export::ConfigExport {
        key_voiceset: key_voiceset,
        output_path: PathBuf::from("./tests/outputs/voiceset"),
        file_tlk_alternate: None,
        file_soundset: None,
    };

    export::export_voiceset(&mut conn, config_export).unwrap();
}
