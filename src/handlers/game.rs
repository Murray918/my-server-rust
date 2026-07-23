use axum::{Json, extract::Path, extract::State};
use sqlx::query_as;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    AppState,
    models::game::GameModel,
    schemas::game::{GameSchema, UpdateGameSchema},
    types::routing_error::{ApiResponse, AppResult},
};

pub async fn create_game_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<GameSchema>,
) -> AppResult<Json<ApiResponse<GameModel>>> {
    let id = Uuid::new_v4();
    let game = query_as!(
        GameModel,
        r#"INSERT INTO games (id, name, creator, plays)
           VALUES ($1, $2, $3, $4)
           RETURNING *"#,
        &id,
        &body.name,
        &body.creator,
        &body.plays
    )
    .fetch_one(&data.db)
    .await?;

    Ok(Json(ApiResponse::success(game)))
}

pub async fn game_list_handler(
    State(data): State<Arc<AppState>>,
) -> AppResult<Json<ApiResponse<Vec<GameModel>>>> {
    let games = sqlx::query_as!(
        GameModel,
        r#"SELECT * 
        FROM games 
        ORDER by name"#
    )
        .fetch_all(&data.db)
        .await?;

    Ok(Json(ApiResponse::success(games)))
}

pub async fn delete_game(
    Path(game_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> AppResult<Json<ApiResponse<GameModel>>> {
    let query_result = sqlx::query_as!(
        GameModel,
        r#"DELETE FROM games WHERE id = $1 RETURNING *"#,
        &game_id
    )
    .fetch_one(&data.db)
    .await?;

   Ok(Json(ApiResponse::success(query_result)))
}

pub async fn update_game_by_id(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateGameSchema>,
) -> AppResult<Json<ApiResponse<GameModel>>> {
    let game = sqlx::query_as!(GameModel, r#"SELECT * FROM games WHERE id = $1"#, &id)
        .fetch_one(&data.db)
        .await?;

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
    .await?;

    Ok(Json(ApiResponse::success(updated_game)))
}
