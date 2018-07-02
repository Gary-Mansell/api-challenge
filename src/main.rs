#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate bson;
extern crate mongodb;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use bson::Bson;
use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};
use rocket::config::{Config, Environment};
use rocket::{Request, State};
use rocket_contrib::Json;
use std::sync::{Arc, Mutex};

const PORT: u16 = 8081;
const DB_HOST: &'static str = "localhost";
const DB_PORT: u16 = 27017;
const DB_NAME: &'static str = "MyApp";

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    // #[serde(rename = "_id")]
    // pub id: String,
    name: String,
    age: i8, // Mongo driver doesn't support unsigned types - (can't use u8)
    email: String,
    address: String,
    balance: f64,
}

#[get("/")]
fn default() -> &'static str {
    println!("Returning default response...");
    "Ready!"
}

#[get("/<id>")]
fn get_person(id: usize) -> &'static str {
    println!("Retrieving: {}...", id);
    "Success!"
}

#[post("/", format = "application/json", data = "<person>")]
fn post_person(db_client: State<Arc<Mutex<mongodb::Client>>>, person: Json<Person>) -> String {
    let db_client = db_client.inner().lock().unwrap();
    let people_coll = db_client.db(DB_NAME).collection("people");
    let person = person.into_inner();

    println!("Creating: {:?}", &person);
    let serialized_person = bson::to_bson(&person).unwrap();
    if let Bson::Document(document) = serialized_person {
        people_coll
            .insert_one(document, None)
            .ok()
            .expect("Failed to insert document!");
    } else {
        println!("Failed to convert BSON into MongoDB document!");
    };

    format!("Created {}!", &person.name);
    format!("Success!")
}

#[delete("/<id>")]
fn delete_person(id: usize) -> String {
    println!("Deleting: {}...", id);
    format!("Deleted {}!", id)
}

#[error(404)]
fn not_found(request: &Request) -> &'static str {
    println!("Not found! {}", request.uri());
    "Not found!"
}

fn main() {
    println!("Connecting to db...");
    let db_client = Client::connect(DB_HOST, DB_PORT).expect("Failed to connect to db!");
    let db_client = Arc::new(Mutex::new(db_client));

    let config = Config::build(Environment::Staging)
        .address("[::1]")
        .port(PORT)
        .workers(1)
        .unwrap();

    println!("Listening on port {}...", PORT);
    rocket::custom(config, false)
        .mount("/", routes![default])
        .mount("/people", routes![get_person, post_person, delete_person])
        .catch(errors![not_found])
        .manage(db_client)
        .launch();
}
