use crate::{
    AppState,
    handlers::game::{create_game_handler, delete_game, game_list_handler, update_game_by_id},
};
use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/game", post(create_game_handler))
        .route("/api/game", get(game_list_handler))
        .route("/api/game/{id}", delete(delete_game))
        .route("/api/game/{id}", patch(update_game_by_id))
        .with_state(app_state)
}
