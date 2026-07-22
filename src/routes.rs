use crate::{
    AppState,
    handlers::{create_game_handler, delete_game, game_list_handler},
};
use axum::{
    Router,
    routing::{delete, get, post},
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/game", post(create_game_handler))
        .route("/api/game", get(game_list_handler))
        .route("/api/game", delete(delete_game))
        .with_state(app_state)
}
