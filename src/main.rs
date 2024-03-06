use rocket::launch;

#[macro_use] extern crate rocket;

pub mod game_endpoint;
pub mod health_endpoint;
use health_endpoint::health_check;
use game_endpoint::{submit_game_results,get_all_games};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("shanghai_game")]
pub struct GameDbMysql(sqlx::MySqlPool);

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check, submit_game_results, get_all_games])
        .mount("/health", routes![health_check]).attach(GameDbMysql::init())
}