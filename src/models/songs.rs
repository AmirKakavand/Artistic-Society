use super::song;
use super::DatabaseResult;
use super::PgConnection;

/// represent database song table
#[derive(Queryable)]
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
        unimplemented!()
    }

    /// gets a song from database
    pub fn get(conn: &mut PgConnection, song_id: i32) -> DatabaseResult<Song> {
        unimplemented!()
    }

    /// gets all songs from database
    pub fn get_all(conn: &mut PgConnection, song_id: i32) -> DatabaseResult<Vec<Song>> {
        unimplemented!()
    }

    /// updates a song in database
    pub fn update(
        conn: &mut PgConnection,
        song_id: i32,
        update_song: NewSong,
    ) -> DatabaseResult<Song> {
        unimplemented!()
    }

    /// removes a song from database
    pub fn delete(conn: &mut PgConnection, sing_id: i32) -> DatabaseResult<Song> {
        unimplemented!()
    }

    /// gets an artist's all songs
    pub fn get_artist_images(conn: &mut PgConnection, artist_id: i32) -> DatabaseResult<Vec<Song>> {
        unimplemented!()
    }
}

/// struct to insert data to song table
#[derive(Insertable)]
#[diesel(table_name = song)]
pub struct NewSong {
    pub name: String,
    pub location: String,
    pub duration: i32,
    pub artist: i32,
    pub genre: Option<String>,
}
