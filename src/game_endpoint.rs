pub mod database_error;
pub mod game_data;

use database_error::DatabaseError;
use game_data::{get_all_games, submit, Game};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::game_endpoint::game_data::{get_summery_from_time_range, Summary};

use crate::GameDbMysql;

use self::game_data::get_all_games_in_range;

#[post("/submit", data = "<results>")]
pub async fn submit_game_results(
    db: Connection<GameDbMysql>,
    results: Json<Game>,
) -> Result<(), DatabaseError> {
    submit(db, results.into_inner()).await?;
    Ok(())
}

#[get("/all")]
pub async fn retrieve_all_games(
    db: Connection<GameDbMysql>,
) -> Result<Json<Vec<Game>>, DatabaseError> {
    Ok(Json(get_all_games(db).await?))
}

#[get("/?<start>&<end>")]
pub async fn retrieve_games_in_range(
    db: Connection<GameDbMysql>,
    start: Option<String>,
    end: Option<String>,
) -> Result<Json<Vec<Game>>, DatabaseError> {
    Ok(Json(get_all_games_in_range(db, start, end).await?))
}

#[get("/summary")]
pub async fn get_summary(db: Connection<GameDbMysql>) -> Result<Json<Summary>, DatabaseError> {
    Ok(Json(get_summery_from_time_range(db).await?))
}
