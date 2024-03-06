use rocket::{
    serde::{json::Json, Serialize},
    Config,
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiHealth {
    pub name: String,
    pub version: String,
    pub rocket_profile: String,
    pub rocket_config: Config,
}

#[get("/")]
pub fn health_check() -> Json<ApiHealth> {
    let output = ApiHealth {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        rocket_profile: std::env::var("ROCKET_PROFILE").unwrap_or_default(),
        rocket_config: Config::figment().extract().unwrap_or_default(),
    };

    Json(output)
}
