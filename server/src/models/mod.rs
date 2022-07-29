pub mod artists;
pub mod images;
mod schema;
pub mod songs;

use diesel::PgConnection;

use self::schema::{artist, image, song};

pub use self::artists::Artist;
pub use self::images::Image;
pub use self::songs::Song;

/// represent database result
pub enum DatabaseResult<T> {
    Successful(T),
    NotFound,
}

impl<T> DatabaseResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            DatabaseResult::Successful(inner) => inner,
            DatabaseResult::NotFound => panic!("NotFound variant can not be unwraped!"),
        }
    }
}
