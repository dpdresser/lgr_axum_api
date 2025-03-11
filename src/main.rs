use axum::{
    Router,
    routing::{delete, get, post},
};
#[macro_use]
extern crate log;
use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

mod handlers;
use handlers::*;
mod models;
mod persistance;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().unwrap();
    info!("Connecting to PostgreSQL DB");

    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DATABASE_URL");

    info!("Connection successful");

    let questions_dao = QuestionsDaoImpl::new(db.clone());
    let answers_dao = AnswersDaoImpl::new(db);

    let app_state = AppState {
        questions_dao: Arc::new(questions_dao),
        answers_dao: Arc::new(answers_dao),
    };

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    info!("Server running at 127.0.0.1:8000");

    axum::serve(listener, app).await.unwrap();
}
