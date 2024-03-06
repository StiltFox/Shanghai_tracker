use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::{
    sqlx::{self, Acquire},
    Connection,
};
use std::collections::HashMap;
use uuid::Uuid;
use rocket_db_pools::sqlx::{Row, Column};

use crate::GameDbMysql;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    month: u8,
    day: u8,
    year: u32,
    id: Option<Uuid>,
    players: HashMap<String, u32>,
}

async fn submit(mut db: Connection<GameDbMysql>, result: Game) -> Result<(), sqlx::Error> {
    let id = result.id.unwrap_or(Uuid::new_v4()).as_simple().to_string();
    let game_insert_statement = format!(
        "insert into game (id, hosted) values (x'{}','{}-{}-{}');",
        &id, result.year, result.month, result.day
    );

    match db.begin().await {
        Ok(mut transaction) => {
            sqlx::query(&game_insert_statement)
                .execute(transaction.as_mut())
                .await?;
            for player in result.players {
                sqlx::query(
                    format!(
                        "insert into scores (game_id, player, final_score) values (x'{}',?,{});",
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
        Err(error) => Err(error),
    }
}

#[post("/submit", data = "<results>")]
pub async fn submit_game_results(
    db: Connection<GameDbMysql>,
    results: Json<Game>,
) -> Result<(), String> {
    match submit(db, results.into_inner()).await {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

#[get("/all")]
pub async fn get_all_games(mut db: Connection<GameDbMysql>) -> Result<Json<Vec<Game>>, String> {
    match sqlx::query("select * from game;").fetch_all(&mut **db).await {
        Ok(game_id_list) => {
            let mut output: Vec<Game> = vec![];

            for row in game_id_list {
                let id = row.get::<&str, &str>("id");
                match sqlx::query(format!("select * from player where id = x'{}'", id).as_str())
                    .fetch_all(&mut **db).await {
                        Ok(scores) => {
                            let retrieved_game = Game {
                                id: Some(uuid::Uuid::parse_str(id)),
                                month: todo!(),
                                day: todo!(), year: todo!(), players: todo!() 
                            };
                        },
                        Err(error) => Err(error.to_string())?,
                    }
            }

            Ok(Json(output))
        },
        Err(error)=> Err(error.to_string())
    }
}
