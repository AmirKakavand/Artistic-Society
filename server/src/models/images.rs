use super::image;
use super::DatabaseResult;
use super::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::Serialize;

/// represent database image table
#[derive(Queryable, Serialize)]
pub struct Image {
    pub id: i32,
    pub name: Option<String>,
    pub location: String,
    pub artist_id: Option<i32>,
}

impl Image {
    /// adds a new song to database
    pub fn add(conn: &mut PgConnection, image: NewImage) -> DatabaseResult<Image> {
        match diesel::insert_into(image::table)
            .values(image)
            .get_result::<Image>(conn)
        {
            Ok(image) => DatabaseResult::Successful(image),
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// gets a song from database
    pub fn get(conn: &mut PgConnection, image_id: i32) -> DatabaseResult<Image> {
        use image::id;
        match image::table.filter(id.eq(image_id)).load::<Image>(conn) {
            Ok(mut image_vec) if !image_vec.is_empty() => {
                DatabaseResult::Successful(image_vec.pop().unwrap())
            }
            Ok(_) => DatabaseResult::NotFound,
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// gets all images from database
    pub fn get_all(conn: &mut PgConnection) -> DatabaseResult<Vec<Image>> {
        match image::table.load::<Image>(conn) {
            Ok(song_vec) => DatabaseResult::Successful(song_vec),
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// updates a song in database
    pub fn update(
        conn: &mut PgConnection,
        image_id: i32,
        update_image: NewImage,
    ) -> DatabaseResult<Image> {
        use image::id;
        match diesel::update(image::table.filter(id.eq(image_id)))
            .set(update_image)
            .get_result::<Image>(conn)
        {
            Ok(image) => DatabaseResult::Successful(image),
            Err(Error::DatabaseError(_, _)) => DatabaseResult::NotFound,
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// removes a song from database
    pub fn delete(conn: &mut PgConnection, image_id: i32) -> DatabaseResult<Image> {
        use image::id;
        match diesel::delete(image::table.filter(id.eq(image_id))).get_result::<Image>(conn) {
            Ok(image) => DatabaseResult::Successful(image),
            Err(Error::DatabaseError(_, _)) => DatabaseResult::NotFound,
            Err(err) => panic!("Error!, message: {}", err),
        }
    }
    /// gets an artist's all images
    pub fn get_artist_images(
        conn: &mut PgConnection,
        artist_id: i32,
    ) -> DatabaseResult<Vec<Image>> {
        use image::artist;
        match image::table
            .filter(artist.eq(artist_id))
            .load::<Image>(conn)
        {
            Ok(image_vec) => DatabaseResult::Successful(image_vec),
            Err(err) => panic!("Error!, message: {}", err),
        }
    }

    /// returns a random unage from database
    pub fn get_random(conn: &mut PgConnection) -> Option<Image> {
        use rand::seq::IteratorRandom;
        let mut rng = rand::thread_rng();

        let images = Image::get_all(conn).unwrap();
        if images.is_empty() {
            return None;
        }
        let index = (0..images.len()).choose(&mut rng).unwrap();

        Some(images[index])
    }
}

/// struct to insert data to image table
#[derive(Insertable, AsChangeset)]
#[diesel(table_name = image)]
pub struct NewImage {
    pub name: Option<String>,
    pub location: String,
    pub artist: Option<i32>,
}
