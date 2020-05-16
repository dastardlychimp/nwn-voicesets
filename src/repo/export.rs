use nwn_files;

use crate::hasher::Hasher;
use crate::soundset2da::Soundset2da;
use crate::repo;
use repo::models;

use nwn_files::types::{
    SsfEntry,
    Resource,
    ResourceType,
    TlkEntry,
    ResRef,
    TlkSound,
    FileType,
};

use std::io::{BufReader, BufWriter};
use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};

const BASE_SOUNDSET_2DA_PATH: &str = "./base/soundset.2da";

pub struct ConfigExport
{
    pub key_voiceset: String,
    pub output_path: PathBuf,
    pub file_tlk_alternate: Option<PathBuf>,
    pub file_soundset: Option<PathBuf>,
}

pub fn export_voiceset(
    conn: &mut repo::Repo,
    config: ConfigExport,
)
    -> Result<(), ()>
{
    let ConfigExport {
        key_voiceset,
        output_path,
        file_tlk_alternate,
        file_soundset
    } = config;

    assert!(output_path.is_dir());

    let voiceset = repo::voiceset::select::one(conn, key_voiceset);
    let iter_data = voiceset.keys_soundset
        .iter()
        .map(|key_soundset| {
            let mut soundset = repo::soundset::select::one(conn, key_soundset);

            let sounds = soundset.keys_sound
                .iter()
                .map(|key_sound| repo::sound::select::one(conn, key_sound))
                .collect::<Vec<models::Sound>>();

            (soundset, sounds)
        });


    let mut builder_hak = nwn_files::ErfFile::new();
    let mut builder_s2da = soundset2da_builder(file_soundset).unwrap();
    let mut builder_tlk = tlk_builder(file_tlk_alternate).unwrap();


    for (soundset, sounds) in iter_data
    {
        let mut ssf_array = Vec::with_capacity(40);
        ssf_array.resize_with(40, Default::default);
        
        let ssf_entries = sounds
            .into_iter()
            .for_each(|sound| {
                // Add entry to tlk table
                let res_ref = sound_res_ref(&sound);

                let id = add_dialog(
                    &mut builder_tlk,
                    sound.transcription,
                    Some(res_ref.clone())
                );

                let ssf_entry = SsfEntry {
                    res_ref: res_ref.clone(),
                    string_ref: Some(id as u32),
                };

                let sound_resource = Resource {
                    name: res_ref,
                    resource_type: ResourceType::wav,
                    data: sound.sound_data,
                };

                ssf_array[sound.idx_soundset] = ssf_entry;
                builder_hak.add_resource(sound_resource);
            });

        let mut ssf_writer = BufWriter::new(Vec::new());
        let mut ssf_builder = nwn_files::SsfBuilder::new();
        ssf_builder.add_entries(ssf_array);
        ssf_builder.write(&mut ssf_writer).unwrap();

        let ssf_res_ref = soundset_res_ref(&soundset);
        let ssf_resource = Resource {
            name: ssf_res_ref.clone(),
            resource_type: ResourceType::ssf,
            data: ssf_writer.into_inner().unwrap()
        };

        builder_hak.add_resource(ssf_resource);

        let tlk_id = add_dialog(
            &mut builder_tlk,
            soundset.name.to_string(),
            None,
        );

        let soundset_row = Soundset2da {
            label: None,
            str_ref: Some(tlk_id as u32),
            res_ref: Some(ssf_res_ref.to_string()),
            soundset_type: Some(soundset.soundset_type),
            gender: Some(soundset.gender),
        };

        builder_s2da.add_row(soundset_row).unwrap();
    }

    let output_path = output_path.join(&voiceset.name);
    if ! output_path.exists() {
        fs::create_dir(&output_path).unwrap();
    }

    let new_dialog_path = output_path.clone().join("alt_dialog.tlk");
    let new_voiceset_path = output_path.clone().join(format!("{}.hak", voiceset.name));

    let mut file_dialog = BufWriter::new(fs::File::create(new_dialog_path).unwrap());
    let mut file_soundset = BufWriter::new(Vec::new());
    let mut file_hak = BufWriter::new(fs::File::create(new_voiceset_path).unwrap());

    builder_s2da
        .write(&mut file_soundset)
        .unwrap();

    let resource_s2da = Resource {
        name: ResRef::try_from("soundset").unwrap(),
        data: file_soundset.into_inner().unwrap(),
        resource_type: ResourceType::x2da,
    };

    builder_hak
        .add_resource(resource_s2da)
        .write(&mut file_hak, FileType::Hak)
        .unwrap();

    builder_tlk
        .write(&mut file_dialog)
        .unwrap();
        
    Ok(())
}

fn add_dialog(
    tlk_builder: &mut nwn_files::TlkFile,
    text: String,
    sound_res_ref: Option<ResRef>
)
    -> usize
{
    let entry_sound = sound_res_ref
        .map(|srf| {
            TlkSound {
                res_ref: srf,
                sound_length: None,
            }
        });
    
    let entry = TlkEntry {
        string: text,
        sound: None,
    };

    tlk_builder.add_entry(entry);

    tlk_builder.next_id() - 1
}

fn soundset2da_builder(file_soundset: Option<PathBuf>)
    -> Result<nwn_files::X2daFile<Soundset2da>, ()>
{
    let f = file_soundset
        .map(fs::File::open)
        .or_else(|| Some(fs::File::open(Path::new(BASE_SOUNDSET_2DA_PATH))))
        .unwrap()
        .expect("Failed to open soundset file.");
    
    let mut reader = BufReader::new(f);
    let base_soundset = nwn_files::X2daFile::parse_from(&mut reader).unwrap();

    Ok(nwn_files::X2daFile::from(base_soundset))
}

fn tlk_builder(file_tlk_alternate: Option<PathBuf>)
    -> Result<nwn_files::TlkFile, ()>
{
    let builder = file_tlk_alternate
        .map(fs::File::open)
        .map(Result::unwrap)
        .map(BufReader::new)
        .map(|mut r| nwn_files::TlkFile::parse_from(&mut r, true).unwrap())
        .unwrap_or_else(nwn_files::TlkFile::new);

    Ok(builder)
}

fn sound_res_ref(sound: &models::Sound)
    -> ResRef
{
    Hasher::new()
        .hash(&sound.idx_soundset)
        .hash(&sound.name)
        .to_res_ref()
}

fn soundset_res_ref(soundset: &models::Soundset)
    -> ResRef
{
    Hasher::new()
        .hash(&soundset.name)
        .to_res_ref()
}