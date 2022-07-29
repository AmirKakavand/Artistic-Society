#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod db;
pub mod models;
pub mod routes;

pub use self::db::get_conn_pool;
pub use routes::stage;
