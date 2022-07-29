use crate::db::DbConn;
use crate::models::{Artist, Image, Song};
use rocket::serde::json::Json;

/// GET to retrieve an image info
#[get("/image")]
pub fn get_image(mut conn: DbConn) -> Result<Json<Image>, Error> {
    let mut errors = Error::build();
    let image = Image::get_random(&mut conn);
    if image.is_none() {
        errors.no_image();
    }
    errors.finish()?;
    Ok(Json(image.unwrap()))
}

/// GET to retrieve an image info
#[get("/song")]
pub fn get_song(mut conn: DbConn) -> Result<Json<Song>, Error> {
    let mut errors = Error::build();
    let song = Song::get_random(&mut conn);
    if song.is_none() {
        errors.no_image();
    }
    errors.finish()?;
    Ok(Json(song.unwrap()))
}

#[derive(Responder)]
#[response(status = 500, content_type = "application/json")]
pub struct Error(String);

impl Error {
    pub fn build() -> ErrorBuilder {
        ErrorBuilder(vec![])
    }
    pub fn new(errors: String) -> Self {
        Error(format!("{{ \" errors \": {} }}", errors))
    }
}

pub struct ErrorBuilder(Vec<&'static str>);

impl ErrorBuilder {
    pub fn format_error_string(&self) -> String {
        let mut temp_str = String::from("[");
        for err in &self.0 {
            temp_str.push_str(&format!("{}, ", err));
        }
        temp_str.push(']');
        temp_str
    }

    pub fn finish(self) -> Result<(), Error> {
        if !self.0.is_empty() {
            Err(Error::new(self.format_error_string()))
        } else {
            Ok(())
        }
    }

    pub fn is_valid(&self) -> Result<(), Error> {
        if !self.0.is_empty() {
            Err(Error::new(self.format_error_string()))
        } else {
            Ok(())
        }
    }

    pub fn no_song(&mut self) {
        self.0.push("no song on disk!");
    }
    pub fn no_image(&mut self) {
        self.0.push("no image on disk!");
    }
}
