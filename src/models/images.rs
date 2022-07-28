use super::image;
use super::DatabaseResult;
use super::PgConnection;

/// represent database image table
pub struct Image {
    pub id: i32,
    pub name: Option<String>,
    pub location: String,
    pub artist_id: i32,
}

impl Image {
    /// adds a new song to database
    pub fn add(conn: &mut PgConnection, image: NewImage) -> DatabaseResult<Image> {
        unimplemented!()
    }

    /// gets a song from database
    pub fn get(conn: &mut PgConnection, image_id: i32) -> DatabaseResult<Image> {
        unimplemented!()
    }

    /// gets all songs from database
    pub fn get_all(conn: &mut PgConnection, image_id: i32) -> DatabaseResult<Vec<Image>> {
        unimplemented!()
    }

    /// updates a song in database
    pub fn update(
        conn: &mut PgConnection,
        image_id: i32,
        update_image: NewImage,
    ) -> DatabaseResult<Image> {
        unimplemented!()
    }

    /// removes a song from database
    pub fn delete(conn: &mut PgConnection, image_id: i32) -> DatabaseResult<Image> {
        unimplemented!()
    }
    /// gets an artist's all songs
    pub fn get_artist_images(
        conn: &mut PgConnection,
        artist_id: i32,
    ) -> DatabaseResult<Vec<Image>> {
        unimplemented!()
    }
}

/// struct to insert data to image table
#[derive(Insertable)]
#[diesel(table_name = image)]
pub struct NewImage {
    pub name: Option<String>,
    pub location: String,
    pub artist: i32,
}
