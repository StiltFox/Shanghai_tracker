use super::database_error::DatabaseError;
use crate::GameDbMysql;
use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::{
    sqlx::{self, Acquire, MySqlConnection, Row},
    Connection,
};
use sqlx::mysql::MySqlRow;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub hosted: NaiveDate,
    pub id: Option<Uuid>,
    pub players: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GameStat {
    person: String,
    value: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Summary {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub highest_win_ratio: GameStat,
    pub lowest_win_ratio: GameStat,
    pub most_wins: GameStat,
    pub least_wins: GameStat,
    pub highest_score: GameStat,
    pub lowest_score: GameStat,
    pub most_games_played: GameStat,
    pub least_games_played: GameStat,
}

pub async fn get_highest_score_in_date_range() -> GameStat {
    todo!()
}

pub async fn submit(mut db: Connection<GameDbMysql>, result: Game) -> Result<(), DatabaseError> {
    let id = result.id.unwrap_or(Uuid::new_v4());

    let mut transaction = db.begin().await?;
    sqlx::query("insert into game (id, hosted) values (?,?);")
        .bind(&id)
        .bind(&result.hosted)
        .execute(transaction.as_mut())
        .await?;
    for player in result.players {
        sqlx::query("insert into scores (game_id, player, final_score) values (?,?,?);")
            .bind(&id)
            .bind(player.0)
            .bind(player.1)
            .execute(transaction.as_mut())
            .await?;
    }
    transaction.commit().await?;
    Ok(())
}

async fn fetch_players(db: &mut MySqlConnection, id: &Uuid,) -> Result<HashMap<String, u32>, DatabaseError> {
    let mut output: HashMap<String, u32> = HashMap::new();
    let players = sqlx::query("select * from scores where game_id = ?")
        .bind(id)
        .fetch_all(db)
        .await?;

    for player in players {
        output.insert(
            player.try_get::<String, &str>("player")?,
            player.try_get::<u32, &str>("final_score")?,
        );
    }

    Ok(output)
}

async fn get_players_for_all_games(mut db: Connection<GameDbMysql>, game_list: Vec<MySqlRow>) -> Result<Vec<Game>, DatabaseError> {
    let mut output: Vec<Game> = vec![];

    for row in game_list {
        let id = row.try_get::<Uuid, &str>("id")?;
        output.push(Game {
            hosted: row.try_get::<NaiveDate, &str>("hosted")?,
            id: Some(id),
            players: fetch_players(db.as_mut(), &id).await?,
        });
    }
    Ok(output)
}

pub async fn get_all_games(mut db: Connection<GameDbMysql>) -> Result<Vec<Game>, DatabaseError> {
    let game_id_list = sqlx::query("select id, hosted from game;")
        .fetch_all(&mut **db)
        .await?;

    Ok(get_players_for_all_games(db, game_id_list).await?)
}

pub fn extract_date(value: Option<String>) -> Option<NaiveDate> {
    match NaiveDate::parse_from_str(value.unwrap_or_default().as_str(), "%Y-%m-%d") {
        Ok(date) => Some(date),
        Err(_) => None,
    }
}

pub fn get_game_date_range_query(start: &Option<NaiveDate>, end: &Option<NaiveDate>) -> String {
    format!(
        "select id, hosted from game where {} and {}",
        start
            .and_then(|date| -> Option<String> {
                Some(format!("hosted >= \"{}\"", date.to_string()))
            })
            .unwrap_or("1=1".to_string()),
        end.and_then(|date| -> Option<String> {
            Some(format!("hosted <= \"{}\"", date.to_string()))
        })
            .unwrap_or("1=1".to_string())
    )
}

pub async fn get_all_games_in_range(mut db: Connection<GameDbMysql>, start: Option<String>, end: Option<String>) -> Result<Vec<Game>, DatabaseError> {
    let start_date = extract_date(start);
    let end_date = extract_date(end);
    let query_string = get_game_date_range_query(&start_date, &end_date);
    let game_id_list = sqlx::query(&query_string).fetch_all(&mut **db).await?;

    Ok(get_players_for_all_games(db, game_id_list).await?)
}
