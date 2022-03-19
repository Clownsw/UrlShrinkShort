mod controller;

use sqlx::mysql::MySqlPool;
use sqlx::Pool;
use tide::Server;

use tide_fluent_routes::prelude::*;

use controller::hello_controller::hello;

#[derive(Debug, Clone)]
pub struct State {
    pub db_pool: MySqlPool,
}

async fn make_db_pool() -> MySqlPool {
    let database_url = std::env::var("DATABASE_URL").unwrap();

    Pool::connect(&database_url).await.unwrap()
}

async fn start() -> tide::Result<Server<State>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_pool = make_db_pool().await;

    Ok(Server::with_state(State { db_pool }))
}

#[async_std::main]
async fn main() {
    let mut app = start().await.unwrap();

    app.register(root().at("/", |route| route.get(hello)));

    app.listen("127.0.0.1:8888").await.unwrap();
}
