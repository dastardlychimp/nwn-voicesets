use crate::schema::voiceset_soundset;

#[derive(Identifiable)]
#[derive(Queryable)]
#[derive(AsChangeset)]
#[changeset_options(treat_none_as_null="true")]
#[table_name="voiceset_soundset"]
pub struct VoicesetSoundset {
    pub id: i32,
    pub id_voiceset: i32,
    pub id_soundset: i32,
}

#[derive(Insertable)]
#[table_name="voiceset_soundset"]
pub struct NewVoicesetSoundset {
    pub id_voiceset: i32,
    pub id_soundset: i32,
}