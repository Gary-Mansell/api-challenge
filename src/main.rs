#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::config::{Config, Environment};
use rocket::{Request, State};

struct Person {
    id: usize,
    name: String,
    age: u8,
    email: String,
    address: String,
}

#[get("/")]
fn default() -> String {
    println!("Sending default response...");
    format!("Ready!")
}

#[get("/<id>")]
fn get_person(id: usize) -> String {
    format!("Id {}!", id)
}

#[post("/<id>")]
fn post_person(id: usize) -> String {
    format!("Id {}!", id)
}

#[delete("/<id>")]
fn delete_person(id: usize) -> String {
    format!("Id {}!", id)
}

#[error(404)]
fn not_found(request: &Request) -> &'static str {
    "Not found!"
}

fn main() {
    // let client = Client::connect("localhost", 27017).expect("Failed to connect to db!");

    // let people_coll = client.db("flexera").collection("people");

    // people_coll
    //     .insert_one(
    //         doc! {
    //             "id": 1,
    //             "name": "Bob",
    //         },
    //         None,
    //     )
    //     .ok()
    //     .expect("Failed to insert document.");

    let config = Config::build(Environment::Staging)
        .address("127.0.0.1")
        .port(8000)
        .workers(1)
        .unwrap();

    rocket::custom(config, false)
        .mount("/", routes![default])
        .mount("/people", routes![get_person, post_person, delete_person])
        .catch(errors![not_found])
        .launch();
}
