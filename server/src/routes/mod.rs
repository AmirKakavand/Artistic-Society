mod enies;

pub use self::enies::{get_image, get_song};

pub fn stage() -> Vec<rocket::Route> {
    routes![get_image, get_song]
}
