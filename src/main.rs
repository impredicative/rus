#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use diesel::prelude::*;
use rocket::response::Redirect;
use rocket_sync_db_pools::{database};
use rocket::serde::{json::Json, Deserialize, Serialize};
use crate::schema::urls;

mod schema {
    table! {
        urls (id) {
            id -> Integer,
            original_url -> Text,
            short_url -> Text,
        }
    }
}

#[database("sqlite_data")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = urls)]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub short_url: String,
}

#[post("/shorten", data = "<url>")]
async fn create_url(conn: DbConn, url: Json<Url>) -> Json<Url> {
    url // Placeholder
}

#[get("/lengthen/<_short_url>")]
async fn get_url(_conn: DbConn, _short_url: String) -> Redirect {
    Redirect::to("/") // Placeholder
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![create_url, get_url])
}
