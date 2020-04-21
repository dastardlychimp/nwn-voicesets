use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[allow(dead_code)]
pub fn read_file_to_vec<P: AsRef<Path>>(file_path: P)
    -> std::io::Result<Vec<u8>>
{
    let mut f = File::open(file_path).unwrap();
    let file_size = f.metadata()?.len() as usize;

    let mut bytes = Vec::with_capacity(file_size);

    f.read_to_end(&mut bytes)?;

    Ok(bytes)
}