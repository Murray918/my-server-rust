use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: &'static str,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: "success",
            data,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Conflict(String),
    Database(sqlx::Error),
    Internal(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),

            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                msg,
            ),

            AppError::Database(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            ),

            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
            ),
        };

        let body = ApiResponse {
            status: "error",
            data: ErrorResponse { message },
        };

        (status, Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;