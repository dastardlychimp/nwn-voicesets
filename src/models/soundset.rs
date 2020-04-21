use crate::schema::soundset;

#[derive(Identifiable)]
#[derive(Queryable)]
#[derive(AsChangeset)]
#[changeset_options(treat_none_as_null="true")]
#[table_name="soundset"]
pub struct Soundset {
    pub id: i32,
    pub name: String,
    pub gender: i32,
    pub soundset_type: i32,
}

#[derive(Insertable)]
#[table_name="soundset"]
pub struct NewSoundset<'a> {
    pub name: &'a str,
    pub gender: i32,
    pub soundset_type: i32,
}