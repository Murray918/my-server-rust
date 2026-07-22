use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::query_as;
use std::sync::Arc;
use uuid::Uuid;

use crate::{AppState, modles::GameModel, schema::GameSchema};

pub async fn create_game_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<GameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4();
    let game = query_as!(
        GameModel,
        r#"INSERT INTO games (id, name, creator, plays) VALUES ($1, $2, $3, $4) RETURNING *"#,
        &id,
        &body.name,
        &body.creator,
        &body.plays
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    if let Err(err) = game {
        if err.to_string().contains("duplicate key value") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Game already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let game_response = json!({
            "status": "success",
            "data": json!({
                "game": game
        })
    });

    Ok(Json(game_response))
}

pub async fn game_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let games = sqlx::query_as!(GameModel, r#"SELECT * FROM games ORDER by name"#)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message" : format!("Database error: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let json_response = serde_json::json!({
        "status": "ok",
        "count" : games.len(),
        "notes": games
    });

    Ok(Json(json_response))
}

pub async fn delete_game(
    Path(game_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        GameModel,
        r#"DELETE FROM games WHERE id = $1 RETURNING *"#,
        &game_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Game not found"
            })),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            })),
        ),
    })?;

    let response = json!({
        "status": "success",
        "message": "Game delete successfully",
        "data": {
            "deleted_game" : query_result
        }
    });

    Ok(Json(response))
}

// pub async fn update_game_by_id(
//     Path(game_id): Path<Uuid>,
//     State(data): State<Arc<AppState>>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
// }
