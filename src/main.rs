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

#[post("/business", format = "json", data = "<business>")]
fn create_business(business: Json<Business>) -> Status {
    let mut businesses = load_businesses();

    if let Some(_index) = businesses
        .iter()
        .position(|item| item.business_name == business.0.business_name)
    {
        return Status::Conflict;
    }

    businesses.push(business.0);
    save_businesses(&businesses);
    Status::Created
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_businesses, create_business])
}
