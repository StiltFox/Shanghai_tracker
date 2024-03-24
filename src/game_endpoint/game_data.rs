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
    value: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Summary {
    pub highest_win_ratio: GameStat,
    pub lowest_win_ratio: GameStat,
    pub most_wins: GameStat,
    pub least_wins: GameStat,
    pub highest_score: GameStat,
    pub lowest_score: GameStat,
    pub most_games_played: GameStat,
    pub least_games_played: GameStat,
}

pub async fn get_summery_from_time_range(mut db: Connection<GameDbMysql>) -> Result<Summary, DatabaseError> {
    let mut output = Summary {
        highest_win_ratio : GameStat {person: "Unknown".to_string(), value: 0},
        lowest_win_ratio: GameStat {person: "Unknown".to_string(), value: 0},
        highest_score: GameStat {person: "Unknown".to_string(), value: 0},
        lowest_score: GameStat {person: "Unknown".to_string(), value: 0},
        most_wins: GameStat {person: "Unknown".to_string(), value: 0},
        least_wins: GameStat {person: "Unknown".to_string(), value: 0},
        most_games_played: GameStat {person: "Unknown".to_string(), value: 0},
        least_games_played: GameStat {person: "Unknown".to_string(), value: 0},
    };

    let player_data = sqlx::query("select * from player_stats").fetch_all(&mut **db).await?;

    for player in player_data {
        let name = player.try_get::<String, &str>("player")?;
        let wins = player.try_get::<u64, &str>("wins")? as u32;
        let total_games_played = player.try_get::<u64, &str>("total_games_played")? as u32;
        let win_ratio = player.try_get::<u64, &str>("win_ratio")? as u32;
        let lowest_score = player.try_get::<u32, &str>("lowest_score")?;
        let highest_score = player.try_get::<u32, &str>("highest_score")?;

        if output.most_wins.person == "Unknown".to_string() || output.most_wins.value < wins {
            output.most_wins.person = name.clone();
            output.most_wins.value = wins.clone();
        }
        if output.least_wins.person == "Unknown".to_string() || output.least_wins.value > wins {
            output.least_wins.person = name.clone();
            output.most_wins.value = wins.clone();
        }
        if output.least_games_played.person == "Unknown".to_string() || output.least_games_played.value > total_games_played {
            output.least_games_played.person = name.clone();
            output.least_games_played.value = total_games_played.clone();
        }
        if output.most_games_played.person == "Unknown".to_string() || output.most_games_played.value < total_games_played {
            output.most_games_played.person = name.clone();
            output.most_games_played.value = total_games_played.clone();
        }
        if output.lowest_score.person == "Unknown".to_string() || output.lowest_score.value > lowest_score {
            output.lowest_score.person = name.clone();
            output.lowest_score.value = lowest_score.clone();
        }
        if output.highest_score.person == "Unknown".to_string() || output.highest_score.value < highest_score {
            output.highest_score.person = name.clone();
            output.highest_score.value = highest_score.clone();
        }
        if output.highest_win_ratio.person == "Unknown".to_string() || output.highest_win_ratio.value < win_ratio {
            output.highest_win_ratio.person = name.clone();
            output.highest_win_ratio.value = win_ratio.clone();
        }
        if output.lowest_win_ratio.person == "Unknown".to_string() || output.lowest_win_ratio.value > win_ratio {
            output.lowest_win_ratio.person = name.clone();
            output.lowest_win_ratio.value = win_ratio.clone();
        }
    }

    Ok(output)
}

pub async fn submit(mut db: Connection<GameDbMysql>, result: Game) -> Result<(), DatabaseError> {
    let id = result.id.unwrap_or(Uuid::new_v4());

    let mut transaction = db.begin().await?;
    sqlx::query("insert into game (id, hosted) values (?,?);").bind(&id).bind(&result.hosted).execute(transaction.as_mut()).await?;
    for player in result.players {
        sqlx::query("insert into scores (game_id, player, final_score) values (?,?,?);").bind(&id).bind(player.0).bind(player.1).execute(transaction.as_mut()).await?;
    }
    transaction.commit().await?;
    Ok(())
}

async fn fetch_players(db: &mut MySqlConnection, id: &Uuid) -> Result<HashMap<String, u32>, DatabaseError> {
    let mut output: HashMap<String, u32> = HashMap::new();
    let players = sqlx::query("select * from scores where game_id = ?").bind(id).fetch_all(db).await?;

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
    let game_id_list = sqlx::query("select id, hosted from game;").fetch_all(&mut **db).await?;

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
        start.and_then(|date| -> Option<String> {
            Some(format!("hosted >= \"{}\"", date.to_string()))
        }).unwrap_or("1=1".to_string()),
        end.and_then(|date| -> Option<String> {
            Some(format!("hosted <= \"{}\"", date.to_string()))
        }).unwrap_or("1=1".to_string())
    )
}

pub async fn get_all_games_in_range(mut db: Connection<GameDbMysql>, start: Option<String>, end: Option<String>) -> Result<Vec<Game>, DatabaseError> {
    let start_date = extract_date(start);
    let end_date = extract_date(end);
    let query_string = get_game_date_range_query(&start_date, &end_date);
    let game_id_list = sqlx::query(&query_string).fetch_all(&mut **db).await?;

    Ok(get_players_for_all_games(db, game_id_list).await?)
}
