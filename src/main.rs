#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_codegen;

use rocket::{Request, State};
use rocket_contrib::JSON;

struct Person {
    id: usize,
    name: String,
    age: u8,
    email: String,
    address: String,
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

fn main() {
    let client = Client::connect("localhost", 27017).expect("Failed to connect to db!");

    let people_coll = client.db("flexera").collection("people");

    people_coll
        .insert_one(
            doc! {
                "id": 1,
                "name": "Bob",
            },
            None,
        )
        .ok()
        .expect("Failed to insert document.");

    rocket::ignite()
        .mount("/people", routes![get_person, post_person, delete_person])
        .launch();
}
