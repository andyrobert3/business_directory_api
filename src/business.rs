use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

const CSV_FILE_PATH: &str = "businesses.csv";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Business {
    pub business_name: String,
    pub business_address: String,
    pub business_number: String,
    pub business_nature: String,
}

pub fn save_businesses(businesses: &Vec<Business>) {
    let file = File::create(CSV_FILE_PATH).unwrap();

    let mut writer = Writer::from_writer(file);
    for business in businesses {
        writer.serialize(business).unwrap();
    }
}

pub fn load_businesses() -> Vec<Business> {
    let mut businesses = vec![];
    let mut file =
        File::open(CSV_FILE_PATH).unwrap_or_else(|_| File::create(CSV_FILE_PATH).unwrap());

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            let mut reader = Reader::from_reader(contents.as_bytes());
            for result in reader.deserialize() {
                let business: Business = result.unwrap();

                print!("{:?}", &contents);
                businesses.push(business);
            }
        }
        Err(_) => {}
    }

    businesses
}
