use crate::schema;
use crate::models::voiceset::{Voiceset, NewVoiceset};
use crate::models::soundset::{Soundset};

use super::DBConnection;

use diesel::prelude::*;

pub mod select {
    use super::*;
    use schema::voiceset::dsl::*;
    
    pub fn all(connection: &DBConnection)
        -> QueryResult<Vec<Voiceset>>
    {
        voiceset.load::<Voiceset>(&connection.0)
    }

    pub fn one(connection: &DBConnection, id_voiceset: i32)
        -> QueryResult<Voiceset>
    {
        voiceset.filter(id.eq(id_voiceset)).first(&connection.0)
    }

    pub fn soundsets(connection: &DBConnection, id_voiceset: i32)
        -> QueryResult<Vec<Soundset>>
    {
        use schema::soundset;

        use schema::voiceset_soundset::dsl::voiceset_soundset;
        
        voiceset
            .filter(id.eq(id_voiceset))
            .inner_join(voiceset_soundset.inner_join(soundset::table))
            .select(soundset::table::all_columns())
            .load::<Soundset>(&connection.0)
    }
}

pub mod insert {
    use super::*;
    use schema::voiceset;
    
    pub fn one<S: AsRef<str>>(connection: &DBConnection, name: S)
        -> QueryResult<Voiceset>
    {
        let new = NewVoiceset {
            name: name.as_ref(),
        };
        
        diesel::insert_into(voiceset::table)
            .values(&new)
            .get_result(&connection.0)
    }
}


pub mod update {
    use super::*;

    pub fn one(connection: &DBConnection, updated_voiceset: &Voiceset)
        -> QueryResult<Voiceset>
    {
        diesel::update(updated_voiceset)
            .set(updated_voiceset)
            .get_result(&connection.0)
    }
}

pub mod delete {
    use super::*;
    use schema::voiceset::dsl::*;

    pub fn one(connection: &DBConnection, id_delete: i32)
        -> QueryResult<usize>
    {
        let filter = voiceset.filter(id.eq(id_delete));
        diesel::delete(filter)
            .execute(&connection.0)
    }
}