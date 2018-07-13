#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate bson;
extern crate mongodb;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use bson::oid::ObjectId;
use bson::Bson;
use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};
use rocket::config::{Config, Environment};
use rocket::{Request, State};
use rocket_contrib::Json;
use std::sync::{Arc, Mutex};

use std::error::Error;
use std::fmt;

const PORT: u16 = 8081;
const DB_HOST: &'static str = "localhost";
const DB_PORT: u16 = 27017;
const DB_NAME: &'static str = "MyApp";

#[derive(Debug)]
struct ApiError {
    details: String,
}

impl ApiError {
    fn new(msg: &str) -> ApiError {
        ApiError {
            details: msg.to_string(),
        }
    }
}

impl Error for ApiError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<mongodb::Error> for ApiError {
    fn from(err: mongodb::Error) -> Self {
        ApiError::new(err.description())
    }
}

impl From<bson::oid::Error> for ApiError {
    fn from(err: bson::oid::Error) -> Self {
        ApiError::new(err.description())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(skip_deserializing)] // Do NOT rename = "_id" (conflicts with returned value)
    id: String,
    name: String,
    age: i8, // Can't use u8 - Mongo driver doesn't support unsigned types
    email: String,
    address: String,
    balance: f64,
}

#[get("/")]
fn default() -> &'static str {
    println!("Returning default response...");
    "Ready!"
}

#[get("/")]
fn list_people(db_client: State<Arc<Mutex<mongodb::Client>>>) -> Result<String, ApiError> {
    let db_client = db_client.inner().lock().unwrap();
    let people_coll = db_client.db(DB_NAME).collection("people");

    people_coll
        .find(None, None)
        .map_err(|err| ApiError::new(err.description()))
        .and_then(|mut cursor| match cursor.has_next() {
            Ok(true) => {
                println!("Found users!");
                let items: Vec<_> = cursor.map(|item| item.unwrap()).collect();
                Ok(format!("{:?}", items))
            }
            Ok(false) => {
                println!("No users found!");
                Ok("No users found!".to_string())
            }
            Err(err) => Err(ApiError::new(err.description())),
        })
}

#[get("/<id>")]
fn get_person(
    db_client: State<Arc<Mutex<mongodb::Client>>>,
    id: String,
) -> Result<String, ApiError> {
    let db_client = db_client.inner().lock().unwrap();
    let people_coll = db_client.db(DB_NAME).collection("people");
    let id: ObjectId = ObjectId::with_string(&id)?;
    println!("Retrieving: {}...", id);

    people_coll
        .find_one(
            Some(doc!{
            "_id": id
            }),
            None,
        )
        .map_err(|err| ApiError::new(err.description()))
        .and_then(|result| match result {
            Some(person) => {
                println!("Found: {:?}", person);
                Ok(format!("{:?}", person))
            }
            None => Err(ApiError::new("Not found!")),
        })
}

#[post("/", format = "application/json", data = "<person>")]
fn post_person(
    db_client: State<Arc<Mutex<mongodb::Client>>>,
    person: Json<Person>,
) -> Result<String, ApiError> {
    let db_client = db_client.inner().lock().unwrap();
    let people_coll = db_client.db(DB_NAME).collection("people");
    let person = person.into_inner();

    println!("Creating: {:?}", &person);
    bson::to_bson(&person)
        .map_err(|err| ApiError::new(err.description()))
        .and_then(|serialized_person: Bson| {
            if let Bson::Document(document) = serialized_person {
                people_coll
                    .insert_one(document, None)
                    .map_err(|err| ApiError::new(err.description()))
                    .and_then(|res| {
                        let id = res
                            .inserted_id
                            .expect("No inserted_id returned!")
                            .as_object_id()
                            .map_or(String::new(), |r| r.to_hex());
                        Ok(id)
                    })
            } else {
                Err(ApiError::new(
                    "Failed to convert BSON into MongoDB document!",
                ))
            }
        })
}

#[delete("/<id>")]
fn delete_person(
    db_client: State<Arc<Mutex<mongodb::Client>>>,
    id: String,
) -> Result<&'static str, ApiError> {
    let db_client = db_client.inner().lock().unwrap();
    let people_coll = db_client.db(DB_NAME).collection("people");
    let id: ObjectId = ObjectId::with_string(&id)?;
    println!("Deleting: {}...", id);

    people_coll
        .delete_many(
            doc!{
            "_id": id
            },
            None,
        )
        .map_err(|err| ApiError::new(err.description()))
        .and_then(|_r| Ok("Success!"))
}

#[error(404)]
fn not_found(request: &Request) -> &'static str {
    println!("Not found! {}", request.uri());
    "Not found!"
}

fn main() {
    println!("Connecting to db...");
    let db_client = Client::connect(DB_HOST, DB_PORT).unwrap();

    let config = Config::build(Environment::Staging)
        .address("[::1]")
        .port(PORT)
        .workers(1)
        .unwrap();

    println!("Listening on port {}...", PORT);
    rocket::custom(config, false)
        .mount("/", routes![default])
        .mount(
            "/people",
            routes![list_people, get_person, post_person, delete_person],
        )
        .catch(errors![not_found])
        .manage(Arc::new(Mutex::new(db_client)))
        .launch();
}
