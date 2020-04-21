use crate::schema::sound;
use std::fmt::Debug;

#[derive(Identifiable)]
#[derive(Queryable)]
#[derive(AsChangeset)]
#[changeset_options(treat_none_as_null="true")]
#[table_name="sound"]
pub struct Sound {
    pub id: i32,
    pub id_soundset: i32,
    pub idx_soundset: i32,
    pub name: String,
    pub transcription: Option<String>,
    pub sound_data: Vec<u8>,
}

#[derive(Insertable)]
#[table_name="sound"]
pub struct NewSound<'a> {
    pub id_soundset: i32,
    pub idx_soundset: i32,
    pub name: &'a str,
    pub transcription: Option<&'a str>,
    pub sound_data: &'a [u8],
}

impl Debug for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sound")
            .field("id", &self.id)
            .field("id_soundset", &self.id_soundset)
            .field("idx_soundset", &self.idx_soundset)
            .field("name", &self.name)
            .field("transcription", &self.transcription)
            .field("sound_data", &format_args!("Length of {}", &self.sound_data.len()))
            .finish()
    }
}