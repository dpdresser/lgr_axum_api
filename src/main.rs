use axum::{
    Router,
    routing::{delete, get, post},
};
#[macro_use]
extern crate log;
use sqlx::postgres::PgPoolOptions;

mod handlers;
use handlers::*;
mod models;
mod persistance;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().unwrap();
    info!("Connecting to PostgreSQL DB");

    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let _db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DATABASE_URL");

    info!("Connection successful");

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

    info!("Server running at 127.0.0.1:8000");

    axum::serve(listener, app).await.unwrap();
}
