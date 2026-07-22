use crate::{AppState, handlers::create_game_handler, handlers::game_list_handler};
use axum::{Router, routing::get, routing::post};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/game", post(create_game_handler))
        .route("/api/game", get(game_list_handler))
        .with_state(app_state)
}
