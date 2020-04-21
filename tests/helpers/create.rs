#![allow(dead_code)]

use nwn_voiceset;
use nwn_voiceset::database;
use database::DBConnection;


pub struct NewSoundset<'a> {
    connection: &'a DBConnection,
    pub id: i32,
}

impl<'a> NewSoundset<'a> {
    pub fn new(connection: &'a DBConnection) -> Self
    {
        let soundset = database::soundset::insert::one(
            connection,
            "Will be dropped soundset",
            0,
            0,
        ).unwrap();

        NewSoundset {
            connection: connection,
            id: soundset.id,
        }
    }
}

impl<'a> Drop for NewSoundset<'a> {
    fn drop(&mut self) {
        database::soundset::delete::one(self.connection, self.id).unwrap();
    }
}

pub struct NewVoiceset<'a> {
    connection: &'a DBConnection,
    pub id: i32,
}

impl<'a> NewVoiceset<'a> {
    pub fn new(connection: &'a DBConnection) -> Self
    {
        let voiceset = database::voiceset::insert::one(
            connection,
            "Voiceset (Will be dropped)",
        ).unwrap();

        NewVoiceset {
            connection: connection,
            id: voiceset.id,
        }
    }
}

impl<'a> Drop for NewVoiceset<'a> {
    fn drop(&mut self) {
        database::voiceset::delete::one(self.connection, self.id).unwrap();
    }
}