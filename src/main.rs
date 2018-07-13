#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_cors;

use rocket_contrib::{Json, Value};
use rocket_cors::{AllowedOrigins, Cors};
use std::collections::HashMap;
use rocket::State;
use rocket::http::Method;
use std::sync::RwLock;

const USERS_JSON: &'static str = include_str!("../userData.json");

fn main() {
    let allowed_origins = AllowedOrigins::all();
    let cors = Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Put].into_iter().map(From::from).collect(),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .manage(Storage::init())
        .attach(cors)
        .mount("/", routes![get_all, create, update, delete]).launch();
}

// ENDPOINTS/ROUTES

#[get("/")]
fn get_all(storage: State<Storage>) -> Json<Vec<UserDto>> {
    let items = storage.users.read().unwrap()
        .iter().map(|(_, v)| v.clone()).collect();
    Json(items)
}

#[post("/", data = "<create_user_request>")]
fn create(create_user_request: Json<CreateUpdateUserDto>, storage: State<Storage>) -> Json<UserDto> {
    let new_id = storage.users.read().unwrap().iter()
        .max_by(|(k1, _), (k2, _)| k1.cmp(k2))
        .map(|(k, _)| *k)
        .unwrap_or(0) + 1;
    let new_user = UserDto::from_request(new_id, &create_user_request);
    storage.users.write().unwrap().insert(new_id, new_user.clone());
    Json(new_user)
}

#[put("/<id>", data = "<update_user_request>")]
fn update(id: i32, update_user_request: Json<CreateUpdateUserDto>, storage: State<Storage>) -> Json<UserDto> {
    let updated_user = UserDto::from_request(id, &update_user_request);
    storage.users.write().unwrap().insert(id, updated_user.clone());
    Json(updated_user)
}

#[delete("/<id>")]
fn delete(id: i32, storage: State<Storage>) -> Json<Value> {
    storage.users.write().unwrap().remove(&id);
    Json(json!({
        "success": true
    }))
}

// STORAGE

struct Storage {
    users: RwLock<HashMap<i32, UserDto>>
}

impl Storage {
    fn init() -> Storage {
        let users = serde_json::from_str(USERS_JSON).expect("invalid json in userData.json");

        Storage {
            users: RwLock::new(users)
        }
    }
}

// CONTRACTS

#[derive(Serialize, Deserialize, Clone)]
struct AddressDto {
    street: String,
    city: String,
    state: String,
    zip: String
}

#[derive(Serialize, Deserialize, Clone)]
struct UserDto {
    id: i32,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    email: Option<String>,
    phone: Option<String>,
    address: AddressDto
}

impl UserDto {
    fn from_request(id: i32, other: &CreateUpdateUserDto) -> UserDto {
        UserDto {
            id: id,
            first_name: other.first_name.clone(),
            last_name: other.last_name.clone(),
            email: other.email.clone(),
            phone: other.phone.clone(),
            address: other.address.clone()
        }
    }
}

#[derive(Deserialize)]
struct CreateUpdateUserDto {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    email: Option<String>,
    phone: Option<String>,
    address: AddressDto
}
