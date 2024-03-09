pub mod database_error;

use database_error::DatabaseError;
use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    time::{macros::format_description, Date},
};
use rocket_db_pools::sqlx::{MySqlConnection, Row};
use rocket_db_pools::{
    sqlx::{self, Acquire},
    Connection,
};
use std::{collections::HashMap, fmt::format};
use uuid::Uuid;

use crate::GameDbMysql;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    month: u8,
    day: u8,
    year: i32,
    id: Option<Uuid>,
    players: HashMap<String, i32>,
}

async fn submit(mut db: Connection<GameDbMysql>, result: Game) -> Result<(), DatabaseError> {
    let id = result.id.unwrap_or(Uuid::new_v4()).to_string();
    let game_insert_statement = format!(
        "insert into game (id, hosted) values ('{}','{}-{}-{}');",
        &id, result.year, result.month, result.day
    );

    let mut transaction = db.begin().await?;
    sqlx::query(&game_insert_statement)
        .execute(transaction.as_mut())
        .await?;
    for player in result.players {
        sqlx::query(
            format!(
                "insert into scores (game_id, player, final_score) values ('{}',?,{});",
                &id, player.1
            )
            .as_str(),
        )
        .bind(player.0)
        .execute(transaction.as_mut())
        .await?;
    }
    transaction.commit().await?;
    Ok(())
}

#[post("/submit", data = "<results>")]
pub async fn submit_game_results(
    db: Connection<GameDbMysql>,
    results: Json<Game>,
) -> Result<(), DatabaseError> {
    submit(db, results.into_inner()).await?;
    Ok(())
}

pub async fn fetch_players(
    db: &mut MySqlConnection,
    id: &str,
) -> Result<HashMap<String, i32>, DatabaseError> {
    let mut output: HashMap<String, i32> = HashMap::new();
    let players = sqlx::query(format!("select * from scores where game_id = '{}'", id).as_str())
        .fetch_all(db)
        .await?;

    for player in players {
        output.insert(
            player.try_get::<String, &str>("player")?,
            player.try_get::<i32, &str>("final_score")?,
        );
    }

    Ok(output)
}

#[get("/all")]
pub async fn get_all_games(
    mut db: Connection<GameDbMysql>,
) -> Result<Json<Vec<Game>>, DatabaseError> {
    let mut output: Vec<Game> = vec![];
    let game_id_list =
        sqlx::query("select id, DATE_FORMAT(hosted, '%Y-%m-%d') as hosted from game;")
            .fetch_all(&mut **db)
            .await?;

    for row in game_id_list {
        let id = row.try_get::<&str, &str>("id")?;
        let date = Date::parse(
            row.try_get::<&str, &str>("hosted")?,
            format_description!("[year]-[month]-[day]"),
        )?;
        let players = fetch_players(db.as_mut(), id).await?;
        output.push(Game {
            month: date.month().into(),
            day: date.day(),
            year: date.year().into(),
            id: Some(Uuid::try_parse(id)?),
            players: players,
        });
    }

    Ok(Json(output))
}
