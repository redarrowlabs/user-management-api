#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate rocket_cors;

use rocket_contrib::{Json, Value};
use rocket_cors::{AllowedOrigins, Cors};
use std::collections::HashMap;
use rocket::State;
use rocket::http::Method;
use std::sync::RwLock;

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

fn main() {
    let (allowed_origins, _) = AllowedOrigins::some(&["*"]);
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

struct Storage {
    users: RwLock<HashMap<i32, UserDto>>
}

impl Storage {
    fn init() -> Storage {
        let mut users = HashMap::new();
        users.insert(1,
            UserDto {
                id: 1,
                first_name: "Foo".to_string(),
                last_name: "Bar".to_string(),
                email: None,
                phone: None
            }
        );
        users.insert(2,
            UserDto {
                id: 2,
                first_name: "Charlie".to_string(),
                last_name: "Brown".to_string(),
                email: Some("charlie.brown@thing.com".to_string()),
                phone: Some("123-456-7890".to_string())
            }
        );
        users.insert(3,
            UserDto {
                id: 3,
                first_name: "Papa".to_string(),
                last_name: "Bear".to_string(),
                email: None,
                phone: None
            }
        );
        Storage {
            users: RwLock::new(users)
        }
    }
}

#[derive(Serialize, Clone)]
struct UserDto {
    id: i32,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    email: Option<String>,
    phone: Option<String>
}

impl UserDto {
    fn from_request(id: i32, other: &CreateUpdateUserDto) -> UserDto {
        UserDto {
            id: id,
            first_name: other.first_name.clone(),
            last_name: other.last_name.clone(),
            email: other.email.clone(),
            phone: other.phone.clone()
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
    phone: Option<String>
}
