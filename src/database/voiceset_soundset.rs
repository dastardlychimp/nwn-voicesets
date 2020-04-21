use crate::schema;
use crate::models::voiceset_soundset::{VoicesetSoundset, NewVoicesetSoundset};

use super::DBConnection;

use diesel::prelude::*;

pub mod select {
    use super::*;
    use schema::voiceset_soundset::dsl::*;
    
    pub fn all(connection: &DBConnection)
        -> QueryResult<Vec<VoicesetSoundset>>
    {
        voiceset_soundset.load::<VoicesetSoundset>(&connection.0)
    }
}

pub mod insert {
    use super::*;
    use schema::voiceset_soundset;
    
    pub fn one(connection: &DBConnection, id_voiceset: i32, id_soundset: i32)
        -> QueryResult<VoicesetSoundset>
    {
        let new = NewVoicesetSoundset {
            id_voiceset,
            id_soundset,
        };
        
        diesel::insert_into(voiceset_soundset::table)
            .values(&new)
            .get_result(&connection.0)
    }
}


pub mod update {
    use super::*;

    pub fn one(connection: &DBConnection, updated_voiceset_soundset: &VoicesetSoundset)
        -> QueryResult<VoicesetSoundset>
    {
        diesel::update(updated_voiceset_soundset)
            .set(updated_voiceset_soundset)
            .get_result(&connection.0)
    }
}

pub mod delete {
    use super::*;
    use schema::voiceset_soundset::dsl::*;

    pub fn one(connection: &DBConnection, id_delete: i32)
        -> QueryResult<usize>
    {
        let filter = voiceset_soundset.filter(id.eq(id_delete));
        diesel::delete(filter)
            .execute(&connection.0)
    }
}