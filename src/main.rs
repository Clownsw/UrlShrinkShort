mod controller;
mod dao;
mod pojo;
mod util;

use log::info;
use sqlx::mysql::MySqlPool;
use sqlx::Pool;
use tide::Server;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

use tide_fluent_routes::prelude::*;

use controller::url_controller::{create_url, redirect_target};

lazy_static! {
    pub static ref LOCAL_URL: Mutex<String> = Mutex::new(String::new());
}

#[derive(Debug, Clone)]
pub struct State {
    pub db_pool: MySqlPool,
}

async fn make_db_pool() -> MySqlPool {
    let database_url = std::env::var("DATABASE_URL").unwrap();

    info!("DATABASE_URL = {}", database_url);

    Pool::connect(&database_url).await.unwrap()
}

async fn start() -> tide::Result<Server<State>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let mut local_url_ref = LOCAL_URL.lock().unwrap();
    *local_url_ref = std::env::var("LOCAL_URL").unwrap();

    info!("LOCAL_URL = {}", *local_url_ref);

    let db_pool = make_db_pool().await;

    Ok(Server::with_state(State { db_pool }))
}

#[async_std::main]
async fn main() {
    let mut app = start().await.unwrap();

    app.register(
        root()
            .at("/api/create", |route| route.post(create_url))
            .at("/t/:target", |route| route.get(redirect_target)),
    );

    app.listen("127.0.0.1:8888").await.unwrap();
}
