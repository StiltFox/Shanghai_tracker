use std::{collections::HashMap, io};
use rocket::{launch, serde::{json::Json, Serialize}};
use uuid::Uuid;

#[macro_use] extern crate rocket;

#[derive(Serialize)]
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

fn submit_game_results(results: Game) -> Json<Game> {
    let mut saved_results = results;
    saved_results.id = uuid::Uuid::new_v4();
    
    Json(saved_results)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check])
        .mount("/health", routes![health_check])
}