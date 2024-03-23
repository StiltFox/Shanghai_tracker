use rocket::launch;

#[macro_use] extern crate rocket;

pub mod game_endpoint;
pub mod health_endpoint;
use health_endpoint::health_check;
use game_endpoint::{submit_game_results,retrieve_all_games,retrieve_games_in_range};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("shanghai_game")]
pub struct GameDbMysql(sqlx::MySqlPool);

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_check])
        .mount("/health", routes![health_check]).attach(GameDbMysql::init())
        .mount("/game", routes![submit_game_results, retrieve_all_games, retrieve_games_in_range])
}