use std::path::Path;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::convert::TryFrom;
use std::fs;
use std::hash::Hash;

use nwn_files;

use nwn_files::types::{
    SsfEntry,
    Resource,
    ResRef,
    ResourceType,
    TlkEntry,
    TlkSound,
    FileType,
};


use crate::hasher::Hasher;
use crate::database;
use crate::models;
use crate::soundset2da::Soundset2da;

use models::soundset::Soundset;
use models::sound::Sound;

use database::voiceset as dbv;
use database::soundset as dbss;
use database::DBConnection;

const BASE_DIALOG_PATH: &str = "./base/dialog.tlk";
const BASE_SOUNDSET_2DA_PATH: &str = "./base/soundset.2da";

pub fn export_voiceset<P: AsRef<Path>>(id_voiceset: i32, output_path: P)
    -> Result<(), ()>
{
    let output_path = output_path.as_ref();
    assert!(output_path.is_dir());
    
    let conn = database::connect();
    
    let voiceset = dbv::select::one(&conn, id_voiceset).unwrap();
    let soundsets = dbv::select::soundsets(&conn, id_voiceset).unwrap();
    
    let mut hak_builder = nwn_files::ErfFile::new();
    let mut tlk_builder = tlk_builder().unwrap();
    let mut s2da_builder = soundset2da_builder().unwrap();
    
    soundsets
        .into_iter()
        .map(|ss| export_soundset(
            &conn,
            &mut hak_builder,
            &mut tlk_builder,
            &mut s2da_builder,
            ss
        ))
        .collect::<Result<Vec<_>, ()>>()?;

    
    let output_path = output_path.join(&voiceset.name);
    if ! output_path.exists() {
        fs::create_dir(&output_path).unwrap();
    }

    let new_dialog_path = output_path.clone().join("dialog.tlk");
    let new_voiceset_path = output_path.clone().join(format!("{}.hak", voiceset.name));

    let mut file_dialog = BufWriter::new(fs::File::create(new_dialog_path).unwrap());
    let mut file_soundset = BufWriter::new(Vec::new());
    let mut file_hak = BufWriter::new(fs::File::create(new_voiceset_path).unwrap());

    s2da_builder
        .write(&mut file_soundset)
        .unwrap();

    let s2da_resource = Resource {
        name: ResRef::try_from("soundset").unwrap(),
        data: file_soundset.into_inner().unwrap(),
        resource_type: ResourceType::x2da,
    };

    hak_builder
        .add_resource(s2da_resource)
        .write(&mut file_hak, FileType::Hak)
        .unwrap();

    tlk_builder
        .write(&mut file_dialog)
        .unwrap();
    
    Ok(())
}

fn export_soundset(
    connection: &DBConnection,
    hak_builder: &mut nwn_files::ErfFile,
    dialog_builder: &mut nwn_files::TlkFile,
    s2da_builder: &mut nwn_files::X2daFile<Soundset2da>,
    soundset: Soundset
)
    -> Result<(), ()>
{
    let sounds = dbss::select::sounds(connection, soundset.id).unwrap();

    let mut ssf_writer = BufWriter::new(Vec::new());
    let mut ssf_builder = build_ssf(dialog_builder, &soundset, &sounds);
    ssf_builder.write(&mut ssf_writer).unwrap();

    let ssf_resource = Resource {
        name: soundset_res_ref(&soundset),
        resource_type: ResourceType::ssf,
        data: ssf_writer.into_inner().unwrap(),
    };

    hak_builder.add_resource(ssf_resource);

    sounds
        .into_iter()
        .for_each(|mut sound| {
            let res_ref = sound_res_ref(&soundset, &sound);
            let data = std::mem::replace(&mut sound.sound_data, Vec::new());
            
            let resource = Resource {
                name: res_ref,
                resource_type: ResourceType::wav,
                data: data,
            };

            hak_builder.add_resource(resource);
        });


    add_to_soundset2da(s2da_builder, dialog_builder, &soundset).unwrap();

    Ok(())
}

fn build_ssf(
    tlk_builder: &mut nwn_files::TlkFile,
    ss: &Soundset,
    sounds: &Vec<Sound>
)
    -> nwn_files::SsfBuilder
{
    let mut ssf_builder = nwn_files::SsfBuilder::new();

    match ss.soundset_type {
        0 => {
            let mut ssf_array = Vec::with_capacity(30);
            ssf_array.resize_with(30, Default::default);

            sounds
                .iter()
                .for_each(|sound| {
                    if let Some(entry) = ssf_array.get_mut(sound.idx_soundset as usize) {
                        let str_ref = if let Some(text) = &sound.transcription {
                            let id = add_dialog(tlk_builder, text.to_string(), Some(sound_res_ref(ss, sound)));
                            Some(id as u32)
                        } else {
                            None
                        };
                        
                        *entry = SsfEntry {
                            res_ref: sound_res_ref(&ss, &sound),
                            string_ref: str_ref,
                        };
                    }        
                });
                
            ssf_builder.add_entries(ssf_array);
        },
        _ => panic!("Unknown soundset type: {}", ss.soundset_type)
    }

    ssf_builder
}

fn soundset2da_builder()
    -> Result<nwn_files::X2daFile<Soundset2da>, ()>
{
    let f = fs::File::open(Path::new(BASE_SOUNDSET_2DA_PATH)).unwrap();
    let mut reader = BufReader::new(f);
    let base_soundset = nwn_files::X2daFile::parse_from(&mut reader).unwrap();

    Ok(nwn_files::X2daFile::from(base_soundset))
}

fn tlk_builder()
    -> Result<nwn_files::TlkFile, ()>
{
    let f = fs::File::open(Path::new(BASE_DIALOG_PATH)).unwrap();
    let mut reader = BufReader::new(f);
    Ok(nwn_files::TlkFile::parse_from(&mut reader, false).unwrap())
}

fn add_to_soundset2da(
    sd2a_builder: &mut nwn_files::X2daFile<Soundset2da>,
    tlk_builder: &mut nwn_files::TlkFile,
    soundset: &Soundset,
)
    -> Result<(), ()>
{
    let tlk_entry = TlkEntry {
        string: soundset.name.to_string(),
        sound: None,
    };

    let str_ref = tlk_builder.next_id() as u32;

    tlk_builder.add_entry(tlk_entry);

    
    let soundset_row = Soundset2da {
        label: None,
        str_ref: Some(str_ref),
        res_ref: Some((*soundset_res_ref(&soundset)).clone()),
        soundset_type: Some(soundset.soundset_type as u32),
        gender: Some(soundset.gender as u32),
    };

    sd2a_builder.add_row(soundset_row).unwrap();

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

fn sound_res_ref(soundset: &Soundset, sound: &Sound)
    -> ResRef
{
    Hasher::new()
        .hash(&sound.idx_soundset)
        .hash(&sound.name)
        .hash(&soundset.name)
        .to_res_ref()
}

fn soundset_res_ref(soundset: &Soundset)
    -> ResRef
{
    Hasher::new()
        .hash(&soundset.name)
        .to_res_ref()
}