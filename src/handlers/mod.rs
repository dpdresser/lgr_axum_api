use axum::{Json, response::IntoResponse};
use serde_json::json;

use crate::models::*;

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    let question_detail = QuestionDetail {
        question_uuid: uuid::Uuid::new_v4().to_string(),
        title: question.title,
        description: question.description,
        created_at: chrono::Local::now().to_string(),
    };

    Json(json!(question_detail))
}

pub async fn read_questions() -> impl IntoResponse {
    let question_detail = QuestionDetail {
        question_uuid: uuid::Uuid::new_v4().to_string(),
        title: "Newly Created Question".to_string(),
        description: "My Description".to_string(),
        created_at: chrono::Local::now().to_string(),
    };

    Json(json!(vec![question_detail]))
}

pub async fn delete_question(Json(_question_uuid): Json<QuestionId>) {}

// ---- CRUD for Answers ----

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    let answer_detail = AnswerDetail {
        answer_uuid: uuid::Uuid::new_v4().to_string(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: chrono::Local::now().to_string(),
    };

    Json(json!(answer_detail))
}

pub async fn read_answers(Json(_question_uuid): Json<QuestionId>) -> impl IntoResponse {
    let answer_detail = AnswerDetail {
        answer_uuid: uuid::Uuid::new_v4().to_string(),
        question_uuid: _question_uuid.question_uuid,
        content: "test question".to_string(),
        created_at: chrono::Local::now().to_string(),
    };

    Json(json!(vec![answer_detail]))
}

pub async fn delete_answer(Json(_answer_id): Json<AnswerId>) -> impl IntoResponse {}
