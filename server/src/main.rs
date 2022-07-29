#[macro_use]
extern crate rocket;

use server::get_conn_pool;
use server::routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::stage())
        .manage(get_conn_pool())
}
