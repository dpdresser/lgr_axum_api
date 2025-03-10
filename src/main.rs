use axum::{
    routing::{delete, get, post},
    Router,
};
// use std::net::SocketAddr;

mod handlers;
mod models;
use handlers::*;

#[tokio::main]
async fn main() {
    // TODO: initialize pretty_env_logger
    // TODO: initialize dotenvy

    // Create new PgPoolOptions instance with a maximum of 5 connections
    // Use dotenv to get the databse url
    // Use unwrap / expect instead of handling errors. If an error occurs
    // at this stage the server should be terminated
    // let pool = todo!();

    // Using slqx, exeute a SQL query that selects all questions from the
    // questions table, use unwrap / expect
    // let recs = todo!();

    info!("********* Question Records *********");
    // TODO: log recs with debug formation using the info! macro

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
