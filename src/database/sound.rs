use crate::schema;
use crate::models::sound::{Sound, NewSound};
use super::DBConnection;

use diesel::prelude::*;

pub mod select {
    use super::*;
    use schema::sound::dsl::*;
    
    pub fn all(connection: &DBConnection)
        -> QueryResult<Vec<Sound>>
    {
        sound.load::<Sound>(&connection.0)
    }
}


pub mod insert {
    use super::*;
    use schema::sound;
    
    pub fn one<S, B>(
        connection: &DBConnection,
        id_soundset: i32,
        idx_soundset: i32,
        name: S,
        transcription: Option<S>,
        sound_data: B,
    )
        -> QueryResult<Sound>
        where
            S: AsRef<str>,
            B: AsRef<[u8]>,
    {
        let transcript = transcription
            .as_ref()
            .map(|t| t.as_ref());
        
        let new = NewSound {
            id_soundset,
            idx_soundset,
            name: name.as_ref(),
            transcription: transcript,
            sound_data: sound_data.as_ref(),
        };

        diesel::insert_into(sound::table)
            .values(&new)
            .get_result(&connection.0)
    }
}

pub mod delete {
    use super::*;
    use schema::sound::dsl::*;

    pub fn one(connection: &DBConnection, id_delete: i32)
        -> QueryResult<usize>
    {
        let filter = sound.filter(id.eq(id_delete));
        diesel::delete(filter)
            .execute(&connection.0)
    }
}