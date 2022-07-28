use chrono::NaiveDate;

use super::artist;
use super::DatabaseResult;
use super::PgConnection;

/// represent database artist table
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub birthdate: Option<NaiveDate>,
}

impl Artist {
    /// adds a new artist to database
    pub fn add(conn: &mut PgConnection, artist: NewArtist) -> DatabaseResult<Artist> {
        unimplemented!()
    }

    /// gets an artist from database
    pub fn get(conn: &mut PgConnection, artist_id: i32) -> DatabaseResult<Artist> {
        unimplemented!()
    }

    /// gets all artists from database
    pub fn get_all(conn: &mut PgConnection, artist_id: i32) -> DatabaseResult<Vec<Artist>> {
        unimplemented!()
    }

    /// updates an artist in database
    pub fn update(
        conn: &mut PgConnection,
        artist_id: i32,
        update_artist: NewArtist,
    ) -> DatabaseResult<Artist> {
        unimplemented!()
    }

    /// removes an artist from database
    pub fn delete(conn: &mut PgConnection, artist_id: i32) -> DatabaseResult<Artist> {
        unimplemented!()
    }
}

/// struct to insert new data to artist table
#[derive(Insertable)]
#[diesel(table_name = artist)]
pub struct NewArtist {
    pub name: String,
    pub birthdate: Option<NaiveDate>,
}
