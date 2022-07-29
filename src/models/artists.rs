use chrono::NaiveDate;

use super::artist;
use super::DatabaseResult;
use super::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

/// represent database artist table
#[derive(Queryable)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub birthdate: Option<NaiveDate>,
}

impl Artist {
    /// adds a new artist to database
    pub fn add(conn: &mut PgConnection, artist: NewArtist) -> DatabaseResult<Artist> {
        match diesel::insert_into(artist::table)
            .values(artist)
            .get_result::<Artist>(conn)
        {
            Ok(song) => DatabaseResult::Successful(song),
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// gets an artist from database
    pub fn get(conn: &mut PgConnection, artist_id: i32) -> DatabaseResult<Artist> {
        use artist::id;
        match artist::table.filter(id.eq(artist_id)).load::<Artist>(conn) {
            Ok(mut artist_vec) if !artist_vec.is_empty() => {
                DatabaseResult::Successful(artist_vec.pop().unwrap())
            }
            Ok(_) => DatabaseResult::NotFound,
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// gets all artists from database
    pub fn get_all(conn: &mut PgConnection) -> DatabaseResult<Vec<Artist>> {
        match artist::table.load::<Artist>(conn) {
            Ok(artist_vec) => DatabaseResult::Successful(artist_vec),
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// updates an artist in database
    pub fn update(
        conn: &mut PgConnection,
        artist_id: i32,
        update_artist: NewArtist,
    ) -> DatabaseResult<Artist> {
        use artist::id;
        match diesel::update(artist::table.filter(id.eq(artist_id)))
            .set(update_artist)
            .get_result::<Artist>(conn)
        {
            Ok(artist) => DatabaseResult::Successful(artist),
            Err(Error::DatabaseError(_, _)) => DatabaseResult::NotFound,
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// removes an artist from database
    pub fn delete(conn: &mut PgConnection, artist_id: i32) -> DatabaseResult<Artist> {
        use artist::id;
        match diesel::delete(artist::table.filter(id.eq(artist_id))).get_result::<Artist>(conn) {
            Ok(artist) => DatabaseResult::Successful(artist),
            Err(Error::DatabaseError(_, _)) => DatabaseResult::NotFound,
            Err(err) => panic!("Error!, message: {}", err),
        }
    }
}

/// struct to insert new data to artist table
#[derive(Insertable, AsChangeset)]
#[diesel(table_name = artist)]
pub struct NewArtist {
    pub name: String,
    pub birthdate: Option<NaiveDate>,
}
