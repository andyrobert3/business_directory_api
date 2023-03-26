use business::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::*;

mod business;

#[get("/businesses")]
fn get_businesses() -> Json<Vec<Business>> {
    let businesses = load_businesses();
    Json(businesses)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_businesses])
}
