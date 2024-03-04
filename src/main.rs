use std::{collections::HashMap, io};
use rocket::{launch, serde::{json::Json, Serialize, Deserialize}};
use uuid::Uuid;

#[macro_use] extern crate rocket;

#[derive(Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    month: u8,
    day: u8,
    year: u32,
    id: Uuid,
    players: HashMap<String,u32>
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiHealth {
    version: String
}

#[get("/")]
fn health_check() -> Json<ApiHealth> {
    let output = ApiHealth {
        version: "0.1.0".to_string()
    };

    Json(output)
}

#[post("/submit", data = "<results>")]
fn submit_game_results(results: Json<Game>) -> Json<Game> {
    let mut submitted_game = results.into_inner();
    if submitted_game.id.is_nil() {
        submitted_game.id = uuid::Uuid::new_v4();
    }

    Json(submitted_game)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check, submit_game_results])
        .mount("/health", routes![health_check])
}