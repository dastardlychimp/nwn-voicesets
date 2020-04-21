use crate::schema;
use crate::models::soundset::{Soundset, NewSoundset};
use crate::models::sound::{Sound};

use super::DBConnection;

use diesel::prelude::*;

pub mod select {
    use super::*;
    use schema::soundset::dsl::*;
    
    pub fn all(connection: &DBConnection)
        -> QueryResult<Vec<Soundset>>
    {
        soundset.load::<Soundset>(&connection.0)
    }

    pub fn sounds(connection: &DBConnection, id_soundset: i32)
        -> QueryResult<Vec<Sound>>
    {
        use schema::sound;
        
        soundset
            .filter(id.eq(id_soundset))
            .inner_join(sound::table)
            .select(sound::table::all_columns())
            .order(sound::idx_soundset)
            .load::<Sound>(&connection.0)
    }
}

pub mod insert {
    use super::*;
    use schema::soundset;
    
    pub fn one<S: AsRef<str>>(connection: &DBConnection, name: S, gender: i32, soundset_type: i32)
        -> QueryResult<Soundset>
    {
        let new = NewSoundset {
            name: name.as_ref(),
            gender: gender,
            soundset_type: soundset_type,
        };
        
        diesel::insert_into(soundset::table)
            .values(&new)
            .get_result(&connection.0)
    }
}


pub mod update {
    use super::*;

    pub fn one(connection: &DBConnection, updated_soundset: &Soundset)
        -> QueryResult<Soundset>
    {
        diesel::update(updated_soundset)
            .set(updated_soundset)
            .get_result(&connection.0)
    }
}

pub mod delete {
    use super::*;
    use schema::soundset::dsl::*;

    pub fn one(connection: &DBConnection, id_delete: i32)
        -> QueryResult<usize>
    {
        let filter = soundset.filter(id.eq(id_delete));
        diesel::delete(filter)
            .execute(&connection.0)
    }
}