use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use sqlx::query_as;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    AppState,
    models::game::GameModel,
    schemas::game::{GameSchema, UpdateGameSchema},
};

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

pub async fn update_game_by_id(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateGameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(GameModel, r#"SELECT * FROM games WHERE id = $1"#, &id)
        .fetch_one(&data.db)
        .await;

    let game = match query_result {
        Ok(game) => game,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Game with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}",e)
                })),
            ));
        }
    };

    let new_name = body.name.as_ref().unwrap_or(&game.name);
    let new_creator = body.creator.as_ref().unwrap_or(&game.creator);
    let new_plays = body.plays.unwrap_or(game.plays);

    let updated_game = sqlx::query_as!(
        GameModel,
        r#"UPDATE games SET name = $1, creator = $2, plays = $3 WHERE id = $4 RETURNING *"#,
        &new_name,
        &new_creator,
        &new_plays,
        &id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            })),
        )
    })?;

    let response = json!({
        "status": "success",
        "data": json!({
            "player": updated_game
        })
    });
    Ok(Json(response))
}
