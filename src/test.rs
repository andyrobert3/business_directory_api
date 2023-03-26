#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::uri;

    // Macro to explicitly state it is a "TEST"
    #[test]
    fn test_get_businesses() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(crate::get_businesses)).dispatch();
        let expected_str = "[{\"business_name\":\"Cold Storage\",\"business_address\":\"Orchard Road\",\"business_number\":\"+6564819181\",\"business_nature\":\"Groceries\"},{ \"business_name\":\"Metacamp\",\"business_address\":\"Chinatown\",\"business_number\":\"+6564898383\",\"business_nature\":\"Education\"}]";

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), expected_str);
    }
}
