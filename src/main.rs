use business::*;
use cors::Cors;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::*;

mod business;
mod cors;
mod test;

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

#[delete("/business", format = "json", data = "<business>")]
fn delete_business(business: Json<Business>) -> Status {
    let mut businesses = load_businesses();

    if let Some(_index) = businesses
        .iter()
        .position(|item| item.business_name == business.0.business_name)
    {
        businesses.remove(_index);
        save_businesses(&businesses);
        Status::NoContent
    } else {
        Status::NotFound
    }
}

#[put("/business", format = "json", data = "<business>")]
fn update_business(business: Json<Business>) -> Status {
    let mut businesses = load_businesses();

    if let Some(index) = businesses
        .iter()
        .position(|item| item.business_name == business.0.business_name)
    {
        businesses[index] = business.0;
        save_businesses(&businesses);
        Status::NoContent
    } else {
        Status::NotFound
    }
}

#[options("/business")]
fn all_options() {}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Cors).mount(
        "/",
        routes![
            get_businesses,
            create_business,
            delete_business,
            update_business,
            all_options
        ],
    )
}
