use rocket::{get, launch, routes};
use rocket_sync_db_pools::database;

#[database("test")]
struct Pool(diesel::SqliteConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Pool::fairing())
}
