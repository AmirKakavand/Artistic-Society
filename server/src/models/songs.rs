use super::song;
use super::DatabaseResult;
use super::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::Serialize;

/// represent database song table
#[derive(Queryable, Serialize)]
pub struct Song {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub duration: i32,
    pub artist_id: i32,
    pub genre: Option<String>,
}

impl Song {
    /// adds a new song to database
    pub fn add(conn: &mut PgConnection, song: NewSong) -> DatabaseResult<Song> {
        match diesel::insert_into(song::table)
            .values(song)
            .get_result::<Song>(conn)
        {
            Ok(song) => DatabaseResult::Successful(song),
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// gets a song from database
    pub fn get(conn: &mut PgConnection, song_id: i32) -> DatabaseResult<Song> {
        use song::id;
        match song::table.filter(id.eq(song_id)).load::<Song>(conn) {
            Ok(mut song_vec) if !song_vec.is_empty() => {
                DatabaseResult::Successful(song_vec.pop().unwrap())
            }
            Ok(_) => DatabaseResult::NotFound,
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// gets all songs from database
    pub fn get_all(conn: &mut PgConnection) -> DatabaseResult<Vec<Song>> {
        match song::table.load::<Song>(conn) {
            Ok(song_vec) => DatabaseResult::Successful(song_vec),
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// updates a song in database
    pub fn update(
        conn: &mut PgConnection,
        song_id: i32,
        update_song: NewSong,
    ) -> DatabaseResult<Song> {
        use song::id;
        match diesel::update(song::table.filter(id.eq(song_id)))
            .set(update_song)
            .get_result::<Song>(conn)
        {
            Ok(song) => DatabaseResult::Successful(song),
            Err(Error::DatabaseError(_, _)) => DatabaseResult::NotFound,
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// removes a song from database
    pub fn delete(conn: &mut PgConnection, sing_id: i32) -> DatabaseResult<Song> {
        use song::id;
        match diesel::delete(song::table.filter(id.eq(sing_id))).get_result::<Song>(conn) {
            Ok(song) => DatabaseResult::Successful(song),
            Err(Error::DatabaseError(_, _)) => DatabaseResult::NotFound,
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// gets an artist's all songs
    pub fn get_artist_images(conn: &mut PgConnection, artist_id: i32) -> DatabaseResult<Vec<Song>> {
        use song::artist;
        match song::table.filter(artist.eq(artist_id)).load::<Song>(conn) {
            Ok(song_vec) => DatabaseResult::Successful(song_vec),
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// returns a random song from database
    pub fn get_random(conn: &mut PgConnection) -> Option<Song> {
        use rand::seq::IteratorRandom;
        let mut rng = rand::thread_rng();

        let songs = Song::get_all(conn).unwrap();
        if songs.is_empty() {
            return None;
        }
        let index = (0..songs.len()).choose(&mut rng).unwrap();

        Some(songs[index])
    }
}

/// struct to insert data to song table
#[derive(Insertable, AsChangeset)]
#[diesel(table_name = song)]
pub struct NewSong {
    pub name: String,
    pub location: String,
    pub duration: i32,
    pub artist: i32,
    pub genre: Option<String>,
}
