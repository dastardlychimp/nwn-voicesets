use crate::schema::voiceset;

#[derive(Identifiable)]
#[derive(Queryable)]
#[derive(AsChangeset)]
#[changeset_options(treat_none_as_null="true")]
#[table_name="voiceset"]
pub struct Voiceset {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="voiceset"]
pub struct NewVoiceset<'a> {
    pub name: &'a str,
}