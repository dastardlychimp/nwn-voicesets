pub mod soundset;
pub mod sound;
pub mod voiceset;
pub mod voiceset_soundset;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv;
use std::env;

fn _connect()
    -> Result<PgConnection, ConnectionError>
{
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    
    
    PgConnection::establish(&database_url)
}

pub fn connect() -> DBConnection
{
    let connection = _connect()
        .expect("Error connecting to the database.");

    DBConnection(connection)
}

pub struct DBConnection(PgConnection);