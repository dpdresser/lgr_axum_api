use axum::{
    Router,
    routing::{delete, get, post},
};
#[macro_use]
extern crate log;
use sqlx::postgres::PgPoolOptions;
// use std::net::SocketAddr;

mod handlers;
mod models;
use handlers::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().unwrap();

    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DATABASE_URL");

    let recs = sqlx::query("SELECT * FROM questions")
        .fetch_all(&db)
        .await
        .unwrap();

    info!("********* Question Records *********");

    info!("{:?}", recs);

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
