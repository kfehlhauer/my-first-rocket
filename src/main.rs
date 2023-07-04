use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;
use std::borrow::Cow;
use std::collections::HashMap;

#[macro_use]
extern crate rocket;

type DataHashMap = Mutex<HashMap<u32, String>>;
type Database<'r> = &'r State<DataHashMap>;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Data<'r> {
    id: u32,
    value: Cow<'r, str>,
}
// Comment this section out to play with the other Data struct
// struct Data {
//     id: u32,
//     value: String,
// }

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/insert", format = "json", data = "<data>")]
async fn new(data: Json<Data<'_>>, database: Database<'_>) -> (Status, Value) {
    let mut db = database.lock().await;

    match db.contains_key(&data.id) {
        false => {
            db.insert(data.id, data.value.to_string());
            (Status::Accepted, json!({ "status": "ok" }))
        }
        true => (
            Status::BadRequest,
            json!({ "status": "failed - record already exists" }),
        ),
    }
}

// Remove the comment this section out to play with the other Data struct
// #[post("/insert", format = "json", data = "<data>")]
// async fn new(data: Json<Data>, database: Database<'_>) -> (Status, Value) {
//     let mut db = database.lock().await;
//     if db.contains_key(&data.id) {
//         return (
//             Status::BadRequest,
//             json!({ "status": "failed - record already exists" }),
//         );
//     }

//     db.insert(data.id, data.value.to_string());
//     (Status::Accepted, json!({ "status": "ok" }))
// }

#[put("/update", format = "json", data = "<data>")]
async fn update(data: Json<Data<'_>>, database: Database<'_>) -> (Status, Value) {
    match database.lock().await.get_mut(&data.id) {
        Some(d) => {
            *d = data.value.to_string();
            (Status::Accepted, json!({ "status": "ok" }))
        }
        None => {
            let status = format!(
                "failed - record does not exist for {}",
                &data.id.to_string()
            );
            (Status::BadRequest, json!({ "status": status }))
        }
    }
}

// Remove the comment this section out to play with the other Data struct
// #[put("/update", format = "json", data = "<data>")]
// async fn update(data: Json<Data>, database: Database<'_>) -> (Status, Value) {
//     match database.lock().await.get_mut(&data.id) {
//         Some(d) => {
//             *d = data.value.to_string();
//             (Status::Accepted, json!({ "status": "ok" }))
//         }
//         None => {
//             let status = format!(
//                 "failed - record does not exist for {}",
//                 &data.id.to_string()
//             );
//             (Status::BadRequest, json!({ "status": status }))
//         }
//     }
// }

#[get("/getdata/<id>")]
async fn get_my_data(id: u32, database: Database<'_>) -> (Status, Value) {
    match database.lock().await.get(&id) {
        Some(d) => (Status::Accepted, json!({ "status": "ok", "data": d })),
        None => (
            Status::BadRequest,
            json!({ "status": "failed - no data found"}),
        ),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_my_data, index, new, update])
        .manage(DataHashMap::new(HashMap::new()))
}
